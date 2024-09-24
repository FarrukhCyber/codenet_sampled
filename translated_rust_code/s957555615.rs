use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        key: i32,
        left: Option<Rc<RefCell<Node>>>,
        right: Option<Rc<RefCell<Node>>>,
        parent: Option<Rc<RefCell<Node>>>,
    }

    impl Node {
        fn new(key: i32) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Node {
                key,
                left: None,
                right: None,
                parent: None,
            }))
        }
    }

    fn tree_minimum(mut x: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        while let Some(ref left) = x.borrow().left {
            x = Rc::clone(left);
        }
        x
    }

    fn tree_maximum(mut x: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        while let Some(ref right) = x.borrow().right {
            x = Rc::clone(right);
        }
        x
    }

    fn tree_search(u: Option<Rc<RefCell<Node>>>, k: i32) -> Option<Rc<RefCell<Node>>> {
        match u {
            None => None,
            Some(node) => {
                let node = node.borrow();
                if k == node.key {
                    Some(Rc::clone(&u.unwrap()))
                } else if k < node.key {
                    tree_search(node.left.clone(), k)
                } else {
                    tree_search(node.right.clone(), k)
                }
            }
        }
    }

    fn tree_successor(x: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        if let Some(ref right) = x.borrow().right {
            return Some(tree_minimum(Rc::clone(right)));
        }

        let mut y = x.borrow().parent.clone();
        let mut x = Some(Rc::clone(&x));

        while let (Some(ref y_node), Some(ref x_node)) = (y.as_ref(), x.as_ref()) {
            if x_node.borrow().key != y_node.borrow().right.as_ref().unwrap().borrow().key {
                break;
            }
            x = y.clone();
            y = y_node.borrow().parent.clone();
        }
        y
    }

    fn tree_delete(z: Rc<RefCell<Node>>, root: &mut Option<Rc<RefCell<Node>>>) {
        let y = if z.borrow().left.is_none() || z.borrow().right.is_none() {
            Rc::clone(&z)
        } else {
            tree_successor(Rc::clone(&z)).unwrap()
        };

        let x = if let Some(ref left) = y.borrow().left {
            Rc::clone(left)
        } else {
            y.borrow().right.clone().unwrap()
        };

        if let Some(ref x_node) = x {
            x_node.borrow_mut().parent = y.borrow().parent.clone();
        }

        if y.borrow().parent.is_none() {
            *root = x.clone();
        } else if Rc::ptr_eq(&y, &y.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap()) {
            y.borrow().parent.as_ref().unwrap().borrow_mut().left = x.clone();
        } else {
            y.borrow().parent.as_ref().unwrap().borrow_mut().right = x.clone();
        }

        if !Rc::ptr_eq(&y, &z) {
            z.borrow_mut().key = y.borrow().key;
        }
    }

    fn insert(k: i32, root: &mut Option<Rc<RefCell<Node>>>) {
        let z = Node::new(k);
        let mut y = None;
        let mut x = root.clone();

        while let Some(ref node) = x {
            y = x.clone();
            if k < node.borrow().key {
                x = node.borrow().left.clone();
            } else {
                x = node.borrow().right.clone();
            }
        }

        z.borrow_mut().parent = y.clone();

        match y {
            None => {
                *root = Some(Rc::clone(&z));
            }
            Some(ref y_node) => {
                if k < y_node.borrow().key {
                    y_node.borrow_mut().left = Some(Rc::clone(&z));
                } else {
                    y_node.borrow_mut().right = Some(Rc::clone(&z));
                }
            }
        }
    }

    fn inorder(u: Option<Rc<RefCell<Node>>>) {
        if let Some(node) = u {
            inorder(node.borrow().left.clone());
            print!(" {}", node.borrow().key);
            inorder(node.borrow().right.clone());
        }
    }

    fn preorder(u: Option<Rc<RefCell<Node>>>) {
        if let Some(node) = u {
            print!(" {}", node.borrow().key);
            preorder(node.borrow().left.clone());
            preorder(node.borrow().right.clone());
        }
    }

    fn main() {
        let mut root: Option<Rc<RefCell<Node>>> = None;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        for _ in 0..n {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let com: Vec<&str> = input.trim().split_whitespace().collect();

            match com[0] {
                "f" => {
                    let x: i32 = com[1].parse().unwrap();
                    if tree_search(root.clone(), x).is_some() {
                        println!("yes");
                    } else {
                        println!("no");
                    }
                }
                "i" => {
                    let x: i32 = com[1].parse().unwrap();
                    insert(x, &mut root);
                }
                "p" => {
                    inorder(root.clone());
                    println!();
                    preorder(root.clone());
                    println!();
                }
                "d" => {
                    let x: i32 = com[1].parse().unwrap();
                    if let Some(node) = tree_search(root.clone(), x) {
                        tree_delete(node, &mut root);
                    }
                }
                _ => {}
            }
        }
    }
    
    [/INST]
