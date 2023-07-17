use std::borrow::Cow;

use uuid::Uuid;

/// Named unique identifier
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    name: Cow<'static, str>,
    uuid: Uuid,
}

impl Identifier {
    pub const fn new(name: &'static str, uuid: u128) -> Self {
        Identifier {
            name: Cow::Borrowed(name),
            uuid: Uuid::from_u128(uuid),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name_unique(&self) -> String {
        format!("{}_{}", self.name(), self.uuid().as_simple().to_string().trim_start_matches('0'))
    }

    pub fn concat(&self, rhs: &Identifier) -> Identifier {
        Identifier {
            name: (self.name.to_string() + "_" + &rhs.name).into(),
            uuid: Uuid::from_u128(self.uuid.as_u128().wrapping_add(rhs.uuid.as_u128())),
        }
    }
}
