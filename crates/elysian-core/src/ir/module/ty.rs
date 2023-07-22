use super::StructDefinition;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Boolean,
    Number,
    Vector2,
    Vector3,
    Vector4,
    Matrix2,
    Matrix3,
    Matrix4,
    Struct(&'static StructDefinition),
}

impl Type {
    pub fn name(&self) -> &'static str {
        match self {
            Type::Boolean => "bool",
            Type::Number => "f32",
            Type::Vector2 => "Vector2",
            Type::Vector3 => "Vector3",
            Type::Vector4 => "Vector4",
            Type::Matrix2 => "Matrix2",
            Type::Matrix3 => "Matrix3",
            Type::Matrix4 => "Matrix4",
            Type::Struct(s) => s.name(),
        }
    }

    pub fn name_unique(&self) -> String {
        match self {
            Type::Boolean
            | Type::Number
            | Type::Vector2
            | Type::Vector3
            | Type::Vector4
            | Type::Matrix2
            | Type::Matrix3
            | Type::Matrix4 => self.name().into(),
            Type::Struct(s) => s.name_unique(),
        }
    }
}
