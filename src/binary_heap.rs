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


    pub fn get_left(&self, i: usize) -> Option<T> {
        self.heap.get(2*i+1).cloned()
    }

    pub fn get_right(&self, i: usize) -> Option<T> {
        self.heap.get(2*i+2).cloned()
    }

    pub fn get_left_index(&self, i: usize) -> usize {
        2*i+1 as usize
    }
    pub fn get_right_index(&self, i: usize) -> usize {
        2*i+2 as usize
    }

    pub fn get_parent_index(&self, i: usize) -> usize {
        (i-1)/2 as usize
    }

    pub fn get_parent(&self, i: usize) -> T {
        if i==0  {
            panic!("Cannot get parent of root node");
        }
        self.heap.get((i-1)/2).unwrap().clone()
    }

    pub fn insert(&mut self, value: T) {
        self.heap.push(value);

        //Bubble up
        let mut i: usize = self.heap.len()-1;
        loop {
            if i == 0 {
                break;
            }

            let parent_i = self.get_parent_index(i);
            //min-heap (minimum val at root)
            if self.get_parent(i) < value {
                //Swap the new value and it's parent
                i = self.get_parent_index(i);
                continue;
            }
            self.heap.swap(i, parent_i); //self.get_parent_index(i));
            break;
        }
    }

    pub fn extract(&mut self) -> Option<T> {
        //If the heap is empty, return None
        if self.heap.len() == 0 {
            return None;
        }
        //Save our root value
        let value = self.heap[0];

        //Move the lowest node on the tree to the top
        self.heap[0] = self.heap[self.heap.len()-1];
        let _trash = self.heap.pop();

        //Percolate down
        let mut i: usize = 0;

        loop {
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
            if smallest != i  {
                //println!("Swapping with {}", self.heap[smallest]);
                self.heap.swap(i, smallest);
                i = smallest;
                continue;
            } else {
                //println!("Nothing smaller found, placing at node {}", smallest);
                break;
            }

        }

        return Some(value);

    }
}

impl<T> fmt::Display for BinaryHeap<T>
    where T: PartialOrd + Copy + fmt::Display + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.heap)
    }
}
