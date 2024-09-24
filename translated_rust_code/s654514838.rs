use std::io;
use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
    key: i32,
    priority: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32, priority: i32) -> Self {
        Node {
            key,
            priority,
            left: None,
            right: None,
        }
    }

    fn find(&self, x: i32) -> bool {
        match self.key.cmp(&x) {
            Ordering::Equal => true,
            Ordering::Less => match &self.right {
                Some(node) => node.find(x),
                None => false,
            },
            Ordering::Greater => match &self.left {
                Some(node) => node.find(x),
                None => false,
            },
        }
    }

    fn inorder(&self) {
        if let Some(ref node) = self.left {
            node.inorder();
        }
        print!(" {}", self.key);
        if let Some(ref node) = self.right {
            node.inorder();
        }
    }

    fn preorder(&self) {
        print!(" {}", self.key);
        if let Some(ref node) = self.left {
            node.preorder();
        }
        if let Some(ref node) = self.right {
            node.preorder();
        }
    }

    fn print(&self) {
        self.inorder();
        println!();
        self.preorder();
        println!();
    }

    fn right_rotate(self) -> Self {
        let mut q = *self.left.unwrap();
        q.right = Some(Box::new(self));
        q
    }

    fn left_rotate(self) -> Self {
        let mut q = *self.right.unwrap();
        q.left = Some(Box::new(self));
        q
    }

    fn insert(mut self, key: i32, priority: i32) -> Self {
        match self.key.cmp(&key) {
            Ordering::Equal => self,
            Ordering::Less => {
                self.right = Some(Box::new(self.right.insert(key, priority)));
                if self.priority < self.right.as_ref().unwrap().priority {
                    self = self.left_rotate();
                }
                self
            }
            Ordering::Greater => {
                self.left = Some(Box::new(self.left.insert(key, priority)));
                if self.priority < self.left.as_ref().unwrap().priority {
                    self = self.right_rotate();
                }
                self
            }
        }
    }

    fn delete(mut self, key: i32) -> Option<Self> {
        match self.key.cmp(&key) {
            Ordering::Less => {
                self.right = self.right.delete(key);
                Some(self)
            }
            Ordering::Greater => {
                self.left = self.left.delete(key);
                Some(self)
            }
            Ordering::Equal => self.delete_node(),
        }
    }

    fn delete_node(self) -> Option<Self> {
        match (self.left, self.right) {
            (None, None) => None,
            (Some(left), None) => Some(*left),
            (None, Some(right)) => Some(*right),
            (Some(left), Some(right)) => {
                if left.priority > right.priority {
                    Some(self.right_rotate().delete(self.key))
                } else {
                    Some(self.left_rotate().delete(self.key))
                }
            }
        }
    }
}

fn main() {
    let mut root = None;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();

    for _ in 0..num {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let com = input.trim();

        match &com[0..1] {
            "i" => {
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let mut iter = input.split_whitespace();
                let x: i32 = iter.next().unwrap().parse().unwrap();
                let y: i32 = iter.next().unwrap().parse().unwrap();
                root = Some(root.insert(x, y));
            }
            "d" => {
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let x: i32 = input.trim().parse().unwrap();
                root = root.delete(x);
            }
            "f" => {
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let x: i32 = input.trim().parse().unwrap();
                if let Some(ref node) = root {
                    if node.find(x) {
                        println!("yes");
                    } else {
                        println!("no");
                    }
                }
            }
            "p" => {
                if let Some(ref node) = root {
                    node.print();
                }
            }
            _ => {}
        }
    }
}

This Rust code is a translation of the given C code. It defines a `Node` struct and implements various methods for it, such as `find`, `inorder`, `preorder`, `print`, `right_rotate`, `left_rotate`, `insert`, `delete`, and `delete_node`. The `main` function reads input from the user and performs the corresponding operations on the tree.
