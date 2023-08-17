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
            [
                f32::from(v.get(&X.into())),
                f32::from(v.get(&Y.into())),
                f32::from(v.get(&Z.into())),
            ]
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
                    id if **id == VECTOR2 => vec2_buffer(i as u32, samples, attr),
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

pub fn samples_to_gltf(samples: &[Struct], attrs: &[PropertyIdentifier]) -> Root {
    let (semantics, bufs): (Vec<_>, Vec<_>) = to_buffers(samples, attrs).into_iter().unzip();

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
        mode: Checked::Valid(Mode::Triangles),
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
    use elysian_ir::module::{StructIdentifier, CONTEXT};

    use crate::util::Vec3;

    use super::*;

    #[test]
    fn test_gltf_export() {
        let sample = |position: [f32; 3], color: [f32; 3]| -> Struct {
            Struct::new(StructIdentifier(CONTEXT))
                .set(
                    POSITION_3D.into(),
                    elysian_ir::ast::Value::Struct(
                        Struct::new(StructIdentifier(VECTOR3))
                            .set(X.into(), position.x().into())
                            .set(Y.into(), position.y().into())
                            .set(Z.into(), position.z().into()),
                    ),
                )
                .set(
                    COLOR.into(),
                    elysian_ir::ast::Value::Struct(
                        Struct::new(StructIdentifier(VECTOR3))
                            .set(X.into(), color.x().into())
                            .set(Y.into(), color.y().into())
                            .set(Z.into(), color.z().into()),
                    ),
                )
        };

        let samples = [
            sample([0.0, 0.5, 0.0], [1.0, 0.0, 0.0]),
            sample([-0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
            sample([0.5, -0.5, 0.0], [0.0, 0.0, 1.0]),
        ];

        let root = samples_to_gltf(&samples, &[POSITION_3D.into(), COLOR.into()]);

        std::fs::write("./ser.gltf", root.to_string().unwrap()).unwrap();
    }
}
