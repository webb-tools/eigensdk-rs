use crate::anvil::testnet::incredible_squaring::*;
use alloy_provider::Provider;
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use alloy_transport_ws::WsConnect;
use incredible_squaring_avs::operator::*;
use k256::ecdsa::SigningKey;
use k256::elliptic_curve::SecretKey;

async fn run_full_incredible_squaring_test() {
    let _ = env_logger::try_init();

    // Runs new Anvil Testnet - used for deploying programmatically in rust
    let contract_addresses = run_anvil_testnet().await;

    // // Runs saved Anvil Testnet - loads from saved chain state JSON file
    // let chain = eigen_utils::test_utils::local_chain::LocalEvmChain::new_with_chain_state(
    //     31337,
    //     String::from("eigen-testnet"),
    //     Path::new("../../eigen-utils/saved-anvil-state.json"),
    //     Some(8545u16),
    // );
    // let chain_id = chain.chain_id();
    // let chain_name = chain.name();
    // println!("chain_id: {:?}", chain_id);
    // println!("chain_name: {:?}", chain_name);

    // let account_one = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    // let account_two = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");

    // let contract_addresses = ContractAddresses {
    //     service_manager: address!("84eA74d481Ee0A5332c457a4d796187F6Ba67fEB"),
    //     registry_coordinator: address!("a82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9"),
    //     operator_state_retriever: address!("95401dc811bb5740090279Ba06cfA8fcF6113778"),
    //     delegation_manager: address!("Cf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"),
    //     avs_directory: address!("5FC8d32690cc91D4c39d9d3abcBD16989F875707"),
    //     operator: account_two,
    // };

    // Implementation version of addresses
    // let contract_addresses = ContractAddresses {
    //     service_manager: address!("84eA74d481Ee0A5332c457a4d796187F6Ba67fEB"),
    //     registry_coordinator: address!("9d4454B023096f34B160D6B654540c56A1F81688"),
    //     operator_state_retriever: address!("95401dc811bb5740090279Ba06cfA8fcF6113778"),
    //     delegation_manager: address!("B7f8BC63BbcaD18155201308C8f3540b07f84F5e"),
    //     avs_directory: address!("0DCd1Bf9A1b36cE34237eEaFef220932846BCD82"),
    //     operator: address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
    // };

    let http_endpoint = "http://127.0.0.1:8545";
    let ws_endpoint = "ws://127.0.0.1:8545";
    let node_config = NodeConfig {
        node_api_ip_port_address: "127.0.0.1:9808".to_string(),
        eth_rpc_url: http_endpoint.to_string(),
        eth_ws_url: ws_endpoint.to_string(),
        bls_private_key_store_path: "./keystore/bls".to_string(),
        ecdsa_private_key_store_path: "./keystore/ecdsa".to_string(),
        incredible_squaring_service_manager_addr: contract_addresses.service_manager.to_string(),
        avs_registry_coordinator_addr: contract_addresses.registry_coordinator.to_string(),
        operator_state_retriever_addr: contract_addresses.operator_state_retriever.to_string(),
        eigen_metrics_ip_port_address: "127.0.0.1:9100".to_string(),
        delegation_manager_addr: contract_addresses.delegation_manager.to_string(),
        avs_directory_addr: contract_addresses.avs_directory.to_string(),
        operator_address: contract_addresses.operator.to_string(),
        enable_metrics: false,
        enable_node_api: false,
        server_ip_port_address: "".to_string(),
    };

    let operator_info_service = OperatorInfoService {};

    let hex_key =
        hex::decode("59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d").unwrap();
    let secret_key = SecretKey::from_slice(&hex_key).unwrap();
    let signing_key = SigningKey::from(secret_key.clone());
    let signer = EigenGadgetSigner {
        signer: PrivateKeySigner::from_signing_key(signing_key),
    };

    println!("Creating HTTP Provider...");

    let http_provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_http(http_endpoint.parse().unwrap())
        .root()
        .clone()
        .boxed();

    println!("Creating WS Provider...");

    let ws_provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_ws(WsConnect::new(ws_endpoint))
        .await
        .unwrap()
        .root()
        .clone()
        .boxed();

    println!("Now setting up Operator!");

    let operator = Operator::<NodeConfig, OperatorInfoService>::new_from_config(
        node_config.clone(),
        EigenGadgetProvider {
            provider: http_provider,
        },
        EigenGadgetProvider {
            provider: ws_provider,
        },
        operator_info_service,
        signer,
    )
    .await
    .unwrap();

    operator.start().await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::Signer;
    use alloy_provider::network::TransactionBuilder;
    use eigen_utils::crypto::bls::KeyPair;
    use k256::ecdsa::VerifyingKey;
    use k256::elliptic_curve::SecretKey;
    use std::path::Path;
    use std::time::Duration;

    #[tokio::test]
    async fn test_anvil() {
        env_logger::init();
        run_full_incredible_squaring_test().await;
    }

    #[tokio::test]
    async fn test_start_chain_from_state() {
        env_logger::init();

        let chain = crate::anvil::local_chain::LocalEvmChain::new_with_chain_state(
            31337,
            String::from("eigen-testnet"),
            Path::new("../../eigen-utils/eigen-gadget-anvil-state.json"),
            Some(8545u16),
        );
        let chain_id = chain.chain_id();
        let chain_name = chain.name();
        println!("chain_id: {:?}", chain_id);
        println!("chain_name: {:?}", chain_name);
        let addresses = chain.addresses();
        println!("addresses: {:?}", addresses);
        tokio::time::sleep(Duration::from_secs(5)).await;
        println!("Now shutting down...");
        chain.shutdown();
    }

    #[tokio::test]
    async fn test_generate_keys() {
        env_logger::init();

        // ---------------- BLS ----------------
        let bls_pair = KeyPair::gen_random().unwrap();
        bls_pair
            .save_to_file("./keystore/bls", BLS_PASSWORD)
            .unwrap();
        let bls_keys = KeyPair::read_private_key_from_file("./keystore/bls", BLS_PASSWORD).unwrap();
        assert_eq!(bls_pair.priv_key, bls_keys.priv_key);
        assert_eq!(bls_pair.pub_key, bls_keys.pub_key);

        //---------------- ECDSA ----------------
        // First Account
        // let hex_key =
        //     hex::decode("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        //         .unwrap();

        // Second Account
        let hex_key =
            hex::decode("59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d")
                .unwrap();

        let secret_key = SecretKey::from_slice(&hex_key).unwrap();
        let signing_key = SigningKey::from(secret_key.clone());
        let public_key = secret_key.public_key();
        let verifying_key = VerifyingKey::from(public_key);
        eigen_utils::crypto::ecdsa::write_key("./keystore/ecdsa", &secret_key, ECDSA_PASSWORD)
            .unwrap();

        let read_ecdsa_secret_key =
            eigen_utils::crypto::ecdsa::read_key("./keystore/ecdsa", ECDSA_PASSWORD).unwrap();
        let read_ecdsa_public_key = read_ecdsa_secret_key.public_key();
        let read_ecdsa_signing_key = SigningKey::from(&read_ecdsa_secret_key);
        let read_ecdsa_verifying_key = VerifyingKey::from(&read_ecdsa_signing_key);

        assert_eq!(secret_key, read_ecdsa_secret_key);
        assert_eq!(public_key, read_ecdsa_public_key);
        assert_eq!(signing_key, read_ecdsa_signing_key);
        assert_eq!(verifying_key, read_ecdsa_verifying_key);
    }
}
