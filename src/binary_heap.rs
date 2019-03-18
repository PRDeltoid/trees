use std::fmt;
use std::cmp::Ordering;

pub struct BinaryHeap<T> {
    heap: Vec<T>,
}

#[derive(Clone, Copy, Default)]
pub struct PriorityTuple<T> {
    priority: u32,
    pub value: T,
}

impl<T> PartialOrd for PriorityTuple<T> {
    fn partial_cmp(&self, other: &PriorityTuple<T>) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl<T> PartialEq for PriorityTuple<T> {
    fn eq(&self, other: &PriorityTuple<T>) -> bool {
        self.priority == other.priority
    }
}

impl<T> fmt::Display for PriorityTuple<T>
    where T: PartialOrd + Copy + fmt::Display + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Priority: {}, Value: {}", self.priority, self.value)
    }
}
impl<T> PriorityTuple<T> {
    pub fn new(priority: u32, value: T) -> PriorityTuple<T> {
        PriorityTuple {
            priority,
            value,
        }
    }
}

#[allow(dead_code)]
impl<T> BinaryHeap<T>
    where T: Default + Copy + PartialOrd + fmt::Display {

    pub fn new() -> BinaryHeap<T> {
        BinaryHeap {
            heap: Vec::new(),
        }
    }

    fn get_left(&self, i: usize) -> Option<T> {
        self.heap.get(2*i+1).cloned()
    }

    fn get_right(&self, i: usize) -> Option<T> {
        self.heap.get(2*i+2).cloned()
    }

    fn get_left_index(&self, i: usize) -> usize {
        2*i+1 as usize
    }

    fn get_right_index(&self, i: usize) -> usize {
        2*i+2 as usize
    }

    fn get_parent(&self, i: usize) -> T {
        if i==0  {
            panic!("Cannot get parent of root node");
        }
        self.heap.get((i-1)/2).unwrap().clone()
    }

    fn get_parent_index(&self, i: usize) -> usize {
        (i-1)/2 as usize
    }

    pub fn insert(&mut self, value: T) {
        self.heap.push(value);

        let i = self.heap.len()-1;
        self.bubble_up(i);
    }

    fn bubble_up(&mut self, i: usize) {
        if i == 0 {
            return;
        }

        let parent_i = self.get_parent_index(i);
        //min-heap (minimum val at root)
        if self.get_parent(i) < self.heap[i] {
            self.bubble_up(parent_i);
            return;
        }
        self.heap.swap(i, parent_i);
    }

    pub fn extract(&mut self) -> Option<T> {
        //If the heap is empty, return None
        if self.heap.len() == 0 {
            return None;
        }
        //Move the lowest node on the tree to the top
        let last = self.heap.len()-1;
        self.heap.swap(0, last);

        //Save and remove the extracted value
        let value = self.heap.pop().unwrap();

        //Percolate down from root to re-Heapify
        self.percolate_down(0);

        return Some(value);

    }

    fn percolate_down(&mut self, i: usize) {
        let mut smallest = i;
        //println!("I am {}", self.heap[smallest]);

        if let Some(left_val) = self.get_left(i) {
            //println!("My left child is {}", left_val);
            if self.heap[smallest] > left_val {
                smallest = self.get_left_index(i);
            }
        }

        if let Some(right_val) = self.get_right(i) {
            //println!("My right child is {}", right_val);
            if self.heap[smallest] > right_val {
                smallest = self.get_right_index(i);
            }
        }
        if smallest != i {
            //println!("Swapping with {}", self.heap[smallest]);
            self.heap.swap(i, smallest);
            self.percolate_down(smallest);
        }
    }

    pub fn get_heap(&self) -> Vec<T> {
        self.heap.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }

    pub fn contains(&self, key: T) -> bool {
        self.heap.contains(key)
    }
}

impl<T> fmt::Display for BinaryHeap<T>
    where T: PartialOrd + Copy + fmt::Display + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.heap)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut bin: BinaryHeap<u32> = BinaryHeap::new();

        bin.insert(1);
        bin.insert(5);
        bin.insert(10);
        bin.insert(3);
        bin.insert(2);
        bin.insert(6);
        bin.insert(7);
        bin.insert(20);
        assert_eq!(bin.heap,vec![1, 2, 6, 5, 3, 10, 7, 20]);
    }

    #[test]
    fn test_extract() {
        //Build a heap
        let mut bin: BinaryHeap<u32> = BinaryHeap::new();

        bin.insert(1);
        bin.insert(5);
        bin.insert(10);
        bin.insert(3);
        bin.insert(2);
        bin.insert(6);
        bin.insert(7);
        bin.insert(20);
        assert_eq!(bin.get_heap(),vec![1, 2, 6, 5, 3, 10, 7, 20]);

        //Extract
        let val = bin.extract();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), 1);
        assert_eq!(bin.get_heap(),vec![2, 3, 6, 5, 20, 10, 7]);

        //Extract again
        let val = bin.extract();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), 2);
        assert_eq!(bin.get_heap(),vec![3, 5, 6, 7, 20, 10]);

    }
}