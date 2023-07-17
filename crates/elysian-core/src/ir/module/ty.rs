use super::StructDefinition;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Boolean,
    Number,
    Vector,
    Struct(&'static StructDefinition),
}

impl Type {
    pub fn name(&self) -> &'static str {
        match self {
            Type::Boolean => "bool",
            Type::Number => "f32",
            Type::Vector => "Vec2",
            Type::Struct(s) => s.name(),
        }
    }

    pub fn name_unique(&self) -> String {
        match self {
            Type::Boolean | Type::Number | Type::Vector => self.name().into(),
            Type::Struct(s) => s.name_unique(),
        }
    }
}
