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
    mesh::{Primitive, Semantic},
    validation::Checked,
    Accessor, Buffer, Index, Mesh, Node, Root, Scene,
};

pub use gltf_json::mesh::Mode;

use crate::util::Unzip3;

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
    format!(
        "data:model/gltf-binary;base64,{}",
        base64::engine::general_purpose::STANDARD_NO_PAD
            .encode(to_padded_byte_vector(buf.into_iter().collect()))
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

fn samples_to_buffer<'a>(
    index: usize,
    samples: &[Struct],
    attrs: impl IntoIterator<Item = &'a PropertyIdentifier>,
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
            let bundle = match val {
                elysian_ir::ast::Value::Number(n) => match n {
                    elysian_core::number::Number::Float(_) => {
                        float_buffer((index + i) as u32, samples, attr)
                    }
                    _ => unimplemented!(),
                },
                elysian_ir::ast::Value::Struct(s) => match &s.id {
                    id if **id == VECTOR2 => {
                        if semantic == Semantic::Positions {
                            vec3_buffer((index + i) as u32, samples, attr)
                        } else {
                            vec2_buffer((index + i) as u32, samples, attr)
                        }
                    }
                    id if **id == VECTOR3 => vec3_buffer((index + i) as u32, samples, attr),
                    id if **id == VECTOR4 => vec4_buffer((index + i) as u32, samples, attr),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            };

            (semantic, bundle)
        })
        .collect()
}

pub fn samples_to_primitive<'a>(
    index: usize,
    primitive: impl IntoIterator<Item = Struct>,
    attrs: impl IntoIterator<Item = &'a PropertyIdentifier>,
    mode: Mode,
) -> (Primitive, Vec<(Buffer, View, Accessor)>) {
    let samples: Vec<_> = primitive.into_iter().collect();
    let (semantics, bundle): (Vec<_>, Vec<_>) = samples_to_buffer(index, &samples, attrs)
        .into_iter()
        .unzip();

    let attributes: BTreeMap<_, _> = semantics
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            (
                Checked::Valid(s),
                Index::<Accessor>::new((index + i) as u32),
            )
        })
        .collect();

    (
        Primitive {
            attributes,
            extensions: Default::default(),
            extras: Default::default(),
            indices: None,
            material: None,
            mode: Checked::Valid(mode),
            targets: None,
        },
        bundle,
    )
}

pub fn samples_to_mesh<'a>(
    index: usize,
    primitives: impl IntoIterator<
        Item = (
            Mode,
            impl IntoIterator<Item = &'a PropertyIdentifier>,
            impl IntoIterator<Item = Struct>,
        ),
    >,
) -> (Mesh, Vec<(Buffer, View, Accessor)>) {
    let (primitives, bundles): (Vec<_>, Vec<_>) = primitives
        .into_iter()
        .enumerate()
        .map(|(i, (mode, attrs, samples))| samples_to_primitive(index + i, samples, attrs, mode))
        .unzip();

    let bundle = bundles.into_iter().flatten().collect();

    let mesh = Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        primitives,
        weights: None,
    };

    (mesh, bundle)
}

pub fn samples_to_nodes<'a>(
    index: usize,
    meshes: impl IntoIterator<
        Item = impl IntoIterator<
            Item = (
                Mode,
                impl IntoIterator<Item = &'a PropertyIdentifier>,
                impl IntoIterator<Item = Struct>,
            ),
        >,
    >,
) -> Vec<(Node, Mesh, Vec<(Buffer, View, Accessor)>)> {
    meshes
        .into_iter()
        .enumerate()
        .map(|(i, mesh)| {
            let (mesh, bundle) = samples_to_mesh(index + i, mesh);

            let node = Node {
                camera: None,
                children: None,
                extensions: Default::default(),
                extras: Default::default(),
                matrix: None,
                mesh: Some(Index::new((index + i) as u32)),
                rotation: None,
                scale: None,
                translation: None,
                skin: None,
                weights: None,
            };

            (node, mesh, bundle)
        })
        .collect()
}

pub fn samples_to_scene<'a>(
    index: usize,
    nodes: impl IntoIterator<
        Item = impl IntoIterator<
            Item = (
                Mode,
                impl IntoIterator<Item = &'a PropertyIdentifier>,
                impl IntoIterator<Item = Struct>,
            ),
        >,
    >,
) -> (Scene, Vec<(Node, Mesh, Vec<(Buffer, View, Accessor)>)>) {
    let bundle = samples_to_nodes(index, nodes);

    (
        Scene {
            extensions: Default::default(),
            extras: Default::default(),
            nodes: vec![Index::new(index as u32)],
        },
        bundle,
    )
}

pub fn samples_to_root<'a>(
    scenes: impl IntoIterator<
        Item = impl IntoIterator<
            Item = impl IntoIterator<
                Item = (
                    Mode,
                    impl IntoIterator<Item = &'a PropertyIdentifier>,
                    impl IntoIterator<Item = Struct>,
                ),
            >,
        >,
    >,
) -> Root {
    let (scenes, bundle): (Vec<_>, Vec<_>) = scenes
        .into_iter()
        .enumerate()
        .map(|(i, scene)| samples_to_scene(i, scene))
        .unzip();

    let (nodes, meshes, bundles): (Vec<_>, Vec<_>, Vec<_>) = bundle.into_iter().flatten().unzip3();
    let (buffers, buffer_views, accessors) = bundles.into_iter().flatten().unzip3();

    Root {
        accessors,
        buffers,
        buffer_views,
        meshes,
        nodes,
        scenes,
        ..Default::default()
    }
}
