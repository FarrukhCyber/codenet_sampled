use std::io;

    const MAX: usize = 25;

    #[derive(Default)]
    struct Node {
        l: i32,
        r: i32,
        par: i32,
    }

    fn init(n: usize, nodes: &mut [Node]) {
        for i in 0..n {
            nodes[i].par = -1;
            nodes[i].l = -1;
            nodes[i].r = -1;
        }
    }

    fn preorder(p: i32, nodes: &[Node]) {
        if p != -1 {
            print!(" {}", p);
            preorder(nodes[p as usize].l, nodes);
            preorder(nodes[p as usize].r, nodes);
        }
    }

    fn inorder(p: i32, nodes: &[Node]) {
        if p != -1 {
            inorder(nodes[p as usize].l, nodes);
            print!(" {}", p);
            inorder(nodes[p as usize].r, nodes);
        }
    }

    fn postorder(p: i32, nodes: &[Node]) {
        if p != -1 {
            postorder(nodes[p as usize].l, nodes);
            postorder(nodes[p as usize].r, nodes);
            print!(" {}", p);
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut nodes = vec![Node::default(); MAX];
        init(n, &mut nodes);

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace().map(|s| s.parse().unwrap());
            let id: i32 = iter.next().unwrap();
            let l: i32 = iter.next().unwrap();
            let r: i32 = iter.next().unwrap();

            if l != -1 {
                nodes[id as usize].l = l;
                nodes[l as usize].par = id;
            }
            if r != -1 {
                nodes[id as usize].r = r;
                nodes[r as usize].par = id;
            }
        }

        let p = nodes.iter().position(|node| node.par == -1).unwrap() as i32;

        println!("Preorder");
        preorder(p, &nodes);
        println!();

        println!("Inorder");
        inorder(p, &nodes);
        println!();

        println!("Postorder");
        postorder(p, &nodes);
        println!();
    }
