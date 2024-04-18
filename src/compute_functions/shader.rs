use std::fmt::Debug;

use log::trace;
use serde::{Deserialize, Serialize};

type SomeFunction = Box<dyn ShaderFunction>;

#[derive(Debug, Serialize, Deserialize)]
pub enum ConstantFunction {
    Constant(f32, f32, f32),
    Coord(u8),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SingleArgFunction {
    Sin(SomeFunction),
    Cos(SomeFunction),
    Tan(SomeFunction),
    Atan(SomeFunction),
    Sinh(SomeFunction),
    Cosh(SomeFunction),
    Abs(SomeFunction),
    Reciprocal(SomeFunction),
    Square(SomeFunction),
    SquareRoot(SomeFunction),
    Loge(SomeFunction),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TwoArgFunction {
    Add(SomeFunction, SomeFunction),
    Subtract(SomeFunction, SomeFunction),
    Multiply(SomeFunction, SomeFunction),
    Divide(SomeFunction, SomeFunction),
    Min(SomeFunction, SomeFunction),
    Max(SomeFunction, SomeFunction),
    Avg(SomeFunction, SomeFunction),
    Mod(SomeFunction, SomeFunction),
    Exponent(SomeFunction, SomeFunction),
    And(SomeFunction, SomeFunction),
    Or(SomeFunction, SomeFunction),
    Xor(SomeFunction, SomeFunction),
}

#[typetag::serde(tag = "type")]
pub trait ShaderFunction: Debug {
    /// Generates inner shader function code
    fn inner_shader(&self) -> String;
    /// Generate complete shader code
    fn get_shader_code(&self) -> String {
        let shader = include_str!("../shaders/compute_frame.wgsl")
            .replace("0.123456789", &self.inner_shader())
            .to_string();
        trace!("generated shader:\n{}", &shader);
        shader
    }
}

#[typetag::serde]
impl ShaderFunction for ConstantFunction {
    fn inner_shader(&self) -> String {
        match self {
            ConstantFunction::Constant(r, g, b) => format!("vec3({},{},{})", r, g, b),
            ConstantFunction::Coord(dim) => match dim {
                0 => "vec3(x,x,x)",
                1 => "vec3(y,y,y)",
                _ => "vec3(z,z,z)",
            }
            .to_string(),
        }
    }
}

#[typetag::serde]
impl ShaderFunction for SingleArgFunction {
    fn inner_shader(&self) -> String {
        match self {
            SingleArgFunction::Sin(arg) => format!("sin({})", arg.inner_shader()),
            SingleArgFunction::Cos(arg) => format!("cos({})", arg.inner_shader()),
            SingleArgFunction::Tan(arg) => format!("tan({})", arg.inner_shader()),
            SingleArgFunction::Atan(arg) => format!("atan({})", arg.inner_shader()),
            SingleArgFunction::Sinh(arg) => format!("sinh({})", arg.inner_shader()),
            SingleArgFunction::Cosh(arg) => format!("cosh({})", arg.inner_shader()),
            SingleArgFunction::Abs(arg) => format!("abs({})", arg.inner_shader()),
            SingleArgFunction::Reciprocal(arg) => format!("1/({})", arg.inner_shader()),
            SingleArgFunction::Square(arg) => format!("pow({},2)", arg.inner_shader()),
            SingleArgFunction::SquareRoot(arg) => format!("sqrt({})", arg.inner_shader()),
            SingleArgFunction::Loge(arg) => {
                format!("log({})", arg.inner_shader())
            }
        }
    }
}

#[typetag::serde]
impl ShaderFunction for TwoArgFunction {
    fn inner_shader(&self) -> String {
        match self {
            TwoArgFunction::Add(arg1, arg2) => {
                format!("({}+{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Subtract(arg1, arg2) => {
                format!("({}-{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Multiply(arg1, arg2) => {
                format!("({}*{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Divide(arg1, arg2) => {
                format!("({}/{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Min(arg1, arg2) => {
                format!("min({},{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Max(arg1, arg2) => {
                format!("max({},{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Avg(arg1, arg2) => {
                format!("({}+{})/2", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Mod(arg1, arg2) => {
                format!("({}%{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Exponent(arg1, arg2) => {
                format!("pow({},{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::And(arg1, arg2) => {
                format!("({}&{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Or(arg1, arg2) => {
                format!("({}|{})", arg1.inner_shader(), arg2.inner_shader())
            }
            TwoArgFunction::Xor(arg1, arg2) => {
                format!("({}^{})", arg1.inner_shader(), arg2.inner_shader())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_shader_string() {
        let compute_function = SingleArgFunction::Sin(Box::new(ConstantFunction::Coord(0)));
        let result = compute_function.get_shader_code();
        println!("{}", result);
    }
}
