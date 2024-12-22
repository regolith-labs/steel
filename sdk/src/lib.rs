pub mod solana {
    #[cfg(feature = "wasm")]
    pub use solana_client_wasm::solana_sdk::*;
    
    #[cfg(not(feature = "wasm"))]
    pub use solana_sdk::*;
}

pub mod rpc_client {
    #[cfg(feature = "wasm")]
    pub use solana_client_wasm::nonblocking::rpc_client::*;

    #[cfg(not(feature = "wasm"))]
    pub use solana_client::nonblocking::rpc_client::*;
}

pub mod program {
    #[cfg(feature = "wasm")]
    pub use solana_extra_wasm::program::*;

    #[cfg(not(feature = "wasm"))]
    pub use solana_sdk::program::*;
}

pub mod account_decoder {
    #[cfg(feature = "wasm")]
    pub use solana_extra_wasm::account_decoder::*;

    #[cfg(not(feature = "wasm"))]
    pub use solana_account_decoder::*;
}

pub mod transaction_status {
    #[cfg(feature = "wasm")]
    pub use solana_extra_wasm::transaction_status::*;

    #[cfg(not(feature = "wasm"))]
    pub use solana_transaction_status::*;
}

pub mod time {
    #[cfg(feature = "wasm")]
    pub use web_time::*;

    #[cfg(not(feature = "wasm"))]
    pub use std::time::*;
}
