use elysian::{
    interpreter::Interpreted,
    ir::module::{AsModule, Dispatch, EvaluateError, SpecializationData},
    mesh::{
        bounds::Bounds,
        subdivision_tree::{Octree, QuadTree},
    },
    r#static::Precompiled,
    shapes::{
        field::Point,
        modify::{IntoGradientNormals, IntoIsosurface},
    },
};

fn main() -> Result<(), EvaluateError> {
    // Create shape module and evaluator
    let module = Point.isosurface(0.8).gradient_normals();

    let module_2d = module.module(&SpecializationData::new_2d()).finalize();

    let evaluator_2d = Dispatch(vec![
        Box::new(Precompiled(&module_2d)),
        Box::new(Interpreted(&module_2d)),
    ]);

    let quad_tree = QuadTree::new(
        Bounds {
            min: [-1.0, -1.0].into(),
            max: [1.0, 1.0].into(),
        },
        4,
    )
    .merge(&evaluator_2d, 0.04)?
    .sample(&evaluator_2d)?
    .collapse(&evaluator_2d)?
    ;

    elysian::mesh::quad_tree_2d(&quad_tree, &evaluator_2d)?;
    elysian::mesh::dual_graph_2d(&quad_tree, &evaluator_2d)?;
    elysian::mesh::marching_squares_2d(&quad_tree, &evaluator_2d)?;
    elysian::mesh::dual_contour_2d(&quad_tree, &evaluator_2d)?;

    let module_3d = module.module(&SpecializationData::new_3d()).finalize();

    let evaluator_3d = Dispatch(vec![
        Box::new(Precompiled(&module_3d)),
        Box::new(Interpreted(&module_3d)),
    ]);

    let octree = Octree::new(
        Bounds {
            min: [-1.0, -1.0, -1.0].into(),
            max: [1.0, 1.0, 1.0].into(),
        },
        4,
    )
    .merge(&evaluator_3d, 0.04)?
    .sample(&evaluator_3d)?
    .collapse(&evaluator_3d)?;

    elysian::mesh::octree_3d(&octree, &evaluator_3d)?;
    elysian::mesh::marching_cubes_3d(&octree, &evaluator_3d)?;
    elysian::mesh::dual_graph_3d(&octree, &evaluator_3d)?;
    elysian::mesh::dual_contour_3d(&octree, &evaluator_3d)?;

    Ok(())
}
