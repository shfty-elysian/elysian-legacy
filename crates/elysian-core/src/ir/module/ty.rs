use std::borrow::Cow;

use super::StructDefinition;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Boolean,
    Number(NumericType),
    Struct(Cow<'static, StructDefinition>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumericType {
    UInt,
    SInt,
    Float,
}

impl NumericType {
    pub fn name(&self) -> &'static str {
        match self {
            NumericType::UInt => "u32",
            NumericType::SInt => "i32",
            NumericType::Float => "f32",
        }
    }
}

impl<'a> Type {
    pub fn name(&'a self) -> &'a str {
        match self {
            Type::Boolean => "bool",
            Type::Number(n) => n.name(),
            Type::Struct(s) => s.name(),
        }
    }

    pub fn name_unique(&self) -> String {
        match self {
            Type::Boolean | Type::Number(_) => self.name().into(),
            Type::Struct(s) => s.name_unique(),
        }
    }
}
