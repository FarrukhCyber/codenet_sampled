use std::io;

    struct Node {
        key: u32,
        next: Option<Box<Node>>,
        prev: Option<Box<Node>>,
    }

    type NodePointer = Option<Box<Node>>;

    fn list_search(key: u32, nil: &NodePointer) -> NodePointer {
        let mut cur = nil.as_ref().and_then(|node| node.next.as_ref());
        while let Some(node) = cur {
            if node.key == key {
                return Some(node.clone());
            }
            cur = node.next.as_ref();
        }
        None
    }

    fn init() -> NodePointer {
        let nil = Box::new(Node {
            key: 0,
            next: None,
            prev: None,
        });
        let mut nil_mut = Some(nil);
        nil_mut.as_mut().unwrap().next = nil_mut.clone();
        nil_mut.as_mut().unwrap().prev = nil_mut.clone();
        nil_mut
    }

    fn print_list(nil: &NodePointer) {
        let mut cur = nil.as_ref().and_then(|node| node.next.as_ref());
        let mut isf = true;
        while let Some(node) = cur {
            if !isf {
                print!(" ");
            }
            print!("{}", node.key);
            cur = node.next.as_ref();
            isf = false;
        }
        println!();
    }

    fn delete_node(t: &mut NodePointer) {
        if let Some(node) = t.take() {
            if let Some(prev) = node.prev.as_mut() {
                prev.next = node.next.clone();
            }
            if let Some(next) = node.next.as_mut() {
                next.prev = node.prev.clone();
            }
        }
    }

    fn delete_first(nil: &mut NodePointer) {
        if let Some(node) = nil.as_mut().and_then(|node| node.next.take()) {
            delete_node(&mut node.next);
        }
    }

    fn delete_last(nil: &mut NodePointer) {
        if let Some(node) = nil.as_mut().and_then(|node| node.prev.take()) {
            delete_node(&mut node.prev);
        }
    }

    fn delete(key: u32, nil: &mut NodePointer) {
        if let Some(node) = list_search(key, nil) {
            delete_node(&mut Some(node));
        }
    }

    fn insert(key: u32, nil: &mut NodePointer) {
        let mut x = Box::new(Node {
            key,
            next: None,
            prev: None,
        });
        if let Some(nil_node) = nil.as_mut() {
            x.next = nil_node.next.take();
            if let Some(next_node) = x.next.as_mut() {
                next_node.prev = Some(x.clone());
            }
            nil_node.next = Some(x.clone());
            x.prev = Some(nil_node.clone());
        }
    }

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        let n: usize = n.trim().parse().unwrap();

        let mut nil = init();

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.split_whitespace();
            let com = parts.next().unwrap();
            let key: u32 = parts.next().unwrap().parse().unwrap();

            match com {
                "i" => {
                    insert(key, &mut nil);
                }
                "d" => {
                    if com.len() > 6 {
                        if &com[6..] == "F" {
                            delete_first(&mut nil);
                        } else if &com[6..] == "L" {
                            delete_last(&mut nil);
                        }
                    } else {
                        delete(key, &mut nil);
                    }
                }
                _ => {}
            }
        }

        print_list(&nil);
    }
