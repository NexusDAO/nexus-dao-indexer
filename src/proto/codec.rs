#[rustfmt::skip]
#[path = "sf.substreams.v1.rs"]
mod pbsubstreams;

#[rustfmt::skip]
#[path = "aleo.record.v1.rs"]
mod pbrecord;

#[rustfmt::skip]
#[path = "nexus_dao.mapping.v1.rs"]
mod pbmapping;

// Kind of bad because we mix stuff from different modules merging everything together
// but I'm a bit unsure about how to fix this properly for now.
pub use pbmapping::*;
pub use pbrecord::*;
pub use pbsubstreams::*;
