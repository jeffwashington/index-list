//! A doubly-linked list implemented in safe Rust.
//!
//! The list elements are stored in a vector which provides an index to the
//! element, where it stores the index of the next and previous element in the
//! list. The index does not change as long as the element is not removed, even
//! when the element changes its position in the list.
//!
//! A new IndexList can be created empty with the `new` method, or created from
//! an existing vector with `IndexList::from`.
//!
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::iter::DoubleEndedIterator;
use std::mem;
use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Index(Option<NonZeroU32>);

impl Index {
    #[inline]
    fn new() -> Index {
        Index { 0: None }
    }
    #[inline]
    /// Returns `true` for a valid index
    ///
    /// A valid index can be used in IndexList method calls.
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
    #[inline]
    /// Returns `true` if the index is invalid
    ///
    /// An invalid index should not be used in any IndexList method calls
    /// because they will always cause `None` to be returned.
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
    #[inline]
    fn get(&self) -> Option<usize> {
        Some(self.0?.get() as usize - 1)
    }
    #[inline]
    fn set(mut self, index: Option<usize>) -> Self {
        if let Some(n) = index {
            if let Ok(num) = NonZeroU32::try_from(n as u32 + 1) {
                self.0 = Some(num);
            } else {
                self.0 = None;
            }
        } else {
            self.0 = None;
        }
        self
    }
}

impl From<u32> for Index {
    fn from(index: u32) -> Index {
        Index::new().set(Some(index as usize))
    }
}

impl From<u64> for Index {
    fn from(index: u64) -> Index {
        Index::new().set(Some(index as usize))
    }
}

impl From<usize> for Index {
    fn from(index: usize) -> Index {
        Index::new().set(Some(index))
    }
}

impl From<Option<usize>> for Index {
    fn from(index: Option<usize>) -> Index {
        Index::new().set(index)
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ndx) = self.0 {
            write!(f, "{}", ndx)
        } else {
            write!(f, "|")
        }
    }
}

#[derive(Clone, Debug, Default)]
struct IndexNode {
    next: Index,
    prev: Index,
}

impl IndexNode {
    #[inline]
    fn new() -> IndexNode {
        IndexNode { next: Index::new(), prev: Index::new() }
    }
    #[inline]
    fn new_next(&mut self, next: Index) -> Index {
        mem::replace(&mut self.next, next)
    }
    #[inline]
    fn new_prev(&mut self, prev: Index) -> Index {
        mem::replace(&mut self.prev, prev)
    }
}

impl fmt::Display for IndexNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}<>{}", self.next, self.prev)
    }
}

#[derive(Clone, Debug, Default)]
struct IndexEnds {
    head: Index,
    tail: Index,
}

impl IndexEnds {
    #[inline]
    fn new() -> Self {
        IndexEnds { head: Index::new(), tail: Index::new() }
    }
    #[inline]
    fn clear(&mut self) {
        self.new_both(Index::new());
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    #[inline]
    fn new_head(&mut self, head: Index) -> Index {
        mem::replace(&mut self.head, head)
    }
    #[inline]
    fn new_tail(&mut self, tail: Index) -> Index {
        mem::replace(&mut self.tail, tail)
    }
    #[inline]
    fn new_both(&mut self, both: Index) {
        self.head = both;
        self.tail = both;
    }
}

impl fmt::Display for IndexEnds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}>=<{}", self.head, self.tail)
    }
}

#[derive(Debug)]
pub struct IndexList<T> {
    elems: Vec<Option<T>>,
    nodes: Vec<IndexNode>,
    used: IndexEnds,
    free: IndexEnds,
    size: usize,
}

impl<T> Default for IndexList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IndexList<T> {
    /// Creates a new empty index list.
    ///
    /// Example:
    /// ```rust
    /// use index_list::IndexList;
    ///
    /// let list = IndexList::<u64>::new();
    /// ```
    pub fn new() -> Self {
        IndexList {
            elems: Vec::new(),
            nodes: Vec::new(),
            used: IndexEnds::new(),
            free: IndexEnds::new(),
            size: 0,
        }
    }
    /// Returns the current capacity of the list.
    ///
    /// This value is always greater than or equal to the length.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let list = IndexList::<u64>::new();
    /// let cap = list.capacity();
    /// assert!(cap >= list.len());
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.elems.len()
    }
    /// Returns the number of valid elements in the list.
    ///
    /// This value is always less than or equal to the capacity.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// # list.insert_first(42);
    /// let first = list.remove_first();
    /// assert!(list.len() < list.capacity());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }
    /// Clears the list be removing all elements, making it empty.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// list.clear();
    /// assert!(list.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.elems.clear();
        self.nodes.clear();
        self.used.clear();
        self.free.clear();
        self.size = 0;
    }
    /// Returns `true` when the list is empty.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// let list = IndexList::<u64>::new();
    /// assert!(list.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.used.is_empty()
    }
    /// Returns `true` if the index is valid.
    #[inline]
    pub fn is_index_used(&self, index: Index) -> bool {
        self.get(index).is_some()
    }
    /// Returns the index of the first element, or `None` if the list is empty.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let list = IndexList::<u64>::new();
    /// let index = list.first_index();
    /// ```
    #[inline]
    pub fn first_index(&self) -> Index {
        self.used.head
    }
    /// Returns the index of the last element, or `None` if the list is empty.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let list = IndexList::<u64>::new();
    /// let index = list.last_index();
    /// ```
    #[inline]
    pub fn last_index(&self) -> Index {
        self.used.tail
    }
    /// Returns the index of the next element, after index, or `None` when the
    /// end is reached.
    ///
    /// *NOTE* that indexes are likely not sequential.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let list = IndexList::<u64>::new();
    /// let mut index = list.first_index();
    /// while index.is_some() {
    ///     // Do something
    ///     index = list.next_index(index);
    /// }
    /// ```
    #[inline]
    pub fn next_index(&self, index: Index) -> Index {
        if let Some(ndx) = index.get() {
            if let Some(node) = self.nodes.get(ndx) {
                return node.next;
            }
        }
        Index::new()
    }
    /// Returns the index of the previous element, before index, or `None` when
    /// the beginning is reached.
    ///
    /// *NOTE* that indexes are likely not sequential.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let list = IndexList::<u64>::new();
    /// let mut index = list.last_index();
    /// while index.is_some() {
    ///     // Do something
    ///     index = list.prev_index(index);
    /// }
    /// ```
    #[inline]
    pub fn prev_index(&self, index: Index) -> Index {
        if let Some(ndx) = index.get() {
            if let Some(node) = self.nodes.get(ndx) {
                return node.prev;
            }
        }
        Index::new()
    }
    /// Move to an index `steps` number of elements away. Positive numbers will
    /// move in the next direction, while negative number in the prev direction.
    ///
    /// Returns the index `steps` elements away, or `None` when the end is
    /// reached.
    ///
    /// *NOTE* that indexes are likely not sequential.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let list = IndexList::from(&mut vec!["A", "B", "C", "D", "E"]);
    /// let mut index = list.first_index();
    /// index = list.move_index(index, 3);
    /// // Do something with the 4:th element
    /// # assert_eq!(list.get(index), Some(&"D"));
    /// index = list.move_index(index, -2);
    /// // Do something with the 2:nd element
    /// # assert_eq!(list.get(index), Some(&"B"));
    /// index = list.move_index(index, -2);
    /// assert!(index.is_none());
    /// ```
    #[inline]
    pub fn move_index(&self, index: Index, steps: i32) -> Index {
        let mut index = index;
        match steps.cmp(&0) {
            Ordering::Greater => {
                (0..steps).for_each(|_| { index = self.next_index(index); });
            },
            Ordering::Less => {
                (0..-steps).for_each(|_| { index = self.prev_index(index); });
            },
            Ordering::Equal => (),
        }
        index
    }
    /// Get a reference to the first element data, or `None`.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let list = IndexList::<u64>::new();
    /// let data = list.get_first();
    /// ```
    #[inline]
    pub fn get_first(&self) -> Option<&T> {
        self.get(self.first_index())
    }
    /// Get a reference to the last element data, or `None`.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let list = IndexList::<u64>::new();
    /// let data = list.get_last();
    /// ```
    #[inline]
    pub fn get_last(&self) -> Option<&T> {
        self.get(self.last_index())
    }
    /// Get an immutable reference to the element data at the index, or `None`.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let list = IndexList::<u64>::new();
    /// # let index = list.first_index();
    /// let data = list.get(index);
    /// ```
    #[inline]
    pub fn get(&self, index: Index) -> Option<&T> {
        let ndx = index.get().unwrap_or(usize::MAX);
        self.elems.get(ndx)?.as_ref()
    }
    /// Get a mutable reference to the first element data, or `None`.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// if let Some(data) = list.get_mut_first() {
    ///     // Update the data somehow
    /// }
    /// ```
    #[inline]
    pub fn get_mut_first(&mut self) -> Option<&mut T> {
        self.get_mut(self.first_index())
    }
    /// Get a mutable reference to the last element data, or `None`.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// if let Some(data) = list.get_mut_last() {
    ///     // Update the data somehow
    /// }
    /// ```
    #[inline]
    pub fn get_mut_last(&mut self) -> Option<&mut T> {
        self.get_mut(self.last_index())
    }
    /// Get a mutable reference to the element data at the index, or `None`.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// # let index = list.first_index();
    /// if let Some(data) = list.get_mut(index) {
    ///     // Update the data somehow
    /// }
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        if let Some(ndx) = index.get() {
            if ndx < self.capacity() {
                return self.elems[ndx].as_mut();
            }
        }
        None
    }
    #[inline]
    /// Peek at next element data, after the index, if any.
    ///
    /// Returns `None` if there is no next index in the list.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec![1, 2, 3]);
    /// # let index = list.first_index();
    /// if let Some(data) = list.peek_next(index) {
    ///     // Consider the next data
    /// #   assert_eq!(*data, 2);
    /// }
    /// ```
    pub fn peek_next(&self, index: Index) -> Option<&T> {
        self.get(self.next_index(index))
    }
    #[inline]
    /// Peek at previous element data, before the index, if any.
    ///
    /// Returns `None` if there is no previous index in the list.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec![1, 2, 3]);
    /// # let index = list.last_index();
    /// if let Some(data) = list.peek_prev(index) {
    ///     // Consider the previous data
    /// #   assert_eq!(*data, 2);
    /// }
    /// ```
    pub fn peek_prev(&self, index: Index) -> Option<&T> {
        self.get(self.prev_index(index))
    }
    /// Returns `true` if the element is in the list.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// # let index = list.insert_first(42);
    /// if list.contains(42) {
    ///     // Find it?
    /// } else {
    ///     // Insert it?
    /// }
    /// ```
    #[inline]
    pub fn contains(&self, elem: T) -> bool
    where T: PartialEq {
        self.elems.contains(&Some(elem))
    }
    /// Returns the index of the element containg the data.
    ///
    /// If there is more than one element with the same data, the one with the
    /// lowest index will always be returned.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::{Index, IndexList};
    /// # let mut list = IndexList::from(&mut vec![1, 2, 3]);
    /// let index = list.index_of(2);
    /// # assert_eq!(index, Index::from(1u32))
    /// ```
    #[inline]
    pub fn index_of(&self, elem: T) -> Index
    where T: PartialEq {
        Index::from(self.elems.iter().position(|e| {
            if let Some(data) = e { data == &elem } else { false }
        }))
    }
    /// Insert a new element at the beginning.
    ///
    /// It is usually not necessary to keep the index, as the element data
    /// can always be found again by walking the list.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// let index = list.insert_first(42);
    /// ```
    pub fn insert_first(&mut self, elem: T) -> Index {
        let this = self.new_node(Some(elem));
        self.linkin_first(this);
        this
    }
    /// Insert a new element at the end.
    ///
    /// It is typically not necessary to store the index, as the data will be
    /// there when walking the list.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// let index = list.insert_last(42);
    /// ```
    pub fn insert_last(&mut self, elem: T) -> Index {
        let this = self.new_node(Some(elem));
        self.linkin_last(this);
        this
    }
    /// Insert a new element before the index.
    ///
    /// If the index is `None` then the new element will be inserted first.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// # let mut index = list.last_index();
    /// index = list.insert_before(index, 42);
    /// ```
    pub fn insert_before(&mut self, index: Index, elem: T) -> Index {
        if index.is_none() {
            return self.insert_first(elem);
        }
        let this = self.new_node(Some(elem));
        self.linkin_this_before_that(this, index);
        this
    }
    /// Insert a new element after the index.
    ///
    /// If the index is `None` then the new element will be inserted last.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// # let mut index = list.first_index();
    /// index = list.insert_after(index, 42);
    /// ```
    pub fn insert_after(&mut self, index: Index, elem: T) -> Index {
        if index.is_none() {
            return self.insert_last(elem);
        }
        let this = self.new_node(Some(elem));
        self.linkin_this_after_that(this, index);
        this
    }
    /// Remove the first element and return its data.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// # list.insert_first(42);
    /// let data = list.remove_first();
    /// # assert_eq!(data, Some(42));
    /// ```
    pub fn remove_first(&mut self) -> Option<T> {
        self.remove(self.first_index())
    }
    /// Remove the last element and return its data.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::<u64>::new();
    /// # list.insert_last(42);
    /// let data = list.remove_last();
    /// # assert_eq!(data, Some(42));
    /// ```
    pub fn remove_last(&mut self) -> Option<T> {
        self.remove(self.last_index())
    }
    /// Remove the element at the index and return its data.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec!["A", "B", "C"]);
    /// # let mut index = list.first_index();
    /// # index = list.next_index(index);
    /// let data = list.remove(index);
    /// # assert_eq!(data, Some("B"));
    /// ```
    pub fn remove(&mut self, index: Index) -> Option<T> {
        let elem_opt = self.remove_elem_at_index(index);
        if elem_opt.is_some() {
            self.linkout_used(index);
            self.linkin_free(index);
        }
        elem_opt
    }
    /// Create a new iterator over all the element.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec![120, 240, 360]);
    /// let total: usize = list.iter().sum();
    /// assert_eq!(total, 720);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter { list: &self, next: self.first_index(), prev: self.last_index() }
    }
    /// Create a vector for all elements.
    ///
    /// Returns a new vector with immutable reference to the elements data.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec![1, 2, 3]);
    /// let vector: Vec<&u64> = list.to_vec();
    /// # assert_eq!(format!("{:?}", vector), "[1, 2, 3]");
    /// ```
    pub fn to_vec(&self) -> Vec<&T> {
        self.iter().filter_map(Option::Some).collect()
    }
    /// Insert all the elements from the vector, which will be drained.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// let mut the_numbers = vec![4, 8, 15, 16, 23, 42];
    /// let list = IndexList::from(&mut the_numbers);
    /// assert_eq!(the_numbers.len(), 0);
    /// assert_eq!(list.len(), 6);
    /// ```
    pub fn from(vec: &mut Vec<T>) -> IndexList<T> {
        let mut list = IndexList::<T>::new();
        vec.drain(..).for_each(|elem| {
            list.insert_last(elem);
        });
        list
    }
    /// Remove any unused indexes at the end by truncating.
    ///
    /// If the unused indexes don't appear at the end, then nothing happens.
    ///
    /// No valid indexes are changed.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec![4, 8, 15, 16, 23, 42]);
    /// list.remove_last();
    /// assert!(list.len() < list.capacity());
    /// list.trim_safe();
    /// assert_eq!(list.len(), list.capacity());
    /// ```
    pub fn trim_safe(&mut self) {
        let removed: Vec<usize> = (self.len()..self.capacity())
            .rev()
            .take_while(|&i| self.is_free(i))
            .collect();
        removed.iter().for_each(|&i| {
            self.linkout_free(Index::from(i));
        });
        if !removed.is_empty() {
            let left = self.capacity() - removed.len();
            self.nodes.truncate(left);
            self.elems.truncate(left);
        }
    }
    /// Remove all unused elements by swapping indexes and then truncating.
    ///
    /// This will reduce the capacity of the list, but only if there are any
    /// unused elements. Length and capacity will be equal after the call.
    ///
    /// *NOTE* that this call may invalidate some indexes.
    ///
    /// While it is possible to tell if an index has become invalid, because
    /// only indexes at or above the new capacity limit has been moved, it is
    /// not recommended to rely on that fact or test for it.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec![4, 8, 15, 16, 23, 42]);
    /// list.remove_first();
    /// assert!(list.len() < list.capacity());
    /// list.trim_swap();
    /// assert_eq!(list.len(), list.capacity());
    /// ```
    pub fn trim_swap(&mut self) {
        let need = self.size;
        // destination is all free node indexes below the needed limit
        let dst: Vec<usize> = self.elems[..need]
            .iter()
            .enumerate()
            .filter(|(n, e)| e.is_none() && n < &need)
            .map(|(n, _e)| n)
            .collect();
        // source is all used node indexes above the needed limit
        let src: Vec<usize> = self.elems[need..]
            .iter()
            .enumerate()
            .filter(|(_n, e)| e.is_some())
            .map(|(n, _e)| n + need)
            .collect();
        debug_assert_eq!(dst.len(), src.len());
        src.iter()
            .zip(dst.iter())
            .for_each(|(s, d)| self.replace_dest_with_source(*s, *d));
        self.free.new_both(Index::new());
        self.elems.truncate(need);
        self.nodes.truncate(need);
    }
    /// Add the elements of the other list at the end.
    ///
    /// The other list will be empty after the call as all its elements have
    /// been moved to this list.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec![4, 8, 15]);
    /// # let mut other = IndexList::from(&mut vec![16, 23, 42]);
    /// let sum_both = list.len() + other.len();
    /// list.append(&mut other);
    /// assert!(other.is_empty());
    /// assert_eq!(list.len(), sum_both);
    /// # assert_eq!(list.to_string(), "[4 >< 8 >< 15 >< 16 >< 23 >< 42]");
    /// ```
    pub fn append(&mut self, other: &mut IndexList<T>) {
        while let Some(elem) = other.remove_first() {
            self.insert_last(elem);
        }
    }
    /// Add the elements of the other list at the beginning.
    ///
    /// The other list will be empty after the call as all its elements have
    /// been moved to this list.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec![16, 23, 42]);
    /// # let mut other = IndexList::from(&mut vec![4, 8, 15]);
    /// let sum_both = list.len() + other.len();
    /// list.prepend(&mut other);
    /// assert!(other.is_empty());
    /// assert_eq!(list.len(), sum_both);
    /// # assert_eq!(list.to_string(), "[4 >< 8 >< 15 >< 16 >< 23 >< 42]");
    /// ```
    pub fn prepend(&mut self, other: &mut IndexList<T>) {
        while let Some(elem) = other.remove_last() {
            self.insert_first(elem);
        }
    }
    /// Split the list by moving the elements from the index to a new list.
    ///
    /// The original list will no longer contain the elements data that was
    /// moved to the other list.
    ///
    /// Example:
    /// ```rust
    /// # use index_list::IndexList;
    /// # let mut list = IndexList::from(&mut vec![4, 8, 15, 16, 23, 42]);
    /// # let mut index = list.first_index();
    /// # index = list.next_index(index);
    /// # index = list.next_index(index);
    /// # index = list.next_index(index);
    /// let total = list.len();
    /// let other = list.split(index);
    /// assert!(list.len() < total);
    /// assert_eq!(list.len() + other.len(), total);
    /// # assert_eq!(list.to_string(), "[4 >< 8 >< 15]");
    /// # assert_eq!(other.to_string(), "[16 >< 23 >< 42]");
    /// ```
    pub fn split(&mut self, index: Index) -> IndexList<T> {
        let mut list = IndexList::<T>::new();
        while self.is_index_used(index) {
            list.insert_first(self.remove_last().unwrap());
        }
        list
    }

    #[inline]
    fn is_used(&self, at: usize) -> bool {
        self.elems[at].is_some()
    }
    fn is_free(&self, at: usize) -> bool {
        self.elems[at].is_none()
    }
    #[inline]
    fn get_mut_indexnode(&mut self, at: usize) -> &mut IndexNode {
        &mut self.nodes[at]
    }
    #[inline]
    fn get_indexnode(&self, at: usize) -> &IndexNode {
        &self.nodes[at]
    }
    #[inline]
    fn set_prev(&mut self, index: Index, new_prev: Index) -> Index {
        if let Some(at) = index.get() {
            self.get_mut_indexnode(at).new_prev(new_prev)
        } else {
            index
        }
    }
    #[inline]
    fn set_next(&mut self, index: Index, new_next: Index) -> Index {
        if let Some(at) = index.get() {
            self.get_mut_indexnode(at).new_next(new_next)
        } else {
            index
        }
    }
    #[inline]
    fn linkin_tail(&mut self, prev: Index, this: Index, next: Index) {
        if next.is_none() {
            let old_tail = self.used.new_tail(this);
            debug_assert_eq!(old_tail, prev);
        }
    }
    #[inline]
    fn linkin_head(&mut self, prev: Index, this: Index, next: Index) {
        if prev.is_none() {
            let old_head = self.used.new_head(this);
            debug_assert_eq!(old_head, next);
        }
    }
    #[inline]
    fn insert_elem_at_index(&mut self, this: Index, elem: Option<T>) {
        if let Some(at) = this.get() {
            self.elems[at] = elem;
            self.size += 1;
        }
    }
    #[inline]
    fn remove_elem_at_index(&mut self, this: Index) -> Option<T> {
        if let Some(at) = this.get() {
            self.size -= 1;
            self.elems[at].take()
        } else {
            None
        }
    }
    fn new_node(&mut self, elem: Option<T>) -> Index {
        let reuse = self.free.head;
        if reuse.is_some() {
            self.insert_elem_at_index(reuse, elem);
            self.linkout_free(reuse);
            return reuse;
        }
        let pos = self.nodes.len();
        self.nodes.push(IndexNode::new());
        self.elems.push(elem);
        self.size += 1;
        Index::from(pos)
    }
    fn linkin_free(&mut self, this: Index) {
        debug_assert_eq!(self.is_index_used(this), false);
        let prev = self.free.tail;
        self.set_next(prev, this);
        self.set_prev(this, prev);
        if self.free.is_empty() {
            self.free.new_both(this);
        } else {
            let old_tail = self.free.new_tail(this);
            debug_assert_eq!(old_tail, prev);
        }
    }
    fn linkin_first(&mut self, this: Index) {
        debug_assert!(self.is_index_used(this));
        let next = self.used.head;
        self.set_prev(next, this);
        self.set_next(this, next);
        if self.used.is_empty() {
            self.used.new_both(this);
        } else {
            let old_head = self.used.new_head(this);
            debug_assert_eq!(old_head, next);
        }
    }
    fn linkin_last(&mut self, this: Index) {
        debug_assert!(self.is_index_used(this));
        let prev = self.used.tail;
        self.set_next(prev, this);
        self.set_prev(this, prev);
        if self.used.is_empty() {
            self.used.new_both(this);
        } else {
            let old_tail = self.used.new_tail(this);
            debug_assert_eq!(old_tail, prev);
        }
    }
    // prev? >< that => prev? >< this >< that
    fn linkin_this_before_that(&mut self, this: Index, that: Index) {
        debug_assert!(self.is_index_used(this));
        debug_assert!(self.is_index_used(that));
        let prev = self.set_prev(that, this);
        let old_next = self.set_next(prev, this);
        if old_next.is_some() { debug_assert_eq!(old_next, that); }
        self.set_prev(this, prev);
        self.set_next(this, that);
        self.linkin_head(prev, this, that);
    }
    // that >< next? => that >< this >< next?
    fn linkin_this_after_that(&mut self, this: Index, that: Index) {
        debug_assert!(self.is_index_used(this));
        debug_assert!(self.is_index_used(that));
        let next = self.set_next(that, this);
        let old_prev = self.set_prev(next, this);
        if old_prev.is_some() { debug_assert_eq!(old_prev, that); }
        self.set_prev(this, that);
        self.set_next(this, next);
        self.linkin_tail(that, this, next);
    }
    // prev >< this >< next => prev >< next
    fn linkout_node(&mut self, this: Index) -> (Index, Index) {
        let next = self.set_next(this, Index::new());
        let prev = self.set_prev(this, Index::new());
        let old_prev = self.set_prev(next, prev);
        if old_prev.is_some() { debug_assert_eq!(old_prev, this); }
        let old_next = self.set_next(prev, next);
        if old_next.is_some() { debug_assert_eq!(old_next, this); }
        (prev, next)
    }
    fn linkout_used(&mut self, this: Index) {
        let (prev, next) = self.linkout_node(this);
        if next.is_none() {
            let old_tail = self.used.new_tail(prev);
            debug_assert_eq!(old_tail, this);
        }
        if prev.is_none() {
            let old_head = self.used.new_head(next);
            debug_assert_eq!(old_head, this);
        }
    }
    fn linkout_free(&mut self, this: Index) {
        let (prev, next) = self.linkout_node(this);
        if next.is_none() {
            let old_tail = self.free.new_tail(prev);
            debug_assert_eq!(old_tail, this);
        }
        if prev.is_none() {
            let old_head = self.free.new_head(next);
            debug_assert_eq!(old_head, this);
        }
    }
    fn replace_dest_with_source(&mut self, src: usize, dst: usize) {
        debug_assert!(self.is_free(dst));
        debug_assert!(self.is_used(src));
        self.linkout_free(Index::from(dst));
        let src_node = self.get_indexnode(src);
        let next = src_node.next;
        let prev = src_node.prev;
        self.linkout_used(Index::from(src));
        self.elems[dst] = self.elems[src].take();
        let this = Index::from(dst);
        if next.is_some() {
            self.linkin_this_before_that(this, next);
        } else if prev.is_some() {
            self.linkin_this_after_that(this, prev);
        } else {
            self.linkin_first(this);
        }
    }
}

impl<T> fmt::Display for IndexList<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elems: Vec<String> = self.iter().map(|x| format!("{}", x)).collect();
        write!(f, "[{}]", elems.join(" >< "))
    }
}

impl<T> From<T> for IndexList<T> {
    fn from(elem: T) -> IndexList<T> {
        let mut list = IndexList::new();
        list.insert_last(elem);
        list
    }
}

pub struct Iter<'a, T> {
    list: &'a IndexList<T>,
    next: Index,
    prev: Index,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.get(self.next);
        self.next = self.list.next_index(self.next);
        item
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let my_len = self.list.len();
        (my_len, Some(my_len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.list.get(self.prev);
        self.prev = self.list.prev_index(self.prev);
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_struct_sizes() {
        assert_eq!(size_of::<Index>(), 4);
        assert_eq!(size_of::<IndexNode>(), 8);
        assert_eq!(size_of::<IndexEnds>(), 8);
        assert_eq!(size_of::<IndexList<u32>>(), 72);
    }
}