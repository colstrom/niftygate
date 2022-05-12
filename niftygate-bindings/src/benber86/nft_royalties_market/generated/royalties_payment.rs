#[allow(dead_code)]
pub mod royalties_payment {
    # [rustfmt :: skip] use ethcontract as ethcontract ;
    #[doc = "Generated by `ethcontract`"]
    #[derive(Clone)]
    pub struct Contract {
        methods: Methods,
    }
    impl Contract {
        #[doc = r" Retrieves the raw contract instance used to generate the type safe"]
        #[doc = r" API for this contract."]
        pub fn raw_contract() -> &'static self::ethcontract::Contract {
            use self::ethcontract::common::artifact::truffle::TruffleLoader;
            use self::ethcontract::private::lazy_static;
            use self::ethcontract::Contract;
            lazy_static! {
                pub static ref CONTRACT: Contract = {
                    # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"RoyaltiesPayment\",\"abi\":[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_payees\",\"type\":\"address[]\"}]},{\"type\":\"function\",\"name\":\"withdrawAll\",\"inputs\":[],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"withdrawErc20\",\"inputs\":[{\"name\":\"token\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"payees\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"addPayee\",\"inputs\":[{\"name\":\"payee\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"balances\",\"inputs\":[{\"name\":\"\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"userIndex\",\"type\":\"uint256\"},{\"name\":\"balance\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"withdraw\",\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"removePayee\",\"inputs\":[{\"name\":\"payee\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"payAll\",\"inputs\":[],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true}],\"anonymous\":false},{\"type\":\"receive\"}],\"bytecode\":\"60806040523480156200001157600080fd5b5060405162000f7938038062000f798339810160408190526200003491620001f7565b6200003f33620000f3565b80516200005490600190602084019062000143565b5060005b600154811015620000eb5760405180604001604052808260016200007d9190620002df565b815260200160008152506002600060018481548110620000a157620000a1620002fa565b60009182526020808320909101546001600160a01b031683528281019390935260409091019020825181559101516001919091015580620000e28162000310565b91505062000058565b50506200032e565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b8280548282559060005260206000209081019282156200019b579160200282015b828111156200019b57825182546001600160a01b0319166001600160a01b0390911617825560209092019160019091019062000164565b50620001a9929150620001ad565b5090565b5b80821115620001a95760008155600101620001ae565b634e487b7160e01b600052604160045260246000fd5b80516001600160a01b0381168114620001f257600080fd5b919050565b600060208083850312156200020b57600080fd5b82516001600160401b03808211156200022357600080fd5b818501915085601f8301126200023857600080fd5b8151818111156200024d576200024d620001c4565b8060051b604051601f19603f83011681018181108582111715620002755762000275620001c4565b6040529182528482019250838101850191888311156200029457600080fd5b938501935b82851015620002bd57620002ad85620001da565b8452938501939285019262000299565b98975050505050505050565b634e487b7160e01b600052601160045260246000fd5b60008219821115620002f557620002f5620002c9565b500190565b634e487b7160e01b600052603260045260246000fd5b6000600019821415620003275762000327620002c9565b5060010190565b610c3b806200033e6000396000f3fe6080604052600436106100a05760003560e01c8063715018a611610064578063715018a614610214578063853828b6146102295780638da5cb5b1461023e578063c7e42b1b1461025c578063dce59c001461027c578063f2fde38b1461029c57600080fd5b806327e235e3146101375780632e1a7d4d146101855780633ed35855146101a75780635806beaf146101c757806363037b0c146101dc57600080fd5b366101325760015434906000906100b79083610aab565b905060005b60015481101561012d578160026000600184815481106100de576100de610acd565b60009182526020808320909101546001600160a01b0316835282019290925260400181206001018054909190610115908490610ae3565b9091555081905061012581610afb565b9150506100bc565b505050005b600080fd5b34801561014357600080fd5b5061016b610152366004610b2b565b6002602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b34801561019157600080fd5b506101a56101a0366004610b4f565b6102bc565b005b3480156101b357600080fd5b506101a56101c2366004610b2b565b6103e7565b3480156101d357600080fd5b506101a5610582565b3480156101e857600080fd5b506101fc6101f7366004610b4f565b6105b6565b6040516001600160a01b03909116815260200161017c565b34801561022057600080fd5b506101a56105e0565b34801561023557600080fd5b506101a5610614565b34801561024a57600080fd5b506000546001600160a01b03166101fc565b34801561026857600080fd5b506101a5610277366004610b2b565b61069b565b34801561028857600080fd5b506101a5610297366004610b2b565b610816565b3480156102a857600080fd5b506101a56102b7366004610b2b565b6108e0565b336000818152600260205260409020546103095760405162461bcd60e51b81526020600482015260096024820152684e6f7420706179656560b81b60448201526064015b60405180910390fd5b6000821161031657600080fd5b3360009081526002602052604090206001015482111561036f5760405162461bcd60e51b8152602060048201526014602482015273496e73756666696369656e742062616c616e636560601b6044820152606401610300565b3360009081526002602052604081206001018054849290610391908490610b68565b9091555050604051339083905b60006040518083038185875af1925050503d80600081146103db576040519150601f19603f3d011682016040523d82523d6000602084013e6103e0565b606091505b5050505050565b6000546001600160a01b031633146104115760405162461bcd60e51b815260040161030090610b7f565b6001600160a01b03811660009081526002602052604090205461043357600080fd5b61043b61097b565b6001600160a01b03811660009081526002602052604081205461046090600190610b68565b6001805491925090610473908290610b68565b8154811061048357610483610acd565b600091825260209091200154600180546001600160a01b0390921691839081106104af576104af610acd565b9060005260206000200160006101000a8154816001600160a01b0302191690836001600160a01b0316021790555060018054806104ee576104ee610bb4565b600082815260209020810160001990810180546001600160a01b0319169055019055600154811461056157610524816001610ae3565b600260006001848154811061053b5761053b610acd565b60009182526020808320909101546001600160a01b031683528201929092526040019020555b506001600160a01b0316600090815260026020526040812081815560010155565b6000546001600160a01b031633146105ac5760405162461bcd60e51b815260040161030090610b7f565b6105b461097b565b565b600181815481106105c657600080fd5b6000918252602090912001546001600160a01b0316905081565b6000546001600160a01b0316331461060a5760405162461bcd60e51b815260040161030090610b7f565b6105b46000610a45565b3360008181526002602052604090205461065c5760405162461bcd60e51b81526020600482015260096024820152684e6f7420706179656560b81b6044820152606401610300565b3360009081526002602052604090206001015461067857600080fd5b33600081815260026020526040808220600101805492905551909190829061039e565b6000546001600160a01b031633146106c55760405162461bcd60e51b815260040161030090610b7f565b6040516370a0823160e01b81523060048201526000906001600160a01b038316906370a0823190602401602060405180830381865afa15801561070c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107309190610bca565b90506000811161073f57600080fd5b60015460009061074f9083610aab565b905060005b60015481101561081057836001600160a01b031663a9059cbb6001838154811061078057610780610acd565b60009182526020909120015460405160e083901b6001600160e01b03191681526001600160a01b039091166004820152602481018590526044016020604051808303816000875af11580156107d9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107fd9190610be3565b508061080881610afb565b915050610754565b50505050565b6000546001600160a01b031633146108405760405162461bcd60e51b815260040161030090610b7f565b6001600160a01b0381166000908152600260205260409020541561086357600080fd5b61086b61097b565b6001805480820182557fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf60180546001600160a01b039093166001600160a01b03199093168317905560408051808201825282548152600060208281018281529582526002905291909120905181559151910155565b6000546001600160a01b0316331461090a5760405162461bcd60e51b815260040161030090610b7f565b6001600160a01b03811661096f5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610300565b61097881610a45565b50565b60005b6001548110156109785760006001828154811061099d5761099d610acd565b60009182526020808320909101546001600160a01b031680835260029091526040909120600101549091508015610a30576001600160a01b03821660008181526002602052604080822060010182905551839181818185875af1925050503d8060008114610a27576040519150601f19603f3d011682016040523d82523d6000602084013e610a2c565b606091505b5050505b50508080610a3d90610afb565b91505061097e565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b634e487b7160e01b600052601160045260246000fd5b600082610ac857634e487b7160e01b600052601260045260246000fd5b500490565b634e487b7160e01b600052603260045260246000fd5b60008219821115610af657610af6610a95565b500190565b6000600019821415610b0f57610b0f610a95565b5060010190565b6001600160a01b038116811461097857600080fd5b600060208284031215610b3d57600080fd5b8135610b4881610b16565b9392505050565b600060208284031215610b6157600080fd5b5035919050565b600082821015610b7a57610b7a610a95565b500390565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b634e487b7160e01b600052603160045260246000fd5b600060208284031215610bdc57600080fd5b5051919050565b600060208284031215610bf557600080fd5b81518015158114610b4857600080fdfea2646970667358221220ef245625cab6f58098918df5dbb236f2b2d6f5c5f617cd2e0709365cd8c7910964736f6c634300080a0033\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
                    contract
                };
            }
            &CONTRACT
        }
        #[doc = r" Creates a new contract instance with the specified `web3`"]
        #[doc = r" provider at the given `Address`."]
        #[doc = r""]
        #[doc = r" Note that this does not verify that a contract with a matching"]
        #[doc = r" `Abi` is actually deployed at the given address."]
        pub fn at<F, B, T>(
            web3: &self::ethcontract::web3::api::Web3<T>,
            address: self::ethcontract::Address,
        ) -> Self
        where
            F: std::future::Future<
                    Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
                > + Send
                + 'static,
            B: std::future::Future<
                    Output = Result<
                        Vec<Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>>,
                        self::ethcontract::web3::Error,
                    >,
                > + Send
                + 'static,
            T: self::ethcontract::web3::Transport<Out = F>
                + self::ethcontract::web3::BatchTransport<Batch = B>
                + Send
                + Sync
                + 'static,
        {
            Contract::with_deployment_info(web3, address, None)
        }
        #[doc = r" Creates a new contract instance with the specified `web3` provider with"]
        #[doc = r" the given `Abi` at the given `Address` and an optional transaction hash."]
        #[doc = r" This hash is used to retrieve contract related information such as the"]
        #[doc = r" creation block (which is useful for fetching all historic events)."]
        #[doc = r""]
        #[doc = r" Note that this does not verify that a contract with a matching `Abi` is"]
        #[doc = r" actually deployed at the given address nor that the transaction hash,"]
        #[doc = r" when provided, is actually for this contract deployment."]
        pub fn with_deployment_info<F, B, T>(
            web3: &self::ethcontract::web3::api::Web3<T>,
            address: self::ethcontract::Address,
            deployment_information: Option<ethcontract::common::DeploymentInformation>,
        ) -> Self
        where
            F: std::future::Future<
                    Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
                > + Send
                + 'static,
            B: std::future::Future<
                    Output = Result<
                        Vec<Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>>,
                        self::ethcontract::web3::Error,
                    >,
                > + Send
                + 'static,
            T: self::ethcontract::web3::Transport<Out = F>
                + self::ethcontract::web3::BatchTransport<Batch = B>
                + Send
                + Sync
                + 'static,
        {
            use self::ethcontract::transport::DynTransport;
            use self::ethcontract::web3::api::Web3;
            use self::ethcontract::Instance;
            let transport = DynTransport::new(web3.transport().clone());
            let web3 = Web3::new(transport);
            let abi = Self::raw_contract().abi.clone();
            let instance =
                Instance::with_deployment_info(web3, abi, address, deployment_information);
            Contract::from_raw(instance)
        }
        #[doc = r" Creates a contract from a raw instance."]
        fn from_raw(instance: self::ethcontract::dyns::DynInstance) -> Self {
            let methods = Methods { instance };
            Contract { methods }
        }
        #[doc = r" Returns the contract address being used by this instance."]
        pub fn address(&self) -> self::ethcontract::Address {
            self.raw_instance().address()
        }
        #[doc = r" Returns the deployment information of the contract"]
        #[doc = r" if it is known, `None` otherwise."]
        pub fn deployment_information(&self) -> Option<ethcontract::common::DeploymentInformation> {
            self.raw_instance().deployment_information()
        }
        #[doc = r" Returns a reference to the default method options used by this"]
        #[doc = r" contract."]
        pub fn defaults(&self) -> &self::ethcontract::contract::MethodDefaults {
            &self.raw_instance().defaults
        }
        #[doc = r" Returns a mutable reference to the default method options used"]
        #[doc = r" by this contract."]
        pub fn defaults_mut(&mut self) -> &mut self::ethcontract::contract::MethodDefaults {
            &mut self.raw_instance_mut().defaults
        }
        #[doc = r" Returns a reference to the raw runtime instance used by this"]
        #[doc = r" contract."]
        pub fn raw_instance(&self) -> &self::ethcontract::dyns::DynInstance {
            &self.methods.instance
        }
        #[doc = r" Returns a mutable reference to the raw runtime instance used by"]
        #[doc = r" this contract."]
        fn raw_instance_mut(&mut self) -> &mut self::ethcontract::dyns::DynInstance {
            &mut self.methods.instance
        }
    }
    impl std::fmt::Debug for Contract {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(RoyaltiesPayment))
                .field(&self.address())
                .finish()
        }
    }
    impl Contract {
        #[doc = "Generated by `ethcontract`"]
        #[allow(clippy::too_many_arguments)]
        pub fn builder<F, B, T>(
            web3: &self::ethcontract::web3::api::Web3<T>,
            payees: Vec<self::ethcontract::Address>,
        ) -> self::ethcontract::dyns::DynDeployBuilder<Self>
        where
            F: std::future::Future<
                    Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
                > + Send
                + 'static,
            B: std::future::Future<
                    Output = Result<
                        Vec<Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>>,
                        self::ethcontract::web3::Error,
                    >,
                > + Send
                + 'static,
            T: self::ethcontract::web3::Transport<Out = F>
                + self::ethcontract::web3::BatchTransport<Batch = B>
                + Send
                + Sync
                + 'static,
        {
            use self::ethcontract::contract::DeployBuilder;
            use self::ethcontract::dyns::DynTransport;
            use self::ethcontract::web3::api::Web3;
            let transport = DynTransport::new(web3.transport().clone());
            let web3 = Web3::new(transport);
            let bytecode = Self::raw_contract().bytecode.clone();
            DeployBuilder::new(web3, bytecode, (payees,)).expect("valid deployment args")
        }
    }
    impl self::ethcontract::contract::Deploy<self::ethcontract::dyns::DynTransport> for Contract {
        type Context = self::ethcontract::common::Bytecode;
        fn bytecode(cx: &Self::Context) -> &self::ethcontract::common::Bytecode {
            cx
        }
        fn abi(_: &Self::Context) -> &self::ethcontract::common::Abi {
            &Self::raw_contract().abi
        }
        fn from_deployment(
            web3: self::ethcontract::dyns::DynWeb3,
            address: self::ethcontract::Address,
            transaction_hash: self::ethcontract::H256,
            _: Self::Context,
        ) -> Self {
            Self::with_deployment_info(&web3, address, Some(transaction_hash.into()))
        }
    }
    impl Contract {
        #[doc = r" Returns an object that allows accessing typed method signatures."]
        pub fn signatures() -> Signatures {
            Signatures
        }
        #[doc = r" Retrieves a reference to type containing all the generated"]
        #[doc = r" contract methods. This can be used for methods where the name"]
        #[doc = r" would collide with a common method (like `at` or `deployed`)."]
        pub fn methods(&self) -> &Methods {
            &self.methods
        }
    }
    #[doc = r" Type containing signatures for all methods for generated contract type."]
    #[derive(Clone, Copy)]
    pub struct Signatures;
    impl Signatures {
        #[doc = "Returns signature for method `withdrawAll()`."]
        #[allow(clippy::type_complexity)]
        pub fn withdraw_all(&self) -> self::ethcontract::contract::Signature<(), ()> {
            self::ethcontract::contract::Signature::new([133, 56, 40, 182])
        }
        #[doc = "Returns signature for method `withdrawErc20(address)`."]
        #[allow(clippy::type_complexity)]
        pub fn withdraw_erc_20(
            &self,
        ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
            self::ethcontract::contract::Signature::new([199, 228, 43, 27])
        }
        #[doc = "Returns signature for method `renounceOwnership()`."]
        #[allow(clippy::type_complexity)]
        pub fn renounce_ownership(&self) -> self::ethcontract::contract::Signature<(), ()> {
            self::ethcontract::contract::Signature::new([113, 80, 24, 166])
        }
        #[doc = "Returns signature for method `payees(uint256):(address)`."]
        #[allow(clippy::type_complexity)]
        pub fn payees(
            &self,
        ) -> self::ethcontract::contract::Signature<
            (self::ethcontract::U256,),
            self::ethcontract::Address,
        > {
            self::ethcontract::contract::Signature::new([99, 3, 123, 12])
        }
        #[doc = "Returns signature for method `transferOwnership(address)`."]
        #[allow(clippy::type_complexity)]
        pub fn transfer_ownership(
            &self,
        ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
            self::ethcontract::contract::Signature::new([242, 253, 227, 139])
        }
        #[doc = "Returns signature for method `addPayee(address)`."]
        #[allow(clippy::type_complexity)]
        pub fn add_payee(
            &self,
        ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
            self::ethcontract::contract::Signature::new([220, 229, 156, 0])
        }
        #[doc = "Returns signature for method `balances(address):(uint256,uint256)`."]
        #[allow(clippy::type_complexity)]
        pub fn balances(
            &self,
        ) -> self::ethcontract::contract::Signature<
            (self::ethcontract::Address,),
            (self::ethcontract::U256, self::ethcontract::U256),
        > {
            self::ethcontract::contract::Signature::new([39, 226, 53, 227])
        }
        #[doc = "Returns signature for method `owner():(address)`."]
        #[allow(clippy::type_complexity)]
        pub fn owner(
            &self,
        ) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
            self::ethcontract::contract::Signature::new([141, 165, 203, 91])
        }
        #[doc = "Returns signature for method `withdraw(uint256)`."]
        #[allow(clippy::type_complexity)]
        pub fn withdraw(
            &self,
        ) -> self::ethcontract::contract::Signature<(self::ethcontract::U256,), ()> {
            self::ethcontract::contract::Signature::new([46, 26, 125, 77])
        }
        #[doc = "Returns signature for method `removePayee(address)`."]
        #[allow(clippy::type_complexity)]
        pub fn remove_payee(
            &self,
        ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
            self::ethcontract::contract::Signature::new([62, 211, 88, 85])
        }
        #[doc = "Returns signature for method `payAll()`."]
        #[allow(clippy::type_complexity)]
        pub fn pay_all(&self) -> self::ethcontract::contract::Signature<(), ()> {
            self::ethcontract::contract::Signature::new([88, 6, 190, 175])
        }
    }
    #[doc = r" Type containing all contract methods for generated contract type."]
    #[derive(Clone)]
    pub struct Methods {
        instance: self::ethcontract::dyns::DynInstance,
    }
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    impl Methods {
        #[doc = "Generated by `ethcontract`"]
        pub fn withdraw_all(&self) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([133, 56, 40, 182], ())
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn withdraw_erc_20(
            &self,
            token: self::ethcontract::Address,
        ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([199, 228, 43, 27], (token,))
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn renounce_ownership(&self) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([113, 80, 24, 166], ())
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn payees(
            &self,
            p0: self::ethcontract::U256,
        ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
            self.instance
                .view_method([99, 3, 123, 12], (p0,))
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn transfer_ownership(
            &self,
            new_owner: self::ethcontract::Address,
        ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([242, 253, 227, 139], (new_owner,))
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn add_payee(
            &self,
            payee: self::ethcontract::Address,
        ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([220, 229, 156, 0], (payee,))
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn balances(
            &self,
            p0: self::ethcontract::Address,
        ) -> self::ethcontract::dyns::DynViewMethodBuilder<(
            self::ethcontract::U256,
            self::ethcontract::U256,
        )> {
            self.instance
                .view_method([39, 226, 53, 227], (p0,))
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn owner(
            &self,
        ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
            self.instance
                .view_method([141, 165, 203, 91], ())
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn withdraw(
            &self,
            amount: self::ethcontract::U256,
        ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([46, 26, 125, 77], (amount,))
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn remove_payee(
            &self,
            payee: self::ethcontract::Address,
        ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([62, 211, 88, 85], (payee,))
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn pay_all(&self) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([88, 6, 190, 175], ())
                .expect("generated call")
        }
    }
    impl std::ops::Deref for Contract {
        type Target = Methods;
        fn deref(&self) -> &Self::Target {
            &self.methods
        }
    }
    impl Contract {
        #[doc = r" Returns a method builder to setup a call to a smart"]
        #[doc = r" contract's fallback function."]
        pub fn fallback<D>(&self, data: D) -> self::ethcontract::dyns::DynMethodBuilder<()>
        where
            D: Into<Vec<u8>>,
        {
            self.raw_instance()
                .fallback(data)
                .expect("generated fallback method")
        }
    }
    #[doc = r" Module containing all generated data models for this contract's"]
    #[doc = r" events."]
    pub mod event_data {
        use super::ethcontract;
        #[derive(
            Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize,
        )]
        pub struct OwnershipTransferred {
            pub previous_owner: self::ethcontract::Address,
            pub new_owner: self::ethcontract::Address,
        }
        impl OwnershipTransferred {
            #[doc = r" Retrieves the signature for the event this data corresponds to."]
            #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
            #[doc = r" this event."]
            pub fn signature() -> self::ethcontract::H256 {
                self::ethcontract::H256([
                    139, 224, 7, 156, 83, 22, 89, 20, 19, 68, 205, 31, 208, 164, 242, 132, 25, 73,
                    127, 151, 34, 163, 218, 175, 227, 180, 24, 111, 107, 100, 87, 224,
                ])
            }
            #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
            #[doc = r" to. For this event the value should always be:"]
            #[doc = r""]
            #[doc = "`OwnershipTransferred(address,address)`"]
            pub fn abi_signature() -> &'static str {
                "OwnershipTransferred(address,address)"
            }
        }
        impl self::ethcontract::tokens::Tokenize for OwnershipTransferred {
            fn from_token(
                token: self::ethcontract::common::abi::Token,
            ) -> Result<Self, self::ethcontract::tokens::Error> {
                let (previous_owner, new_owner) =
                    self::ethcontract::tokens::Tokenize::from_token(token)?;
                Ok(OwnershipTransferred {
                    previous_owner,
                    new_owner,
                })
            }
            fn into_token(self) -> self::ethcontract::common::abi::Token {
                unimplemented!("events are only decoded, not encoded")
            }
        }
    }
    impl Contract {
        #[doc = r" Retrieves a handle to a type containing for creating event"]
        #[doc = r" streams for all the contract events."]
        pub fn events(&self) -> Events<'_> {
            Events {
                instance: self.raw_instance(),
            }
        }
    }
    pub struct Events<'a> {
        instance: &'a self::ethcontract::dyns::DynInstance,
    }
    impl Events<'_> {
        #[doc = r" Generated by `ethcontract`."]
        pub fn ownership_transferred(&self) -> self::event_builders::OwnershipTransferredBuilder {
            self::event_builders::OwnershipTransferredBuilder(
                self.instance
                    .event(self::ethcontract::H256([
                        139, 224, 7, 156, 83, 22, 89, 20, 19, 68, 205, 31, 208, 164, 242, 132, 25,
                        73, 127, 151, 34, 163, 218, 175, 227, 180, 24, 111, 107, 100, 87, 224,
                    ]))
                    .expect("generated event filter"),
            )
        }
    }
    #[doc = r" Module containing the generated event stream builders with type safe"]
    #[doc = r" filter methods for this contract's events."]
    pub mod event_builders {
        use super::ethcontract;
        use super::event_data;
        #[doc = "A builder for creating a filtered stream of `OwnershipTransferred` events."]
        pub struct OwnershipTransferredBuilder(
            #[doc = r" The inner event builder."]
            pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::OwnershipTransferred>,
        );
        impl OwnershipTransferredBuilder {
            #[doc = r" Sets the starting block from which to stream logs for."]
            #[doc = r""]
            #[doc = r" If left unset defaults to the latest block."]
            #[allow(clippy::wrong_self_convention)]
            pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
                self.0 = (self.0).from_block(block);
                self
            }
            #[doc = r" Sets the last block from which to stream logs for."]
            #[doc = r""]
            #[doc = r" If left unset defaults to the streaming until the end of days."]
            #[allow(clippy::wrong_self_convention)]
            pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
                self.0 = (self.0).to_block(block);
                self
            }
            #[doc = r" Limits the number of events that can be retrieved by this filter."]
            #[doc = r""]
            #[doc = r" Note that this parameter is non-standard."]
            pub fn limit(mut self, value: usize) -> Self {
                self.0 = (self.0).limit(value);
                self
            }
            #[doc = r" Sets the polling interval. This is used as the interval between"]
            #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
            pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
                self.0 = (self.0).poll_interval(value);
                self
            }
            #[doc = "Adds a filter for the previousOwner event parameter."]
            pub fn previous_owner(
                mut self,
                topic: self::ethcontract::Topic<self::ethcontract::Address>,
            ) -> Self {
                self.0 = (self.0).topic0(topic);
                self
            }
            #[doc = "Adds a filter for the newOwner event parameter."]
            pub fn new_owner(
                mut self,
                topic: self::ethcontract::Topic<self::ethcontract::Address>,
            ) -> Self {
                self.0 = (self.0).topic1(topic);
                self
            }
            #[doc = r" Returns a future that resolves with a collection of all existing"]
            #[doc = r" logs matching the builder parameters."]
            pub async fn query(
                self,
            ) -> std::result::Result<
                std::vec::Vec<self::ethcontract::Event<self::event_data::OwnershipTransferred>>,
                self::ethcontract::errors::EventError,
            > {
                (self.0).query().await
            }
            #[doc = r" Creates an event stream from the current event builder."]
            pub fn stream(
                self,
            ) -> impl self::ethcontract::futures::stream::Stream<
                Item = std::result::Result<
                    self::ethcontract::StreamEvent<self::event_data::OwnershipTransferred>,
                    self::ethcontract::errors::EventError,
                >,
            > {
                (self.0).stream()
            }
        }
    }
    impl Contract {
        #[doc = r" Returns a log stream with all events."]
        pub fn all_events(&self) -> self::ethcontract::dyns::DynAllEventsBuilder<Event> {
            self::ethcontract::dyns::DynAllEventsBuilder::new(
                self.raw_instance().web3(),
                self.address(),
                self.deployment_information(),
            )
        }
    }
    #[doc = r" A contract event."]
    #[derive(Clone, Debug, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub enum Event {
        OwnershipTransferred(self::event_data::OwnershipTransferred),
    }
    impl self::ethcontract::contract::ParseLog for Event {
        fn parse_log(
            log: self::ethcontract::RawLog,
        ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
            let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([139 , 224 , 7 , 156 , 83 , 22 , 89 , 20 , 19 , 68 , 205 , 31 , 208 , 164 , 242 , 132 , 25 , 73 , 127 , 151 , 34 , 163 , 218 , 175 , 227 , 180 , 24 , 111 , 107 , 100 , 87 , 224]) => Ok (Event :: OwnershipTransferred (log . clone () . decode (Contract :: raw_contract () . abi . event ("OwnershipTransferred") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
            if let Some(Ok(data)) = standard_event {
                return Ok(data);
            }
            Err(self::ethcontract::errors::ExecutionError::from(
                self::ethcontract::common::abi::Error::InvalidData,
            ))
        }
    }
}
pub use self::royalties_payment::Contract as RoyaltiesPayment;