#[allow(dead_code)]
pub mod minimal_forwarder_upgradeable {
  # [rustfmt :: skip] use ethcontract as ethcontract ;
  #[doc = "Generated by `ethcontract`"]
  #[derive(Clone)]
  pub struct Contract {
    methods: Methods,
  }
  impl Contract {
    #[doc = r" Retrieves the truffle artifact used to generate the type safe"]
    #[doc = r" API for this contract."]
    pub fn artifact() -> &'static self::ethcontract::Artifact {
      use self::ethcontract::private::lazy_static;
      use self::ethcontract::Artifact;
      lazy_static! {
        pub static ref ARTIFACT: Artifact = {
          # [allow (unused_mut)] let mut artifact = Artifact :: from_json ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"MinimalForwarderUpgradeable\",\n  \"sourceName\": \"contracts/metatx/MinimalForwarderUpgradeable.sol\",\n  \"abi\": [\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"from\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"to\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"value\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"gas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"nonce\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"data\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct MinimalForwarderUpgradeable.ForwardRequest\",\n          \"name\": \"req\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"signature\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"execute\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"getNonce\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"from\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"to\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"value\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"gas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"nonce\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"data\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct MinimalForwarderUpgradeable.ForwardRequest\",\n          \"name\": \"req\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"signature\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"verify\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    }\n  ],\n  \"bytecode\": \"0x608060405234801561001057600080fd5b5061098f806100206000396000f3fe6080604052600436106100345760003560e01c80632d0335ab1461003957806347153f8214610082578063bf5d3bdb146100a3575b600080fd5b34801561004557600080fd5b5061006f610054366004610742565b6001600160a01b031660009081526035602052604090205490565b6040519081526020015b60405180910390f35b610095610090366004610770565b6100d3565b60405161007992919061085d565b3480156100af57600080fd5b506100c36100be366004610770565b610281565b6040519015158152602001610079565b600060606100e2858585610281565b61014e5760405162461bcd60e51b815260206004820152603260248201527f4d696e696d616c466f727761726465723a207369676e617475726520646f6573604482015271081b9bdd081b585d18da081c995c5d595cdd60721b60648201526084015b60405180910390fd5b61015d608086013560016108e5565b6035600061016e6020890189610742565b6001600160a01b03166001600160a01b03168152602001908152602001600020819055506000808660200160208101906101a89190610742565b6001600160a01b0316606088013560408901356101c860a08b018b610899565b6101d560208d018d610742565b6040516020016101e793929190610818565b60408051601f198184030181529082905261020191610841565b600060405180830381858888f193505050503d806000811461023f576040519150601f19603f3d011682016040523d82523d6000602084013e610244565b606091505b509092509050610259603f6060890135610909565b5a1161027557634e487b7160e01b600052600160045260246000fd5b90969095509350505050565b60008061039484848080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061038e92507fdd8f4b70b0f4393e889bd39128a30628a78b61816a9eb8199759e7a349657e4891506102f1905060208a018a610742565b61030160408b0160208c01610742565b60408b013560608c013560808d013561031d60a08f018f610899565b60405161032b929190610808565b6040805191829003822060208301989098526001600160a01b0396871690820152949093166060850152608084019190915260a083015260c082015260e08101919091526101000160405160208183030381529060405280519060200120610400565b90610454565b90506080850135603560006103ac6020890189610742565b6001600160a01b03166001600160a01b03168152602001908152602001600020541480156103f757506103e26020860186610742565b6001600160a01b0316816001600160a01b0316145b95945050505050565b600061044e61040d6104f8565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b92915050565b60008151604114156104885760208201516040830151606084015160001a61047e86828585610578565b935050505061044e565b8151604014156104b057602082015160408301516104a7858383610718565b9250505061044e565b60405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610145565b60006105737f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61052760015490565b6002546040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b905090565b60007f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08211156105f55760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610145565b8360ff16601b148061060a57508360ff16601c145b6106615760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c604482015261756560f01b6064820152608401610145565b6040805160008082526020820180845288905260ff871692820192909252606081018590526080810184905260019060a0016020604051602081039080840390855afa1580156106b5573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166103f75760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610145565b60006001600160ff1b03821660ff83901c601b0161073886828785610578565b9695505050505050565b600060208284031215610753578081fd5b81356001600160a01b0381168114610769578182fd5b9392505050565b600080600060408486031215610784578182fd5b833567ffffffffffffffff8082111561079b578384fd5b9085019060c082880312156107ae578384fd5b909350602085013590808211156107c3578384fd5b818601915086601f8301126107d6578384fd5b8135818111156107e4578485fd5b8760208285010111156107f5578485fd5b6020830194508093505050509250925092565b6000828483379101908152919050565b6000838583375060609190911b6bffffffffffffffffffffffff19169101908152601401919050565b60008251610853818460208701610929565b9190910192915050565b60008315158252604060208301528251806040840152610884816060850160208701610929565b601f01601f1916919091016060019392505050565b6000808335601e198436030181126108af578283fd5b83018035915067ffffffffffffffff8211156108c9578283fd5b6020019150368190038213156108de57600080fd5b9250929050565b6000821982111561090457634e487b7160e01b81526011600452602481fd5b500190565b60008261092457634e487b7160e01b81526012600452602481fd5b500490565b60005b8381101561094457818101518382015260200161092c565b83811115610953576000848401525b5050505056fea26469706673582212200ae59cd9edb1957af701b1c1a7a045ca5532cda1467f984ab02ba4ba53e44faf64736f6c63430008030033\",\n  \"deployedBytecode\": \"0x6080604052600436106100345760003560e01c80632d0335ab1461003957806347153f8214610082578063bf5d3bdb146100a3575b600080fd5b34801561004557600080fd5b5061006f610054366004610742565b6001600160a01b031660009081526035602052604090205490565b6040519081526020015b60405180910390f35b610095610090366004610770565b6100d3565b60405161007992919061085d565b3480156100af57600080fd5b506100c36100be366004610770565b610281565b6040519015158152602001610079565b600060606100e2858585610281565b61014e5760405162461bcd60e51b815260206004820152603260248201527f4d696e696d616c466f727761726465723a207369676e617475726520646f6573604482015271081b9bdd081b585d18da081c995c5d595cdd60721b60648201526084015b60405180910390fd5b61015d608086013560016108e5565b6035600061016e6020890189610742565b6001600160a01b03166001600160a01b03168152602001908152602001600020819055506000808660200160208101906101a89190610742565b6001600160a01b0316606088013560408901356101c860a08b018b610899565b6101d560208d018d610742565b6040516020016101e793929190610818565b60408051601f198184030181529082905261020191610841565b600060405180830381858888f193505050503d806000811461023f576040519150601f19603f3d011682016040523d82523d6000602084013e610244565b606091505b509092509050610259603f6060890135610909565b5a1161027557634e487b7160e01b600052600160045260246000fd5b90969095509350505050565b60008061039484848080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061038e92507fdd8f4b70b0f4393e889bd39128a30628a78b61816a9eb8199759e7a349657e4891506102f1905060208a018a610742565b61030160408b0160208c01610742565b60408b013560608c013560808d013561031d60a08f018f610899565b60405161032b929190610808565b6040805191829003822060208301989098526001600160a01b0396871690820152949093166060850152608084019190915260a083015260c082015260e08101919091526101000160405160208183030381529060405280519060200120610400565b90610454565b90506080850135603560006103ac6020890189610742565b6001600160a01b03166001600160a01b03168152602001908152602001600020541480156103f757506103e26020860186610742565b6001600160a01b0316816001600160a01b0316145b95945050505050565b600061044e61040d6104f8565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b92915050565b60008151604114156104885760208201516040830151606084015160001a61047e86828585610578565b935050505061044e565b8151604014156104b057602082015160408301516104a7858383610718565b9250505061044e565b60405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610145565b60006105737f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61052760015490565b6002546040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b905090565b60007f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08211156105f55760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610145565b8360ff16601b148061060a57508360ff16601c145b6106615760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c604482015261756560f01b6064820152608401610145565b6040805160008082526020820180845288905260ff871692820192909252606081018590526080810184905260019060a0016020604051602081039080840390855afa1580156106b5573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166103f75760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610145565b60006001600160ff1b03821660ff83901c601b0161073886828785610578565b9695505050505050565b600060208284031215610753578081fd5b81356001600160a01b0381168114610769578182fd5b9392505050565b600080600060408486031215610784578182fd5b833567ffffffffffffffff8082111561079b578384fd5b9085019060c082880312156107ae578384fd5b909350602085013590808211156107c3578384fd5b818601915086601f8301126107d6578384fd5b8135818111156107e4578485fd5b8760208285010111156107f5578485fd5b6020830194508093505050509250925092565b6000828483379101908152919050565b6000838583375060609190911b6bffffffffffffffffffffffff19169101908152601401919050565b60008251610853818460208701610929565b9190910192915050565b60008315158252604060208301528251806040840152610884816060850160208701610929565b601f01601f1916919091016060019392505050565b6000808335601e198436030181126108af578283fd5b83018035915067ffffffffffffffff8211156108c9578283fd5b6020019150368190038213156108de57600080fd5b9250929050565b6000821982111561090457634e487b7160e01b81526011600452602481fd5b500190565b60008261092457634e487b7160e01b81526012600452602481fd5b500490565b60005b8381101561094457818101518382015260200161092c565b83811115610953576000848401525b5050505056fea26469706673582212200ae59cd9edb1957af701b1c1a7a045ca5532cda1467f984ab02ba4ba53e44faf64736f6c63430008030033\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("valid artifact JSON") ;
          artifact
        };
      }
      &ARTIFACT
    }
    #[doc = r" Creates a new contract instance with the specified `web3`"]
    #[doc = r" provider at the given `Address`."]
    #[doc = r""]
    #[doc = r" Note that this does not verify that a contract with a matching"]
    #[doc = r" `Abi` is actually deployed at the given address."]
    pub fn at<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
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
    pub fn with_deployment_info<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
      deployment_information: Option<ethcontract::common::DeploymentInformation>,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
    {
      use self::ethcontract::transport::DynTransport;
      use self::ethcontract::web3::api::Web3;
      use self::ethcontract::Instance;
      let transport = DynTransport::new(web3.transport().clone());
      let web3 = Web3::new(transport);
      let abi = Self::artifact().abi.clone();
      let instance = Instance::with_deployment_info(web3, abi, address, deployment_information);
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
      f.debug_tuple(stringify!(MinimalForwarderUpgradeable))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = "Generated by `ethcontract`"]
    #[allow(clippy::too_many_arguments)]
    pub fn builder<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
    ) -> self::ethcontract::dyns::DynDeployBuilder<Self>
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
    {
      use self::ethcontract::contract::DeployBuilder;
      use self::ethcontract::dyns::DynTransport;
      use self::ethcontract::web3::api::Web3;
      let transport = DynTransport::new(web3.transport().clone());
      let web3 = Web3::new(transport);
      let bytecode = Self::artifact().bytecode.clone();
      DeployBuilder::new(web3, bytecode, ()).expect("valid deployment args")
    }
  }
  impl self::ethcontract::contract::Deploy<self::ethcontract::dyns::DynTransport> for Contract {
    type Context = self::ethcontract::common::Bytecode;
    fn bytecode(cx: &Self::Context) -> &self::ethcontract::common::Bytecode {
      cx
    }
    fn abi(_: &Self::Context) -> &self::ethcontract::common::Abi {
      &Self::artifact().abi
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
    #[doc = r" Retrieves a reference to type containing all the generated"]
    #[doc = r" contract methods. This can be used for methods where the name"]
    #[doc = r" would collide with a common method (like `at` or `deployed`)."]
    pub fn methods(&self) -> &Methods {
      &self.methods
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
    pub fn execute(
      &self,
      req: (
        self::ethcontract::Address,
        self::ethcontract::Address,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
      ),
      signature: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<(bool, self::ethcontract::tokens::Bytes<Vec<u8>>)>
    {
      self
        .instance
        .method([71, 21, 63, 130], (req, signature))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_nonce(
      &self,
      from: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([45, 3, 53, 171], (from,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn verify(
      &self,
      req: (
        self::ethcontract::Address,
        self::ethcontract::Address,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
      ),
      signature: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([191, 93, 59, 219], (req, signature))
        .expect("generated call")
    }
  }
  impl std::ops::Deref for Contract {
    type Target = Methods;
    fn deref(&self) -> &Self::Target {
      &self.methods
    }
  }
}
pub use self::minimal_forwarder_upgradeable::Contract as MinimalForwarderUpgradeable;