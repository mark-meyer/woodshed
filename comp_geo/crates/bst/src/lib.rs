use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::Debug;

#[derive(Debug)]
pub struct BST<K, V> {
    /// Simple Balanced AVL Binary Search Tree
    /// Supports:
    /// - insert
    /// - find
    /// - delete
    root: Option<Box<BSTNode<K, V>>>,
}

#[derive(Debug)]
struct BSTNode<K, V> {
    key: K,
    value: V,
    height: u8,
    left: Option<Box<BSTNode<K, V>>>,
    right: Option<Box<BSTNode<K, V>>>,
}

impl<K: Ord + Debug, V: Debug> Default for BST<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord + Debug, V: Debug> BST<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(node) = self.root.take() {
            // We take the root, let it transform itself,
            // and catch whatever it returns.
            self.root = Some(node.insert(key, value));
        } else {
            self.root = Some(Box::new(BSTNode::new(key, value)));
        }
    }

    pub fn find(&self, key: &K) -> Option<&V> {
        // as_ref() converts Option<Box> to Option<&Box>
        self.root.as_ref().and_then(|n| n.find(key))
    }

    pub fn neighbors(&self, key: &K) -> (Option<&V>, Option<&V>) {
        match &self.root {
            None => (None, None),
            Some(root) => (root.predecessor(key), root.successor(key))
        }
    }

    pub fn delete(&mut self, key: &K) {
        if let Some(root_node) = self.root.take() {
            self.root = root_node.delete(key);
        }
    }
}

impl<K: Ord + Debug, V: Debug> BSTNode<K, V> {
    fn new(key: K, value: V) -> Self {
        BSTNode {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    // AVL tree
    // Invarient: for every node the balance factor is -1, 0, 1
    // otherwise rebalance is necessary
    fn get_height(node: &Option<Box<Self>>) -> u8 {
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
            self
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

    fn insert(mut self: Box<Self>, key: K, value: V) -> Box<Self> {
        match key.cmp(&self.key) {
            Less => {
                if let Some(left) = self.left.take() {
                    self.left = Some(left.insert(key, value))
                } else {
                    self.left = Some(Box::new(BSTNode::new(key, value)));
                }
            }
            Greater | Equal => {
                if let Some(right) = self.right.take() {
                    self.right = Some(right.insert(key, value));
                } else {
                    self.right = Some(Box::new(BSTNode::new(key, value)));
                }
            }
        };
        self.rebalance()
    }

    fn delete(mut self: Box<Self>, key: &K) -> Option<Box<Self>> {
        // Take and return
        // takes ownership of node and returns the node
        // that should replace it
        let mut result = match key.cmp(&self.key) {
            Less => {
                if let Some(left) = self.left.take() {
                    self.left = left.delete(key)
                }
                Some(self)
            }
            Greater => {
                if let Some(right) = self.right.take() {
                    self.right = right.delete(key)
                }
                Some(self)
            }
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
        } else {
            let right_child = self.right.take();
            (self, right_child)
        }
    }

    fn min(&self) -> &V {
        match &self.left {
            Some(left) => left.min(),
            None => &self.value,
        }
    }

    fn max(&self) -> &V {
        match &self.right {
            Some(right) => right.max(),
            None => &self.value
        }
    }

    fn successor(&self, key: &K) -> Option<&V> {
        // find the value immediately after key in sort order
        // if key is less than any value, return the min
        match key.cmp(&self.key) {
            Equal => self.right.as_ref().map(|n| n.min()), 
            Less => match &self.left{
                Some(left) => left.successor(key).or(Some(&self.value)),
                None => Some(&self.value)
            },
            Greater => self.right.as_ref().and_then(|right| right.successor(key)),
        }
    }

    fn predecessor(&self, key: &K) -> Option<&V> {
        // find the value immediately before key in sort order
        // if key is greater than any value, return the max
        match key.cmp(&self.key) {
            Equal => self.left.as_ref().map(|n| n.max()), 
            Greater => match &self.right{
                Some(right) => right.predecessor(key).or(Some(&self.value)),
                None => Some(&self.value)
            },
            Less => self.left.as_ref().and_then(|left| left.predecessor(key)),
        }
    }

    fn find(&self, key: &K) -> Option<&V> {
        match key.cmp(&self.key) {
            Less => self.left.as_ref()?.find(key),
            Equal => Some(&self.value),
            Greater => self.right.as_ref()?.find(key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_avl_invariants<K: Ord + Debug, V: Debug>(node: &Option<Box<BSTNode<K, V>>>) {
        if let Some(n) = node {
            let h_l = BSTNode::get_height(&n.left);
            let h_r = BSTNode::get_height(&n.right);
            
            // Height must be correctly updated
            assert_eq!(n.height, 1 + h_l.max(h_r));
            
            // Balance factor must be within [-1, 1]
            let diff = (h_r as i32 - h_l as i32).abs();
            assert!(diff <= 1, "Balance factor at key {:?} is {}", n.key, diff);

            assert_avl_invariants(&n.left);
            assert_avl_invariants(&n.right);
        }
    }

     #[test]
    fn complexity_test() {
        let mut n = BST::new();
        for i in 0..100 { n.insert(i, "data"); }
        assert_avl_invariants(&n.root); 
    }

    #[test]
    fn new() {
        let mut n = BST::new();
        n.insert(100, 'a');
        assert_eq!(n.root.as_ref().unwrap().key, 100);
        assert!(n.root.as_ref().unwrap().left.is_none());
        assert!(n.root.as_ref().unwrap().right.is_none());
    }

    #[test]
    fn insert() {
        let mut n = BST::new();
        n.insert(100, "a");
        n.insert(99, "b");
        assert!(n
            .root
            .as_ref()
            .unwrap()
            .left
            .as_ref()
            .is_some_and(|n| n.key == 99));
        assert!(n.root.as_ref().unwrap().right.is_none());

        n.insert(101, "c");
        assert!(n
            .root
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .is_some_and(|n| n.key == 101));
    }

    #[test]
    fn find() {
        let mut n = BST::new();
        n.insert(10, "ten");
        n.insert(5, "five");
        n.insert(11, "eleven");
        n.insert(99, "ninety-nine");
        n.insert(1, "one");

        assert_eq!(n.find(&11).unwrap(), &"eleven");
        assert_eq!(n.find(&1).unwrap(), &"one");
        assert_eq!(n.find(&2), None);
    }

    #[test]
    fn delete_node() {
        let mut n = BST::new();
        n.insert(10, "10");
        n.insert(5, "5");
        n.insert(15, "15");
        n.insert(11, "11");
        n.insert(20, "20");
        n.insert(1, "1");
        assert_eq!(n.find(&10).unwrap(), &"10");
        n.delete(&10);
        assert_eq!(n.find(&10), None);
        // should be the min child of right tree
        assert_eq!(n.root.as_ref().unwrap().key, 11);
        
        assert_avl_invariants(&n.root); 
    }
    #[test]
    fn delete_node_immediate_succesor() {
        // handle case where right child does not
        // have left tree
        let mut n = BST::new();
        n.insert(10, "10");
        n.insert(5, "5");
        n.insert(15, "15");
        n.insert(20, "20");
        n.delete(&10);
        let root = n.root.as_ref().unwrap();
        assert_eq!(root.key, 15);
        assert_eq!(root.left.as_ref().unwrap().key, 5);
        assert_eq!(root.right.as_ref().unwrap().key, 20);
    }

    #[test]
    fn height_insert() {
        let mut n = BST::new();
        n.insert(10, "10");

        assert_eq!(&n.root.as_ref().unwrap().height, &1);
        n.insert(5, "5");
        assert_eq!(&n.root.as_ref().unwrap().height, &2);

        n.insert(2, "2");
        // rebalances
        assert_eq!(&n.root.as_ref().unwrap().height, &2);
        assert_eq!(&n.root.as_ref().unwrap().key, &5);

        n.insert(12, "12");
        assert_eq!(&n.root.as_ref().unwrap().height, &3);

    }

    #[test]
    fn height_delete() {
        let mut n = BST::new();
        n.insert(10, "10");
        n.insert(5, "5");
        n.insert(15, "15");
        n.insert(14, "14");
        n.insert(11, "11");
        assert_eq!(&n.root.as_ref().unwrap().height, &3);
        n.delete(&10);
        assert_eq!(&n.root.as_ref().unwrap().height, &3);
        assert_eq!(&n.root.as_ref().unwrap().key, &11);
    }

    #[test]
    fn balance_factor() {
        let mut n = BST::new();
        n.insert(10, "10");
        assert_eq!(n.root.as_ref().unwrap().balance_factor(), 0);
        n.insert(5, "5");
        assert_eq!(n.root.as_ref().unwrap().balance_factor(), -1);
        n.insert(15, "15");
        assert_eq!(n.root.as_ref().unwrap().balance_factor(), 0);
        n.insert(25, "25");
        assert_eq!(n.root.as_ref().unwrap().balance_factor(), 1);
    }

    #[test]
    fn avl() {
        let mut n = BST::new();
        n.insert(1, "1");
        n.insert(2, "2");
        n.insert(3, "3");
        n.insert(4, "4");
        n.insert(5, "5");
        let root = n.root.as_ref().unwrap();

        assert_avl_invariants(&n.root); 
        assert_eq!(&root.key, &2);
        assert_eq!(&root.left.as_ref().unwrap().key, &1);
        assert_eq!(&root.right.as_ref().unwrap().key, &4)
    }
    #[test]
    fn avl_reverse(){
        let mut n = BST::new();
        n.insert(5, "5");
        n.insert(4, "4");
        n.insert(3, "3");
        n.insert(2, "2");
        n.insert(1, "1");
        let root = n.root.as_ref().unwrap();

        assert_avl_invariants(&n.root); 
        assert_eq!(&root.key, &4);
        assert_eq!(&root.left.as_ref().unwrap().key, &2);
        assert_eq!(&root.right.as_ref().unwrap().key, &5)
    }

    #[test]
    fn min_max() {
        let mut n = BST::new();
        n.insert(1, "1");
        n.insert(2, "2");
        n.insert(3, "3");
        n.insert(4, "4");
        n.insert(5, "5");
        n.insert(5, "6");

        let root = n.root.as_ref().unwrap();
        assert_eq!(root.min(), &"1");
        assert_eq!(root.max(), &"6");
    }

    #[test]
    fn test_succesor() {
        let mut n = BST::new();
        n.insert(2, "2");
        n.insert(1, "1");
        n.insert(3, "3");
        n.insert(4, "4");
        n.insert(5, "5");
        n.insert(6, "6");

        let root = n.root.as_ref().unwrap();
        // when the node isn't found return the min
        assert_eq!(root.successor(&0), Some(&"1"));
        assert_eq!(root.successor(&2), Some(&"3"));
        assert_eq!(root.successor(&1), Some(&"2"));
        assert_eq!(root.successor(&3), Some(&"4"));
        assert_eq!(root.successor(&4), Some(&"5"));
        assert_eq!(root.successor(&5), Some(&"6"));
    }

        #[test]
    fn test_predecessor() {
        let mut n = BST::new();
        n.insert(2, "2");
        n.insert(1, "1");
        n.insert(3, "3");
        n.insert(4, "4");
        n.insert(5, "5");
        n.insert(6, "6");

        let root = n.root.as_ref().unwrap();
        assert_eq!(root.predecessor(&2), Some(&"1"));
        assert_eq!(root.predecessor(&3), Some(&"2"));
        assert_eq!(root.predecessor(&4), Some(&"3"));
        assert_eq!(root.predecessor(&5), Some(&"4"));
        assert_eq!(root.predecessor(&6), Some(&"5"));
        // when we fall off the max end, the max value is the predecessor
        assert_eq!(root.predecessor(&700), Some(&"6"));
    }
}
