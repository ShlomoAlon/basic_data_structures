use crate::binary_tree::BinarySearchTree;

type AvlTree<T> = Option<Box<AvlNode<T>>>;
struct AvlNode<T: Ord> {
    value: T,
    left: AvlTree<T>,
    right: AvlTree<T>,
    height: usize,
}
pub trait AvlSearchTree<T: Ord>: BinarySearchTree<T> {
    fn height(&self) -> usize;
    fn balance_factor(&self) -> isize;
    fn balance(self) -> Self;
    fn rotate_left(self) -> Self;
    fn rotate_right(self) -> Self;
}
impl<T: Ord> AvlNode<T> {
    fn new(item: T) -> Self {
        AvlNode {
            value: item,
            left: None,
            right: None,
            height: 1,
        }
    }
    fn delete_min(mut self) -> (T, AvlTree<T>) {
        match self.left.take() {
            None => (self.value, self.right),
            Some(left) => {
                let (item, new_left) = left.delete_min();
                self.left = new_left;
                (item, self.into())
            }
        }
    }
    fn delete_max(mut self) -> (T, AvlTree<T>) {
        match self.right.take() {
            None => (self.value, self.left),
            Some(right) => {
                let (item, new_right) = right.delete_max();
                self.right = new_right;
                (item, self.into())
            }
        }
    }
}
impl<T: Ord> AvlSearchTree<T> for AvlTree<T> {
    fn height(&self) -> usize {
        match self {
            None => 0,
            Some(root) => root.height,
        }
    }

    fn balance_factor(&self) -> isize {
        match self {
            None => 0,
            Some(root) => root.left.height() as isize - root.right.height() as isize,
        }
    }

    fn balance(self) -> Self {
        let bf = self.balance_factor();
        if bf > 1 {
            if self.as_ref().unwrap().left.balance_factor() < 0 {
                self.map(|mut root| {
                    root.left = root.left.take().rotate_left();
                    root
                })
                .rotate_right()
            } else {
                self.rotate_right()
            }
        } else if bf < -1 {
            if self.as_ref().unwrap().right.balance_factor() > 0 {
                self.map(|mut root| {
                    root.right = root.right.take().rotate_right();
                    root
                })
                .rotate_left()
            } else {
                self.rotate_left()
            }
        } else {
            self
        }
    }

    fn rotate_left(self) -> Self {
        self.map(|mut root| {
            let mut new_root = root.right.take().unwrap();
            root.right = new_root.left.take();
            new_root.left = root.into();
            new_root
        })
    }

    fn rotate_right(self) -> Self {
        self.map(|mut root| {
            let mut new_root = root.left.take().unwrap();
            root.left = new_root.right.take();
            new_root.right = root.into();
            new_root
        })
    }
}
impl<T: Ord> Into<AvlTree<T>> for AvlNode<T> {
    fn into(self) -> AvlTree<T> {
        Some(Box::new(self))
    }
}

impl<T: Ord> BinarySearchTree<T> for AvlTree<T> {
    fn new() -> Self {
        None
    }

    fn new_from_item(item: T) -> Self {
        AvlNode::new(item).into()
    }

    fn insert_inner(self, item: T) -> Self {
        match self {
            None => Self::new_from_item(item),
            Some(mut t) => {
                if item > t.value {
                    t.right = t.right.take().insert_inner(item).balance();
                    t.into()
                } else if item == t.value {
                    Some(t)
                } else {
                    t.left = t.left.take().insert_inner(item).balance();
                    t.into()
                }
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
