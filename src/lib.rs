/// Render quantum size (audio graph is rendered in blocks of this size)
pub const BUFFER_SIZE: u32 = 512;

pub mod context;
pub mod node;

pub(crate) mod control;
pub(crate) mod graph;

/// Input/output with this index does not exist
#[derive(Debug)]
pub struct IndexSizeError {}

use std::fmt;
impl fmt::Display for IndexSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for IndexSizeError {}
