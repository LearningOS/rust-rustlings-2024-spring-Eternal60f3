/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Display;

pub struct Heap<T>
where
    T: Default + Display,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Display,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        self.items.push(value);
        let mut curr = self.count;
        let mut p = self.parent_idx(curr);

        while p >= 1 {
            // println!("up: {}", curr);
            if (self.comparator)(&self.items[curr], &self.items[p]) {
                self.items.swap(curr, p);
                curr = p;
                p = self.parent_idx(curr);
            } else {
                break;
            }
        }
        // println!("peek: {}", self.items[1]);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn need_down_child(&self, idx: usize) -> usize {
        //TODO
        let li = self.left_child_idx(idx);
        let ri = self.right_child_idx(idx);
        if li > self.count {
            return self.count + 1;
        } else if ri > self.count {
            return li;
        }
        
        if (self.comparator)(&self.items[li], &self.items[ri]) {
            li
        } else {
            ri
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Display,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Display,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            return None;
        } 

        println!("count: {}", self.count);
        self.items.swap(1, self.count);
        self.count -= 1;
        let mut curr = 1;
        let mut child = self.need_down_child(curr);
        while child < self.count {
            println!("curr: {curr}, child: {child}");
            if (self.comparator)(&self.items[child], &self.items[curr]) {
                self.items.swap(child, curr);
                curr = child;
                child = self.need_down_child(curr);
            } else {
                break;
            }
        }
        self.items.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Display,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Display,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}