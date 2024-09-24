use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct Node {
        key: i32,
        p: Option<Rc<RefCell<Node>>>,
        l: Option<Rc<RefCell<Node>>>,
        r: Option<Rc<RefCell<Node>>>,
    }

    type NodePointer = Option<Rc<RefCell<Node>>>;

    fn find(x: NodePointer, k: i32) -> NodePointer {
        let mut x = x;
        while let Some(ref node) = x {
            let node = node.borrow();
            if k == node.key {
                break;
            }
            x = if k < node.key { node.l.clone() } else { node.r.clone() };
        }
        x
    }

    fn insert(r: &mut NodePointer, k: i32) {
        let mut y = None;
        let mut x = r.clone();

        let z = Some(Rc::new(RefCell::new(Node {
            key: k,
            p: None,
            l: None,
            r: None,
        })));

        while let Some(ref node) = x {
            y = x.clone();
            let node = node.borrow();
            x = if k < node.key { node.l.clone() } else { node.r.clone() };
        }

        if let Some(ref mut node) = z {
            node.borrow_mut().p = y.clone();

            if y.is_none() {
                *r = z.clone();
            } else if let Some(ref y_node) = y {
                if k < y_node.borrow().key {
                    y_node.borrow_mut().l = z.clone();
                } else {
                    y_node.borrow_mut().r = z.clone();
                }
            }
        }
    }

    fn inorder(u: NodePointer) {
        if let Some(node) = u {
            inorder(node.borrow().l.clone());
            print!(" {}", node.borrow().key);
            inorder(node.borrow().r.clone());
        }
    }

    fn preorder(u: NodePointer) {
        if let Some(node) = u {
            print!(" {}", node.borrow().key);
            preorder(node.borrow().l.clone());
            preorder(node.borrow().r.clone());
        }
    }

    fn print(r: NodePointer) {
        inorder(r.clone());
        println!();
        preorder(r);
        println!();
    }

    fn main() {
        let mut r = None;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        for _ in 0..n {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let s: Vec<char> = input.trim().chars().collect();

            if s[0] == 'f' {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let x: i32 = input.trim().parse().unwrap();
                let t = find(r.clone(), x);
                if t.is_none() {
                    println!("no");
                } else {
                    println!("yes");
                }
            } else if s[0] == 'i' {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let z: i32 = input.trim().parse().unwrap();
                insert(&mut r, z);
            } else {
                print(r.clone());
            }
        }
    }
