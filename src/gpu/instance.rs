use log::debug;

use crate::error::GpuError;

#[derive(Debug)]
pub struct GpuInstance {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuInstance {
    pub async fn new() -> Result<Self, GpuError> {
        // Instantiates instance of WebGPU
        let instance = wgpu::Instance::default();

        // `request_adapter` instantiates the general connection to the GPU
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or(GpuError::NoAdapter)?;

        // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
        //  `features` being the available features.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .unwrap();

        let result = Self { device, queue };
        debug!("got gpu: {:?}", &result);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use pollster::block_on;

    use super::*;

    #[test]
    fn create_gpu() {
        let gpu = block_on(GpuInstance::new()).unwrap();
        println!("{:?}", gpu);
    }
}
