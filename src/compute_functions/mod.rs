use enum_methods::EnumMethods;
use enum_methods_derive::EnumMethods;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub mod image;
pub mod shader;
pub mod utils;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ComputeFunction {
    Zero(Box<ConstantFunction>),
    One(Box<SingleArgFunction>),
    Two(Box<TwoArgFunction>),
    #[default]
    Placeholder,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConstantFunction {
    Constant(f32, f32, f32),
    Coord(u8),
}

#[derive(Debug, Serialize, Deserialize, Clone, EnumIter, EnumMethods)]
pub enum SingleArgFunction {
    Sin(ComputeFunction),
    Cos(ComputeFunction),
    Tan(ComputeFunction),
    Atan(ComputeFunction),
    Sinh(ComputeFunction),
    Cosh(ComputeFunction),
    Abs(ComputeFunction),
    Reciprocal(ComputeFunction),
    Square(ComputeFunction),
    SquareRoot(ComputeFunction),
    Loge(ComputeFunction),
}

#[derive(Debug, Serialize, Deserialize, Clone, EnumIter, EnumMethods)]
pub enum TwoArgFunction {
    Add(ComputeFunction, ComputeFunction),
    Subtract(ComputeFunction, ComputeFunction),
    Multiply(ComputeFunction, ComputeFunction),
    Divide(ComputeFunction, ComputeFunction),
    Min(ComputeFunction, ComputeFunction),
    Max(ComputeFunction, ComputeFunction),
    Avg(ComputeFunction, ComputeFunction),
    Mod(ComputeFunction, ComputeFunction),
    Exponent(ComputeFunction, ComputeFunction),
    And(ComputeFunction, ComputeFunction),
    Or(ComputeFunction, ComputeFunction),
    Xor(ComputeFunction, ComputeFunction),
}
