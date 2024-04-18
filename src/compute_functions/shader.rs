#[derive(Debug, Clone)]
pub enum ComputeFunction {
    Constant(f32),
    Coord(u8),
    Sin(Box<ComputeFunction>),
    Cos(Box<ComputeFunction>),
}

pub trait ShaderFunction {
    /// Generates inner shader function code
    fn inner_shader(self) -> String;
    /// Generate complete shader code
    fn generate_shader(self) -> String;
}

impl ShaderFunction for ComputeFunction {
    fn inner_shader(self) -> String {
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

    fn generate_shader(self) -> String {
        include_str!("../shaders/compute_frame.wgsl")
            .replace("0.123456789", &self.inner_shader())
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_shader_string() {
        let compute_function = ComputeFunction::Sin(Box::new(ComputeFunction::Coord(0)));
        let result = compute_function.generate_shader();
        println!("{}", result);
    }
}
