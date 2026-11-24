use std::cmp::Ordering;

pub fn find<T, U>(array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
    U: Ord + Copy,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    };

    let mut array = array.to_vec();

    array.sort();

    let num = array.len();
    let mid = num / 2_usize;
    match key.cmp(array.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&array[..mid], key),
        Ordering::Greater => find(&array[(mid + 1)..], key).map(|i| i + mid + 1),
    }
}
