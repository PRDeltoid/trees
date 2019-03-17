use std::fmt;

pub struct BinaryHeap<T> {
    heap: Vec<T>,
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
        assert_eq!(0,0);
    }

    #[test]
    fn test_extract() {
        assert_eq!(0,0);
    }
}