use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// See [Chunk]
pub type CubeArray = Vec<[[String; 16]; 16]>;

/// Represents a chunk of cubes, which is a variable height 16x16 arrangement of blocks.
/// TODO: [ LONG TERM ] ||| Implement compression.
#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    cubes: CubeArray,
}

#[macro_export]
macro_rules! string_clone {
    ($clone_string: expr) => {
        $clone_string
    };
}

impl Chunk {
    pub fn from_cube_array(cube_array: CubeArray) -> Self {
        Chunk { cubes: cube_array }
    }
}

/// Allows turning a chunk into a CubeArray by dereferencing it.
impl Deref for Chunk {
    type Target = CubeArray;

    fn deref(&self) -> &Self::Target {
        &self.cubes
    }
}
