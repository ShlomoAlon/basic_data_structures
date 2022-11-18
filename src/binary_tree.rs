type BinaryTree<T> = Option<Box<Node<T>>>;

pub struct Node<T: Ord> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord + Default> Node<T> {
    fn new(item: T) -> Self {
        Node {
            value: item,
            left: None,
            right: None,
        }
    }
    fn delete_min(mut self) -> (T, BinaryTree<T>) {
        match self.left.take() {
            None => (self.value, self.right),
            Some(left) => {
                let (item, new_left) = left.delete_min();
                self.left = new_left;
                (item, self.into())
            }
        }
    }
}
impl<T: Ord> Into<Option<Box<Node<T>>>> for Node<T> {
    fn into(self) -> Option<Box<Node<T>>> {
        Some(Box::new(self))
    }
}

pub trait BinarySearchTree<T: Ord> {
    fn new() -> Self;
    fn new_from_item(item: T) -> Self;
    fn insert_inner(self, item: T) -> Self;
    fn insert_item(&mut self, item: T);
    fn delete_inner(&mut self, item: T) -> Self;
    fn delete(&mut self, item: T);
    fn contained(&self, item: T) -> bool;
    fn in_order(&self) -> Vec<&T>;
    fn pre_order(&self) -> Vec<&T>;
}

impl<T: Ord> BinarySearchTree<T> for BinaryTree<T> {
    fn new() -> Self {
        None
    }

    fn new_from_item(item: T) -> Self {
        Some(Box::new(Node::new(item)))
    }

    fn insert_inner(self, item: T) -> Self {
        match self {
            None => Self::new_from_item(item),
            Some(mut node) => {
                if item < node.value {
                    node.left = node.left.take().insert_inner(item);
                } else if item > node.value {
                    node.right = node.right.take().insert_inner(item);
                }
                node.into()
            }
        }
    }

    fn insert_item(&mut self, item: T) {
        *self = self.take().insert_inner(item);
    }

    fn delete_inner(&mut self, item: T) -> Self {
        match self.take() {
            None => None,
            Some(mut node) => {
                if item < node.value {
                    node.left = node.left.take().delete_inner(item);
                    node.into()
                } else if item > node.value {
                    node.right = node.right.take().delete_inner(item);
                    node.into()
                } else {
                    match (node.left.take(), node.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => left.into(),
                        (None, Some(right)) => right.into(),
                        (Some(left), Some(right)) => {
                            (node.value, node.right) = right.delete_min();
                            node.left = left.into();
                            node.into()
                        }
                    }
                }
            }
        }
    }

    fn delete(&mut self, item: T) {
        *self = self.take().delete_inner(item);
    }
    fn contained(&self, item: T) -> bool {
        match self {
            None => false,
            Some(node) => {
                if item < node.value {
                    node.left.contained(item)
                } else if item > node.value {
                    node.right.contained(item)
                } else {
                    true
                }
            }
        }
    }

    fn in_order(&self) -> Vec<&T> {
        match self {
            None => vec![],
            Some(node) => {
                let mut left = node.left.in_order();
                left.push(&node.value);
                left.extend(node.right.in_order());
                left
            }
        }
    }

    fn pre_order(&self) -> Vec<&T> {
        match self {
            None => vec![],
            Some(node) => {
                let mut left = vec![&node.value];
                left.extend(node.left.pre_order());
                left.extend(node.right.pre_order());
                left
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_delete_simple() {
        let mut tree = BinaryTree::new_from_item(5);
        tree.insert_item(10);
        assert!(tree.contained(10));
        assert!(tree.contained(5));
        assert!(!tree.contained(6));
        assert!(!tree.contained(8));
    }
    #[test]
    fn test_delete_min() {
        let mut tree = BinaryTree::new_from_item(5);
        tree.insert_item(10);
        tree.insert_item(3);
        tree.insert_item(4);
        tree.insert_item(2);
        tree.insert_item(1);
        tree.insert_item(0);
        tree.insert_item(100);
        let (min, tree) = tree.unwrap().delete_min();
        assert_eq!(tree.in_order(), vec![&1, &2, &3, &4, &5, &10, &100]);
        assert_eq!(min, 0);
    }
    #[test]
    fn test_delete() {
        let mut tree = BinaryTree::new_from_item(5);
        tree.insert_item(10);
        tree.insert_item(6);
        tree.insert_item(8);
        tree.insert_item(7);
        tree.insert_item(9);
        tree.insert_item(4);
        tree.insert_item(3);
        tree.insert_item(2);
        tree.insert_item(1);
        tree.delete(5);
        assert!(!tree.contained(5));
        assert!(tree.contained(10));
        assert!(tree.contained(6));
        assert!(tree.contained(8));
        assert!(tree.contained(7));
        assert!(tree.contained(9));
        assert!(tree.contained(4));
        assert!(tree.contained(3));
        assert!(tree.contained(2));
        assert!(tree.contained(1));
    }
    #[test]
    fn test_delete3() {
        let mut tree = BinaryTree::new_from_item(5);
        tree.insert_item(10);
        tree.insert_item(6);
        tree.insert_item(8);
        tree.insert_item(7);
        tree.insert_item(9);
        tree.insert_item(4);
        tree.insert_item(3);
        tree.insert_item(2);
        tree.insert_item(1);
        tree.delete(10);
        tree.delete(6);
        tree.delete(100);
        tree.delete(8);
        assert_eq!(tree.in_order(), vec![&1, &2, &3, &4, &5, &7, &9]);
    }
}
