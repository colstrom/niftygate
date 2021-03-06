#[allow(dead_code)]
pub mod erc20_migrator {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"ERC20Migrator\",\"abi\":[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"legacyToken\",\"type\":\"address\"}]},{\"type\":\"function\",\"name\":\"beginMigration\",\"inputs\":[{\"name\":\"newToken_\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"newToken\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"migrateAll\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"legacyToken\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"migrate\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"},{\"name\":\"amount\",\"type\":\"uint256\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"}],\"bytecode\":\"608060405234801561001057600080fd5b50604051610e80380380610e808339818101604052602081101561003357600080fd5b8101908080519060200190929190505050600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614156100ca576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602f815260200180610e51602f913960400191505060405180910390fd5b806000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050610d378061011a6000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c8063104d26141461005c57806313155455146100a05780635a8cadb1146100ea578063ad68ebf71461012e578063c42bd05a1461017c575b600080fd5b61009e6004803603602081101561007257600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff1690602001909291905050506101c6565b005b6100a8610444565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b61012c6004803603602081101561010057600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919050505061046d565b005b61017a6004803603604081101561014457600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919080359060200190929190505050610679565b005b610184610858565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b600073ffffffffffffffffffffffffffffffffffffffff16600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161461026d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526028815260200180610c386028913960400191505060405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614156102f3576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602c815260200180610cad602c913960400191505060405180910390fd5b8073ffffffffffffffffffffffffffffffffffffffff1663aa271e1a306040518263ffffffff1660e01b8152600401808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060206040518083038186803b15801561037057600080fd5b505afa158015610384573d6000803e3d6000fd5b505050506040513d602081101561039a57600080fd5b8101908080519060200190929190505050610400576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526029815260200180610c606029913960400191505060405180910390fd5b80600160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b60008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166370a08231836040518263ffffffff1660e01b8152600401808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060206040518083038186803b15801561050d57600080fd5b505afa158015610521573d6000803e3d6000fd5b505050506040513d602081101561053757600080fd5b8101908080519060200190929190505050905060008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663dd62ed3e84306040518363ffffffff1660e01b8152600401808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019250505060206040518083038186803b15801561061e57600080fd5b505afa158015610632573d6000803e3d6000fd5b505050506040513d602081101561064857600080fd5b8101908080519060200190929190505050905060006106678383610882565b90506106738482610679565b50505050565b600073ffffffffffffffffffffffffffffffffffffffff16600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161415610721576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526024815260200180610c896024913960400191505060405180910390fd5b61076f8230836000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1661089b909392919063ffffffff16565b600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166340c10f1983836040518363ffffffff1660e01b8152600401808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200182815260200192505050602060405180830381600087803b15801561081857600080fd5b505af115801561082c573d6000803e3d6000fd5b505050506040513d602081101561084257600080fd5b8101908080519060200190929190505050505050565b6000600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b60008183106108915781610893565b825b905092915050565b61099b848573ffffffffffffffffffffffffffffffffffffffff166323b872dd905060e01b858585604051602401808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019350505050604051602081830303815290604052907bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506109a1565b50505050565b6109c08273ffffffffffffffffffffffffffffffffffffffff16610bec565b610a32576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601f8152602001807f5361666545524332303a2063616c6c20746f206e6f6e2d636f6e74726163740081525060200191505060405180910390fd5b600060608373ffffffffffffffffffffffffffffffffffffffff16836040518082805190602001908083835b60208310610a815780518252602082019150602081019050602083039250610a5e565b6001836020036101000a0380198251168184511680821785525050505050509050019150506000604051808303816000865af19150503d8060008114610ae3576040519150601f19603f3d011682016040523d82523d6000602084013e610ae8565b606091505b509150915081610b60576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260208152602001807f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c656481525060200191505060405180910390fd5b600081511115610be657808060200190516020811015610b7f57600080fd5b8101908080519060200190929190505050610be5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602a815260200180610cd9602a913960400191505060405180910390fd5b5b50505050565b60008060007fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47060001b9050833f9150808214158015610c2e57506000801b8214155b9250505091905056fe45524332304d69677261746f723a206d6967726174696f6e20616c7265616479207374617274656445524332304d69677261746f723a206e6f742061206d696e74657220666f72206e657720746f6b656e45524332304d69677261746f723a206d6967726174696f6e206e6f74207374617274656445524332304d69677261746f723a206e657720746f6b656e20697320746865207a65726f20616464726573735361666545524332303a204552433230206f7065726174696f6e20646964206e6f742073756363656564a265627a7a723158205a064192404be6131cfc0b78bc909dfcd8e9efb27ac3c1942abf79d69361574564736f6c6343000511003245524332304d69677261746f723a206c656761637920746f6b656e20697320746865207a65726f2061646472657373\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(ERC20Migrator))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = "Generated by `ethcontract`"]
    #[allow(clippy::too_many_arguments)]
    pub fn builder<F, B, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      legacy_token: self::ethcontract::Address,
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
      DeployBuilder::new(web3, bytecode, (legacy_token,)).expect("valid deployment args")
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
    #[doc = "Returns signature for method `beginMigration(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn begin_migration(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([16, 77, 38, 20])
    }
    #[doc = "Returns signature for method `newToken():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn new_token(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([196, 43, 208, 90])
    }
    #[doc = "Returns signature for method `migrateAll(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn migrate_all(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([90, 140, 173, 177])
    }
    #[doc = "Returns signature for method `legacyToken():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn legacy_token(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([19, 21, 84, 85])
    }
    #[doc = "Returns signature for method `migrate(address,uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn migrate(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::Address, self::ethcontract::U256),
      (),
    > {
      self::ethcontract::contract::Signature::new([173, 104, 235, 247])
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
    pub fn begin_migration(
      &self,
      new_token: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([16, 77, 38, 20], (new_token,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn new_token(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([196, 43, 208, 90], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn migrate_all(
      &self,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([90, 140, 173, 177], (account,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn legacy_token(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([19, 21, 84, 85], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn migrate(
      &self,
      account: self::ethcontract::Address,
      amount: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([173, 104, 235, 247], (account, amount))
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
pub use self::erc20_migrator::Contract as ERC20Migrator;
