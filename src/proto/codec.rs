#[rustfmt::skip]
#[path = "sf.substreams.v1.rs"]
mod pbsubstreams;

#[rustfmt::skip]
#[path = "aleo.extracted.v1.rs"]
mod pbaleo;

// Kind of bad because we mix stuff from different modules merging everything together
// but I'm a bit unsure about how to fix this properly for now.
pub use pbaleo::*;
pub use pbsubstreams::*;
