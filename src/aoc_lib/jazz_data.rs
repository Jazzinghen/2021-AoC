use core::fmt::Debug;
use std::{collections::VecDeque, f32::NEG_INFINITY};

use hashbrown::HashSet;
use itertools::Itertools;

#[derive(Debug, Default, PartialEq)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub arena: Vec<Node<T>>,
}

#[derive(Debug, Default, PartialEq)]
pub struct BinaryTree<T>
where
    T: PartialEq,
{
    pub arena: Vec<BinaryNode<T>>,
    available_nodes: VecDeque<usize>,
}

impl<T> BinaryTree<T>
where
    T: PartialEq,
{
    pub fn new(arena: Vec<BinaryNode<T>>, available_idx: Vec<usize>) -> Self {
        Self {
            arena,
            available_nodes: available_idx.into_iter().sorted_unstable().collect(),
        }
    }

    pub fn remove_node(&mut self, idx: usize) {
        // Remove reference to current node from its parent
        if let Some(parent) = self.arena[idx].parent {
            let parent_node = self.arena.get_mut(parent).unwrap();
            if parent_node.left.map_or(false, |left| left == idx) {
                parent_node.left = None;
            } else if parent_node.right.map_or(false, |right| right == idx) {
                parent_node.right = None;
            }
        }

        let mut remove_front: Vec<usize> = vec![idx];
        // Then traverse all the subtree from the node to remove to check for children
        while let Some(current_idx) = remove_front.pop() {
            // Add the node idx as available
            self.available_nodes.push_back(current_idx);
            let current_node = &self.arena[current_idx];
            if let Some(left) = current_node.left {
                remove_front.push(left);
            }
            if let Some(right) = current_node.right {
                remove_front.push(right);
            }
        }
    }

    pub fn add_new_node(&mut self, new_node: BinaryNode<T>) -> usize {
        let new_idx = if let Some(idx) = self.available_nodes.pop_front() {
            self.arena[idx] = new_node;
            idx
        } else {
            self.arena.push(new_node);
            self.arena.len() - 1
        };

        self.arena[new_idx].idx = new_idx;
        new_idx
    }

    pub fn get_free_nodes(&self) -> &VecDeque<usize> {
        &self.available_nodes
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
