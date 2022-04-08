use core::fmt::{Debug, Formatter, Result};

#[derive(Debug, Default, PartialEq)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub arena: Vec<Node<T>>,
}

#[derive(Default, PartialEq)]
pub struct BinaryTree<T>
where
    T: PartialEq,
{
    pub arena: Vec<BinaryNode<T>>,
}

impl<T> Debug for BinaryTree<T>
where
    T: PartialEq + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "BinaryTree {{")?;
        writeln!(f, "arena: vec![")?;
        for entry in self.arena.iter() {
            writeln!(f, "{:?}", entry)?;
            writeln!(f, ",")?;
        }
        writeln!(f, "],")?;
        writeln!(f, "}}")
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<T>
where
    T: PartialEq,
{
    pub idx: usize,
    pub value: Option<T>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(idx: usize, value: Option<T>) -> Self {
        Self {
            idx,
            value,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryNode<T>
where
    T: PartialEq,
{
    pub idx: usize,
    pub value: Option<T>,
    pub depth: usize,
    pub parent: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl<T> BinaryNode<T>
where
    T: PartialEq,
{
    pub fn new(idx: usize, value: Option<T>, depth: usize) -> Self {
        Self {
            idx,
            value,
            depth,
            parent: None,
            left: None,
            right: None,
        }
    }
}
