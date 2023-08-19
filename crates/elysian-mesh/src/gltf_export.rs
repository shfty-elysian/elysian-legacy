use std::{collections::BTreeMap, mem};

use base64::Engine;

use elysian_core::{number::Number, property_identifier::PropertyIdentifier};
use elysian_ir::ast::{
    Struct, COLOR, NORMAL, POSITION_2D, POSITION_3D, TANGENT_2D, TANGENT_3D, UV, VECTOR2, VECTOR3,
    VECTOR4, W, X, Y, Z,
};
use gltf_json::{
    accessor::{ComponentType, GenericComponentType, Type},
    buffer::{Target, View},
    mesh::{Mode, Primitive, Semantic},
    validation::Checked,
    Accessor, Buffer, Index, Mesh, Node, Root, Scene,
};

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

fn to_data_uri<T>(buf: impl IntoIterator<Item = T>) -> String {
    let buf = to_padded_byte_vector(buf.into_iter().collect());
    format!(
        "data:model/gltf-binary;base64,{}",
        base64::engine::general_purpose::STANDARD_NO_PAD.encode(buf)
    )
}

fn buffer<T>(
    index: u32,
    values: Vec<T>,
    component_ty: ComponentType,
    ty: Type,
) -> (Buffer, View, Accessor)
where
    T: std::fmt::Debug,
{
    let vertex_size = mem::size_of::<T>();
    let vertex_count = values.len();
    let byte_length = (vertex_size * vertex_count) as u32;

    let buffer = Buffer {
        byte_length,
        extensions: Default::default(),
        extras: Default::default(),
        uri: Some(to_data_uri(values)),
    };

    let buffer_view = View {
        buffer: Index::new(index),
        byte_length: buffer.byte_length,
        byte_offset: None,
        byte_stride: Some(vertex_size as u32),
        extensions: Default::default(),
        extras: Default::default(),
        target: Some(Checked::Valid(Target::ArrayBuffer)),
    };

    let accessor = Accessor {
        buffer_view: Some(Index::new(index)),
        byte_offset: 0,
        count: vertex_count as u32,
        component_type: Checked::Valid(GenericComponentType(component_ty)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Checked::Valid(ty),
        min: None,
        max: None,
        normalized: false,
        sparse: None,
    };

    (buffer, buffer_view, accessor)
}

fn float_buffer(
    index: u32,
    samples: &[Struct],
    attr: &PropertyIdentifier,
) -> (Buffer, View, Accessor) {
    let values = samples
        .into_iter()
        .map(|v| {
            let elysian_ir::ast::Value::Number(Number::Float(f)) = v.get(attr) else {
                panic!();
            };

            f as f32
        })
        .collect::<Vec<_>>();

    buffer(index, values, ComponentType::F32, Type::Scalar)
}

fn vec2_buffer(
    index: u32,
    samples: &[Struct],
    attr: &PropertyIdentifier,
) -> (Buffer, View, Accessor) {
    let values = samples
        .into_iter()
        .map(|v| {
            let elysian_ir::ast::Value::Struct(s) = v.get(attr) else { panic!() };
            s
        })
        .map(|v| [f32::from(v.get(&X.into())), f32::from(v.get(&Y.into()))])
        .collect::<Vec<_>>();

    buffer(index, values, ComponentType::F32, Type::Vec2)
}

fn vec3_buffer(
    index: u32,
    samples: &[Struct],
    attr: &PropertyIdentifier,
) -> (Buffer, View, Accessor) {
    let values = samples
        .into_iter()
        .map(|v| {
            let elysian_ir::ast::Value::Struct(s) = v.get(attr) else { panic!() };
            s
        })
        .map(|v| {
            if v.id == VECTOR3.into() {
                [
                    f32::from(v.get(&X.into())),
                    f32::from(v.get(&Y.into())),
                    f32::from(v.get(&Z.into())),
                ]
            } else if v.id == VECTOR2.into() {
                [
                    f32::from(v.get(&X.into())),
                    f32::from(v.get(&Y.into())),
                    0.0,
                ]
            } else {
                panic!("Invalid Struct Type")
            }
        })
        .collect::<Vec<_>>();

    buffer(index, values, ComponentType::F32, Type::Vec3)
}

fn vec4_buffer(
    index: u32,
    samples: &[Struct],
    attr: &PropertyIdentifier,
) -> (Buffer, View, Accessor) {
    let values = samples
        .into_iter()
        .map(|v| {
            let elysian_ir::ast::Value::Struct(s) = v.get(attr) else { panic!() };
            s
        })
        .map(|v| {
            [
                f32::from(v.get(&X.into())),
                f32::from(v.get(&Y.into())),
                f32::from(v.get(&Z.into())),
                f32::from(v.get(&W.into())),
            ]
        })
        .collect::<Vec<_>>();

    buffer(index, values, ComponentType::F32, Type::Vec4)
}

fn to_buffers(
    samples: &[Struct],
    attrs: &[PropertyIdentifier],
) -> Vec<(Semantic, (Buffer, View, Accessor))> {
    attrs
        .into_iter()
        .enumerate()
        .map(|(i, attr)| {
            let semantic = match &attr {
                a if ***a == POSITION_2D => Semantic::Positions,
                a if ***a == POSITION_3D => Semantic::Positions,
                a if ***a == NORMAL => Semantic::Normals,
                a if ***a == TANGENT_2D => Semantic::Tangents,
                a if ***a == TANGENT_3D => Semantic::Tangents,
                a if ***a == COLOR => Semantic::Colors(0),
                a if ***a == UV => Semantic::TexCoords(0),
                _ => unimplemented!(),
            };

            let val = samples[0].get(attr);
            let bufs = match val {
                elysian_ir::ast::Value::Number(n) => match n {
                    elysian_core::number::Number::Float(_) => float_buffer(i as u32, samples, attr),
                    _ => unimplemented!(),
                },
                elysian_ir::ast::Value::Struct(s) => match &s.id {
                    id if **id == VECTOR2 => {
                        if semantic == Semantic::Positions {
                            vec3_buffer(i as u32, samples, attr)
                        } else {
                            vec2_buffer(i as u32, samples, attr)
                        }
                    }
                    id if **id == VECTOR3 => vec3_buffer(i as u32, samples, attr),
                    id if **id == VECTOR4 => vec4_buffer(i as u32, samples, attr),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            };

            (semantic, bufs)
        })
        .collect()
}

pub fn samples_to_gltf(
    samples: impl IntoIterator<Item = Struct>,
    attrs: &[PropertyIdentifier],
    mode: Mode,
) -> Root {
    let samples: Vec<_> = samples.into_iter().collect();
    let (semantics, bufs): (Vec<_>, Vec<_>) = to_buffers(&samples, attrs).into_iter().unzip();

    let attributes: BTreeMap<_, _> = semantics
        .into_iter()
        .enumerate()
        .map(|(i, s)| (Checked::Valid(s), Index::<Accessor>::new(i as u32)))
        .collect();

    let (buffers, bufs): (Vec<_>, Vec<_>) = bufs
        .into_iter()
        .map(|(buffer, view, accessor)| (buffer, (view, accessor)))
        .unzip();

    let (buffer_views, accessors): (Vec<_>, Vec<_>) = bufs.into_iter().unzip();

    let primitive = Primitive {
        attributes,
        extensions: Default::default(),
        extras: Default::default(),
        indices: None,
        material: None,
        mode: Checked::Valid(mode),
        targets: None,
    };

    let mesh = Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        primitives: vec![primitive],
        weights: None,
    };

    let node = Node {
        camera: None,
        children: None,
        extensions: Default::default(),
        extras: Default::default(),
        matrix: None,
        mesh: Some(Index::new(0)),
        rotation: None,
        scale: None,
        translation: None,
        skin: None,
        weights: None,
    };

    Root {
        accessors,
        buffers,
        buffer_views,
        meshes: vec![mesh],
        nodes: vec![node],
        scenes: vec![Scene {
            extensions: Default::default(),
            extras: Default::default(),
            nodes: vec![Index::new(0)],
        }],
        ..Default::default()
    }
}

#[cfg(test)]
mod test {
    use elysian_interpreter::Interpreted;
    use elysian_ir::module::{AsModule, Dispatch, Evaluate, EvaluateError, SpecializationData};
    use elysian_shapes::{
        field::Point,
        modify::{ClampMode, IntoElongateAxis, IntoIsosurface, IntoManifold},
    };
    use elysian_static::Precompiled;

    use crate::{
        dual_contour::DualContour,
        quad_tree::{Bounds, QuadTree},
    };

    use super::*;

    #[test]
    fn test_gltf_export() -> Result<(), EvaluateError> {
        let module = Point
            .isosurface(0.3)
            .elongate_axis([0.1, 0.0], ClampMode::Dir, ClampMode::Dir)
            .manifold()
            .isosurface(0.1)
            .module(&SpecializationData::new_2d());

        let evaluator = Dispatch(vec![
            Box::new(Precompiled(&module)),
            Box::new(Interpreted(&module)),
        ]);

        let contours = QuadTree::new(
            Bounds {
                min: [-1.0, -1.0],
                max: [1.0, 1.0],
            },
            5,
        )
        .merge(&evaluator, 0.001)?
        .collapse(&evaluator)?
        .dual_contour(&evaluator, 5.0)?;

        let samples = contours
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

        let root = samples_to_gltf(
            samples.into_iter().flatten(),
            &[POSITION_2D.into()],
            Mode::Lines,
        );

        std::fs::write("./ser.gltf", root.to_string()?)?;

        Ok(())
    }
}
