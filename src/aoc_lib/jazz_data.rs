#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub arena: Vec<Node<T>>,
}

#[derive(Debug)]
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
