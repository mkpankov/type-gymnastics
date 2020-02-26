use std::sync::Arc;
use std::{collections::HashMap, hash::Hash};
use std::{convert::TryFrom, fmt, marker::PhantomData};

pub trait Label {
    fn label(&self) -> &str;
}

pub trait Id {
    type K;
    fn id(&self) -> Self::K;
}

pub trait ToCompositeId {
    fn composite_id<K>(&self) -> CompositeId<K>
    where
        K: Eq + Hash + Clone + fmt::Display;
}

pub trait ErasedRecord: Id {
    fn erase(self) -> Box<Arc<dyn ErasedRecord<K = u32>>>;
}

impl<T> ErasedRecord for T
where
    T: Id<K = u32> + ToCompositeId + 'static,
{
    fn erase(self) -> Box<Arc<dyn ErasedRecord<K = u32>>> {
        Box::new(Arc::new(self))
    }
}

pub struct CompositeId<X>(pub X, pub X)
where
    X: Eq + Hash;

#[derive(serde::Deserialize, Debug)]
pub struct Foo {
    id: u32,
}

impl Id for Foo {
    type K = u32;
    fn id(&self) -> <Foo as Id>::K {
        self.id
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Bar {
    id: String,
}

impl Id for Bar {
    type K = String;
    fn id(&self) -> <Bar as Id>::K {
        self.id.clone()
    }
}

fn main() {}
