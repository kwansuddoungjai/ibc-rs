//! Re-exports data structures and implementations of all the IBC core (TAO) modules/components.
#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(not(test), deny(clippy::disallowed_methods, clippy::disallowed_types,))]
#![deny(
    warnings,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]

/// Re-exports IBC handler entrypoints from the `ibc-core-handler` crate for
/// added convenience.
pub mod entrypoint {
    #[doc(inline)]
    pub use ibc_core_handler::entrypoint::*;
}

/// Re-exports IBC primitive types from the `ibc-primitives` crate
pub mod primitives {
    #[doc(inline)]
    pub use ibc_primitives::*;
}

/// Re-exports ICS-02 implementation from the `ibc-core-client` crate
pub mod client {
    #[doc(inline)]
    pub use ibc_core_client::*;
}

/// Re-exports ICS-03 implementation from the `ibc-core-connection` crate
pub mod connection {
    #[doc(inline)]
    pub use ibc_core_connection::*;
}

/// Re-exports ICS-04 implementation from the `ibc-core-channel` crate
pub mod channel {
    #[doc(inline)]
    pub use ibc_core_channel::*;
}

/// Re-exports ICS-23 data structures from the `ibc-core-commitment-types` crate
pub mod commitment_types {
    #[doc(inline)]
    pub use ibc_core_commitment_types::*;
}

/// Re-exports ICS-24 implementation from the `ibc-core-host` crate
pub mod host {
    #[doc(inline)]
    pub use ibc_core_host::*;
}

/// Re-exports ICS-25 implementation from the `ibc-core-handler` crate
pub mod handler {
    #[doc(inline)]
    pub use ibc_core_handler::*;
}

/// Re-exports ICS-26 implementation from the `ibc-core-router` crate
pub mod router {
    #[doc(inline)]
    pub use ibc_core_router::*;
}
