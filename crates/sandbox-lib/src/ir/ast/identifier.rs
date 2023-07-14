use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    name: &'static str,
    uuid: Uuid,
}

impl Identifier {
    pub const fn new(name: &'static str, uuid: u128) -> Self {
        Identifier {
            name,
            uuid: Uuid::from_u128(uuid),
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name_unique(&self) -> String {
        format!("{}_{}", self.name(), self.uuid())
    }
}

