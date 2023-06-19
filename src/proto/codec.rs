#[rustfmt::skip]
#[path = "sf.substreams.v1.rs"]
mod pbsubstreams;

#[rustfmt::skip]
#[path = "sf.aleo.record.v1.rs"]
mod pbrecord;

// Kind of bad because we mix stuff from different modules merging everything together
// but I'm a bit unsure about how to fix this properly for now.
pub use pbrecord::*;
pub use pbsubstreams::*;
