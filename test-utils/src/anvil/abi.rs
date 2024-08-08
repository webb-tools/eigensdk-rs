use alloy_sol_types::abi;
use alloy_sol_types::abi::Token;
use sha3::{Digest, Keccak256};

/// Macro that acts as the Rust equivalent of Solidity's `abi.encodeWithSelector`. Returns [alloy_primitives::Bytes]
#[macro_export]
macro_rules! encode_params {
    ($function_signature:expr, $($param:expr),*) => {{
        let mut hasher = Keccak256::new();
        hasher.update($function_signature);
        let function_selector = &hasher.finalize()[..4];
        let mut data = Vec::from(function_selector);

        $(
            let p = $param.tokenize();
            data.extend_from_slice(abi::encode(&p).as_slice());
        )*

        alloy_primitives::Bytes::from(data)
    }};
}
