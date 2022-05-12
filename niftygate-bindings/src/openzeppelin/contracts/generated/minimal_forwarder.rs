#[allow(dead_code)]
pub mod minimal_forwarder {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"MinimalForwarder\",\"abi\":[{\"type\":\"constructor\",\"inputs\":[]},{\"type\":\"function\",\"name\":\"execute\",\"inputs\":[{\"name\":\"req\",\"type\":\"tuple\",\"components\":[{\"type\":\"address\"},{\"type\":\"address\"},{\"type\":\"uint256\"},{\"type\":\"uint256\"},{\"type\":\"uint256\"},{\"type\":\"bytes\"}]},{\"name\":\"signature\",\"type\":\"bytes\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"},{\"name\":\"\",\"type\":\"bytes\"}],\"constant\":false,\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"verify\",\"inputs\":[{\"name\":\"req\",\"type\":\"tuple\",\"components\":[{\"type\":\"address\"},{\"type\":\"address\"},{\"type\":\"uint256\"},{\"type\":\"uint256\"},{\"type\":\"uint256\"},{\"type\":\"bytes\"}]},{\"name\":\"signature\",\"type\":\"bytes\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getNonce\",\"inputs\":[{\"name\":\"from\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"}],\"bytecode\":\"61012060405234801561001157600080fd5b50604080518082018252601081526f26b4b734b6b0b62337b93bb0b93232b960811b6020808301918252835180850185526005815264302e302e3160d81b908201529151902060c08181527fae209a0b48f21c054280f2455d32cf309387644879d9acbd8ffc19916381188560e08190524660a081815286517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818801819052818901969096526060810193909352608080840192909252308382015286518084039091018152919092019094528351939092019290922090526101005260805160a05160c05160e05161010051610b4461013360003960006104ce0152600061051d015260006104f80152600061047a015260006104a30152610b446000f3fe6080604052600436106100345760003560e01c80632d0335ab1461003957806347153f8214610082578063bf5d3bdb146100a3575b600080fd5b34801561004557600080fd5b5061006f6100543660046108fe565b6001600160a01b031660009081526020819052604090205490565b6040519081526020015b60405180910390f35b61009561009036600461092c565b6100d3565b604051610079929190610a19565b3480156100af57600080fd5b506100c36100be36600461092c565b610280565b6040519015158152602001610079565b600060606100e2858585610280565b61014e5760405162461bcd60e51b815260206004820152603260248201527f4d696e696d616c466f727761726465723a207369676e617475726520646f6573604482015271081b9bdd081b585d18da081c995c5d595cdd60721b60648201526084015b60405180910390fd5b61015d60808601356001610a9a565b60008061016d60208901896108fe565b6001600160a01b03166001600160a01b03168152602001908152602001600020819055506000808660200160208101906101a791906108fe565b6001600160a01b0316606088013560408901356101c760a08b018b610a55565b6101d460208d018d6108fe565b6040516020016101e6939291906109d4565b60408051601f1981840301815290829052610200916109fd565b600060405180830381858888f193505050503d806000811461023e576040519150601f19603f3d011682016040523d82523d6000602084013e610243565b606091505b509092509050610258603f6060890135610abe565b5a1161027457634e487b7160e01b600052600160045260246000fd5b90969095509350505050565b60008061039384848080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061038d92507fdd8f4b70b0f4393e889bd39128a30628a78b61816a9eb8199759e7a349657e4891506102f0905060208a018a6108fe565b61030060408b0160208c016108fe565b60408b013560608c013560808d013561031c60a08f018f610a55565b60405161032a9291906109c4565b6040805191829003822060208301989098526001600160a01b0396871690820152949093166060850152608084019190915260a083015260c082015260e081019190915261010001604051602081830303815290604052805190602001206103fe565b90610452565b905060808501356000806103aa60208901896108fe565b6001600160a01b03166001600160a01b03168152602001908152602001600020541480156103f557506103e060208601866108fe565b6001600160a01b0316816001600160a01b0316145b95945050505050565b600061044c61040b610476565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b92915050565b6000806000610461858561056c565b9150915061046e816105dc565b509392505050565b60007f00000000000000000000000000000000000000000000000000000000000000004614156104c757507f0000000000000000000000000000000000000000000000000000000000000000610569565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c090920190925280519101205b90565b6000808251604114156105a35760208301516040840151606085015160001a610597878285856107e2565b945094505050506105d5565b8251604014156105cd57602083015160408401516105c28683836108cf565b9350935050506105d5565b506000905060025b9250929050565b60008160048111156105fe57634e487b7160e01b600052602160045260246000fd5b1415610609576107df565b600181600481111561062b57634e487b7160e01b600052602160045260246000fd5b14156106795760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610145565b600281600481111561069b57634e487b7160e01b600052602160045260246000fd5b14156106e95760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610145565b600381600481111561070b57634e487b7160e01b600052602160045260246000fd5b14156107645760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610145565b600481600481111561078657634e487b7160e01b600052602160045260246000fd5b14156107df5760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c604482015261756560f01b6064820152608401610145565b50565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561081957506000905060036108c6565b8460ff16601b1415801561083157508460ff16601c14155b1561084257506000905060046108c6565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015610896573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166108bf576000600192509250506108c6565b9150600090505b94509492505050565b6000806001600160ff1b03831660ff84901c601b016108f0878288856107e2565b935093505050935093915050565b60006020828403121561090f578081fd5b81356001600160a01b0381168114610925578182fd5b9392505050565b600080600060408486031215610940578182fd5b833567ffffffffffffffff80821115610957578384fd5b9085019060c0828803121561096a578384fd5b9093506020850135908082111561097f578384fd5b818601915086601f830112610992578384fd5b8135818111156109a0578485fd5b8760208285010111156109b1578485fd5b6020830194508093505050509250925092565b6000828483379101908152919050565b6000838583375060609190911b6bffffffffffffffffffffffff19169101908152601401919050565b60008251610a0f818460208701610ade565b9190910192915050565b60008315158252604060208301528251806040840152610a40816060850160208701610ade565b601f01601f1916919091016060019392505050565b6000808335601e19843603018112610a6b578283fd5b83018035915067ffffffffffffffff821115610a85578283fd5b6020019150368190038213156105d557600080fd5b60008219821115610ab957634e487b7160e01b81526011600452602481fd5b500190565b600082610ad957634e487b7160e01b81526012600452602481fd5b500490565b60005b83811015610af9578181015183820152602001610ae1565b83811115610b08576000848401525b5050505056fea264697066735822122053d18eea9cfddf3496703497ba0711c4caff2d2220280bab4776f1f67731324c64736f6c63430008030033\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(MinimalForwarder))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = "Generated by `ethcontract`"]
    #[allow(clippy::too_many_arguments)]
    pub fn builder<F, B, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
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
      DeployBuilder::new(web3, bytecode, ()).expect("valid deployment args")
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
    #[doc = "Returns signature for method `execute((address,address,uint256,uint256,uint256,bytes),bytes):(bool,bytes)`."]
    #[allow(clippy::type_complexity)]
    pub fn execute(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        (
          self::ethcontract::Address,
          self::ethcontract::Address,
          self::ethcontract::U256,
          self::ethcontract::U256,
          self::ethcontract::U256,
          self::ethcontract::tokens::Bytes<Vec<u8>>,
        ),
        self::ethcontract::tokens::Bytes<Vec<u8>>,
      ),
      (bool, self::ethcontract::tokens::Bytes<Vec<u8>>),
    > {
      self::ethcontract::contract::Signature::new([71, 21, 63, 130])
    }
    #[doc = "Returns signature for method `verify((address,address,uint256,uint256,uint256,bytes),bytes):(bool)`."]
    #[allow(clippy::type_complexity)]
    pub fn verify(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        (
          self::ethcontract::Address,
          self::ethcontract::Address,
          self::ethcontract::U256,
          self::ethcontract::U256,
          self::ethcontract::U256,
          self::ethcontract::tokens::Bytes<Vec<u8>>,
        ),
        self::ethcontract::tokens::Bytes<Vec<u8>>,
      ),
      bool,
    > {
      self::ethcontract::contract::Signature::new([191, 93, 59, 219])
    }
    #[doc = "Returns signature for method `getNonce(address):(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn get_nonce(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::Address,),
      self::ethcontract::U256,
    > {
      self::ethcontract::contract::Signature::new([45, 3, 53, 171])
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
  }
  impl std::ops::Deref for Contract {
    type Target = Methods;
    fn deref(&self) -> &Self::Target {
      &self.methods
    }
  }
}
pub use self::minimal_forwarder::Contract as MinimalForwarder;