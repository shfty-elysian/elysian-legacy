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
        base64::engine::general_purpose::STANDARD
            .encode(to_padded_byte_vector(buf.into_iter().collect()))
    )
}

fn buffer<T>(root: &mut Root, values: Vec<T>, component_ty: ComponentType, ty: Type, target: Target)
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
        buffer: Index::new(root.buffers.len() as u32),
        byte_length: buffer.byte_length,
        byte_offset: None,
        byte_stride: Some(vertex_size as u32),
        extensions: Default::default(),
        extras: Default::default(),
        target: Some(Checked::Valid(target)),
    };

    let accessor = Accessor {
        buffer_view: Some(Index::new(root.buffer_views.len() as u32)),
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

    root.buffers.push(buffer);
    root.buffer_views.push(buffer_view);
    root.accessors.push(accessor);
}

fn index_buffer(root: &mut Root, indices: impl IntoIterator<Item = u32>) {
    let indices = indices.into_iter().collect::<Vec<_>>();
    buffer(
        root,
        indices,
        ComponentType::U32,
        Type::Scalar,
        Target::ElementArrayBuffer,
    )
}

fn float_buffer(root: &mut Root, samples: &[Struct], attr: &PropertyIdentifier) {
    let values = samples
        .into_iter()
        .map(|v| {
            let elysian_ir::ast::Value::Number(Number::Float(f)) = v.get(attr) else {
                panic!();
            };

            f as f32
        })
        .collect::<Vec<_>>();

    buffer(
        root,
        values,
        ComponentType::F32,
        Type::Scalar,
        Target::ArrayBuffer,
    )
}

fn vec2_buffer(root: &mut Root, samples: &[Struct], attr: &PropertyIdentifier) {
    let values = samples
        .into_iter()
        .map(|v| {
            let elysian_ir::ast::Value::Struct(s) = v.get(attr) else { panic!() };
            s
        })
        .map(|v| [f32::from(v.get(&X.into())), f32::from(v.get(&Y.into()))])
        .collect::<Vec<_>>();

    buffer(
        root,
        values,
        ComponentType::F32,
        Type::Vec2,
        Target::ArrayBuffer,
    )
}

fn vec3_buffer(root: &mut Root, samples: &[Struct], attr: &PropertyIdentifier) {
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

    buffer(
        root,
        values,
        ComponentType::F32,
        Type::Vec3,
        Target::ArrayBuffer,
    )
}

fn vec4_buffer(root: &mut Root, samples: &[Struct], attr: &PropertyIdentifier) {
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

    buffer(
        root,
        values,
        ComponentType::F32,
        Type::Vec4,
        Target::ArrayBuffer,
    )
}

fn samples_to_buffer<'a>(
    root: &mut Root,
    samples: impl IntoIterator<Item = Struct>,
    attrs: impl IntoIterator<Item = &'a PropertyIdentifier>,
) -> BTreeMap<Checked<Semantic>, Index<Accessor>> {
    let samples: Vec<_> = samples.into_iter().collect();

    attrs
        .into_iter()
        .map(|attr| {
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
            match val {
                elysian_ir::ast::Value::Number(n) => match n {
                    elysian_core::number::Number::Float(_) => float_buffer(root, &samples, attr),
                    _ => unimplemented!(),
                },
                elysian_ir::ast::Value::Struct(s) => match &s.id {
                    id if **id == VECTOR2 => {
                        if semantic == Semantic::Positions {
                            vec3_buffer(root, &samples, attr)
                        } else {
                            vec2_buffer(root, &samples, attr)
                        }
                    }
                    id if **id == VECTOR3 => vec3_buffer(root, &samples, attr),
                    id if **id == VECTOR4 => vec4_buffer(root, &samples, attr),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            };

            (
                Checked::Valid(semantic),
                Index::new(root.accessors.len() as u32 - 1),
            )
        })
        .collect()
}

pub fn samples_to_primitive<'a>(
    root: &mut Root,
    primitive: impl IntoIterator<Item = Struct>,
    indices: Option<impl IntoIterator<Item = u32>>,
    attrs: impl IntoIterator<Item = &'a PropertyIdentifier>,
    mode: Mode,
) -> Primitive {
    let attrs: Vec<_> = attrs.into_iter().collect();

    let samples: Vec<_> = primitive.into_iter().collect();
    let attributes: BTreeMap<_, _> = samples_to_buffer(root, samples, attrs);

    let mut primitive = Primitive {
        attributes,
        extensions: Default::default(),
        extras: Default::default(),
        indices: None,
        material: None,
        mode: Checked::Valid(mode),
        targets: None,
    };

    if let Some(indices) = indices {
        index_buffer(root, indices);
        primitive.indices = Some(Index::new(root.buffers.len() as u32 - 1));
    }

    primitive
}

pub fn samples_to_mesh<'a>(
    root: &mut Root,
    primitives: impl IntoIterator<
        Item = (
            Mode,
            impl IntoIterator<Item = &'a PropertyIdentifier>,
            impl IntoIterator<Item = Struct>,
            Option<impl IntoIterator<Item = u32>>,
        ),
    >,
) {
    let primitives: Vec<_> = primitives
        .into_iter()
        .map(|(mode, attrs, samples, indices)| {
            samples_to_primitive(root, samples, indices, attrs, mode)
        })
        .collect();

    let mesh = Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        primitives,
        weights: None,
    };

    root.meshes.push(mesh);
}

pub fn samples_to_node<'a>(
    root: &mut Root,
    meshes: impl IntoIterator<
        Item = impl IntoIterator<
            Item = (
                Mode,
                impl IntoIterator<Item = &'a PropertyIdentifier>,
                impl IntoIterator<Item = Struct>,
                Option<impl IntoIterator<Item = u32>>,
            ),
        >,
    >,
) {
    meshes.into_iter().for_each(|mesh| {
        samples_to_mesh(root, mesh);

        let node = Node {
            camera: None,
            children: None,
            extensions: Default::default(),
            extras: Default::default(),
            matrix: None,
            mesh: Some(Index::new(root.meshes.len() as u32 - 1)),
            rotation: None,
            scale: None,
            translation: None,
            skin: None,
            weights: None,
        };

        root.nodes.push(node);
    })
}

pub fn samples_to_scene<'a>(
    root: &mut Root,
    nodes: impl IntoIterator<
        Item = impl IntoIterator<
            Item = (
                Mode,
                impl IntoIterator<Item = &'a PropertyIdentifier>,
                impl IntoIterator<Item = Struct>,
                Option<impl IntoIterator<Item = u32>>,
            ),
        >,
    >,
) {
    let from = root.nodes.len();
    samples_to_node(root, nodes);

    root.scenes.push(Scene {
        extensions: Default::default(),
        extras: Default::default(),
        nodes: (from..root.nodes.len())
            .into_iter()
            .map(|i| Index::new(i as u32))
            .collect(),
    })
}

pub fn samples_to_root<'a>(
    scenes: impl IntoIterator<
        Item = impl IntoIterator<
            Item = impl IntoIterator<
                Item = (
                    Mode,
                    impl IntoIterator<Item = &'a PropertyIdentifier>,
                    impl IntoIterator<Item = Struct>,
                    Option<impl IntoIterator<Item = u32>>,
                ),
            >,
        >,
    >,
) -> Root {
    let mut root = Root::default();

    scenes
        .into_iter()
        .for_each(|scene| samples_to_scene(&mut root, scene));

    root
}
