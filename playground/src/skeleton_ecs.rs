use std::{
    cell::Ref, cell::RefMut, collections::HashMap, hash::Hash, ops::Deref, ops::DerefMut,
    sync::atomic::AtomicUsize, sync::atomic::Ordering,
};

use store::{IterStoreFields, Storable, Store};

// Entity
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct EntityID(usize);

impl EntityID {
    pub fn next() -> Self {
        static ENTITY_ID: AtomicUsize = AtomicUsize::new(0);
        EntityID(ENTITY_ID.fetch_add(1, Ordering::Relaxed))
    }
}

// Components
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Storable for Position {
    type Storage = HashMap<EntityID, Self>;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Velocity {
    x: i64,
    y: i64,
}

impl Storable for Velocity {
    type Storage = HashMap<EntityID, Self>;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct EntityRef(pub EntityID);

impl Storable for EntityRef {
    type Storage = HashMap<EntityID, Self>;
}

impl Deref for EntityRef {
    type Target = EntityID;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct EntityTag;

impl Storable for EntityTag {
    type Storage = HashMap<EntityID, Self>;
}

// Systems
struct PositionIntegratorSystem {}

impl PositionIntegratorSystem {
    fn run(&self, store: &Store) {
        IterStoreFields::<EntityID, (Ref<Velocity>, RefMut<Position>)>::iter(store).for_each(
            |(_, (velocity, mut position))| {
                position.x += velocity.x;
                position.y += velocity.y;
            },
        );
    }
}

struct PrinterSystem {}

impl PrinterSystem {
    fn run(&self, store: &Store) {
        IterStoreFields::<EntityID, (Ref<Position>, Ref<Velocity>)>::iter(store).for_each(
            |(key, (position, velocity))| {
                println!(
                    "ID: {:?}\nPosition: {:#?}\nVelocity: {:#?}",
                    key, position, velocity
                );
            },
        );
    }
}

struct PositionRefSystem {}

impl PositionRefSystem {
    fn run(&self, store: &Store) {
        let (referencing_entities, referenced_entities): (Vec<EntityID>, Vec<EntityID>) =
            IterStoreFields::<EntityID, (Ref<EntityRef>,)>::iter(store)
                .map(|(referencer_key, (entity_ref,))| (referencer_key, **entity_ref))
                .unzip();

        referencing_entities
            .into_iter()
            .zip(IterStoreFields::<EntityID, (Ref<Position>,)>::iter_keys(
                store,
                referenced_entities,
            ))
            .for_each(|(referencing_entity, (referenced_key, (position,)))| {
                println!(
                    "Entity {:?} referenced entity {:?} with position {:?} ",
                    referencing_entity, referenced_key, position
                );
            });
    }
}

// Main Loop
pub fn main() {
    let mut store = Store::default();
    store.add_storage_for::<Position>();
    store.add_storage_for::<Velocity>();
    store.add_storage_for::<EntityRef>();
    store.add_storage_for::<EntityTag>();

    let entity_a = EntityID::next();
    let entity_b = EntityID::next();
    let entity_c = EntityID::next();
    let entity_d = EntityID::next();

    {
        let mut position_storage = store.get_storage::<Position>().borrow_mut();
        position_storage.insert(entity_a, Position { x: 0, y: 0 });
        position_storage.insert(entity_b, Position { x: 5, y: 5 });
        position_storage.insert(entity_c, Position { x: 10, y: 10 });
        position_storage.insert(entity_d, Position { x: 15, y: 15 });
    }

    {
        let mut velocity_storage = store.get_storage::<Velocity>().borrow_mut();
        velocity_storage.insert(entity_a, Velocity { x: 1, y: 1 });
        velocity_storage.insert(entity_b, Velocity { x: -1, y: 2 });
        velocity_storage.insert(entity_c, Velocity { x: -3, y: -3 });
        velocity_storage.insert(entity_d, Velocity { x: -5, y: -5 });
    }

    {
        let mut entity_ref_storage = store.get_storage::<EntityRef>().borrow_mut();
        entity_ref_storage.insert(entity_a, EntityRef(entity_d));
        entity_ref_storage.insert(entity_b, EntityRef(entity_a));
        entity_ref_storage.insert(entity_c, EntityRef(entity_b));
        entity_ref_storage.insert(entity_d, EntityRef(entity_c));
    }

    {
        let mut entity_tag_storage = store.get_storage::<EntityTag>().borrow_mut();
        entity_tag_storage.insert(entity_a, EntityTag);
        entity_tag_storage.insert(entity_c, EntityTag);
    }

    let position_integrator = PositionIntegratorSystem {};
    let printer = PrinterSystem {};
    let position_ref = PositionRefSystem {};
    for _i in 0..4 {
        position_integrator.run(&store);
        printer.run(&store);
        position_ref.run(&store);
    }
}
