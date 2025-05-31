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
