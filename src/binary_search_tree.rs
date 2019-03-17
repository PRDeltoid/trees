use std::fmt;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum SortOrder {
    Inorder,
    Preorder,
    Postorder,
}

struct BinaryTreeNode<T>
    where T: PartialOrd + fmt::Display + fmt::Debug + Clone + Copy {
    left: Option<Box<BinaryTreeNode<T>>>,
    right: Option<Box<BinaryTreeNode<T>>>,
    val: T,
}

#[allow(dead_code)]
impl<T> BinaryTreeNode<T>
    where T: PartialOrd + fmt::Display + fmt::Debug + Clone + Copy {
    pub fn new(value: T) -> BinaryTreeNode<T> {
        BinaryTreeNode {
            left: None,
            right: None,
            val: value,
        }
    }

    pub fn get_node_tree(node: &Option<Box<BinaryTreeNode<T>>>, sort: SortOrder) -> Vec<T> {
        let mut nodes = vec![];
        if node.is_none() {
            return nodes;
        }
        match sort {
            SortOrder::Inorder => {
                if let Some(node) = node {
                    if node.left.is_some() {
                        nodes.extend(BinaryTreeNode::get_node_tree(&node.left, sort));
                    }
                    nodes.push(node.val.clone());
                    if node.right.is_some() {
                        nodes.extend(BinaryTreeNode::get_node_tree(&node.right, sort));
                    }
                }
            },
            SortOrder::Preorder => {
                if let Some(node) = node {
                    nodes.push(node.val);
                    if node.left.is_some() {
                        nodes.extend(BinaryTreeNode::get_node_tree(&node.left, sort));
                    }
                    if node.right.is_some() {
                        nodes.extend(BinaryTreeNode::get_node_tree(&node.right, sort));
                    }
                }
            },
            SortOrder::Postorder => {
                if let Some(node) = node {
                    if node.left.is_some() {
                        nodes.extend(BinaryTreeNode::get_node_tree(&node.left, sort));
                    }
                    if node.right.is_some() {
                        nodes.extend(BinaryTreeNode::get_node_tree(&node.right, sort));
                    }
                    //println!("{}", node.val);
                    nodes.push(node.val);
                }
            },
        }
        return nodes;
    }
}

pub struct BinarySearchTree<T>
    where T: PartialOrd + Copy + fmt::Display + fmt::Debug {
    root: Option<Box<BinaryTreeNode<T>>>,
}

#[allow(dead_code)]
impl<T> BinarySearchTree<T>
    where T: PartialOrd + Copy + fmt::Display + fmt::Debug {
    pub fn new(values: &Vec<T>) -> BinarySearchTree<T> {
        let mut tree = BinarySearchTree {
            root: None,
        };
        for value in values {
            BinarySearchTree::insert(&mut tree.root, value.clone());
        }
        tree
    }

    fn insert(node: &mut Option<Box<BinaryTreeNode<T>>>, value: T) {
        match node {
            Some(inner) => {
                if value < inner.val {
                    BinarySearchTree::insert(&mut inner.left, value);
                } else {
                    BinarySearchTree::insert(&mut inner.right, value);
                }
            },
            None => {
                *node = Some(Box::new(BinaryTreeNode::new(value)));
            }
        }
    }

    pub fn traverse(&self, sort: SortOrder) -> Vec<T> {
        let bst = BinaryTreeNode::get_node_tree(&self.root, sort);
        return bst;
    }
}

impl<T> fmt::Display for BinarySearchTree<T>
    where T: PartialOrd + Copy + fmt::Display + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.traverse(SortOrder::Inorder))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
