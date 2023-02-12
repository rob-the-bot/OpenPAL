pub mod atomic;
pub mod clump;
pub mod frame;
pub mod geometry;
pub mod material;

use std::io::Cursor;

use crate::rwbs::{ChunkHeader, ChunkType};

use self::clump::Clump;

pub fn read_dff(data: &[u8]) -> anyhow::Result<Vec<Clump>> {
    let mut cursor = Cursor::new(data);
    let mut chunks = vec![];

    while !cursor.is_empty() {
        let chunk = ChunkHeader::read(&mut cursor)?;
        match chunk.ty {
            ChunkType::CLUMP => chunks.push(Clump::read(&mut cursor)?),
            _ => cursor.set_position(cursor.position() + chunk.length as u64),
        }
    }

    Ok(chunks)
}
