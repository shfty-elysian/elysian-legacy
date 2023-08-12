use std::borrow::Cow;

use uuid::Uuid;

/// Named unique identifier
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier {
    pub name: Cow<'static, str>,
    pub uuid: Uuid,
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.uuid)
    }
}

impl IntoIterator for Identifier {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Identifier {
    /// Construct a const identifier
    pub const fn new(name: &'static str, uuid: u128) -> Self {
        Identifier {
            name: Cow::Borrowed(name),
            uuid: Uuid::from_u128(uuid),
        }
    }

    /// Construct a runtime identifier
    pub fn new_dynamic(name: &'static str) -> Self {
        Identifier {
            name: Cow::Borrowed(name),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name_unique(&self) -> String {
        let uuid = self.uuid();
        if *uuid == Uuid::default() {
            self.name().into()
        } else {
            format!(
                "{}_{}",
                self.name(),
                self.uuid().as_simple().to_string().trim_start_matches('0')
            )
        }
    }

    pub fn concat(&self, rhs: &Identifier) -> Identifier {
        Identifier {
            name: (self.name.to_string() + "_" + &rhs.name).into(),
            uuid: Uuid::from_u128(self.uuid.as_u128().wrapping_add(rhs.uuid.as_u128())),
        }
    }
}
