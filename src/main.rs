use std::sync::Arc;
use std::{collections::HashMap, hash::Hash};
use std::{convert::TryFrom, fmt};

pub trait Label {
    fn label(&self) -> &str;
}

pub trait EndpointName {
    fn endpoint_name() -> &'static str;
}

pub trait EndpointNameSelf {
    fn endpoint_name(&self) -> &'static str;
}

impl<T: EndpointName> EndpointNameSelf for T {
    fn endpoint_name(&self) -> &'static str {
        Self::endpoint_name()
    }
}

pub trait Id<K>
where
    K: Eq + Hash,
{
    fn id(&self) -> K;
}

pub trait ToCompositeId {
    fn composite_id(&self) -> CompositeId;
}

pub trait ErasedRecord<K>: Label + EndpointNameSelf + Id<K>
where
    K: Eq + Hash,
{
}

impl<T, K> ErasedRecord<K> for T
where
    T: Label + EndpointNameSelf + Id<K> + ToCompositeId,
    K: Eq + Hash,
{
}

fn erase<K>(x: Arc<impl ErasedRecord<K> + 'static>) -> Box<Arc<dyn ErasedRecord<K>>>
where
    K: Eq + Hash,
{
    Box::new(x)
}

#[derive(Default, PartialEq, Clone, Debug)]
struct Foo {
    a: u32,
    b: u32,
    label: String,
}

impl Label for Foo {
    fn label(&self) -> &str {
        &self.label
    }
}

impl Label for &Foo {
    fn label(&self) -> &str {
        &self.label
    }
}

impl EndpointName for Foo {
    fn endpoint_name() -> &'static str {
        "foo"
    }
}

impl ToCompositeId for Foo {
    fn composite_id(&self) -> CompositeId {
        CompositeId(self.a, self.b)
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct ArcCache {
    pub foo: HashMap<u32, Arc<Foo>>,
}

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash,
)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct CompositeId(pub u32, pub u32);

impl fmt::Display for CompositeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl From<CompositeId> for String {
    fn from(x: CompositeId) -> Self {
        format!("{}", x)
    }
}

impl TryFrom<String> for CompositeId {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let xs: Vec<_> = s.split(':').collect();

        if xs.len() != 2 {
            return Err("Could not convert to CompositeId, String did not contain 2 parts.".into());
        }

        let x = xs[0].parse::<u32>()?;
        let y = xs[1].parse::<u32>()?;

        Ok(Self(x, y))
    }
}

impl ArcCache {
    /// Given a `CompositeId`, returns an `ErasedRecord` if
    /// a matching one exists.
    pub fn get_erased_record<K>(
        &self,
        composite_id: &CompositeId,
    ) -> Option<Box<Arc<dyn ErasedRecord<K>>>>
    where
        K: Eq + Hash,
    {
        self.foo.get(&composite_id.1).cloned().map(erase)
    }
}

impl Id<u32> for Foo {
    fn id(&self) -> u32 {
        self.a
    }
}

fn main() {}