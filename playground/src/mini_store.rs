use std::{
    any::Any,
    any::TypeId,
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    collections::{BTreeMap, HashMap},
    convert::TryInto,
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
};

enum AssocMap<K, V> {
    HashMap(HashMap<K, V>),
    BTreeMap(BTreeMap<K, V>),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum FieldType<T> {
    Some,
    None,
    Immutable,
    Mutable,
    MaybeImmutable,
    MaybeMutable,
    #[non_exhaustive]
    Phantom(PhantomData<T>),
}

#[derive(Debug)]
enum Field<'a, T> {
    Some,
    None,
    Immutable(Ref<'a, T>),
    Mutable(RefMut<'a, T>),
    MaybeImmutable(Option<Ref<'a, T>>),
    MaybeMutable(Option<RefMut<'a, T>>),
}

#[derive(Default)]
struct MiniStore<K> {
    storage: HashMap<TypeId, RefCell<Box<dyn Any>>>,
    _phantom: PhantomData<K>,
}

impl<K> MiniStore<K>
where
    K: Ord + Hash + 'static,
{
    fn create_storage_for<T>() -> Box<dyn Any>
    where
        T: 'static,
    {
        Box::new(AssocMap::<K, T>::BTreeMap(BTreeMap::new()))
    }

    fn set<T>(&mut self, key: K, value: T)
    where
        T: 'static,
    {
        let mut storage = self
            .storage
            .entry(TypeId::of::<T>())
            .or_insert_with(|| RefCell::new(Self::create_storage_for::<T>()))
            .borrow_mut();

        let storage = storage.downcast_mut::<AssocMap<K, T>>().unwrap();

        match storage {
            AssocMap::HashMap(hash_map) => hash_map.insert(key, value),
            AssocMap::BTreeMap(btree_map) => btree_map.insert(key, value),
        };
    }

    fn get<'a, T>(&'a self, key: &K) -> Option<Ref<'a, T>>
    where
        T: 'static,
    {
        let storage = self.storage.get(&TypeId::of::<T>()).unwrap();
        let storage_ref = storage.borrow();
        let storage = storage_ref.downcast_ref::<AssocMap<K, T>>().unwrap();
        match &*storage {
            AssocMap::HashMap(hash_map) => {
                if hash_map.contains_key(key) {
                    Some(Ref::map(storage_ref, move |storage| {
                        let storage = storage.downcast_ref::<AssocMap<K, T>>().unwrap();
                        if let AssocMap::HashMap(hash_map) = storage {
                            hash_map.get(key).unwrap()
                        } else {
                            panic!();
                        }
                    }))
                } else {
                    None
                }
            }
            AssocMap::BTreeMap(btree_map) => {
                if btree_map.contains_key(key) {
                    Some(Ref::map(storage_ref, move |storage| {
                        let storage = storage.downcast_ref::<AssocMap<K, T>>().unwrap();
                        if let AssocMap::BTreeMap(btree_map) = storage {
                            btree_map.get(key).unwrap()
                        } else {
                            panic!();
                        }
                    }))
                } else {
                    None
                }
            }
        }
    }

    fn get_mut<'a, T>(&'a self, key: &K) -> Option<RefMut<'a, T>>
    where
        T: 'static,
    {
        let storage = self.storage.get(&TypeId::of::<T>()).unwrap();
        let mut storage_ref = storage.borrow_mut();
        let storage = storage_ref.downcast_mut::<AssocMap<K, T>>().unwrap();
        match &*storage {
            AssocMap::HashMap(hash_map) => {
                if hash_map.contains_key(key) {
                    Some(RefMut::map(storage_ref, move |storage| {
                        let storage = storage.downcast_mut::<AssocMap<K, T>>().unwrap();
                        if let AssocMap::HashMap(hash_map) = storage {
                            hash_map.get_mut(key).unwrap()
                        } else {
                            panic!();
                        }
                    }))
                } else {
                    None
                }
            }
            AssocMap::BTreeMap(btree_map) => {
                if btree_map.contains_key(key) {
                    Some(RefMut::map(storage_ref, move |storage| {
                        let storage = storage.downcast_mut::<AssocMap<K, T>>().unwrap();
                        if let AssocMap::BTreeMap(btree_map) = storage {
                            btree_map.get_mut(key).unwrap()
                        } else {
                            panic!();
                        }
                    }))
                } else {
                    None
                }
            }
        }
    }

    fn query_field<'a, T>(&'a self, field_type: FieldType<T>, key: &K) -> Field<'a, T>
    where
        T: 'static,
    {
        match field_type {
            FieldType::Some => Field::Some,
            FieldType::None => Field::None,
            FieldType::Immutable => Field::Immutable(
                self.get::<T>(key)
                    .unwrap_or_else(|| panic!("No field of type {:?}", std::any::type_name::<T>())),
            ),
            FieldType::Mutable => Field::Mutable(
                self.get_mut::<T>(key)
                    .unwrap_or_else(|| panic!("No field of type {:?}", std::any::type_name::<T>())),
            ),
            FieldType::MaybeImmutable => Field::MaybeImmutable(self.get::<T>(key)),
            FieldType::MaybeMutable => Field::MaybeMutable(self.get_mut::<T>(key)),
            FieldType::Phantom(_) => panic!(),
        }
    }
}

trait MiniStoreQuery<'a, Key, Signature> {
    type GetOutput;

    fn get(&'a self, signature: Signature, key: &Key) -> Self::GetOutput;
}

impl<'a, K, T0> MiniStoreQuery<'a, K, (FieldType<T0>,)> for MiniStore<K>
where
    K: Ord + Hash + 'static,
    T0: Debug + 'static,
{
    type GetOutput = (Field<'a, T0>,);

    fn get(&'a self, sig: (FieldType<T0>,), key: &K) -> Self::GetOutput {
        let t0: Field<T0> = self.query_field::<T0>(sig.0, key);
        (t0,)
    }
}

impl<'a, K, T0, T1> MiniStoreQuery<'a, K, (FieldType<T0>, FieldType<T1>)> for MiniStore<K>
where
    K: Ord + Hash + 'static,
    T0: Debug + 'static,
    T1: Debug + 'static,
{
    type GetOutput = (Field<'a, T0>, Field<'a, T1>);

    fn get(&'a self, sig: (FieldType<T0>, FieldType<T1>), key: &K) -> Self::GetOutput {
        let t0: Field<T0> = self.query_field::<T0>(sig.0, key);
        let t1: Field<T1> = self.query_field::<T1>(sig.1, key);
        (t0, t1)
    }
}

impl<'a, K, T0, T1, T2> MiniStoreQuery<'a, K, (FieldType<T0>, FieldType<T1>, FieldType<T2>)>
    for MiniStore<K>
where
    K: Ord + Hash + 'static,
    T0: Debug + 'static,
    T1: Debug + 'static,
    T2: Debug + 'static,
{
    type GetOutput = (Field<'a, T0>, Field<'a, T1>, Field<'a, T2>);

    fn get(
        &'a self,
        sig: (FieldType<T0>, FieldType<T1>, FieldType<T2>),
        key: &K,
    ) -> Self::GetOutput {
        let t0: Field<T0> = self.query_field::<T0>(sig.0, key);
        let t1: Field<T1> = self.query_field::<T1>(sig.1, key);
        let t2: Field<T2> = self.query_field::<T2>(sig.2, key);
        (t0, t1, t2)
    }
}

impl<'a, K, T0, T1, T2, T3>
    MiniStoreQuery<'a, K, (FieldType<T0>, FieldType<T1>, FieldType<T2>, FieldType<T3>)>
    for MiniStore<K>
where
    K: Ord + Hash + 'static,
    T0: Debug + 'static,
    T1: Debug + 'static,
    T2: Debug + 'static,
    T3: Debug + 'static,
{
    type GetOutput = (Field<'a, T0>, Field<'a, T1>, Field<'a, T2>, Field<'a, T3>);

    fn get(
        &'a self,
        sig: (FieldType<T0>, FieldType<T1>, FieldType<T2>, FieldType<T3>),
        key: &K,
    ) -> Self::GetOutput {
        let t0: Field<T0> = self.query_field::<T0>(sig.0, key);
        let t1: Field<T1> = self.query_field::<T1>(sig.1, key);
        let t2: Field<T2> = self.query_field::<T2>(sig.2, key);
        let t3: Field<T3> = self.query_field::<T3>(sig.3, key);
        (t0, t1, t2, t3)
    }
}

fn main() {
    let mut mini_store = MiniStore::<i32>::default();

    mini_store.set(0, true);
    mini_store.set(1, false);
    mini_store.set(2, false);
    mini_store.set(3, false);

    mini_store.set(0, "Foo");
    mini_store.set(1, "Bar");
    mini_store.set(2, "Baz");

    mini_store.set(0, 1234);
    mini_store.set(1, 5678);

    mini_store.set(0, "Hmm".to_string());

    mini_store.get::<bool>(&0);
    mini_store.get::<bool>(&1);
    mini_store.get::<bool>(&2);
    mini_store.get::<bool>(&3);

    if let (Field::MaybeImmutable(maybe_string),) =
        MiniStoreQuery::get(&mini_store, (FieldType::<String>::MaybeImmutable,), &4)
    {
        println!("Arity 1 result: {:?}", maybe_string);
    };

    if let (Field::Immutable(string), Field::Mutable(int)) = MiniStoreQuery::get(
        &mini_store,
        (FieldType::<String>::Immutable, FieldType::<i32>::Mutable),
        &0,
    ) {
        println!("Arity 2 result: {:?}, {:?}", string, int);
    };
}
