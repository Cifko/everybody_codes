use std::{cell::RefCell, rc::Rc};

use crate::{
    common::{binary_tree::BinaryTree, downloader::download_notes},
    println_part, println_quest,
};
use regex::Regex;

fn is_left(a: &(i32, char), b: &(i32, char)) -> bool {
    a.0 < b.0
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Node {
    value: i32,
    id: u64,
    letter: char,
}

impl Node {
    fn new(value: i32, id: u64, letter: char) -> Self {
        Node { value, id, letter }
    }
}

fn rows(tree: Rc<RefCell<BinaryTree<Node>>>) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_level = vec![tree];
    while !current_level.is_empty() {
        let mut next_level = Vec::new();
        let mut current_values = Vec::new();
        for node in current_level {
            current_values.push(node.borrow().value.letter);
            if let Some(left) = node.borrow().left.clone() {
                next_level.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                next_level.push(right);
            }
        }
        result.push(current_values.iter().collect());
        current_level = next_level;
    }
    result
}

fn part1() -> anyhow::Result<()> {
    let input1 = download_notes(1, 2, 1)?;
    let re = Regex::new(r"ADD id=(\d+) left=\[(\d+),(.)\] right=\[(\d+),(.)\]")?;
    let mut left = None;
    let mut right = None;
    let parsed: Vec<_> = input1
        .lines()
        .filter_map(|line| {
            re.captures(line).map(|caps| {
                let id = caps[1].parse::<u64>().unwrap();
                let left_int = caps[2].parse::<i32>().unwrap();
                let left_char = caps[3].chars().next().unwrap();
                let right_int = caps[4].parse::<i32>().unwrap();
                let right_char = caps[5].chars().next().unwrap();

                (
                    id,
                    Node::new(left_int, id, left_char),
                    Node::new(right_int, id, right_char),
                )
            })
        })
        .collect();
    for (_id, left_node, right_node) in parsed {
        match left {
            None => left = Some(Rc::new(RefCell::new(BinaryTree::new(left_node)))),
            Some(ref mut node) => {
                node.borrow_mut().insert(left_node);
            }
        }
        match right {
            None => right = Some(Rc::new(RefCell::new(BinaryTree::new(right_node)))),
            Some(ref mut node) => {
                node.borrow_mut().insert(right_node);
            }
        }
    }
    let binding = rows(left.unwrap());
    let left_rows = binding.iter().max_by_key(|r| r.len()).unwrap();
    let binding = rows(right.unwrap());
    let right_rows = binding.iter().max_by_key(|r| r.len()).unwrap();
    println_part!(1, "{}{}", left_rows, right_rows);
    Ok(())
}

enum Op {
    Add {
        id: u64,
        left: (i32, char),
        right: (i32, char),
    },
    Swap(u64),
}

fn part2() -> anyhow::Result<()> {
    let input2 = download_notes(1, 2, 2)?;
    let re = Regex::new(r"((ADD) id=(\d+) left=\[(\d+),(.)\] right=\[(\d+),(.)\])|((SWAP) (\d+))")?;
    let mut left = None;
    let mut right = None;
    // let ids = HashMap::new();
    let parsed: Vec<_> = input2
        .lines()
        .filter_map(|line| {
            re.captures(line).and_then(|caps| {
                if caps.get(2).is_some() {
                    let id = caps[3].parse::<u64>().unwrap();
                    let left_int = caps[4].parse::<i32>().unwrap();
                    let left_char = caps[5].chars().next().unwrap();
                    let right_int = caps[6].parse::<i32>().unwrap();
                    let right_char = caps[7].chars().next().unwrap();
                    let left_node = Node::new(left_int, id, left_char);
                    let right_node = Node::new(right_int, id, right_char);
                    Some(Op::Add {
                        id,
                        left: (left_node.value, left_node.letter),
                        right: (right_node.value, right_node.letter),
                    })
                } else {
                    Some(Op::Swap(caps[10].parse::<u64>().unwrap()))
                }
            })
        })
        .collect();
    for op in parsed {
        match op {
            Op::Add {
                id,
                left: (l_val, l_char),
                right: (r_val, r_char),
            } => {
                let left_node = Node::new(l_val, id, l_char);
                let right_node = Node::new(r_val, id, r_char);
                match &left {
                    None => left = Some(Rc::new(RefCell::new(BinaryTree::new(left_node)))),
                    Some(node) => {
                        node.borrow_mut().insert(left_node);
                    }
                }
                match &right {
                    None => right = Some(Rc::new(RefCell::new(BinaryTree::new(right_node)))),
                    Some(node) => {
                        node.borrow_mut().insert(right_node);
                    }
                }
            }
            Op::Swap(id) => {
                let mut nodes = BinaryTree::find(left.as_ref().unwrap(), &|n: &Node| n.id == id);
                nodes.extend(BinaryTree::find(right.as_ref().unwrap(), &|n: &Node| {
                    n.id == id
                }));
                let (left_slice, right_slice) = nodes.split_at_mut(1);
                let left_node = &left_slice[0];
                let right_node = &right_slice[0];
                std::mem::swap(
                    &mut left_node.borrow_mut().value.letter,
                    &mut right_node.borrow_mut().value.letter,
                );
                std::mem::swap(
                    &mut left_node.borrow_mut().value.value,
                    &mut right_node.borrow_mut().value.value,
                );
            }
        }
    }
    let binding = rows(left.unwrap());
    let left_rows = binding.iter().max_by_key(|r| r.len()).unwrap();
    let binding = rows(right.unwrap());
    let right_rows = binding.iter().max_by_key(|r| r.len()).unwrap();
    println_part!(2, "{}{}", left_rows, right_rows);
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let input2 = download_notes(1, 2, 3)?;
    let re = Regex::new(r"((ADD) id=(\d+) left=\[(\d+),(.)\] right=\[(\d+),(.)\])|((SWAP) (\d+))")?;
    let mut left = None;
    let mut right = None;
    // let ids = HashMap::new();
    let parsed: Vec<_> = input2
        .lines()
        .filter_map(|line| {
            re.captures(line).and_then(|caps| {
                if caps.get(2).is_some() {
                    let id = caps[3].parse::<u64>().unwrap();
                    let left_int = caps[4].parse::<i32>().unwrap();
                    let left_char = caps[5].chars().next().unwrap();
                    let right_int = caps[6].parse::<i32>().unwrap();
                    let right_char = caps[7].chars().next().unwrap();
                    let left_node = Node::new(left_int, id, left_char);
                    let right_node = Node::new(right_int, id, right_char);
                    Some(Op::Add {
                        id,
                        left: (left_node.value, left_node.letter),
                        right: (right_node.value, right_node.letter),
                    })
                } else {
                    Some(Op::Swap(caps[10].parse::<u64>().unwrap()))
                }
            })
        })
        .collect();
    for op in parsed {
        match op {
            Op::Add {
                id,
                left: (l_val, l_char),
                right: (r_val, r_char),
            } => {
                let left_node = Node::new(l_val, id, l_char);
                let right_node = Node::new(r_val, id, r_char);
                match &left {
                    None => left = Some(Rc::new(RefCell::new(BinaryTree::new(left_node)))),
                    Some(node) => {
                        node.borrow_mut().insert(left_node);
                    }
                }
                match &right {
                    None => right = Some(Rc::new(RefCell::new(BinaryTree::new(right_node)))),
                    Some(node) => {
                        node.borrow_mut().insert(right_node);
                    }
                }
            }
            Op::Swap(id) => {
                let mut nodes = BinaryTree::find(left.as_ref().unwrap(), &|n: &Node| n.id == id);
                nodes.extend(BinaryTree::find(right.as_ref().unwrap(), &|n: &Node| {
                    n.id == id
                }));
                let (left_slice, right_slice) = nodes.split_at_mut(1);
                let left_node = &mut left_slice[0];
                let right_node = &mut right_slice[0];
                std::mem::swap(
                    &mut left_node.borrow_mut().value.letter,
                    &mut right_node.borrow_mut().value.letter,
                );
                std::mem::swap(
                    &mut left_node.borrow_mut().value.value,
                    &mut right_node.borrow_mut().value.value,
                );
                std::mem::swap(
                    &mut left_node.borrow_mut().left,
                    &mut right_node.borrow_mut().left,
                );
                std::mem::swap(
                    &mut left_node.borrow_mut().right,
                    &mut right_node.borrow_mut().right,
                );
            }
        }
    }
    let binding = rows(left.unwrap());
    let left_rows = binding.iter().rev().max_by_key(|r| r.len()).unwrap();
    let binding = rows(right.unwrap());
    let right_rows = binding.iter().rev().max_by_key(|r| r.len()).unwrap();
    println_part!(3, "{}{}", left_rows, right_rows);
    Ok(())
}

pub fn run() -> anyhow::Result<()> {
    println_quest!(2);
    part1()?;
    part2()?;
    part3()?;
    Ok(())
}
