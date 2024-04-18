use std::path::Path;

use image::RgbImage;

use crate::compute_functions::image::Resolution;

fn encode_image(resolution: &Resolution, buffer: Vec<u8>, path: &str) {
    let rgb_image = RgbImage::from_vec(resolution.0, resolution.1, buffer).unwrap();
    rgb_image.save(path).unwrap();
}

#[cfg(test)]
mod tests {
    use pollster::block_on;

    use crate::{
        compute_functions::{
            image::{Bounds, ImageConfig},
            shader::ComputeFunction,
        },
        gpu::instance::GpuInstance,
    };

    use super::*;

    fn test_render() -> Vec<f32> {
        let function = ComputeFunction::Sin(Box::new(ComputeFunction::Coord(0)));
        let config = ImageConfig {
            resolution: Resolution::new(10, 10),
            bounds: Bounds::new(0.0, 0.0, 0.0, 1.0, 1.0),
        };
        let gpu = block_on(GpuInstance::new()).unwrap();
        let result = block_on(gpu.generate_buffer(&config, &function)).unwrap();
        println!("{:?}", &result);
        result
    }

    #[test]
    fn test_encode() {
        let data: Vec<u8> = test_render()
            .iter()
            .map(|x| (x * 255.0).floor() as u8)
            .collect();
        encode_image(&Resolution::new(10, 10), data, "test/test.png");
    }
}
