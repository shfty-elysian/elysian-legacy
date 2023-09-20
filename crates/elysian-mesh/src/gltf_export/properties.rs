use elysian_core::property_identifier::PropertyIdentifier;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Properties<'a>(Vec<&'a PropertyIdentifier>);

impl<'a> FromIterator<&'a PropertyIdentifier> for Properties<'a> {
fn from_iter<T: IntoIterator<Item = &'a PropertyIdentifier>>(iter: T) -> Self {
    Properties(iter.into_iter().collect())
}
}

impl<'a> IntoIterator for Properties<'a> {
type Item = &'a PropertyIdentifier;

type IntoIter = std::vec::IntoIter<Self::Item>;

fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
}
}

