use macroquad::prelude::*;

use rapier3d::dynamics::{BodyStatus, RigidBodyBuilder, RigidBodySet};
use rapier3d::geometry::ColliderSet;
use rapier3d::{geometry::ColliderBuilder, pipeline::PhysicsPipeline};

// pub mod shape_constructor;
// pub use self::shape_constructor::CubeConstructor;
use ron::de::{from_reader, from_str};
use serde::{Deserialize, Serialize};
use std::fs::read_dir;
use std::fs::File;
use std::path::Path;

use crate::error::MResult;
use std::collections::HashMap;

/// Defines all properties of a cube.
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Cube {
    /// The user-facing name of the cube, which is displayed in-game
    pub human_readable: String,

    /// The unique ID describing the cube, usually prefixed by the source and a colon.
    /// Cubes provided by the game itself are prefixed with minerust:
    pub id: String,

    /// Defines whether or not the cube is affected by gravity.
    pub dynamic: bool,

    /// Relative path to the cube's texture. Path gets updated when switching resource pack
    /// TODO: Let this be a key into an asset library, to accelerate texture loading and optimize memory usage.
    pub texture: String,

    /// The following value is in ticks, for the minimum tool
    /// What affects this value:
    /// - Tool level 1: Higher tool level means a decrease in the time consumed to break the block
    /// - Tool level 2: Lower tool level means an increase in the time consumed to break the block
    /// - Tool: Using the wrong tool will make the mining of a block very slow
    /// - Enchantments: Enchantments do the same thing as `Tool level 1`
    pub break_time: u8,

    /// Shovel, Pickaxe, etc
    pub effective_tool: String,

    /// The minimum tool level required to collect this block.
    pub tool_level: u8,

    /// The [State] describing this block
    pub state: State,
}

/// Defines the state of a cube. Not all cubes use this.
/// This is also the only cube-related data saved to disk, except for the ID (TODO: Actually save it to disk ;))
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct State {
    pub rotation: u8,
    pub texture_idx: u8,
}

impl Cube {
    /// Loads cube definitions from the specified directory.
    pub fn load_all_defs<P: AsRef<Path>>(path: P) -> MResult<HashMap<String, Cube>> {
        let mut def_map = HashMap::new();
        for definition in read_dir(path)? {
            let file = File::open(definition?.path())?;
            let cube: Cube = match from_reader(file) {
                Ok(x) => x,
                Err(e) => from_str(include_str!("../assets/cubes/debug.ron")).unwrap(),
            };
            def_map.insert(cube.id.clone(), cube);
        }
        Ok(def_map)
    }
}
