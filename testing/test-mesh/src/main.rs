use elysian::{
    interpreter::Interpreted,
    ir::{
        ast::{Struct, POSITION_2D},
        module::{AsModule, Dispatch, Evaluate, EvaluateError, SpecializationData},
    },
    mesh::{
        dual_contour::{feature, DualContour},
        marching_squares::MarchingSquares,
        quad_tree::{Bounds, QuadCell, QuadCellType, QuadTree},
        util::Vec2,
    },
    r#static::Precompiled,
    shapes::{
        field::Point,
        modify::{ClampMode, IntoElongateAxis, IntoIsosurface},
    },
};
use image::{ImageBuffer, Rgb};
use viuer::Config;

fn main() -> Result<(), EvaluateError> {
    let module = test_shapes::test_shape()
        .module(&SpecializationData::new_2d())
        .finalize();

    let evaluator = Dispatch(vec![
        Box::new(Precompiled(&module)),
        Box::new(Interpreted(&module)),
    ]);

    let level = 5;
    let epsilon = 1.0;

    let quad_tree = QuadTree::new(
        Bounds {
            min: [-1.0, -1.0],
            max: [1.0, 1.0],
        },
        level,
    )
    .merge(&evaluator, 0.04)?
    .collapse(&evaluator)?;

    let marching_squares = quad_tree
        .marching_squares(&evaluator)?
        .into_iter()
        .map(|(_, [(_, from), (_, to)])| {
            Ok(<[Struct; 2]>::try_from([
                evaluator
                    .sample_2d(from)?
                    .set(POSITION_2D.into(), from.into()),
                evaluator.sample_2d(to)?.set(POSITION_2D.into(), to.into()),
            ])
            .unwrap())
        })
        .collect::<Result<Vec<_>, EvaluateError>>()?;

    let dual_contour = quad_tree
        .dual_contour(&evaluator, epsilon)?
        .into_iter()
        .map(|[from, to]| {
            Ok(<[Struct; 2]>::try_from([
                evaluator
                    .sample_2d(from)?
                    .set(POSITION_2D.into(), from.into()),
                evaluator.sample_2d(to)?.set(POSITION_2D.into(), to.into()),
            ])
            .unwrap())
        })
        .collect::<Result<Vec<_>, EvaluateError>>()?;

    let width = 80;
    let height = 80;
    let mut image = ImageBuffer::new(width, height);

    quad_tree.iter().for_each(
        |QuadCell {
             bounds: Bounds { min, max },
             ty,
         }| {
            let min_x = ((min.x() * 0.5 + 0.5) * width as f64).floor() as u32;
            let min_y = ((min.y() * 0.5 + 0.5) * height as f64).floor() as u32;

            let max_x = ((max.x() * 0.5 + 0.5) * width as f64).floor() as u32;
            let max_y = ((max.y() * 0.5 + 0.5) * height as f64).floor() as u32;

            let color_primary = 1.0;
            let color_secondary = 0.5;
            let p = Rgb(match ty {
                QuadCellType::Empty => [color_primary, color_secondary, color_secondary],
                QuadCellType::Full => [color_secondary, color_secondary, color_primary],
                QuadCellType::Contour => [color_secondary, color_primary, color_secondary],
            });

            imageproc::drawing::draw_filled_rect_mut(
                &mut image,
                imageproc::rect::Rect::at(min_x as i32, min_y as i32)
                    .of_size(max_x - min_x, max_y - min_y),
                p,
            );

            let p = Rgb(match ty {
                QuadCellType::Empty => [color_secondary, 0.0, 0.0],
                QuadCellType::Full => [0.0, 0.0, color_secondary],
                QuadCellType::Contour => [0.0, color_secondary, 0.0],
            });

            imageproc::drawing::draw_hollow_rect_mut(
                &mut image,
                imageproc::rect::Rect::at(min_x as i32, min_y as i32)
                    .of_size(max_x - min_x, max_y - min_y),
                p,
            );
        },
    );

    for [from, to] in marching_squares.iter() {
        let from = <[f64; 2]>::from(from.get(&POSITION_2D.into()));
        let to = <[f64; 2]>::from(to.get(&POSITION_2D.into()));

        let from_x = ((from.x() * 0.5 + 0.5) * image.width() as f64).ceil() as u32;
        let from_y = ((from.y() * 0.5 + 0.5) * image.height() as f64).ceil() as u32;

        let to_x = ((to.x() * 0.5 + 0.5) * image.width() as f64).ceil() as u32;
        let to_y = ((to.y() * 0.5 + 0.5) * image.height() as f64).ceil() as u32;

        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (from_x as f32 - 0.5, from_y as f32 - 0.5),
            (to_x as f32 - 0.5, to_y as f32 - 0.5),
            Rgb([0.0, 1.0, 1.0]),
        );
    }

    /*
    for [from, to] in dual_contour.iter() {
        let from = <[f64; 2]>::from(from.get(&POSITION_2D.into()));
        let to = <[f64; 2]>::from(to.get(&POSITION_2D.into()));

        let from_x = ((from.x() * 0.5 + 0.5) * image.width() as f64).ceil() as u32;
        let from_y = ((from.y() * 0.5 + 0.5) * image.height() as f64).ceil() as u32;

        let to_x = ((to.x() * 0.5 + 0.5) * image.width() as f64).ceil() as u32;
        let to_y = ((to.y() * 0.5 + 0.5) * image.height() as f64).ceil() as u32;

        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (from_x as f32 - 0.5, from_y as f32 - 0.5),
            (to_x as f32 - 0.5, to_y as f32 - 0.5),
            Rgb([1.0, 0.0, 1.0]),
        );
    }

    quad_tree.iter().for_each(|cell| {
        if let Ok(Some((sides, feature))) = feature(&evaluator, cell.bounds, epsilon) {
            let x = ((feature.x() * 0.5 + 0.5) * width as f64).floor() as u32;
            let y = ((feature.y() * 0.5 + 0.5) * height as f64).floor() as u32;

            imageproc::drawing::draw_cross_mut(&mut image, Rgb([1.0; 3]), x as i32, y as i32)
        }
    });
    */

    viuer::print(
        &image.into(),
        &Config {
            transparent: false,
            absolute_offset: false,
            x: 0,
            y: 0,
            restore_cursor: false,
            width: None,
            height: None,
            truecolor: true,
            use_kitty: false,
            use_iterm: true,
        },
    )
    .unwrap();

    Ok(())
}
