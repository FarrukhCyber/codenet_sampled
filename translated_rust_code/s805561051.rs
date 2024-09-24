use std::io;

    #[derive(Debug)]
    struct BT {
        parent: i32,
        right: i32,
        left: i32,
    }

    fn preorder(bt: &Vec<BT>, a: i32) {
        if a == -1 {
            return;
        }

        print!(" {}", a);
        preorder(bt, bt[a as usize].left);
        preorder(bt, bt[a as usize].right);
    }

    fn inorder(bt: &Vec<BT>, a: i32) {
        if a == -1 {
            return;
        }

        inorder(bt, bt[a as usize].left);
        print!(" {}", a);
        inorder(bt, bt[a as usize].right);
    }

    fn postorder(bt: &Vec<BT>, a: i32) {
        if a == -1 {
            return;
        }

        postorder(bt, bt[a as usize].left);
        postorder(bt, bt[a as usize].right);
        print!(" {}", a);
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut bt = vec![BT { parent: -1, right: -1, left: -1 }; n];

        for i in 0..n {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let num: usize = input.trim().parse().unwrap();

            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let split = input.split_whitespace().collect::<Vec<&str>>();
            let left: i32 = split[0].parse().unwrap();
            let right: i32 = split[1].parse().unwrap();

            bt[num].left = left;
            bt[num].right = right;

            if left != -1 && right != -1 {
                bt[left as usize].parent = num as i32;
                bt[right as usize].parent = num as i32;
            } else if left != -1 {
                bt[left as usize].parent = num as i32;
            } else if right != -1 {
                bt[right as usize].parent = num as i32;
            }
        }

        let mut i = 0;
        while bt[i].parent != -1 {
            i += 1;
        }

        println!("Preorder");
        preorder(&bt, i as i32);
        println!();

        println!("Inorder");
        inorder(&bt, i as i32);
        println!();

        println!("Postorder");
        postorder(&bt, i as i32);
        println!();
    }
    `
    The provided Rust code is a safe translation of the given C code. It defines a struct `BT` to represent the nodes of a binary tree, and implements three functions `preorder`, `inorder`, and `postorder` to traverse the tree in different orders. The `main` function reads the number of nodes in the tree and their left and right children, constructs the tree, and then calls the three traversal functions to print the nodes in preorder, inorder, and postorder.
