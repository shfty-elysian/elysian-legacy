use super::StructIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Boolean,
    Number(NumericType),
    Struct(StructIdentifier),
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
            NumericType::UInt => "UInt",
            NumericType::SInt => "SInt",
            NumericType::Float => "Float",
        }
    }
}

impl<'a> Type {
    pub fn name(&'a self) -> &'a str {
        match self {
            Type::Boolean => "Bool",
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
