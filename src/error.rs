use thiserror::Error;
use wgpu::RequestDeviceError;

#[derive(Debug, Error)]
pub enum GpuError {
    #[error("Failed to get gpu adapter")]
    NoAdapter,
    #[error("Failed to request device: {0}")]
    RequestDeviceError(RequestDeviceError),
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Bad argument")]
    BadArg,
}
