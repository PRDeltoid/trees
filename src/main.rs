mod binary_search_tree;

use binary_search_tree::SortOrder;
use binary_search_tree::BinarySearchTree;

fn main() {
    let v = vec![3, 2, 5, 1, 4];
    let tree = BinarySearchTree::new(&v);
    println!("In Order: {}", tree);
    println!{"Pre Order: {:?}", tree.traverse(SortOrder::Preorder)};
    println!("Post Order: {:?}", tree.traverse(SortOrder::Postorder));
}

