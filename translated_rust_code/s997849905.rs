use std::io;
    use std::fmt;

    struct Node {
        key: i32,
        next: Option<Box<Node>>,
        prev: Option<Box<Node>>,
    }

    impl Node {
        fn new(key: i32) -> Node {
            Node {
                key,
                next: None,
                prev: None,
            }
        }
    }

    impl fmt::Display for Node {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    struct List {
        head: Node,
    }

    impl List {
        fn new() -> List {
            let head = Node::new(0);
            List {
                head: head,
            }
        }

        fn print_list(&self) {
            let mut cur = &self.head.next;
            let mut snl = true;
            while let Some(node) = cur {
                if !snl {
                    print!(" ");
                }
                print!("{}", node);
                cur = &node.next;
                snl = false;
            }
            println!();
        }

        fn delete_first(&mut self) {
            if let Some(node) = self.head.next.take() {
                if let Some(next_node) = node.next {
                    next_node.prev = None;
                    self.head.next = Some(next_node);
                }
            }
        }

        fn delete_last(&mut self) {
            let mut cur = &self.head.next;
            while let Some(node) = cur {
                if let Some(next_node) = &node.next {
                    if next_node.is_none() {
                        node.next = None;
                        break;
                    }
                }
                cur = &node.next;
            }
        }

        fn delete(&mut self, skey: i32) {
            let mut cur = &self.head.next;
            while let Some(node) = cur {
                if node.key == skey {
                    if let Some(prev_node) = &node.prev {
                        if let Some(next_node) = &node.next {
                            prev_node.next = Some(next_node.clone());
                            next_node.prev = Some(prev_node.clone());
                        } else {
                            prev_node.next = None;
                        }
                    }
                    break;
                }
                cur = &node.next;
            }
        }

        fn insert(&mut self, skey: i32) {
            let new = Node::new(skey);
            if let Some(next_node) = self.head.next.take() {
                new.next = Some(next_node.clone());
                next_node.prev = Some(Box::new(new.clone()));
                self.head.next = Some(Box::new(new));
            } else {
                self.head.next = Some(Box::new(new));
            }
        }
    }

    fn main() {
        let mut list = List::new();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        for _ in 0..n {
            let mut command = String::new();
            io::stdin().read_line(&mut command).unwrap();
            let command = command.trim();

            if command == "insert" {
                let mut key = String::new();
                io::stdin().read_line(&mut key).unwrap();
                let key: i32 = key.trim().parse().unwrap();
                list.insert(key);
            } else if command == "deleteFirst" {
                list.delete_first();
            } else if command == "deleteLast" {
                list.delete_last();
            } else {
                let mut key = String::new();
                io::stdin().read_line(&mut key).unwrap();
                let key: i32 = key.trim().parse().unwrap();
                list.delete(key);
            }
        }

        list.print_list();
    }
