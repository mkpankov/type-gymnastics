use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

pub trait Label {
    fn label(&self) -> &str;
}

pub trait Id {
    type K;
    fn id(&self) -> Self::K;
}

pub trait ToCompositeId<Z> {
    fn composite_id(&self) -> CompositeId<Z>;
}

pub trait ErasedRecord: Id {
    fn erase(self) -> Arc<dyn ErasedRecord<K = <Self as Id>::K>>;
}

// impl<T> ErasedRecord for T
// where
//     T: Id<K = u32> + ToCompositeId<<Self as Id>::K> + 'static,
// {
//     fn erase(self) -> Arc<dyn ErasedRecord<K = u32>> {
//         Arc::new(self)
//     }
// }

impl<T> ErasedRecord for Arc<T>
where
    T: Id + ToCompositeId<<Self as Id>::K> + 'static,
{
    fn erase(self) -> Arc<dyn ErasedRecord<K = <T as Id>::K>> {
        Arc::new(self)
    }
}

impl<T> Id for Arc<T>
where
    T: Id,
{
    type K = <T as Id>::K;
    fn id(&self) -> <T as Id>::K {
        self.as_ref().id()
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct CompositeId<X>(pub u32, pub X);

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Foo {
    id: u32,
}

impl Id for Foo {
    type K = u32;
    fn id(&self) -> <Foo as Id>::K {
        self.id
    }
}

impl ToCompositeId<u32> for Foo {
    fn composite_id(&self) -> CompositeId<u32> {
        CompositeId(0, self.id)
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Bar {
    id: String,
}

impl Id for Bar {
    type K = String;
    fn id(&self) -> <Bar as Id>::K {
        self.id.clone()
    }
}

impl ToCompositeId<String> for Bar {
    fn composite_id(&self) -> CompositeId<String> {
        CompositeId(1, self.id.clone())
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct ArcCache {
    pub foo: HashMap<u32, Arc<Foo>>,
    pub bar: HashMap<String, Arc<Bar>>,
    pub content_type: HashMap<u32, ContentType>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct ContentType {
    model: String,
}

impl ArcCache {
    pub fn get_erased_record_u32(&self, id: (u32, u32)) -> Option<Arc<dyn ErasedRecord<K = u32>>> {
        let content_type = self.content_type.get(&id.0)?;

        match content_type.model.as_ref() {
            "foo" => self.foo.get(&id.1).cloned().map(|x| x.erase()),
            // "bar" => self.bar.get(&id.1).cloned().map(|x| x.erase()),
            _ => None,
        }
    }

    pub fn get_erased_record_string(
        &self,
        id: (u32, String),
    ) -> Option<Arc<dyn ErasedRecord<K = String>>> {
        let content_type = self.content_type.get(&id.0)?;

        match content_type.model.as_ref() {
            // "foo" => self.foo.get(&id.1).cloned().map(|x| x.erase()),
            "bar" => self.bar.get(&id.1).cloned().map(|x| x.erase()),
            _ => None,
        }
    }
}

#[derive(Default, PartialEq, Debug)]
struct AvailableAction {
    composite_id: CompositeId<u32>,
}

type ActionMap = BTreeMap<String, Vec<(Arc<AvailableAction>, Box<Arc<dyn ErasedRecord<K = u32>>>)>>;

fn group_actions_by_label(xs: Vec<Arc<AvailableAction>>, cache: &ArcCache) -> ActionMap {
    let mut x = xs.into_iter().fold(BTreeMap::new(), |mut x, action| {
        match cache.get_erased_record_u32((action.composite_id.0, action.composite_id.1)) {
            Some(r) => {
                // let xs = x.entry(r.label().to_string()).or_insert_with(|| vec![]);

                // xs.push((action, r));
            }
            None => {
                println!(
                    "Discarded action {:?} because it did not have an erased type",
                    action
                );
            }
        };

        x
    });

    x
}

fn main() {}
