#[ link(name = "rustfrp",
        vers = "0.0",
        uuid = "760ae947-ed4a-4f52-8a9d-2aee27335220") ];

#[ desc = "FRP Rust Implementation" ];
#[ license = "Zlib/libpng" ];
#[ author = "tiffany" ];

#[ crate_type="lib" ];

pub mod multicast;
pub mod signal;

