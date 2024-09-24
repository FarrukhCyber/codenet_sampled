use std::cell::RefCell;
    use std::rc::Rc;

    struct Node {
        key: u32,
        next: Option<Rc<RefCell<Node>>>,
        prev: Option<Rc<RefCell<Node>>>,
    }

    type NodePointer = Option<Rc<RefCell<Node>>>;

    fn list_search(key: u32, nil: &NodePointer) -> NodePointer {
        let mut cur = nil.as_ref().unwrap().borrow().next.clone();
        while let Some(node) = cur.clone() {
            if node.borrow().key == key {
                break;
            }
            cur = node.borrow().next.clone();
        }
        cur
    }

    fn init() -> NodePointer {
        let nil = Rc::new(RefCell::new(Node {
            key: 0,
            next: None,
            prev: None,
        }));
        let nil_clone = Some(Rc::clone(&nil));
        nil.borrow_mut().next = nil_clone.clone();
        nil.borrow_mut().prev = nil_clone;
        Some(nil)
    }

    fn print_list(nil: &NodePointer) {
        let mut cur = nil.as_ref().unwrap().borrow().next.clone();
        let mut isf = true;
        loop {
            if cur.is_none() {
                break;
            }
            if !isf {
                print!(" ");
            }
            print!("{}", cur.as_ref().unwrap().borrow().key);
            cur = cur.as_ref().unwrap().borrow().next.clone();
            isf = false;
        }
        println!();
    }

    fn delete_node(t: NodePointer) {
        if let Some(node) = t {
            let prev = node.borrow().prev.clone();
            let next = node.borrow().next.clone();
            if let Some(prev_node) = prev {
                prev_node.borrow_mut().next = next.clone();
            }
            if let Some(next_node) = next {
                next_node.borrow_mut().prev = prev;
            }
        }
    }

    fn delete_first(nil: &NodePointer) {
        let t = nil.as_ref().unwrap().borrow().next.clone();
        if t.is_some() {
            delete_node(t);
        }
    }

    fn delete_last(nil: &NodePointer) {
        let t = nil.as_ref().unwrap().borrow().prev.clone();
        if t.is_some() {
            delete_node(t);
        }
    }

    fn delete_key(key: u32, nil: &NodePointer) {
        let t = list_search(key, nil);
        if t.is_some() {
            delete_node(t);
        }
    }

    fn insert(key: u32, nil: &NodePointer) {
        let x = Rc::new(RefCell::new(Node {
            key,
            prev: None,
            next: None,
        }));
        let next = nil.as_ref().unwrap().borrow().next.clone();
        x.borrow_mut().next = next.clone();
        x.borrow_mut().prev = Some(Rc::clone(nil));
        if let Some(next_node) = next {
            next_node.borrow_mut().prev = Some(Rc::clone(&x));
        }
        nil.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&x));
    }

    fn main() {
        let mut nil = init();
        let mut size = 0;
        let mut np = 0;
        let mut nd = 0;
        let n = 5; // replace with the actual number of operations
        for _ in 0..n {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let com = parts[0];
            let key = parts[1].parse::<u32>().unwrap();
            if com == "i" {
                insert(key, &nil);
                np += 1;
                size += 1;
            } else if com == "d" {
                if com.len() > 6 {
                    if &com[6..] == "F" {
                        delete_first(&nil);
                    } else if &com[6..] == "L" {
                        delete_last(&nil);
                    }
                } else {
                    delete_key(key, &nil);
                    nd += 1;
                }
                size -= 1;
            }
        }
        print_list(&nil);
    }
