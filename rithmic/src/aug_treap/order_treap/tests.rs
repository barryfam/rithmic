use itertools::Itertools;

use super::list::List;
use super::sorted_list::SortedList;

#[test]
fn test_list() {
    let mut list = List::from_iter([3, 1, 4, 1, 5, 9]);

    assert_eq!(list[0], 3);
    assert_eq!(list[4], 5);
    assert_eq!(list[5], 9);

    list.remove(3);
    assert_eq!(list.iter().copied().collect_vec(), vec![3, 1, 4, 5, 9]);

    list.insert(1, 7);
    assert_eq!(list.iter().copied().collect_vec(), vec![3, 7, 1, 4, 5, 9]);

    list.replace(0, 2);
    assert_eq!(list.iter().copied().collect_vec(), vec![2, 7, 1, 4, 5, 9]);

    list.replace_slice(3.., [8, 2, 8]);
    assert_eq!(list.iter().copied().collect_vec(), vec![2, 7, 1, 8, 2, 8]);
}

#[test]
fn test_sorted_list() {
    let mut sl = SortedList::new();

    sl.insert(13);
    sl.insert(7);
    sl.insert(11);
    sl.insert(5);
    sl.insert(3);
    sl.insert(3);
    sl.insert(3);

    assert_eq!(sl.len(), 7);
    assert_eq!(sl.range_len(3..7), 4);
    assert_eq!(sl.range(5..).copied().collect_vec(), vec![5, 7, 11, 13]);
    assert_eq!(sl.range(4..=11).rev().copied().collect_vec(), vec![11, 7, 5]);

    assert_eq!(sl.position(&3), Some(0));
    assert_eq!(sl.trisect(&3), (0, 3));
    assert_eq!(sl.position(&5), Some(3));
    assert_eq!(sl.position(&7), Some(4));
    assert_eq!(sl.position(&11), Some(5));
    assert_eq!(sl.position(&13), Some(6));
    assert_eq!(sl.position(&10), None);

    sl.remove(&3);
    sl.remove(&7);

    assert_eq!(sl.len(), 5);

    assert_eq!(sl.position(&5), Some(2));
    assert_eq!(sl.position(&11), Some(3));
    assert_eq!(sl.position(&13), Some(4));

    assert_eq!(sl[0], 3);
    assert_eq!(sl[1], 3);
    assert_eq!(sl[2], 5);
    assert_eq!(sl[3], 11);
    assert_eq!(sl[4], 13);

    sl.remove_all(&3);
    sl.remove_range(12..);

    assert_eq!(sl.len(), 2);
}
