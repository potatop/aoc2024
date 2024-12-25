use std::collections::HashSet;

#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq,
{
    pub idx: usize,
    pub val: T,
    // pub parent: Option<usize>,
    pub children: HashSet<usize>,
}

#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub arena: Vec<Node<T>>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            // parent: None,
            children: HashSet::new(),
        }
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self {
            arena: Vec::<Node<T>>::new(),
        }
    }

    pub fn node(&mut self, val: T) -> usize {
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }
}
