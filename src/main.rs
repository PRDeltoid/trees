mod binary_search_tree;
mod binary_heap;

use binary_heap::BinaryHeap;

fn main() {

    let mut bin: BinaryHeap<u32> = BinaryHeap::new();

    bin.insert(1);
    bin.insert(5);
    bin.insert(10);
    bin.insert(3);

    println!("{}", bin);

    bin.insert(2);
    bin.insert(6);
    bin.insert(7);
    bin.insert(20);

    println!("{}", bin);

    if let Some(val) = bin.extract() {
        println!("{:?}", val);
    }
    println!("{}", bin);

    if let Some(val) = bin.extract() {
        println!("{:?}", val);
    }

    println!("{}", bin);

    if let Some(val) = bin.extract() {
        println!("{:?}", val);
    }

    println!("{}", bin);
}

