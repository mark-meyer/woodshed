use std::cmp::Ordering::{Greater, Less, Equal};

#[derive(Debug)]
pub struct BST<T> {
    /// Simple Binary Search Tree
    /// No attempt at keeping it balanced
    /// Supports:
    /// - insert
    /// - find 
    /// - delete
    root: Option<Box<BSTNode<T>>>
}

#[derive(Debug)]
struct BSTNode<T> {
    key: T,
    left: Option<Box<BSTNode<T>>>,
    right: Option<Box<BSTNode<T>>>,
}

impl<T: Ord> BST<T>{
    pub fn new()  -> Self  {
        Self {
            root: None
        }
    }

    pub fn insert(&mut self, key: T) {
        match self.root {
            None => {self.root = Some(Box::new(BSTNode::new(key)))},
            Some(ref mut node) => {node.insert(key);}
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

impl<T: Ord> BSTNode<T> {
    fn new(key: T) -> Self{
        BSTNode {
            key,
            left: None,
            right: None
        }
    }
    
    fn insert(&mut self, key: T) {
        let target = match key.cmp(&self.key) {
            Less            => &mut self.left,
            Greater | Equal => &mut self.right
        };
        match target {
            Some(node) => node.insert(key),
            None       => *target = Some(Box::new(BSTNode::new(key)))
        };
    }

    fn delete(mut self: Box<Self>, key: &T) -> Option<Box<Self>> {
        // Take and return
        // takes ownership of node and returns the node
        // that should replace it
        match key.cmp(&self.key) {
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
        }
    }
        
    fn extract_min(mut self: Box<Self>) -> (Box<Self>, Option<Box<Self>>) {
        if let Some(left) = self.left.take() {
            let (min_node, new_left) = left.extract_min();
            self.left = new_left;
            (min_node, Some(self))
        } else  { 
            // this is the min node
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
}
