use std::borrow::Cow;

use wgpu::util::DeviceExt;

use crate::compute_functions::{
    image::{self, ImageConfig},
    shader::{ComputeFunction, ShaderFunction},
};

use super::instance::GpuInstance;

impl GpuInstance {
    pub async fn generate_buffer(&self, image_config: &ImageConfig, function: &ComputeFunction) {
        // Create shader
        let shader_code = function.get_shader_code();
        let shader_module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&shader_code)),
            });

        // Create arg buffers
        let bounds_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bounds"),
                contents: &bincode::serialize(&image_config.bounds).unwrap(),
                usage: wgpu::BufferUsages::STORAGE, // | wgpu::BufferUsages::COPY_DST
                                                    // | wgpu::BufferUsages::COPY_SRC,
            });
        let resolution_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("resolution"),
                contents: &bincode::serialize(&image_config.resolution).unwrap(),
                usage: wgpu::BufferUsages::STORAGE, // | wgpu::BufferUsages::COPY_DST
                                                    // | wgpu::BufferUsages::COPY_SRC,
            });

        // Create output buffer
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (32 * image_config.resolution.0 * image_config.resolution.1) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Create read buffer
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (32 * image_config.resolution.0 * image_config.resolution.1) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    }
}
