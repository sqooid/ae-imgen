use serde::{Deserialize, Serialize};

pub mod image;
pub mod shader;
pub mod utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ComputeFunction {
    Zero(Box<ConstantFunction>),
    One(Box<SingleArgFunction>),
    Two(Box<TwoArgFunction>),
    Placeholder,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConstantFunction {
    Constant(f32, f32, f32),
    Coord(u8),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl SingleArgFunction {
    pub fn arg(&self) -> &ComputeFunction {
        match self {
            SingleArgFunction::Sin(arg) => arg,
            SingleArgFunction::Cos(arg) => arg,
            SingleArgFunction::Tan(arg) => arg,
            SingleArgFunction::Atan(arg) => arg,
            SingleArgFunction::Sinh(arg) => arg,
            SingleArgFunction::Cosh(arg) => arg,
            SingleArgFunction::Abs(arg) => arg,
            SingleArgFunction::Reciprocal(arg) => arg,
            SingleArgFunction::Square(arg) => arg,
            SingleArgFunction::SquareRoot(arg) => arg,
            SingleArgFunction::Loge(arg) => arg,
        }
    }
}

impl TwoArgFunction {
    pub fn arg<T: Into<usize>>(&self, i: T) -> &ComputeFunction {
        let index: usize = i.into();
        match self {
            TwoArgFunction::Add(a1, a2) => [a1, a2][index],
            TwoArgFunction::Subtract(a1, a2) => [a1, a2][index],
            TwoArgFunction::Multiply(a1, a2) => [a1, a2][index],
            TwoArgFunction::Divide(a1, a2) => [a1, a2][index],
            TwoArgFunction::Min(a1, a2) => [a1, a2][index],
            TwoArgFunction::Max(a1, a2) => [a1, a2][index],
            TwoArgFunction::Avg(a1, a2) => [a1, a2][index],
            TwoArgFunction::Mod(a1, a2) => [a1, a2][index],
            TwoArgFunction::Exponent(a1, a2) => [a1, a2][index],
            TwoArgFunction::And(a1, a2) => [a1, a2][index],
            TwoArgFunction::Or(a1, a2) => [a1, a2][index],
            TwoArgFunction::Xor(a1, a2) => [a1, a2][index],
        }
    }
}
