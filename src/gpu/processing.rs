use std::borrow::Cow;

use wgpu::util::DeviceExt;

use crate::compute_functions::{
    image::ImageConfig,
    shader::{ComputeFunction, ShaderFunction},
};

use super::instance::GpuInstance;

impl GpuInstance {
    pub async fn generate_buffer(
        &self,
        image_config: &ImageConfig,
        function: &ComputeFunction,
    ) -> Option<Vec<f32>> {
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

        let pixels = image_config.resolution.0 * image_config.resolution.1;
        let buffer_size = pixels * 4 * 3;
        // Create output buffer
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: buffer_size as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Create read buffer
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: buffer_size as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let compute_pipeline =
            self.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: &shader_module,
                    entry_point: "main",
                });

        // Instantiates the bind group, once again specifying the binding of buffers.
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: output_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: resolution_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: bounds_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.insert_debug_marker("compute buffer");
            let groups = (pixels as f32 / 32.0).ceil();
            println!("groups: {:?}", &groups);
            cpass.dispatch_workgroups(groups as u32, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, buffer_size as u64);
        self.queue.submit(Some(encoder.finish()));
        // Note that we're not calling `.await` here.
        let buffer_slice = staging_buffer.slice(..);
        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let (sender, receiver) = flume::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        self.device.poll(wgpu::Maintain::wait()).panic_on_timeout();

        // Awaits until `buffer_future` can be read from
        if let Ok(Ok(())) = receiver.recv_async().await {
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();
            // Since contents are got in bytes, this converts these bytes back to u32
            let result = bytemuck::cast_slice(&data).to_vec();

            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data);
            staging_buffer.unmap(); // Unmaps buffer from memory
                                    // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                    //   delete myPointer;
                                    //   myPointer = NULL;
                                    // It effectively frees the memory

            // Returns data from buffer
            Some(result)
        } else {
            panic!("failed to run compute on gpu!")
        }
    }
}

#[cfg(test)]
mod tests {
    use pollster::block_on;

    use crate::compute_functions::image::{Bounds, Resolution};

    use super::*;

    #[test]
    fn test_render() {
        let function = ComputeFunction::Sin(Box::new(ComputeFunction::Coord));
        let config = ImageConfig {
            resolution: Resolution::new(10, 10),
            bounds: Bounds::new(0.0, 0.0, 0.0, 1.0, 1.0),
        };
        let gpu = block_on(GpuInstance::new()).unwrap();
        let result = block_on(gpu.generate_buffer(&config, &function));
        println!("{:?}", &result);
    }
}
