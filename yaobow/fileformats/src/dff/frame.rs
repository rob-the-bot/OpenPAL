use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use common::read_ext::ReadExt;
use serde::Serialize;

use crate::rwbs::Vec3f;

#[derive(Debug, Serialize)]
pub struct Frame {
    pub right: Vec3f,
    pub up: Vec3f,
    pub at: Vec3f,
    pub pos: Vec3f,
    pub parent: u32,
    pub unknown: u32,
}

impl Frame {
    pub fn read(cursor: &mut dyn Read) -> anyhow::Result<Self> {
        let right = Self::read_vec3(cursor)?;
        let up = Self::read_vec3(cursor)?;
        let at = Self::read_vec3(cursor)?;
        let pos = Self::read_vec3(cursor)?;
        let parent = cursor.read_u32_le()?;
        let unknown = cursor.read_u32_le()?;

        Ok(Self {
            right,
            up,
            at,
            pos,
            parent,
            unknown,
        })
    }

    fn read_vec3(cursor: &mut dyn Read) -> anyhow::Result<Vec3f> {
        let x = cursor.read_f32::<LittleEndian>()?;
        let y = cursor.read_f32::<LittleEndian>()?;
        let z = cursor.read_f32::<LittleEndian>()?;
        Ok(Vec3f { x, y, z })
    }
}
