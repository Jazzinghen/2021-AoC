use std::collections::HashMap;
use std::fmt::{self, Debug};

use itertools::Itertools;

#[derive(Debug, Default, PartialEq)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub arena: HashMap<usize, Node<T>>,
}

#[derive(Default, PartialEq)]
pub struct BinaryTree<T>
where
    T: PartialEq,
{
    pub arena: HashMap<usize, BinaryNode<T>>,
}

impl<T> fmt::Debug for BinaryTree<T>
where
    T: PartialEq + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BinaryTree {{")?;
        writeln!(f, "arena: HashMap::from([")?;
        for (key, entry) in self
            .arena
            .iter()
            .sorted_by(|(first, _), (second, _)| Ord::cmp(first, second))
        {
            writeln!(f, "\t(")?;
            writeln!(f, "\t\t{},", key)?;
            writeln!(f, "{:?}", entry)?;
            writeln!(f, "\t),")?;
        }
        writeln!(f, "]),")?;
        writeln!(f, "}}")
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<T>
where
    T: PartialEq,
{
    pub idx: usize,
    pub value: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(idx: usize, value: T) -> Self {
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
    pub value: T,
    pub parent: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl<T> BinaryNode<T>
where
    T: PartialEq,
{
    pub fn new(idx: usize, value: T) -> Self {
        Self {
            idx,
            value,
            parent: None,
            left: None,
            right: None,
        }
    }
}
