mod tests;

use itertools::Itertools;
use std::cmp::max;
use std::collections::BTreeMap;

pub fn fmt(items: &BTreeMap<u128, u128>) -> String {
    items
        .iter()
        .map(|(start, end)| format!("{start}..{end}"))
        .join(",")
}

pub fn internal_add(items: &mut BTreeMap<u128, u128>, len: &mut u128, start: u128, end: u128) {
    assert!(start < end); // !!!cmk check that length is not zero
                          // !!! cmk would be nice to have a partition_point function that returns two iterators
    let mut before = items.range_mut(..=start);
    if let Some((start_before, end_before)) = before.next() {
        if *end_before < start {
            insert(items, len, start, end);
            *len += end - start;
        } else if *end_before < end {
            *len += end - *end_before;
            *end_before = end;
            let start_before = *start_before;
            delete_extra(items, len, start_before, end);
        } else {
            // completely contained, so do nothing
        }
    } else {
        insert(items, len, start, end);
        *len += end - start;
    }
}

fn delete_extra(items: &mut BTreeMap<u128, u128>, len: &mut u128, start: u128, end: u128) {
    let mut after = items.range_mut(start..);
    let (start_after, start_end) = after.next().unwrap(); // !!! cmk assert that there is a next
    assert!(start == *start_after && end == *start_end); // !!! cmk real assert
                                                         // !!!cmk would be nice to have a delete_range function
    let mut end_new = end;
    let delete_list = after
        .map_while(|(start_delete, end_delete)| {
            if *start_delete <= end {
                end_new = max(end_new, *end_delete);
                *len -= *end_delete - *start_delete;
                Some(*start_delete)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if end_new > end {
        *len += end_new - end;
        *start_end = end_new;
    }
    for start in delete_list {
        items.remove(&start);
    }
}
fn insert(items: &mut BTreeMap<u128, u128>, len: &mut u128, start: u128, end: u128) {
    let was_there = items.insert(start, end);
    assert!(was_there.is_none());
    // !!!cmk real assert
    delete_extra(items, len, start, end);
}

// !!!cmk can I use a Rust range?
// !!!cmk allow negatives and any size

struct RangeSetInt {
    _items: BTreeMap<u128, u128>, // !!!cmk usize?
                                  // !!!cmk underscore?
}

impl RangeSetInt {
    fn new() -> RangeSetInt {
        RangeSetInt {
            _items: BTreeMap::new(),
        }
    }

    fn clear(&mut self) {
        self._items.clear();
    }

    // !!!cmk keep this in a field
    fn len(&self) -> u128 {
        self._items.values().sum()
    }

    // fn _internal_add(&mut self, start: u128, length: u128) {
    //     // !!!cmk put this shortcut back?
    //     // if self._items.len() == 0 {
    //     //     self._items.insert(start, length);
    //     //     return;
    //     // }

    //     // https://stackoverflow.com/questions/49599833/how-to-find-next-smaller-key-in-btreemap-btreeset
    //     // https://stackoverflow.com/questions/35663342/how-to-modify-partially-remove-a-range-from-a-btreemap
    //     // !!!cmk rename index to "range"
    //     let range = self._items.range(..start);
    //     let mut peekable_forward = range.clone().peekable();
    //     let peek_forward = peekable_forward.peek();
    //     let mut peekable_backwards = range.rev().peekable();
    //     let peek_backwards = peekable_backwards.peek();
    //     if let Some(peek_forward) = peek_forward {
    //         let mut peek_forward = *peek_forward;
    //         if *peek_forward.0 == start {
    //             if length > *peek_forward.1 {
    //                 peek_forward.1 = &length;
    //                 // previous_range = peek_forward;
    //                 // peek_forward = peekable_forward.next(); // index should point to the following range for the remainder of this method
    //                 todo!()
    //             } else {
    //                 todo!();
    //             }
    //         }
    //     } else {
    //         println!("self._items.insert(start, length);");
    //         if let Some(previous_range) = peek_backwards {
    //             // nothing
    //         } else {
    //             return;
    //         }
    //     }

    //     todo!();
    //     //             return;
    //     //         }
    //     //     } else if index == 0 {
    //     //         self._items.insert(index, RangeX { start, length });
    //     //         previous_index = index;
    //     //         index += 1 // index_of_miss should point to the following range for the remainder of this method
    //     //     } else {
    //     //         previous_index = index - 1;
    //     //         let previous_range: &mut RangeX = &mut self._items[previous_index];

    //     //         if previous_range.end() >= start {
    //     //             let new_length = start + length - previous_range.start;
    //     //             if new_length <= previous_range.length {
    //     //                 return;
    //     //             } else {
    //     //                 previous_range.length = new_length;
    //     //             }
    //     //         } else {
    //     //             // after previous range, not contiguous with previous range
    //     //             self._items.insert(index, RangeX { start, length });
    //     //             previous_index = index;
    //     //             index += 1;
    //     //         }
    //     //     }
    //     // }

    //     // let previous_range: &RangeX = &self._items[previous_index];
    //     // let previous_end = previous_range.end();
    //     // while index < self._items.len() {
    //     //     let range: &RangeX = &self._items[index];
    //     //     if previous_end < range.start {
    //     //         break;
    //     //     }
    //     //     let range_end = range.end();
    //     //     if previous_end < range_end {
    //     //         self._items[previous_index].length = range_end - previous_range.start;
    //     //         index += 1;
    //     //         break;
    //     //     }
    //     //     index += 1;
    //     // }
    //     // self._items.drain(previous_index + 1..index);
    // }
}
