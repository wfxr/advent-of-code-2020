use std::collections::HashMap;
use std::io::{self, BufRead};

mod hashmap_solution {
    use super::*;
    #[rustfmt::skip]
    pub fn p1_solve(orbits: &HashMap<String, String>) -> usize {
        let mut cache: HashMap<String, usize> = HashMap::new();
        orbits.iter().map(|(child, _)| orbit_depth(orbits, &mut cache, child)).sum()
    }

    #[rustfmt::skip]
    pub fn p2_solve(orbits: &HashMap<String, String>) -> usize {
        let you_path = orbit_path(orbits, "YOU");
        let san_path = orbit_path(orbits, "SAN");
        let common_path = you_path.iter().rev()
            .zip(san_path.iter().rev())
            .take_while(|(you, san)| you == san)
            .count();
        you_path.len() + san_path.len() - 2 * common_path
    }

    fn orbit_path(orbits: &HashMap<String, String>, star: &str) -> Vec<String> {
        let mut path = Vec::new();
        let mut star = star;
        while let Some(parent) = orbits.get(star) {
            path.push(parent.to_owned());
            star = parent;
        }
        path
    }

    fn orbit_depth(orbits: &HashMap<String, String>, cache: &mut HashMap<String, usize>, star: &str) -> usize {
        match cache.get(star) {
            Some(&depth) => depth,
            None => {
                let depth = match orbits.get(star) {
                    Some(parent) => 1 + orbit_depth(orbits, cache, parent),
                    None => 0,
                };
                cache.insert(star.to_owned(), depth);
                depth
            }
        }
    }
}

mod ntree_solution {
    use super::*;
    use core::cell::RefCell;
    use std::rc::Rc;
    struct TreeNode {
        star:   String,
        childs: Vec<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        fn new(star: String) -> TreeNode {
            TreeNode {
                star,
                childs: Vec::new(),
            }
        }
        fn add(&mut self, child: String) {
            self.childs.push(Rc::new(RefCell::new(TreeNode::new(child))))
        }

        fn add_all(&mut self, childs: Vec<String>) {
            childs.into_iter().for_each(|c| self.add(c))
        }

        fn distance(&self, star: &str) -> Option<usize> {
            if self.star == star {
                Some(0)
            } else {
                match self.childs.iter().filter_map(|c| c.borrow().distance(star)).next() {
                    Some(s) => Some(s + 1),
                    None => None,
                }
            }
        }
    }

    fn transform(orbits: &HashMap<String, String>) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        orbits.iter().for_each(|(child, parent)| {
            map.entry(parent.clone()).or_insert_with(Vec::new).push(child.clone());
        });
        map
    }

    fn build_tree(orbits: &HashMap<String, String>) -> Rc<RefCell<TreeNode>> {
        let orbits = transform(orbits);
        let root = Rc::new(RefCell::new(TreeNode::new("COM".to_owned())));
        do_build_tree(&orbits, root.clone());
        root
    }

    fn do_build_tree(orbits: &HashMap<String, Vec<String>>, root: Rc<RefCell<TreeNode>>) {
        let mut root = root.borrow_mut();
        if let Some(childs) = orbits.get(&root.star) {
            root.add_all(childs.clone());
            for child in &root.childs {
                do_build_tree(orbits, child.clone())
            }
        };
    }

    fn depth_sum(tree: &Rc<RefCell<TreeNode>>, depth: usize) -> usize {
        depth
            + tree
                .borrow()
                .childs
                .iter()
                .map(|child| depth_sum(child, depth + 1))
                .sum::<usize>()
    }

    fn lca(tree: &Rc<RefCell<TreeNode>>, a: &str, b: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let node = tree.borrow();
        if node.star == a || node.star == b {
            return Some(tree.clone());
        }

        let mut it = node.childs.iter().filter_map(|child| lca(child, a, b));
        match (it.next(), it.next()) {
            (Some(_), Some(_)) => Some(tree.clone()),
            (child, None) => child,
            _ => None,
        }
    }

    pub fn p2_solve(orbits: &HashMap<String, String>) -> usize {
        let tree = build_tree(orbits);
        let lca = lca(&tree, "YOU", "SAN").unwrap();
        let lca = lca.borrow();
        lca.distance("YOU").unwrap() + lca.distance("SAN").unwrap() - 2
    }

    pub fn p1_solve(orbits: &HashMap<String, String>) -> usize {
        let tree = build_tree(orbits);
        depth_sum(&tree, 0)
    }
}

#[rustfmt::skip]
fn main() {
    let inputs: HashMap<_, _> = io::stdin().lock().lines().map(|l| l.unwrap())
        .filter_map(|line| {
            let mut it = line.split(')');
            match (it.next(), it.next()) {
                (Some(parent), Some(child)) => Some((child.to_owned(), parent.to_owned())),
                _ => None
            }
        })
        .collect();

    // solution using hashmap
    {
        use hashmap_solution::*;
        let result = p1_solve(&inputs);
        println!("s1 - part1 result: {}", result);
        assert_eq!(117672, result);

        let result = p2_solve(&inputs);
        println!("s1 - part2 result: {}", result);
        assert_eq!(277, result);
    }

    // solution using ntree
    {
        use ntree_solution::*;
        let result = p1_solve(&inputs);
        println!("s2 - part1 result: {}", result);
        assert_eq!(117672, result);

        let result = p2_solve(&inputs);
        println!("s2 - part2 result: {}", result);
        assert_eq!(277, result);
    }
}
