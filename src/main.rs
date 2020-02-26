use std::sync::Arc;
use std::{collections::HashMap, hash::Hash};
use std::{convert::TryFrom, fmt};

pub trait Label {
    fn label(&self) -> &str;
}

pub trait Id<K>
where
    K: Eq + Hash,
{
    fn id(&self) -> K;
}

pub trait ToCompositeId {
    fn composite_id<K>(&self) -> CompositeId<K>
    where
        K: Eq + Hash + Clone + fmt::Display;
}

pub trait ErasedRecord<K>: Id<K>
where
    K: Eq + Hash,
{
}

impl<T, K> ErasedRecord<K> for T
where
    T: Id<K> + ToCompositeId,
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

impl ToCompositeId for Foo {
    fn composite_id<K>(&self) -> CompositeId<K>
    where
        K: Eq + Hash + Clone + fmt::Display,
    {
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
pub struct CompositeId<K>(pub K, pub K)
where
    K: Eq + Hash + Clone + fmt::Display;

impl<K> fmt::Display for CompositeId<K>
where
    K: Eq + Hash + Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl<K> From<CompositeId<K>> for String
where
    K: Eq + Hash + Clone + fmt::Display,
{
    fn from(x: CompositeId<K>) -> Self {
        format!("{}", x)
    }
}

impl<K> TryFrom<String> for CompositeId<K>
where
    K: Eq + Hash + Clone + fmt::Display,
{
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
        composite_id: &CompositeId<K>,
    ) -> Option<Box<Arc<dyn ErasedRecord<K>>>>
    where
        K: Eq + Hash + Clone + fmt::Display,
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
