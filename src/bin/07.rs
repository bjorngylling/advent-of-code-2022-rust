use id_tree::InsertBehavior::*;
use id_tree::*;

#[derive(Debug)]
struct File {
    n: String,
    s: u32,
    dir: bool,
}

impl File {
    fn new(name: String, size: u32, dir: bool) -> Self {
        File {
            n: name,
            s: size,
            dir,
        }
    }
}

fn build_tree(input: &str) -> Tree<File> {
    let mut tree = TreeBuilder::new().with_node_capacity(20).build();
    let mut cur = tree
        .insert(Node::new(File::new("/".to_string(), 0, true)), AsRoot)
        .unwrap()
        .clone();
    let root = cur.clone();
    for l in input.lines() {
        let mut l = l.split_whitespace().collect::<Vec<&str>>();
        if l.len() == 2 {
            l.push("");
        }
        match l[0..=2] {
            ["$", "ls", ""] => (),
            ["$", "cd", "/"] => cur = root.clone(),
            ["$", "cd", ".."] => cur = tree.ancestor_ids(&cur).unwrap().next().unwrap().clone(),
            ["$", "cd", d] => {
                cur = tree
                    .children_ids(&cur)
                    .unwrap()
                    .find(|n| tree.get(n).unwrap().data().n == d.to_string())
                    .unwrap()
                    .clone()
            }
            ["dir", d, ""] => {
                tree.insert(
                    Node::new(File::new(d.to_string(), 0, true)),
                    UnderNode(&cur),
                )
                .expect("missing node");
            }
            [s, d, ""] => {
                tree.insert(
                    Node::new(File::new(d.to_string(), s.parse().unwrap(), false)),
                    UnderNode(&cur),
                )
                .expect("missing node");
            }
            _ => (),
        }
    }

    for id in tree.traverse_post_order_ids(&root).unwrap() {
        let n = tree.get(&id).unwrap();
        if n.data().dir {
            let s: u32 = tree.children(&id).unwrap().map(|n| n.data().s).sum();
            tree.get_mut(&id).unwrap().data_mut().s = s;
        }
    }

    tree
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree = build_tree(input);
    let root = tree.root_node_id().unwrap();

    Some(
        tree.traverse_post_order(&root)
            .unwrap()
            .filter(|n| n.data().dir)
            .map(|n| n.data().s)
            .filter(|&s| s < 100_000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree = build_tree(input);
    let root = tree.root_node_id().unwrap();

    let min_del = 30_000_000 - (70_000_000 - tree.get(&root).unwrap().data().s);
    let mut l = tree
        .traverse_post_order(&root)
        .unwrap()
        .filter(|n| n.data().dir)
        .map(|n| n.data().s)
        .collect::<Vec<_>>();
    l.sort();
    Some(*l.iter().find(|&&s| s >= min_del).unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
