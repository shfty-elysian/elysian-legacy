use super::StructDefinition;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Boolean,
    Number,
    Vector2,
    Vector3,
    Vector4,
    Struct(&'static StructDefinition),
}

impl Type {
    pub fn name(&self) -> &'static str {
        match self {
            Type::Boolean => "bool",
            Type::Number => "f32",
            Type::Vector2 => "Vec2",
            Type::Vector3 => "Vec3",
            Type::Vector4 => "Vec4",
            Type::Struct(s) => s.name(),
        }
    }

    pub fn name_unique(&self) -> String {
        match self {
            Type::Boolean | Type::Number | Type::Vector2 | Type::Vector3 | Type::Vector4 => {
                self.name().into()
            }
            Type::Struct(s) => s.name_unique(),
        }
    }
}
