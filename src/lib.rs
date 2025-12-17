#[cfg(feature = "radar")]
pub mod radar {
    include!(concat!(env!("OUT_DIR"), "/radar.rs"));
}
#[cfg(feature = "ingest")]
pub mod geyser {
    include!(concat!(env!("OUT_DIR"), "/geyser.rs"));
}

#[cfg(feature = "ingest")]
pub use geyser::*;

#[cfg(feature = "radar")]
pub use radar::*;