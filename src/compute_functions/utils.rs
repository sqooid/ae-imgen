use std::collections::VecDeque;

use super::ComputeFunction;

impl ComputeFunction {
    /// BFS search through self as root
    /// Returns in order in form (node, parent)
    pub fn bfs<'a>(&'a self) -> Vec<(&'a Self, &'a Self)> {
        let mut frontier: VecDeque<(&ComputeFunction, &ComputeFunction)> =
            VecDeque::from([(self, self)]);
        while !frontier.is_empty() {
            // Can unwrap because we already checked that it's not empty
            let (current, parent) = frontier.pop_front().unwrap();
            match current {
                ComputeFunction::Constant(inner) => {}
                ComputeFunction::One(inner) => frontier.push_back((inner.arg(), current)),
                ComputeFunction::Two(inner) => frontier.push_back((inner.arg(0), current)),
            }
        }
        todo!()
    }
}
