use std::collections::HashSet;

pub(super) fn shift_left(marker: &mut Vec<char>, c: char) {
    marker.splice(0..1, []);
    marker.push(c);
}

pub(super) fn is_unique(marker: &mut Vec<char>) -> bool {
    let mut unique = HashSet::<char>::new();
    for char in marker {
        if !unique.insert(*char) {
            return false;
        }
    }
    true
}
