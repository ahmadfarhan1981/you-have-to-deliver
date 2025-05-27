use std::marker::PhantomData;
use std::sync::atomic::AtomicU64;

pub struct IdGenerator<T> {
    next_id : AtomicU64,
    _marker: PhantomData<T>,
}
impl<T> IdGenerator<T> {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(0),
            _marker: PhantomData,
        }
    }
    pub fn generate(&self) -> u64 {
        self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn doubles_the_input() {
    //     assert_eq!(calculate_skill_score(4), 8);
    // }
}