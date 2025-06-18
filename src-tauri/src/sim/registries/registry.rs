use crate::sim::utils::term::{bold, cyan};
use dashmap::DashMap;
use std::any::type_name;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::atomic::{AtomicU32, Ordering};
use strum_macros::Display;
use tracing::warn;
use crate::sim::person::skills::SkillId;

/// A generic registry that manages items with unique IDs.
///
/// # Type Parameters
/// - `T`: The type of item stored in the registry. Typically Entity, etc.
/// - `I` : The ID field of the Entity
///
pub struct Registry<I, T>
where
    I: Eq + Hash + Copy, 
    T: Eq + Hash + Copy, 
{
    name: String,
    id_to_entity: DashMap<I, T>,
    entity_to_id: DashMap<T, I>,
    next_id: AtomicU32,
}

impl<I, T> Debug for Registry<I, T>
where
    I: Eq + Hash + Copy,
    T: Eq + Hash + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = self.id_to_entity.len();
        let next = self.next_id.load(Ordering::Relaxed);

        write!(
            f,
            "{}[{} entries. next_id={}]",
            bold(&cyan(self.name.as_str())),
            bold(&count.to_string()),
            bold(&next.to_string())
        )
    }
}

impl<I, T> Display for Registry<I, T>
where
    I: Eq + Hash + Copy,
    T: Eq + Hash + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = self.id_to_entity.len();
        let next = self.next_id.load(Ordering::Relaxed);

        write!(
            f,
            "{}[{} entries. next_id={}]",
            bold(&cyan(self.name.as_str())),
            bold(&count.to_string()),
            bold(&next.to_string())
        )
    }
}
//
impl<I, T> Registry<I, T>
where
    I: Eq + Hash + Copy,
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            name: type_name::<I>().to_string(),
            id_to_entity: DashMap::new(),
            entity_to_id: DashMap::new(),
            next_id: AtomicU32::new(1),
        }
    }

    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            id_to_entity: DashMap::new(),
            entity_to_id: DashMap::new(),
            next_id: AtomicU32::new(1),
        }
    }
    pub fn generate_id(&self) -> u32 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn insert(&self, id: I, entity: T) {
        self.id_to_entity.insert(id, entity);
        self.entity_to_id.insert(entity, id);
    }

    pub fn get_entity_from_id(&self, id: &I) -> Option<T> {
        self.id_to_entity.get(id).map(|entry| *entry.value())
    }

    pub fn get_id_from_entity(&self, entity: &T) -> Option<I> {
        self.entity_to_id.get(entity).map(|entry| *entry.value())
    }

    pub fn remove(&self, id: &I) {
        if let Some((_id, entity)) = self.id_to_entity.remove(id) {
            self.entity_to_id.remove(&entity);
        }
    }
    pub fn clear(&self) {
        self.id_to_entity.clear();
        self.entity_to_id.clear();
        // Reset next_id to 1, consistent with new() for 1-based IDs.
        self.next_id.store(1, Ordering::Relaxed);
    }
    // pub fn iter(&self) -> impl Iterator<Item = (&I, &T)> {
    //     self.id_to_entity.iter()
    // }

    /// Repopulates the registry from an iterator of (ID, entity) pairs.
    ///
    /// This method clears any existing data in the registry and then inserts all
    /// items from the provided iterator. The internal `next_id` counter is updated
    /// to be greater than the maximum ID encountered in the input data, ensuring
    /// subsequent calls to `generate_id()` produce unique IDs.
    ///
    /// # Type Parameters
    /// - `Iter`: An iterator yielding `(I, T)` tuples.
    ///
    /// # Constraints
    /// - `I` must also implement `Ord` and `Into<u32>` for this method,
    ///   to allow finding the maximum ID and updating the `next_id` counter.
    pub fn repopulate_from_entities<Iter>(&self, entities: Iter)
    where
        Iter: IntoIterator<Item = (I, T)>,
        I: Ord + Into<u32>, // Additional constraints for this specific method
    {
        self.id_to_entity.clear();
        self.entity_to_id.clear();

        let mut max_id_val: u32 = 0;

        for (id, entity) in entities {
            self.id_to_entity.insert(id, entity);
            self.entity_to_id.insert(entity, id);

            let current_id_u32: u32 = id.into();
            if current_id_u32 > max_id_val {
                max_id_val = current_id_u32;
            }
        }
        self.next_id.store(max_id_val + 1, Ordering::Relaxed);
    }

    /// Repopulates the registry from an iterator of (ID, entity) pairs without updating `next_id`.
    ///
    /// This method clears any existing data in the registry and then inserts all
    /// items from the provided iterator. The internal `next_id` counter is NOT updated.
    ///
    /// # Warning
    /// The caller is responsible for ensuring that `next_id` is set appropriately
    /// if `generate_id()` will be used after calling this function, to prevent ID collisions.
    ///
    /// # Type Parameters
    /// - `Iter`: An iterator yielding `(I, T)` tuples.
    pub fn repopulate_from_entities_no_next_id_update<Iter>(&self, entities: Iter)
    where
        Iter: IntoIterator<Item = (I, T)>,
    {
        self.id_to_entity.clear();
        self.entity_to_id.clear();

        warn!("Registry '{}': repopulated without updating next_id. Caller must ensure next_id is managed if generate_id() is used.", self.name);
        for (id, entity) in entities {
            self.id_to_entity.insert(id, entity);
            self.entity_to_id.insert(entity, id);
        }
    }
}

// pub struct AutoIncrementingId{
//     value: u32,
//     next_id: AtomicU32,
// }
// impl AutoIncrementingId {
//     pub fn new() -> Self {
//         Self{
//             value: 0,
//             next_id: AtomicU32::new(0),
//         }
//     }
// }

// pub struct IdGenerator{
//     next_id: AtomicU32,
//
// }
// impl IdGenerator {
//     pub fn generate_id(&self) -> u32 {
//         self.next_id.fetch_add(1, Ordering::Relaxed)
//     }
// }
#[derive(Default)]
pub struct GlobalSkillNameMap(pub DashMap<SkillId,String>);
