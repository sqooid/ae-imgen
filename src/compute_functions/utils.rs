use rand::seq::SliceRandom;
use std::{collections::VecDeque, vec};

use rand::{
    distributions::{Distribution, Standard, WeightedIndex},
    thread_rng, Rng,
};

use crate::{
    compute_functions::{ConstantFunction, SingleArgFunction, TwoArgFunction},
    error::ApplicationError,
};

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
                ComputeFunction::Zero(_inner) => {}
                ComputeFunction::One(inner) => frontier.push_back((inner.arg(), current)),
                ComputeFunction::Two(inner) => {
                    frontier.push_back((inner.arg(0usize), current));
                    frontier.push_back((inner.arg(1usize), current))
                }
                ComputeFunction::Placeholder => todo!(),
            }
        }
        return nodes;
    }

    pub fn random(arg_weights: &[f32; 3]) -> Result<Self, ApplicationError> {
        let arg_indices = [0, 1, 2];
        let dist = WeightedIndex::new(arg_weights).map_err(|_| ApplicationError::BadArg)?;
        let mut rng = thread_rng();
        let arg_count = arg_indices[dist.sample(&mut rng)];
        let functions = match arg_count {
            0 => {
                let v0: f32 = Standard.sample(&mut rng);
                let v1: f32 = Standard.sample(&mut rng);
                let v2: f32 = Standard.sample(&mut rng);
                let dim: u8 = rng.gen_range(0..3);
                vec![
                    ComputeFunction::Zero(Box::new(ConstantFunction::Constant(v0, v1, v2))),
                    ComputeFunction::Zero(Box::new(ConstantFunction::Coord(dim))),
                ]
            }
            1 => {
                vec![
                    ComputeFunction::One(Box::new(SingleArgFunction::Sin(Self::Placeholder))),
                    ComputeFunction::One(Box::new(SingleArgFunction::Cos(Self::Placeholder))),
                    ComputeFunction::One(Box::new(SingleArgFunction::Tan(Self::Placeholder))),
                    ComputeFunction::One(Box::new(SingleArgFunction::Atan(Self::Placeholder))),
                    ComputeFunction::One(Box::new(SingleArgFunction::Cosh(Self::Placeholder))),
                    ComputeFunction::One(Box::new(SingleArgFunction::Abs(Self::Placeholder))),
                    ComputeFunction::One(Box::new(SingleArgFunction::Reciprocal(
                        Self::Placeholder,
                    ))),
                    ComputeFunction::One(Box::new(SingleArgFunction::Square(Self::Placeholder))),
                    ComputeFunction::One(Box::new(SingleArgFunction::SquareRoot(
                        Self::Placeholder,
                    ))),
                    ComputeFunction::One(Box::new(SingleArgFunction::Loge(Self::Placeholder))),
                ]
            }
            _ => {
                vec![
                    ComputeFunction::Two(Box::new(TwoArgFunction::Add(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Subtract(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Multiply(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Divide(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Min(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Max(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Avg(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Mod(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Exponent(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::And(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Or(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                    ComputeFunction::Two(Box::new(TwoArgFunction::Xor(
                        Self::Placeholder,
                        Self::Placeholder,
                    ))),
                ]
            }
        };
        let func = functions
            .choose(&mut rng)
            .ok_or_else(|| ApplicationError::BadArg)?;
        Ok(func.to_owned())
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
            ComputeFunction::One(Box::new(SingleArgFunction::Sin(ComputeFunction::Zero(
                Box::new(ConstantFunction::Coord(0)),
            )))),
            ComputeFunction::One(Box::new(SingleArgFunction::Loge(ComputeFunction::Zero(
                Box::new(ConstantFunction::Constant(0.1, 0.2, 0.3)),
            )))),
        )));
        let nodes = compute_function.bfs();
        for node in &nodes {
            println!("{:?}", node);
        }
    }

    #[test]
    fn test_random() {
        let random_func = ComputeFunction::random(&[1.0, 1.0, 1.0]).unwrap();
        println!("{:?}", &random_func);
    }
}
