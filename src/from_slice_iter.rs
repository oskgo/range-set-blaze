use core::{iter::FusedIterator, ops::RangeInclusive, slice::ChunksExact};

use crate::Integer;

/// cmk update docs
/// Turns a [`SortedDisjoint`] iterator into a [`SortedDisjoint`] iterator of its complement,
/// i.e., all the integers not in the original iterator, as sorted & disjoint ranges.
///
/// # Example
///
/// ```
/// use range_set_blaze::{NotIter, SortedDisjoint, CheckSortedDisjoint};
///
/// let a = CheckSortedDisjoint::from([1u8..=2, 5..=100]);
/// let b = NotIter::new(a);
/// assert_eq!(b.to_string(), "0..=0, 3..=4, 101..=255");
///
/// // Or, equivalently:
/// let b = !CheckSortedDisjoint::from([1u8..=2, 5..=100]);
/// assert_eq!(b.to_string(), "0..=0, 3..=4, 101..=255");
/// ```
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct FromSliceIter<'a, T>
where
    T: Integer,
{
    prefix_iter: core::slice::Iter<'a, T>,
    previous_range: Option<RangeInclusive<T>>,
    chunks: ChunksExact<'a, T>,
    suffix: &'a [T],
    slice_len: usize,
}

impl<'a, T: 'a> FromSliceIter<'a, T>
where
    T: Integer,
{
    /// cmk update docs Create a new [`NotIter`] from a [`SortedDisjoint`] iterator. See [`NotIter`] for an example.
    pub fn new(slice: &'a [T]) -> Self {
        let (prefix, chunks, suffix) = T::as_aligned_chunks(slice);
        FromSliceIter {
            prefix_iter: prefix.iter(),
            previous_range: None,
            chunks,
            suffix,
            slice_len: slice.len(),
        }
    }
}

impl<'a, T> FusedIterator for FromSliceIter<'a, T> where T: Integer {}

impl<'a, T: 'a> Iterator for FromSliceIter<'a, T>
where
    T: Integer,
{
    type Item = RangeInclusive<T>;

    fn next(&mut self) -> Option<RangeInclusive<T>> {
        if let Some(before) = self.prefix_iter.next() {
            return Some(*before..=*before);
        }
        for chunk in self.chunks.by_ref() {
            if T::is_consecutive(chunk) {
                let this_start = chunk[0];
                let this_end = chunk[chunk.len() - 1];

                if let Some(inner_previous_range) = self.previous_range.as_mut() {
                    // if some and previous is some and adjacent, combine
                    if *inner_previous_range.end() + T::one() == this_start {
                        *inner_previous_range = *(inner_previous_range.start())..=this_end;
                    } else {
                        // if some and previous is some but not adjacent, flush previous, set previous to this range.
                        let result = Some(inner_previous_range.clone());
                        *inner_previous_range = this_start..=this_end;
                        return result;
                    }
                } else {
                    // if some and previous is None, set previous to this range.
                    self.previous_range = Some(this_start..=this_end);
                }
            } else {
                // If none, flush previous range, set it to none, output this chunk as a bunch of singletons.
                self.prefix_iter = chunk.iter();
                if let Some(previous) = self.previous_range.take() {
                    debug_assert!(self.previous_range.is_none());
                    return Some(previous);
                }
                if let Some(before) = self.prefix_iter.next() {
                    return Some(*before..=*before);
                }
            }
        }

        // at the very, very end, flush previous.
        if let Some(previous) = &self.previous_range.take() {
            debug_assert!(self.previous_range.is_none());
            return Some(previous.clone());
        }

        self.prefix_iter = self.suffix.iter();
        self.suffix = &[];

        self.prefix_iter.next().map(|before| *before..=*before)
    }

    // We could have one less or one more than the iter.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let low = if self.slice_len > 0 {
            self.slice_len - 1
        } else {
            0
        };
        let high = if self.slice_len < usize::MAX {
            self.slice_len + 1
        } else {
            usize::MAX
        };
        (low, Some(high))
    }
}
