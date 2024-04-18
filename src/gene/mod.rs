use serde::{Deserialize, Serialize};

pub mod mating;
pub mod mutation;

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Contains all information needed to perfectly reproduce an image
/// Things like z, resolution and bounds are more part of the "phenotype" and so are not contained here
pub struct Gene {}
