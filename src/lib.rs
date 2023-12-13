mod read;
mod stats;
mod write;

pub use read::*;
pub use stats::*;
pub use write::*;

const CHUNK_SIZE: usize = 16 * 1024;
