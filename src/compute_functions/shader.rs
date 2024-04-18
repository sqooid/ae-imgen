use std::fmt::format;

use log::trace;

#[derive(Debug, Clone)]
pub enum ComputeFunction {
    // Constants
    Constant(f32, f32, f32),
    Coord,
    // One arg
    Sin(Box<ComputeFunction>),
    Cos(Box<ComputeFunction>),
    Tan(Box<ComputeFunction>),
    Atan(Box<ComputeFunction>),
    Sinh(Box<ComputeFunction>),
    Cosh(Box<ComputeFunction>),
    Abs(Box<ComputeFunction>),
    Reciprocal(Box<ComputeFunction>),
    Square(Box<ComputeFunction>),
    SquareRoot(Box<ComputeFunction>),
    Loge(Box<ComputeFunction>),
    // Multiple arg
    Add(Box<ComputeFunction>, Box<ComputeFunction>),
    Subtract(Box<ComputeFunction>, Box<ComputeFunction>),
    Multiply(Box<ComputeFunction>, Box<ComputeFunction>),
    Divide(Box<ComputeFunction>, Box<ComputeFunction>),
    Min(Box<ComputeFunction>, Box<ComputeFunction>),
    Max(Box<ComputeFunction>, Box<ComputeFunction>),
    Avg(Box<ComputeFunction>, Box<ComputeFunction>),
    Mod(Box<ComputeFunction>, Box<ComputeFunction>),
    Exponent(Box<ComputeFunction>, Box<ComputeFunction>),
    And(Box<ComputeFunction>, Box<ComputeFunction>),
    Or(Box<ComputeFunction>, Box<ComputeFunction>),
    Xor(Box<ComputeFunction>, Box<ComputeFunction>),
}

pub trait ShaderFunction {
    /// Generates inner shader function code
    fn inner_shader(&self) -> String;
    /// Generate complete shader code
    fn get_shader_code(&self) -> String;
}

impl ShaderFunction for ComputeFunction {
    fn inner_shader(&self) -> String {
        match self {
            ComputeFunction::Constant(r, g, b) => format!("vec3({},{},{})", r, g, b),
            ComputeFunction::Coord => "vec3(x,y,z)".to_string(),
            ComputeFunction::Sin(arg) => format!("sin({})", arg.inner_shader()),
            ComputeFunction::Cos(arg) => format!("cos({})", arg.inner_shader()),
            ComputeFunction::Tan(arg) => format!("tan({})", arg.inner_shader()),
            ComputeFunction::Atan(arg) => format!("atan({})", arg.inner_shader()),
            ComputeFunction::Sinh(arg) => format!("sinh({})", arg.inner_shader()),
            ComputeFunction::Cosh(arg) => format!("cosh({})", arg.inner_shader()),
            ComputeFunction::Abs(arg) => format!("abs({})", arg.inner_shader()),
            ComputeFunction::Reciprocal(arg) => format!("1/({})", arg.inner_shader()),
            ComputeFunction::Square(arg) => format!("pow({},2)", arg.inner_shader()),
            ComputeFunction::SquareRoot(arg) => format!("sqrt({})", arg.inner_shader()),
            ComputeFunction::Loge(arg) => {
                format!("log({})", arg.inner_shader())
            }
            ComputeFunction::Add(arg1, arg2) => {
                format!("({}+{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Subtract(arg1, arg2) => {
                format!("({}-{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Multiply(arg1, arg2) => {
                format!("({}*{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Divide(arg1, arg2) => {
                format!("({}/{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Min(arg1, arg2) => {
                format!("min({},{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Max(arg1, arg2) => {
                format!("max({},{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Avg(arg1, arg2) => {
                format!("({}+{})/2", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Mod(arg1, arg2) => {
                format!("({}%{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Exponent(arg1, arg2) => {
                format!("pow({},{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::And(arg1, arg2) => {
                format!("({}&{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Or(arg1, arg2) => {
                format!("({}|{})", arg1.inner_shader(), arg2.inner_shader())
            }
            ComputeFunction::Xor(arg1, arg2) => {
                format!("({}^{})", arg1.inner_shader(), arg2.inner_shader())
            }
        }
    }

    fn get_shader_code(&self) -> String {
        let shader = include_str!("../shaders/compute_frame.wgsl")
            .replace("0.123456789", &self.inner_shader())
            .to_string();
        trace!("generated shader:\n{}", &shader);
        shader
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_shader_string() {
        let compute_function = ComputeFunction::Sin(Box::new(ComputeFunction::Coord));
        let result = compute_function.get_shader_code();
        println!("{}", result);
    }
}
