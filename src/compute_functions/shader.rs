use log::trace;

#[derive(Debug, Clone)]
pub enum ComputeFunction {
    Constant(f32),
    Coord(u8),
    Sin(Box<ComputeFunction>),
    Cos(Box<ComputeFunction>),
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
            ComputeFunction::Constant(value) => value.to_string(),
            ComputeFunction::Coord(dim) => match dim {
                0 => "x".to_string(),
                1 => "y".to_string(),
                _ => "z".to_string(),
            },
            ComputeFunction::Sin(arg) => format!("sin({})", arg.inner_shader()),
            ComputeFunction::Cos(arg) => format!("cos({})", arg.inner_shader()),
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
        let compute_function = ComputeFunction::Sin(Box::new(ComputeFunction::Coord(0)));
        let result = compute_function.get_shader_code();
        println!("{}", result);
    }
}
