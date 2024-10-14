/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord + Clone,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord + Clone,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => match self.left {
                Some(ref mut left_child) => left_child.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            },
            Ordering::Greater => match self.right {
                Some(ref mut right_child) => right_child.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            },
            Ordering::Equal => {} // Duplicate values are not inserted
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone,
    {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut root_node) => root_node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut current = self.root.as_ref();
        let mut found = false;

        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => current = node.left.as_ref(),
                Ordering::Greater => current = node.right.as_ref(),
                Ordering::Equal => {
                    found = true;
                    break;
                }
            }
        }

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}