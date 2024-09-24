use std::io;

    #[derive(Debug)]
    struct Node {
        parent: Option<*mut Node>,
        left: Option<*mut Node>,
        right: Option<*mut Node>,
        key: i32,
    }

    fn insert(root: &mut Option<*mut Node>, key: i32) {
        let new_node = Box::new(Node {
            parent: None,
            left: None,
            right: None,
            key,
        });
        let mut parent_node = None;
        let mut index_node = root;

        while let Some(node) = index_node {
            parent_node = index_node;
            unsafe {
                if new_node.key < (*node).key {
                    index_node = (*node).left;
                } else {
                    index_node = (*node).right;
                }
            }
        }

        let new_node_ptr = Box::into_raw(new_node);
        if let Some(parent) = parent_node {
            unsafe {
                if key < (*parent).key {
                    (*parent).left = Some(new_node_ptr);
                } else {
                    (*parent).right = Some(new_node_ptr);
                }
            }
        } else {
            *root = Some(new_node_ptr);
        }
    }

    fn print(root: Option<*mut Node>) {
        print_in_order(root);
        println!();
        print_pre_order(root);
        println!();
    }

    fn print_pre_order(node: Option<*mut Node>) {
        if let Some(node) = node {
            unsafe {
                print!(" {}", (*node).key);
                print_pre_order((*node).left);
                print_pre_order((*node).right);
            }
        }
    }

    fn print_in_order(node: Option<*mut Node>) {
        if let Some(node) = node {
            unsafe {
                print_in_order((*node).left);
                print!(" {}", (*node).key);
                print_in_order((*node).right);
            }
        }
    }

    fn main() {
        let mut root = None;
        let mut num_commands = String::new();

        io::stdin().read_line(&mut num_commands).unwrap();
        let num_commands: i32 = num_commands.trim().parse().unwrap();

        for _ in 0..num_commands {
            let mut command = String::new();
            io::stdin().read_line(&mut command).unwrap();

            if command.trim() == "insert" {
                let mut insert_key = String::new();
                io::stdin().read_line(&mut insert_key).unwrap();
                let insert_key: i32 = insert_key.trim().parse().unwrap();
                insert(&mut root, insert_key);
            } else {
                print(root);
            }
        }
    }
