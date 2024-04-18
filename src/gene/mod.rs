use serde::{Deserialize, Serialize};

use crate::compute_functions::ComputeFunction;

pub mod mating;
pub mod mutation;

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Contains all information needed to perfectly reproduce an image
/// Things like z, resolution and bounds are more part of the "phenotype" and so are not contained here
pub struct Gene {
    seed: f32,
    function: ComputeFunction,
}
