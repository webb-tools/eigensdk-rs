use crate::encode_params;
use alloy::signers::Signer;
use alloy_primitives::{address, bytes::Bytes, Address, Keccak256, Uint, U256};
use alloy_provider::network::{TransactionBuilder, TxSigner};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types_eth::BlockId;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::{
    abi::Encoder,
    abi::{self, token::*},
    private::SolTypeValue,
    SolValue, Word,
};
use alloy_transport_ws::WsConnect;
use anvil::spawn;
use ark_bn254::{Fq as F, Fr, G1Affine, G2Affine, G2Projective};
use eigen_contracts::{
    RegistryCoordinator::{OperatorSetParam, StrategyParams},
    *,
};
use gadget_common::subxt_signer::bip39::rand_core::OsRng;
use incredible_squaring_avs::avs::{
    IncredibleSquaringServiceManager, IncredibleSquaringTaskManager,
};
use k256::{ecdsa::VerifyingKey, elliptic_curve::SecretKey};
use std::{path::Path, time::Duration};
use url::Url;

pub static BLS_PASSWORD: &str = "BLS_PASSWORD";
pub static ECDSA_PASSWORD: &str = "ECDSA_PASSWORD";
pub static TASK_RESPONSE_WINDOW_BLOCK: u32 = 10;
pub static TASK_DURATION_BLOCKS: u32 = 0;
// static QUORUM_THRESHOLD_PERCENTAGE: U256 = U256::from(100);
pub static AGGREGATOR_ADDR: Address = address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720");
pub static TASK_GENERATOR_ADDR: Address = address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720");

pub struct ContractAddresses {
    pub service_manager: Address,
    pub registry_coordinator: Address,
    pub operator_state_retriever: Address,
    pub delegation_manager: Address,
    pub avs_directory: Address,
    pub operator: Address,
}
pub async fn run_anvil_testnet() -> ContractAddresses {
    // Initialize the logger
    let _ = env_logger::try_init();

    let (api, mut handle) = spawn(anvil::NodeConfig::test().with_port(8545)).await;
    api.anvil_auto_impersonate_account(true).await.unwrap();

    let _http_provider = ProviderBuilder::new()
        .on_http(Url::parse(&handle.http_endpoint()).unwrap())
        .root()
        .clone();
    // todo: http_provider is unused

    let provider = ProviderBuilder::new()
        .on_builtin(&handle.ws_endpoint())
        .await
        .unwrap();

    let accounts = handle.dev_wallets().collect::<Vec<_>>();
    let from = accounts[0].address();
    let _to = accounts[1].address();

    let dev_account = accounts[0].address();

    let _amount = handle
        .genesis_balance()
        .checked_div(U256::from(2u64))
        .unwrap();

    let _gas_price = provider.get_gas_price().await.unwrap();

    // Empty address for initial deployment of all contracts
    let empty_address = Address::default();

    // let strategy_manager_addr = address!("Dc64a140Aa3E981100a9becA4E685f962f0cF6C9");
    // let delegation_manager_addr = address!("Cf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9");
    // let avs_directory_addr = address!("5FC8d32690cc91D4c39d9d3abcBD16989F875707");
    // let proxy_admin_addr = address!("5FbDB2315678afecb367f032d93F642f64180aa3");
    // let pauser_registry_addr = address!("e7f1725E7734CE288F8367e1Bb143E90bb3F0512");
    // let base_strategy_addr = address!("322813Fd9A801c5507c9de605d63CEA4f2CE6c44");

    // Deploy Eigenlayer Contracts
    // let strategy_manager_addr = address!("Dc64a140Aa3E981100a9becA4E685f962f0cF6C9");
    // let delegation_manager_addr = address!("Cf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9");
    // let avs_directory_addr = address!("5FC8d32690cc91D4c39d9d3abcBD16989F875707");
    // let proxy_admin_addr = address!("5FbDB2315678afecb367f032d93F642f64180aa3");
    // let pauser_registry_addr = address!("e7f1725E7734CE288F8367e1Bb143E90bb3F0512");
    // let base_strategy_addr = address!("322813Fd9A801c5507c9de605d63CEA4f2CE6c44");

    // let istrategy_manager = IStrategyManager::new(strategy_manager_addr, provider.clone());
    // let idelegation_manager =
    //     IDelegationManager::new(delegation_manager_addr, provider.clone());
    // let iavs_directory = IAVSDirectory::new(avs_directory_addr, provider.clone());
    // let proxy_admin = ProxyAdmin::new(proxy_admin_addr, provider.clone());
    // let pauser_registry = PauserRegistry::new(pauser_registry_addr, provider.clone());
    // let base_strategy = StrategyBaseTVLLimits::new(base_strategy_addr, provider.clone());

    let istrategy_manager = IStrategyManager::deploy(provider.clone()).await.unwrap();
    let &strategy_manager_addr = istrategy_manager.address();

    let idelegation_manager = IDelegationManager::deploy(provider.clone()).await.unwrap();
    let &delegation_manager_addr = idelegation_manager.address();

    let iavs_directory = IAVSDirectory::deploy(provider.clone()).await.unwrap();
    let &avs_directory_addr = iavs_directory.address();

    let proxy_admin = ProxyAdmin::deploy_builder(provider.clone())
        .from(dev_account)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap()
        .contract_address
        .unwrap();
    let proxy_admin_addr = proxy_admin;
    // let &proxy_admin_addr = proxy_admin.address();

    let pauser_registry = PauserRegistry::deploy(provider.clone()).await.unwrap();
    let &pauser_registry_addr = pauser_registry.address();

    let base_strategy = StrategyBaseTVLLimits::deploy(provider.clone(), Default::default())
        .await
        .unwrap();
    let &base_strategy_addr = base_strategy.address();

    let erc20_mock = ERC20Mock::deploy(provider.clone()).await.unwrap();
    let &erc20_mock_addr = erc20_mock.address();

    let ierc20 = IERC20::new(erc20_mock_addr, provider.clone());
    let &ierc20_addr = ierc20.address();

    // Function with signature initialize(uint256,uint256,address,address) and selector 0x019e2729.
    let function_signature = "initialize(uint256,uint256,address,address)";
    // let params = vec![
    //     1.tokenize(),
    //     100.tokenize(),
    //     ierc20_addr.tokenize(),
    //     pauser_registry_addr.tokenize(),
    //     // WordToken(Word::from(1)),
    //     // WordToken(Word::from(100)),
    //     // WordToken(Word::from(ierc20_addr.as_slice())),
    //     // WordToken(Word::from(pauser_registry_addr.as_slice())),
    // ];
    // let encoded_data = encode_with_selector(function_signature, params);
    // let encoded_data = encode_params!(function_signature, 1, 100, ierc20_addr, pauser_registry_addr);

    let mut hasher = Keccak256::new();
    hasher.update(function_signature);
    let function_selector = &hasher.finalize()[..4];
    let hex_selector = hex::encode(function_selector);
    log::info!("Function selector as hex: {:?}", hex_selector);
    let mut data = Vec::from(function_selector);
    // let encoded_param = SolValue::abi_encode(&1);
    // data.extend_from_slice(&encoded_param);
    // let encoded_param = SolValue::abi_encode(&100);
    // data.extend_from_slice(&encoded_param);
    // let encoded_param = SolValue::abi_encode(&ierc20_addr);
    // data.extend_from_slice(&encoded_param);
    // let encoded_param = SolValue::abi_encode(&pauser_registry_addr);
    // data.extend_from_slice(&encoded_param);
    let token = 1.tokenize();
    let encoded_param = abi::encode(&token);
    data.extend(&encoded_param);
    let token = 100.tokenize();
    let encoded_param = abi::encode(&token);
    data.extend(&encoded_param);
    let token = ierc20_addr.tokenize();
    let encoded_param = abi::encode(&token);
    data.extend(&encoded_param);
    let token = pauser_registry_addr.tokenize();
    let encoded_param = abi::encode(&token);
    data.extend(&encoded_param);

    let encoded_data = alloy_primitives::Bytes::from(data);

    let strategy_proxy = TransparentUpgradeableProxy::deploy(
        provider.clone(),
        base_strategy_addr,
        proxy_admin_addr,
        encoded_data,
    )
    .await
    .unwrap();
    let &strategy_proxy_addr = strategy_proxy.address();

    let erc20_mock_strategy = StrategyBaseTVLLimits::deploy(provider.clone(), strategy_proxy_addr)
        .await
        .unwrap();
    let &erc20_mock_strategy_addr = erc20_mock_strategy.address();

    let strategies = vec![erc20_mock_strategy_addr];

    let add_strategies = istrategy_manager
        .addStrategiesToDepositWhitelist(strategies.clone(), vec![false])
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    println!("Add Strategies Receipt: {:?}", add_strategies);

    // Deploy Incredible Squaring Contracts
    let number_of_strategies = strategies.len();
    println!("Number of Strategies: {:?}", number_of_strategies);

    // let incredible_squaring_proxy_admin = ProxyAdmin::deploy(provider.clone()).await.unwrap();
    // let &incredible_squaring_proxy_admin_addr = incredible_squaring_proxy_admin.address();
    let incredible_squaring_proxy_admin = ProxyAdmin::deploy_builder(provider.clone())
        .from(dev_account)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap()
        .contract_address
        .unwrap();
    let incredible_squaring_proxy_admin_addr = incredible_squaring_proxy_admin;
    let incredible_squaring_proxy_admin =
        ProxyAdmin::new(incredible_squaring_proxy_admin_addr, provider.clone());
    let owner = incredible_squaring_proxy_admin
        .owner()
        .call()
        .await
        .unwrap()
        ._0;
    println!("Owner: {:?}", owner);

    let pausers = vec![dev_account, dev_account];

    // let incredible_squaring_pauser_registry_addr =
    //     PauserRegistry::deploy_builder(provider.clone())
    //         .from(dev_account)
    //         .send()
    //         .await
    //         .unwrap()
    //         .get_receipt()
    //         .await
    //         .unwrap()
    //         .contract_address
    //         .unwrap();
    // println!("Pauser Registry Address: {:?}", incredible_squaring_pauser_registry_addr);
    // let incredible_squaring_pauser_registry =
    //     PauserRegistry::new(incredible_squaring_pauser_registry_addr, provider.clone());
    // incredible_squaring_pauser_registry.address();
    // println!("Pauser Registry: {:?}", incredible_squaring_pauser_registry);

    let incredible_squaring_pauser_registry =
        PauserRegistry::deploy(provider.clone()).await.unwrap();
    let &incredible_squaring_pauser_registry_addr = incredible_squaring_pauser_registry.address();
    println!(
        "Pauser Registry Address: {:?}",
        incredible_squaring_pauser_registry_addr
    );

    // let unpauser = incredible_squaring_pauser_registry.unpauser().from(dev_account).call().await.unwrap()._0;
    // println!("Unpauser: {:?}", unpauser);
    //
    // let is_pauser = incredible_squaring_pauser_registry
    //     .isPauser(dev_account)
    //     .from(dev_account)
    //     .call()
    //     .await
    //     .unwrap();
    // println!("Is Dev Account 0 Pauser: {:?}", is_pauser._0);

    let empty_contract = EmptyContract::deploy(provider.clone()).await.unwrap();
    let &empty_contract_addr = empty_contract.address();

    let incredible_squaring_service_manager = IncredibleSquaringServiceManager::new(
        TransparentUpgradeableProxy::deploy(
            provider.clone(),
            empty_contract_addr,
            incredible_squaring_proxy_admin_addr,
            Bytes::from(""),
        )
        .await
        .unwrap()
        .address()
        .clone(),
        provider.clone(),
    );
    let &incredible_squaring_service_manager_addr = incredible_squaring_service_manager.address();

    let incredible_squaring_task_manager = IncredibleSquaringTaskManager::new(
        TransparentUpgradeableProxy::deploy(
            provider.clone(),
            empty_contract_addr,
            incredible_squaring_proxy_admin_addr,
            Bytes::from(""),
        )
        .await
        .unwrap()
        .address()
        .clone(),
        provider.clone(),
    );
    let &incredible_squaring_task_manager_addr = incredible_squaring_task_manager.address();

    let registry_coordinator = RegistryCoordinator::new(
        TransparentUpgradeableProxy::deploy(
            provider.clone(),
            empty_contract_addr,
            incredible_squaring_proxy_admin_addr,
            Bytes::from(""),
        )
        .await
        .unwrap()
        .address()
        .clone(),
        provider.clone(),
    );
    let &registry_coordinator_addr = registry_coordinator.address();

    let bls_apk_registry = BlsApkRegistry::new(
        TransparentUpgradeableProxy::deploy(
            provider.clone(),
            empty_contract_addr,
            incredible_squaring_proxy_admin_addr,
            Bytes::from(""),
        )
        .await
        .unwrap()
        .address()
        .clone(),
        provider.clone(),
    );
    let &bls_apk_registry_addr = bls_apk_registry.address();

    let bls_apk_registry = IBlsApkRegistry::new(
        TransparentUpgradeableProxy::deploy(
            provider.clone(),
            empty_contract_addr,
            incredible_squaring_proxy_admin_addr,
            Bytes::from(""),
        )
        .await
        .unwrap()
        .address()
        .clone(),
        provider.clone(),
    );
    let &bls_apk_registry_addr = bls_apk_registry.address();

    let index_registry = IIndexRegistry::new(
        TransparentUpgradeableProxy::deploy(
            provider.clone(),
            empty_contract_addr,
            incredible_squaring_proxy_admin_addr,
            Bytes::from(""),
        )
        .await
        .unwrap()
        .address()
        .clone(),
        provider.clone(),
    );
    let &index_registry_addr = index_registry.address();

    let stake_registry = IStakeRegistry::new(
        TransparentUpgradeableProxy::deploy(
            provider.clone(),
            empty_contract_addr,
            incredible_squaring_proxy_admin_addr,
            Bytes::from(""),
        )
        .await
        .unwrap()
        .address()
        .clone(),
        provider.clone(),
    );
    let &stake_registry_addr = stake_registry.address();

    let operator_state_retriever = OperatorStateRetriever::deploy(provider.clone())
        .await
        .unwrap();
    let &operator_state_retriever_addr = operator_state_retriever.address();

    //Now, deploy the implementation contracts using the proxy contracts as inputs
    let stake_registry_implementation = StakeRegistry::deploy(
        provider.clone(),
        registry_coordinator_addr,
        delegation_manager_addr,
    )
    .await
    .unwrap();
    let &stake_registry_implementation_addr = stake_registry_implementation.address();
    let stake_registry_upgrade = incredible_squaring_proxy_admin
        .upgrade(stake_registry_addr, stake_registry_implementation_addr)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    log::info!(
        "Stake Registry Upgrade Receipt: {:?}",
        stake_registry_upgrade
    );

    let bls_apk_registry_implementation =
        BlsApkRegistry::deploy(provider.clone(), registry_coordinator_addr)
            .await
            .unwrap();
    let &bls_apk_registry_implementation_addr = bls_apk_registry_implementation.address();
    let bls_apk_registry_upgrade = incredible_squaring_proxy_admin
        .upgrade(bls_apk_registry_addr, bls_apk_registry_implementation_addr)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    log::info!(
        "Bls Apk Registry Upgrade Receipt: {:?}",
        bls_apk_registry_upgrade
    );

    let index_registry_implementation =
        IndexRegistry::deploy(provider.clone(), registry_coordinator_addr)
            .await
            .unwrap();
    let &index_registry_implementation_addr = index_registry_implementation.address();
    let index_registry_upgrade = incredible_squaring_proxy_admin
        .upgrade(index_registry_addr, index_registry_implementation_addr)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    log::info!(
        "Index Registry Upgrade Receipt: {:?}",
        index_registry_upgrade
    );

    let registry_coordinator_implementation = RegistryCoordinator::deploy(
        provider.clone(),
        incredible_squaring_service_manager_addr,
        stake_registry_addr,
        bls_apk_registry_addr,
        index_registry_addr,
    )
    .await
    .unwrap();
    let &registry_coordinator_implementation_addr = registry_coordinator_implementation.address();

    let number_of_quorums = 1;
    // For each quorum we want to set up, we must define QuorumOperatorSetParam, minimumStakeForQuorum, and strategyParams
    let mut quorum_operator_set_params = Vec::<OperatorSetParam>::new();
    for i in 0..number_of_quorums {
        log::info!("Deploying quorum {}", i);
        quorum_operator_set_params.push(OperatorSetParam {
            maxOperatorCount: 10000,
            kickBIPsOfOperatorStake: 15000,
            kickBIPsOfTotalStake: 100,
        });
    }
    // Set to 0 for each quorum
    let mut quorums_minimum_stake = Vec::<u128>::new();
    let mut quorums_strategy_params = Vec::<Vec<StrategyParams>>::new();
    for j in 0..number_of_quorums {
        quorums_strategy_params.push(Vec::<StrategyParams>::new());
        quorums_minimum_stake.push(0);
        for k in 0..number_of_strategies {
            quorums_strategy_params[j].push(StrategyParams {
                strategy: strategies[j],
                multiplier: 1,
            });
        }
    }

    // Function with signature initialize(address,address,address,address,uint256,(uint32,uint16,uint16)[],uint96[],(address,uint96)[][]) and selector 0xdd8283f3.
    let function_signature = "initialize(address,address,address,address,uint256,(uint32,uint16,uint16)[],uint96[],(address,uint96)[][])";

    let single_test = 0.tokenize();
    println!("Word Tokenize: {:?}", single_test);
    let vec_test = quorum_operator_set_params.tokenize();
    println!("Vec Tokenize: {:?}", vec_test);

    let encoded_word = abi::encode(&single_test);

    let encoded_vec = abi::encode(&vec_test);

    println!(
        "Quorums Strategy Params: Strategy: {:?} Multiplier: {:?}",
        quorums_strategy_params[0][0].strategy, quorums_strategy_params[0][0].multiplier
    );
    println!("Quorum Operator Set Params: Max Operators: {:?}, Kick BIPs of Operator Stake: {:?}, Kick BIPs of Total Stake: {:?}", quorum_operator_set_params[0].maxOperatorCount, quorum_operator_set_params[0].kickBIPsOfOperatorStake, quorum_operator_set_params[0].kickBIPsOfTotalStake);
    println!("Quorums Minimum Stake: {:?}", quorums_minimum_stake);

    // let mut hasher = Keccak256::new();
    // hasher.update(function_signature);
    // let function_selector = &hasher.finalize()[..4];
    // let mut data = Vec::from(function_selector);
    // data.extend_from_slice(&abi::encode(&pausers[0].tokenize()));
    // data.extend_from_slice(&abi::encode(&pausers[0].tokenize()));
    // data.extend_from_slice(&abi::encode(&pausers[0].tokenize()));
    // data.extend_from_slice(&abi::encode(&pausers[1].tokenize()));
    // data.extend_from_slice(&abi::encode(&0.tokenize()));
    // data.extend_from_slice(&abi::encode(&quorum_operator_set_params.tokenize()));
    // data.extend_from_slice(&abi::encode(&quorums_minimum_stake.tokenize()));
    // data.extend_from_slice(&abi::encode(&quorums_strategy_params.tokenize()));
    // let encoded_data = alloy_primitives::Bytes::from(data);

    let encoded_data = encode_params!(
        function_signature,
        pausers[0],
        pausers[0],
        pausers[0],
        pausers[1],
        0,
        quorum_operator_set_params,
        quorums_minimum_stake,
        quorums_strategy_params
    );
    // let registry_coordinator_upgrade = incredible_squaring_proxy_admin
    //     .upgradeAndCall(
    //         registry_coordinator_addr,
    //         registry_coordinator_implementation_addr,
    //         encoded_data,
    //     )
    //     // .from(dev_account)
    //     .send()
    //     .await
    //     .unwrap()
    //     .get_receipt()
    //     .await
    //     .unwrap();
    // log::info!(
    //     "Registry Coordinator Upgrade Receipt: {:?}",
    //     registry_coordinator_upgrade
    // );

    let registry_coordinator_upgrade = incredible_squaring_proxy_admin
        .upgrade(
            registry_coordinator_addr,
            registry_coordinator_implementation_addr,
        )
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    assert!(registry_coordinator_upgrade.status());

    let incredible_squaring_service_manager_implementation =
        IncredibleSquaringServiceManager::deploy(
            provider.clone(),
            avs_directory_addr,
            registry_coordinator_addr,
            stake_registry_addr,
            incredible_squaring_task_manager_addr,
        )
        .await
        .unwrap();
    let &incredible_squaring_service_manager_implementation_addr =
        incredible_squaring_service_manager_implementation.address();
    let incredible_squaring_service_manager_upgrade = incredible_squaring_proxy_admin
        .upgrade(
            incredible_squaring_service_manager_addr,
            incredible_squaring_service_manager_implementation_addr,
        )
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    log::info!(
        "Incredible Squaring Service Manager Upgrade Receipt: {:?}",
        incredible_squaring_service_manager_upgrade
    );

    // Function with signature initialize(address,address,address,address) and selector 0xf8c8765e.
    let function_signature = "initialize(address,address,address,address)";
    // let params = vec![
    //     pauser_registry_addr.tokenize(),
    //     pausers[0].tokenize(),
    //     AGGREGATOR_ADDR.tokenize(),
    //     TASK_GENERATOR_ADDR.tokenize(),
    // ];
    // let encoded_data =
    //     eigen_utils::test_utils::abi::encode_with_selector(function_signature, params);

    let encoded_data = encode_params!(
        function_signature,
        pauser_registry_addr,
        pausers[0],
        AGGREGATOR_ADDR,
        TASK_GENERATOR_ADDR
    );

    println!(
        "Registry Coordinator Address: {:?}",
        registry_coordinator_addr
    );
    // let incredible_squaring_task_manager_implementation =
    //     IncredibleSquaringTaskManager::deploy(
    //         provider.clone(),
    //         registry_coordinator_addr,
    //         // TASK_RESPONSE_WINDOW_BLOCK,
    //         100u32,
    //     )
    //     .await
    //     .unwrap();
    let incredible_squaring_task_manager_implementation_addr =
        IncredibleSquaringTaskManager::deploy_builder(
            provider.clone(),
            registry_coordinator_addr,
            TASK_RESPONSE_WINDOW_BLOCK,
        )
        .from(dev_account)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap()
        .contract_address
        .unwrap();

    // let &incredible_squaring_task_manager_implementation_addr =
    //     incredible_squaring_task_manager_implementation.address();
    let incredible_squaring_task_manager_upgrade = incredible_squaring_proxy_admin
        .upgradeAndCall(
            incredible_squaring_task_manager_addr,
            incredible_squaring_service_manager_implementation_addr,
            alloy_primitives::Bytes::from(encoded_data),
        )
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    log::info!(
        "Incredible Squaring Task Manager Upgrade Receipt: {:?}",
        incredible_squaring_task_manager_upgrade
    );

    log::info!("ERC20MOCK ADDRESS: {:?}", erc20_mock_addr);
    log::info!("ERC20MOCK STRATEGY ADDRESS: {:?}", erc20_mock_strategy_addr);
    log::info!(
        "INCREDIBLE SQUARING TASK MANAGER ADDRESS: {:?}",
        incredible_squaring_task_manager_addr
    );
    log::info!(
        "INCREDIBLE SQUARING TASK MANAGER IMPLEMENTATION ADDRESS: {:?}",
        incredible_squaring_task_manager_implementation_addr
    );
    log::info!(
        "INCREDIBLE SQUARING SERVICE MANAGER ADDRESS: {:?}",
        incredible_squaring_service_manager_addr
    );
    log::info!(
        "INCREDIBLE SQUARING SERVICE MANAGER IMPLEMENTATION ADDRESS: {:?}",
        incredible_squaring_service_manager_implementation_addr
    );
    log::info!(
        "REGISTRY COORDINATOR ADDRESS: {:?}",
        registry_coordinator_addr
    );
    log::info!(
        "REGISTRY COORDINATOR IMPLEMENTATION ADDRESS: {:?}",
        registry_coordinator_implementation_addr
    );
    log::info!(
        "OPERATOR STATE RETRIEVER ADDRESS: {:?}",
        operator_state_retriever_addr
    );

    // let _block = provider
    //     .get_block(BlockId::latest(), false.into())
    //     .await
    //     .unwrap()
    //     .unwrap();
    //
    // api.anvil_set_auto_mine(true).await.unwrap();
    // let run_testnet = async move {
    //     let serv = handle.servers.pop().unwrap();
    //     let res = serv.await.unwrap();
    //     res.unwrap();
    // };
    // let spawner_task_manager_address = task_manager_addr.clone();
    // // let spawner_provider = provider.clone();
    // let spawner_provider = provider;
    // let task_spawner = async move {
    //     let manager = IncredibleSquaringTaskManager::new(
    //         spawner_task_manager_address,
    //         spawner_provider.clone(),
    //     );
    //     loop {
    //         api.mine_one().await;
    //         log::info!("About to create new task");
    //         tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    //         let result = manager
    //             .createNewTask(U256::from(2), 100u32, Bytes::from("0"))
    //             .send()
    //             .await
    //             .unwrap()
    //             .watch()
    //             .await
    //             .unwrap();
    //         api.mine_one().await;
    //         log::info!("Created new task: {:?}", result);
    //         // let latest_task = manager.latestTaskNum().call().await.unwrap()._0;
    //         // log::info!("Latest task: {:?}", latest_task);
    //         // let task_hash = manager.allTaskHashes(latest_task).call().await.unwrap()._0;
    //         // log::info!("Task info: {:?}", task_hash);
    //     }
    // };
    // tokio::spawn(run_testnet);
    // tokio::spawn(task_spawner);

    ContractAddresses {
        service_manager: incredible_squaring_service_manager_implementation_addr,
        registry_coordinator: registry_coordinator_implementation_addr,
        operator_state_retriever: operator_state_retriever_addr,
        delegation_manager: delegation_manager_addr,
        avs_directory: avs_directory_addr,
        operator: from,
    }
}
