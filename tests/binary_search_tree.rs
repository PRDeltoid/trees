extern crate libtrees;

use libtrees::binary_search_tree::BinarySearchTree;
use libtrees::binary_search_tree::SortOrder;

#[test]
fn traverse_inorder() {
    let v = vec![3,2,5,1,4];
    let tree = BinarySearchTree::new(&v);
    assert_eq!(tree.traverse(SortOrder::Inorder), vec![1,2,3,4,5])
}

#[test]
fn traverse_preorder() {
    let v = vec![3,2,5,1,4];
    let tree = BinarySearchTree::new(&v);
    assert_eq!(tree.traverse(SortOrder::Preorder), vec![3,2,1,5,4])
}

#[test]
fn traverse_postorder() {
    let v = vec![3,2,5,1,4];
    let tree = BinarySearchTree::new(&v);
    assert_eq!(tree.traverse(SortOrder::Postorder), vec![1,2,4,5,3])
}
