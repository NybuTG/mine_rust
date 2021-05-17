use crate::chunk::Chunk;
use crate::cube::{Cube, State};
use crate::error::MResult;
use std::path::Path;

// TODO: [ LONG TERM ] ||| Add a seed so generation stays the same on word reload

/**
Represents a disk-backed save file.
Chunks are indexed by special chunk coordinates, with the spawn chunk being given the coordinates (0, 0).
The chunk north of it would be (0, 1), the one south-west of it (-1, -1) etc.
Should any of the methods (except [Savefile::open()]) return errors, the save file is corrupted, and probably irrecoverably so.
*/
pub struct Savefile {
    /// Stores data related to players, e.g. inventory, health etc.
    playerdata: sled::Tree,

    /// Stores all modified chunks in a world
    chunks: sled::Tree,

    /// Stores block state at a XYZ position
    states: sled::Tree,
}

impl Savefile {
    /// Opens a save file at a given path
    pub fn open<P: AsRef<Path>>(path: P) -> MResult<Savefile> {
        let db = sled::open(path)?;
        Ok(Self {
            playerdata: db.open_tree("playerdata")?,
            chunks: db.open_tree("chunks")?,
            states: db.open_tree("states")?,
        })
    }

    /// Returns a chunk with the given chunk coordinates. Will return None if no chunk with said coordinates is found
    pub fn get_chunk(&self, x: isize, y: isize) -> MResult<Option<Chunk>> {
        let key = rmp_serde::to_vec(&(x, y)).unwrap();
        if let Some(data) = self.chunks.get(&key)? {
            Ok(Some(rmp_serde::from_read_ref(&data)?))
        } else {
            Ok(None)
        }
    }

    /// Saves a chunk at the given coordinates to the save file
    pub fn set_chunk(&self, x: isize, y: isize, chunk: &Chunk) -> MResult<()> {
        let key = rmp_serde::to_vec(&(x, y)).unwrap();
        self.chunks.insert(key, rmp_serde::to_vec(chunk)?)?;
        Ok(())
    }

    /// Returns a state at XYZ. Will return None if no chunk with said coordinates is found
    pub fn get_state(self, x: isize, y: isize, z: usize) -> MResult<Option<State>> {
        let key = rmp_serde::to_vec(&(x, y, z)).unwrap();
        if let Some(data) = self.states.get(&key)? {
            Ok(Some(rmp_serde::from_read_ref(&data)?))
        } else {
            Ok(None)
        }
    }

    /// Saves a state at position XYZ of state [State]
    pub fn set_state(&self, x: isize, y: isize, z: usize, state: &State) -> MResult<()> {
        // Create a key to access to access the state of a position at XYZ
        let key = rmp_serde::to_vec(&(x, y, z)).unwrap();

        // Insert into tree
        self.states.insert(key, rmp_serde::to_vec(state)?)?;

        Ok(())
    }

    pub fn get_playerdata(&self) {
        todo!()
    }

    pub fn set_playerdata(&self) {
        todo!()
    }
}
