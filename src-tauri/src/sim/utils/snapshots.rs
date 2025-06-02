use dashmap::DashSet;

/// Replaces `target` with `new` if they are not equal. Returns `true` if changed.
pub fn replace_if_changed<T: PartialEq<N> + From<N>, N>(target: &mut T, new: N) -> bool {
    if *target != new {
        *target = T::from(new);
        true
    } else {
        false
    }
}

// /// Replaces `target` with `new` if they are not equal. Returns `true` if changed.
// pub fn replace_if_changed_ref<T: PartialEq + Clone>(target: &mut T, new: &T) -> bool {
//     if target != new {
//         *target = new.clone();
//         true
//     } else {
//         false
//     }
// }

/// Converts a borrowed `DashSet<T>` into a `Vec<T>`.
///
/// This function **borrows** the `DashSet`, meaning the original set
/// remains unchanged and can be used after the function call.
/// It returns a `Vec` containing cloned copies of all its elements.
///
/// The order of elements in the resulting `Vec` is not guaranteed
/// due to the nature of hash sets.
///
/// # Type Parameters
/// - `T`: The type of elements stored in the `DashSet`.
///        Must implement `Eq`, `Hash`, and crucially, **`Clone`**.
///
/// # Arguments
/// - `dashset`: A shared reference (`&`) to the `DashSet` to convert.
///
/// # Returns
/// A `Vec` containing cloned `T` values from the `DashSet`.
///
/// # Examples
/// ```
/// use dashmap::DashSet;
///
/// let my_set = DashSet::new();
/// my_set.insert("apple");
/// my_set.insert("banana");
/// my_set.insert("cherry");
///
/// // Borrow the set
/// let vec_elements = convert_borrowed_dashset_to_vec(&my_set);
///
/// // The original set is still available
/// assert!(my_set.contains("apple"));
///
/// // The order is not guaranteed, so sort for predictable assertion
/// let mut expected = vec!["apple", "banana", "cherry"];
/// expected.sort_unstable();
///
/// let mut actual = vec_elements;
/// actual.sort_unstable();
///
/// assert_eq!(actual, expected);
/// ```
pub fn convert_dashset_to_vec<T>(dashset: &DashSet<T>) -> Vec<T>
where
    T: Eq + std::hash::Hash + Clone + Ord,
{
    let mut vec: Vec<T> = dashset
        .iter()
        .map(|item_ref_multi| item_ref_multi.clone()) // Explicitly clone the value inside RefMulti
        // Alternatively, if `T` is Copy, you could do `.map(|item_ref_multi| *item_ref_multi)`
        .collect();
    vec.sort_unstable();
    vec
}
