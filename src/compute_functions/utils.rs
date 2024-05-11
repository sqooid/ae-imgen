use std::{collections::VecDeque, vec};

use super::ComputeFunction;

impl ComputeFunction {
    /// BFS search through self as root
    /// Returns in order in form (node, parent)
    pub fn bfs<'a>(&'a self) -> Vec<(&'a Self, &'a Self)> {
        let mut frontier: VecDeque<(&ComputeFunction, &ComputeFunction)> =
            VecDeque::from([(self, self)]);
        let mut nodes: Vec<(&ComputeFunction, &ComputeFunction)> = vec![];
        while !frontier.is_empty() {
            // Can unwrap because we already checked that it's not empty
            let (current, parent) = frontier.pop_front().unwrap();
            nodes.push((current, parent));
            match current {
                ComputeFunction::Constant(_inner) => {}
                ComputeFunction::One(inner) => frontier.push_back((inner.arg(), current)),
                ComputeFunction::Two(inner) => {
                    frontier.push_back((inner.arg(0usize), current));
                    frontier.push_back((inner.arg(1usize), current))
                }
            }
        }
        return nodes;
    }
}

#[cfg(test)]
mod tests {
    use crate::compute_functions::{ConstantFunction, SingleArgFunction, TwoArgFunction};

    use super::*;

    #[test]
    fn test_bfs() {
        // Add
        //     Sin
        //         Coord(0)
        //     Loge
        //         Constant(0.1,0.2,0.3)
        let compute_function = ComputeFunction::Two(Box::new(TwoArgFunction::Add(
            ComputeFunction::One(Box::new(SingleArgFunction::Sin(ComputeFunction::Constant(
                Box::new(ConstantFunction::Coord(0)),
            )))),
            ComputeFunction::One(Box::new(SingleArgFunction::Loge(
                ComputeFunction::Constant(Box::new(ConstantFunction::Constant(0.1, 0.2, 0.3))),
            ))),
        )));
        let nodes = compute_function.bfs();
        for node in &nodes {
            println!("{:?}", node);
        }
    }
}
