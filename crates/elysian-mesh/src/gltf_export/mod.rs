use std::mem;

use base64::Engine;

use elysian_core::{number::Number, property_identifier::PropertyIdentifier};
use elysian_ir::ast::{Struct, VECTOR2, VECTOR3, W, X, Y, Z};
use gltf_json::{
    accessor::{ComponentType, GenericComponentType, Type},
    buffer::{Target, View},
    validation::Checked,
    Accessor, Buffer, Index, Root,
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

fn index_buffer(root: &mut Root, indices: Indices) {
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

macro_rules! iterator_newtype {
    ($t:ident, $u:ident) => {
        #[derive(Debug, Default, Clone, PartialEq, PartialOrd, Hash)]
        pub struct $t(Vec<$u>);

        impl FromIterator<$u> for $t {
            fn from_iter<T: IntoIterator<Item = $u>>(iter: T) -> Self {
                Self(iter.into_iter().collect())
            }
        }

        impl IntoIterator for $t {
            type Item = $u;

            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
}

mod samples;
pub use samples::*;

mod points;
pub use points::*;

iterator_newtype!(Indices, u32);

mod properties;
pub use properties::*;

mod export_primitive;
pub use export_primitive::*;

mod export_primitives;
pub use export_primitives::*;

mod nodes;
pub use nodes::*;

mod scenes;
pub use scenes::*;

mod point_primitive;
pub use point_primitive::*;

mod point_primitives;
pub use point_primitives::*;
