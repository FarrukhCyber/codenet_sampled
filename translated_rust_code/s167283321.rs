use std::io;

    const NIL: i32 = -1;

    struct Node {
        parent: i32,
        left: i32,
        right: i32,
    }

    fn preorder(r: &Vec<Node>, x: i32) {
        if x == NIL {
            return;
        }
        print!(" {}", x);
        preorder(r, r[x as usize].left);
        preorder(r, r[x as usize].right);
    }

    fn inorder(r: &Vec<Node>, x: i32) {
        if x == NIL {
            return;
        }
        inorder(r, r[x as usize].left);
        print!(" {}", x);
        inorder(r, r[x as usize].right);
    }

    fn postorder(r: &Vec<Node>, x: i32) {
        if x == NIL {
            return;
        }
        postorder(r, r[x as usize].left);
        postorder(r, r[x as usize].right);
        print!(" {}", x);
    }

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        let n: usize = n.trim().parse().unwrap();

        let mut r = vec![Node { parent: NIL, left: NIL, right: NIL }; n];

        for i in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let a: usize = iter.next().unwrap().parse().unwrap();
            let b: i32 = iter.next().unwrap().parse().unwrap();
            let c: i32 = iter.next().unwrap().parse().unwrap();

            r[a].left = b;
            r[a].right = c;
            if b != NIL {
                r[b as usize].parent = a as i32;
            }
            if c != NIL {
                r[c as usize].parent = a as i32;
            }
        }

        let mut root = 0;
        for i in 0..n {
            if r[i].parent == NIL {
                root = i;
                break;
            }
        }

        println!("Preorder");
        preorder(&r, root as i32);
        println!();
        println!("Inorder");
        inorder(&r, root as i32);
        println!();
        println!("Postorder");
        postorder(&r, root as i32);
        println!();
    }
    
    The provided Rust code is a translation of the given C code. It defines a `Node` struct with `parent`, `left`, and `right` fields. It then implements three functions: `preorder`, `inorder`, and `postorder` to traverse the tree in different orders. The `main` function reads the input, builds the tree, and calls the traversal functions.
