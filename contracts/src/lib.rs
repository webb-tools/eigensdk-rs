use alloy_sol_types::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    AVSDirectory,
    "./out/AVSDirectory.sol/AVSDirectory.json"
);

sol!(
    #[allow(missing_docs)]
    #[derive(Debug)]
    #[sol(rpc)]
    BlsApkRegistry,
    "./out/BLSApkRegistry.sol/BLSApkRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DelegationManager,
    "./out/DelegationManager.sol/DelegationManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EigenPod,
    "./lib/eigenlayer-middleware/out/EigenPod.sol/EigenPod.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EigenPodManager,
    "./out/EigenPodManager.sol/EigenPodManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EigenStrategy,
    "./lib/eigenlayer-middleware/lib/eigenlayer-contracts/out//EigenStrategy.sol/EigenStrategy.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EmptyContract,
    "./lib/eigenlayer-middleware/out/EmptyContract.sol/EmptyContract.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Mock,
    "./lib/eigenlayer-middleware/lib/eigenlayer-contracts/out/ERC20Mock.sol/ERC20Mock.json"
);

sol!(
    #[allow(missing_docs)]
    #[derive(Debug)]
    #[sol(rpc)]
    IBlsApkRegistry,
    "./out/IBLSApkRegistry.sol/IBLSApkRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IAVSDirectory,
    "./out/IAVSDirectory.sol/IAVSDirectory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IDelegationManager,
    "./out/IDelegationManager.sol/IDelegationManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "./out/IERC20.sol/IERC20.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IIndexRegistry,
    "./out/IIndexRegistry.sol/IIndexRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IndexRegistry,
    "./lib/eigenlayer-middleware/out/IndexRegistry.sol/IndexRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IStakeRegistry,
    "./out/IStakeRegistry.sol/IStakeRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ISlasher,
    "./out/ISlasher.sol/ISlasher.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IStrategy,
    "./out/IStrategy.sol/IStrategy.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IStrategyManager,
    "./out/IStrategyManager.sol/IStrategyManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    OperatorStateRetriever,
    "./out/OperatorStateRetriever.sol/OperatorStateRetriever.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PauserRegistry,
    "./lib/eigenlayer-middleware/out/IPauserRegistry.sol/IPauserRegistry.json"
);

sol!(
    #[allow(missing_docs, clippy::too_many_arguments)]
    #[sol(rpc)]
    RegistryCoordinator,
    "./out/RegistryCoordinator.sol/RegistryCoordinator.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ProxyAdmin,
    "./lib/eigenlayer-middleware/out/ProxyAdmin.sol/ProxyAdmin.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ServiceManagerBase,
    "./out/ServiceManagerBase.sol/ServiceManagerBase.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    StakeRegistry,
    "./out/StakeRegistry.sol/StakeRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    StrategyBaseTVLLimits,
    "./lib/eigenlayer-middleware/lib/eigenlayer-contracts/out/StrategyBaseTVLLimits.sol/StrategyBaseTVLLimits.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    StrategyManager,
    "./out/StrategyManager.sol/StrategyManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TransparentUpgradeableProxy,
    "./lib/eigenlayer-middleware/out/TransparentUpgradeableProxy.sol/TransparentUpgradeableProxy.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EIP1271SignatureUtils,
    "./out/EIP1271SignatureUtils.sol/EIP1271SignatureUtils.json"
);
