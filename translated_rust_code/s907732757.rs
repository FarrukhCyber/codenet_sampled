use std::io;

    struct Node {
        key: i32,
        next: Option<Box<Node>>,
        prev: Option<Box<Node>>,
    }

    impl Node {
        fn new(key: i32) -> Self {
            Node {
                key,
                next: None,
                prev: None,
            }
        }
    }

    struct DoublyLinkedList {
        head: Option<Box<Node>>,
    }

    impl DoublyLinkedList {
        fn new() -> Self {
            DoublyLinkedList { head: None }
        }

        fn insert(&mut self, key: i32) {
            let mut new_node = Box::new(Node::new(key));

            match &mut self.head {
                Some(head) => {
                    new_node.next = Some(head.next.take());
                    new_node.prev = Some(head.clone());

                    if let Some(next_node) = &mut new_node.next {
                        next_node.prev = Some(new_node.clone());
                    }

                    head.next = Some(new_node);
                }
                None => {
                    self.head = Some(new_node);
                }
            }
        }

        fn delete(&mut self, key: i32) {
            let mut current = &mut self.head;

            while let Some(node) = current {
                if node.key == key {
                    if let Some(prev_node) = &mut node.prev {
                        prev_node.next = node.next.take();
                    }

                    if let Some(next_node) = &mut node.next {
                        next_node.prev = node.prev.take();
                    }

                    break;
                }

                current = &mut node.next;
            }
        }

        fn delete_first(&mut self) {
            if let Some(head) = &mut self.head {
                self.head = head.next.take();

                if let Some(next_node) = &mut self.head {
                    next_node.prev = None;
                }
            }
        }

        fn delete_last(&mut self) {
            let mut current = &mut self.head;

            while let Some(node) = current {
                if node.next.is_none() {
                    if let Some(prev_node) = &mut node.prev {
                        prev_node.next = None;
                    }

                    break;
                }

                current = &mut node.next;
            }
        }

        fn print_list(&self) {
            let mut current = &self.head;

            while let Some(node) = current {
                print!("{}", node.key);

                if let Some(next_node) = &node.next {
                    print!(" ");
                }

                current = &node.next;
            }

            println!();
        }
    }

    fn main() {
        let mut list = DoublyLinkedList::new();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        for _ in 0..n {
            let mut op = String::new();
            io::stdin().read_line(&mut op).unwrap();

            match op.trim() {
                "insert" => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let a: i32 = input.trim().parse().unwrap();
                    list.insert(a);
                }
                "delete" => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let a: i32 = input.trim().parse().unwrap();
                    list.delete(a);
                }
                "deleteFirst" => {
                    list.delete_first();
                }
                "deleteLast" => {
                    list.delete_last();
                }
                _ => {}
            }
        }

        list.print_list();
    }
