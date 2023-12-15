use rhai::{Dynamic, ImmutableString, Map};

#[derive(Debug, Default, Clone, Hash)]
pub struct Context(Map);

impl Context {
    pub fn new(ctx: Map) -> Self {
        Context(ctx)
    }

    pub fn unwrap(self) -> Map {
        self.0
    }

    pub fn get(&mut self, key: &str) -> Result<&Dynamic, ImmutableString> {
        self.0
            .get(key)
            .ok_or_else(|| format!("Missing key: {key:}").into())
    }

    pub fn set<'a>(&mut self, key: &str, value: Dynamic) {
        self.0.insert(key.into(), value);
    }

    pub fn contains(&mut self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn append(&mut self, mut other: Map) {
        self.0.append(&mut other);
    }

    pub fn concat(mut self, other: Map) -> Self {
        self.append(other);
        self
    }
}
