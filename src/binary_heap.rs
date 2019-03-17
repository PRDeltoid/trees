mod binary_heap {
    struct BinaryTreeNode<T>
        where T: PartialOrd + fmt::Display + fmt::Debug + Clone + Copy {
        left: Option<Box<BinaryTreeNode<T>>>,
        right: Option<Box<BinaryTreeNode<T>>>,
        val: T,
    }

}