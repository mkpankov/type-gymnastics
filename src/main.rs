use std::sync::Arc;
use std::{collections::HashMap, hash::Hash};
use std::{convert::TryFrom, fmt};

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

fn main() {}
