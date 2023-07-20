use std::borrow::Cow;

use uuid::Uuid;

use crate::ir::module::SpecializationData;

use super::{Expr, TypeSpec};

/// Named unique identifier
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    name: Cow<'static, str>,
    uuid: Uuid,
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

    pub fn call<T, I: IntoIterator<Item = Expr<T>>>(&self, args: I) -> Expr<T>
    where
        T: TypeSpec,
    {
        Expr::Call {
            function: self.clone(),
            args: args.into_iter().collect(),
        }
    }

    pub fn specialize(self, spec: &SpecializationData) -> Self {
        spec.specialize_id(self)
    }
}
