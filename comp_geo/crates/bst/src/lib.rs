use std::cmp::Ordering::{Greater, Less, Equal};
use std::fmt::Debug;

#[derive(Debug)]
pub struct BST<T> {
    /// Simple Balanced AVL Binary Search Tree
    /// Supports:
    /// - insert
    /// - find 
    /// - delete
    root: Option<Box<BSTNode<T>>>
}

#[derive(Debug)]
struct BSTNode<T> {
    key: T,
    height: u8,
    left: Option<Box<BSTNode<T>>>,
    right: Option<Box<BSTNode<T>>>,
}

impl<T: Ord + Debug> BST<T>{
    pub fn new()  -> Self  {
        Self {
            root: None,
        }
    }

    pub fn insert(&mut self, key: T) {
        if let Some(node) = self.root.take() {
            // We take the root, let it transform itself, 
            // and catch whatever it returns.
            self.root = Some(node.insert(key));
        } else {
            self.root = Some(Box::new(BSTNode::new(key)));
        }
    }

    pub fn find(&self, key: &T) -> Option<&T> {
        // as_ref() converts Option<Box> to Option<&Box>
        self.root.as_ref().and_then(|n| n.find(key))
    }


    pub fn delete(&mut self, key: &T) {
        if let Some(root_node) = self.root.take() {
            self.root = root_node.delete(key);
        }
    }
}

impl<T: Ord + Debug> BSTNode<T> {
    fn new(key: T) -> Self{
        BSTNode {
            key,
            height: 1,
            left: None,
            right: None
        }
    }
    
    fn get_height(node: &Option<Box<BSTNode<T>>>) -> u8 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn balance_factor(&self) -> i8 {
        let h_left = Self::get_height(&self.left);
        let h_right = Self::get_height(&self.right);
        (h_right as i32 - h_left as i32) as i8
    }
    
    fn rebalance(mut self: Box<Self>) -> Box<Self> {
        self.update_height();
        
        let factor = self.balance_factor();
        
        if factor < -1 {
            // Left is too heavy
            if self.left.as_ref().map_or(0, |n| n.balance_factor()) > 0 {
                let left_child = self.left.take().unwrap();
                self.left = Some(left_child.rotate_left());
            }
            self.rotate_right()
        } else if factor > 1 {
            // Right is too heavy
            if self.right.as_ref().map_or(0, |n| n.balance_factor()) < 0 {
                let right_child = self.right.take().unwrap();
                self.right = Some(right_child.rotate_right());
            }
            self.rotate_left()
        } else {
            self // No rotation needed
        }
    }

    fn rotate_left(mut self: Box<Self>) -> Box<Self> {
        // unhook right
        let mut new_root = self.right.take().expect("Rotation needs child");
        self.right = new_root.left.take();
        // re-hook
        new_root.left = Some(self);

        // update heights
        new_root.left.as_mut().unwrap().update_height();
        new_root.update_height();

        new_root
    }

    fn rotate_right(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.left.take().expect("Rotation needs child");
        self.left = new_root.right.take();
        new_root.right = Some(self);

        new_root.right.as_mut().unwrap().update_height();
        new_root.update_height();

        new_root
    }

    fn update_height(&mut self) {
        let h_left = Self::get_height(&self.left);
        let h_right = Self::get_height(&self.right);
        self.height = 1 + h_left.max(h_right);
    }

    fn insert(mut self: Box<Self>, key: T) -> Box<Self>{
        match key.cmp(&self.key) {
            Less => {
                if let Some(left) = self.left.take() {
                    self.left = Some(left.insert(key))
                } else {
                    self.left = Some(Box::new(BSTNode::new(key)));
                }
            }, 
            Greater | Equal => {
                if let Some(right) = self.right.take() {
                    self.right = Some(right.insert(key));
                } else {
                    self.right = Some(Box::new(BSTNode::new(key)));
                }
            } 
        };
        self.rebalance()
    }

    fn delete(mut self: Box<Self>, key: &T) -> Option<Box<Self>> {
        // Take and return
        // takes ownership of node and returns the node
        // that should replace it
        let mut result = match key.cmp(&self.key) {
            Less => {
                if let Some(left) = self.left.take() {
                    self.left = left.delete(key)
                }
                Some(self)
            },
            Greater => {
                 if let Some(right) = self.right.take() {
                    self.right = right.delete(key)
                }
                Some(self)
            },
            Equal => {
                // We need to replace ourself.
                // For this it just means returning something other
                // than ourself — the parent will drop us.
                match (self.left.take(), self.right.take()) {
                    (None, None) => None,               // no children, replace with nothing
                    (Some(left), None) => Some(left),   // replace with left child
                    (None, Some(right)) => Some(right), // replace with right child
                    (Some(left), Some(right)) => {      
                        // difficult case. Find the min member of the right tree
                        // it will be greater than our left tree and less
                        // than everything on the right.
                        let (mut successor, new_right) = right.extract_min();
                        successor.right = new_right;
                        successor.left = Some(left);
                        Some(successor)
                    }
                }
            }
        };
        if let Some(ref mut node) = result {
            node.update_height();
        }
        result
    }
        
    fn extract_min(mut self: Box<Self>) -> (Box<Self>, Option<Box<Self>>) {
        if let Some(left) = self.left.take() {
            let (min_node, new_left) = left.extract_min();
            self.left = new_left;
            self.update_height();
            (min_node, Some(self))
        } else  { 
            let right_child = self.right.take();
            (self, right_child)
        }
    }

    fn find(&self, key: &T) -> Option<&T> {
        match key.cmp(&self.key) {
            Less => self.left.as_ref()?.find(key),
            Equal => Some(&self.key),
            Greater => self.right.as_ref()?.find(key)
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut n = BST::new();
        n.insert(100);
        assert_eq!(n.root.as_ref().unwrap().key, 100);
        assert!(n.root.as_ref().unwrap().left.is_none());
        assert!(n.root.as_ref().unwrap().right.is_none());
    }

    #[test]
    fn insert() {
        let mut n = BST::new();
        n.insert(100);
        n.insert(99);
        assert!(n.root.as_ref().unwrap().left.as_ref().is_some_and(|n| n.key == 99));
        assert!(n.root.as_ref().unwrap().right.is_none());

        n.insert(101);
        assert!(n.root.as_ref().unwrap().right.as_ref().is_some_and(|n| n.key == 101));
    }

    #[test]
    fn find() {
        let mut n = BST::new();
        n.insert(10);
        n.insert(5);
        n.insert(11);
        n.insert(99);
        n.insert(1);

        assert_eq!(n.find(&11), Some(&11));
        assert_eq!(n.find(&1), Some(&1));
        assert_eq!(n.find(&2), None);    
    }

    #[test]
    fn delete_node() {
        let mut n = BST::new();
        n.insert(10);
        n.insert(5);
        n.insert(15);
        n.insert(11);
        n.insert(20);
        n.insert(1);
        assert_eq!(n.find(&10), Some(&10));
        n.delete(&10);
        assert_eq!(n.find(&10), None);
        // should be the min child of right tree
        assert_eq!(&n.root.unwrap().key, &11)
    }
    #[test]    
    fn delete_node_immediate_succesor() {
        // handle case where right child does not 
        // have left tree
        let mut n = BST::new();
        n.insert(10);
        n.insert(5);
        n.insert(15);
        n.insert(20);
        n.delete(&10);
        let root = n.root.as_ref().unwrap();
        assert_eq!(root.key, 15);
        assert_eq!(root.left.as_ref().unwrap().key, 5);
        assert_eq!(root.right.as_ref().unwrap().key, 20);
    }

    #[test]
    fn height_insert() {
        let mut n = BST::new();
        n.insert(10);

        assert_eq!(&n.root.as_ref().unwrap().height, &1);
        n.insert(5);
        assert_eq!(&n.root.as_ref().unwrap().height, &2);

        n.insert(2);
        // rebalances
        assert_eq!(&n.root.as_ref().unwrap().height, &2);
        assert_eq!(&n.root.as_ref().unwrap().key, &5);
       
        n.insert(12);
        assert_eq!(&n.root.as_ref().unwrap().height, &3);
    }

    #[test]
    fn height_delete() {
        let mut n = BST::new();
        n.insert(10);
        n.insert(5);
        n.insert(15);
        n.insert(14);
        n.insert(11);
        assert_eq!(&n.root.as_ref().unwrap().height, &3);
        n.delete(&10);
        assert_eq!(&n.root.as_ref().unwrap().height, &3);
        assert_eq!(&n.root.as_ref().unwrap().key, &11);
    }

    #[test]
    fn balance_factor() {
        let mut n = BST::new();
        n.insert(10);
        assert_eq!(n.root.as_ref().unwrap().balance_factor(), 0);
        n.insert(5);
        assert_eq!(n.root.as_ref().unwrap().balance_factor(), -1);
        n.insert(15);
        assert_eq!(n.root.as_ref().unwrap().balance_factor(), 0);
        n.insert(25);
        assert_eq!(n.root.as_ref().unwrap().balance_factor(), 1);
    }
}
