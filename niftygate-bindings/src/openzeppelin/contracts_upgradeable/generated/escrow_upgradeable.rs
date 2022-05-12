#[allow(dead_code)]
pub mod escrow_upgradeable {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"EscrowUpgradeable\",\"abi\":[{\"type\":\"function\",\"name\":\"withdraw\",\"inputs\":[{\"name\":\"payee\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"deposit\",\"inputs\":[{\"name\":\"payee\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"depositsOf\",\"inputs\":[{\"name\":\"payee\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Withdrawn\",\"inputs\":[{\"name\":\"payee\",\"type\":\"address\",\"indexed\":true},{\"name\":\"weiAmount\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Deposited\",\"inputs\":[{\"name\":\"payee\",\"type\":\"address\",\"indexed\":true},{\"name\":\"weiAmount\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false}],\"bytecode\":\"608060405234801561001057600080fd5b50610797806100206000396000f3fe6080604052600436106100705760003560e01c80638da5cb5b1161004e5780638da5cb5b146100c1578063e3a9db1a146100ee578063f2fde38b14610132578063f340fa011461015257610070565b806351cff8d914610075578063715018a6146100975780638129fc1c146100ac575b600080fd5b34801561008157600080fd5b50610095610090366004610682565b610165565b005b3480156100a357600080fd5b50610095610207565b3480156100b857600080fd5b5061009561023d565b3480156100cd57600080fd5b506033546040516001600160a01b0390911681526020015b60405180910390f35b3480156100fa57600080fd5b50610124610109366004610682565b6001600160a01b031660009081526065602052604090205490565b6040519081526020016100e5565b34801561013e57600080fd5b5061009561014d366004610682565b6102b1565b610095610160366004610682565b610349565b6033546001600160a01b031633146101985760405162461bcd60e51b815260040161018f906106f3565b60405180910390fd5b6001600160a01b03811660008181526065602052604081208054919055906101c090826103dd565b816001600160a01b03167f7084f5476618d8e60b11ef0d7d3f06914655adb8793e28ff7f018d4c76d505d5826040516101fb91815260200190565b60405180910390a25050565b6033546001600160a01b031633146102315760405162461bcd60e51b815260040161018f906106f3565b61023b60006104fb565b565b600054610100900460ff1680610256575060005460ff16155b6102725760405162461bcd60e51b815260040161018f906106a5565b600054610100900460ff16158015610294576000805461ffff19166101011790555b61029c61054d565b80156102ae576000805461ff00191690555b50565b6033546001600160a01b031633146102db5760405162461bcd60e51b815260040161018f906106f3565b6001600160a01b0381166103405760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161018f565b6102ae816104fb565b6033546001600160a01b031633146103735760405162461bcd60e51b815260040161018f906106f3565b6001600160a01b03811660009081526065602052604081208054349283929161039d908490610728565b90915550506040518181526001600160a01b038316907f2da466a7b24304f47e87fa2e1e5a81b9831ce54fec19055ce277ca2f39ba42c4906020016101fb565b8047101561042d5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e6365000000604482015260640161018f565b6000826001600160a01b03168260405160006040518083038185875af1925050503d806000811461047a576040519150601f19603f3d011682016040523d82523d6000602084013e61047f565b606091505b50509050806104f65760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d61792068617665207265766572746564000000000000606482015260840161018f565b505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600054610100900460ff1680610566575060005460ff16155b6105825760405162461bcd60e51b815260040161018f906106a5565b600054610100900460ff161580156105a4576000805461ffff19166101011790555b6105ac6105b8565b6105b4610622565b61029c5b600054610100900460ff16806105d1575060005460ff16155b6105ed5760405162461bcd60e51b815260040161018f906106a5565b600054610100900460ff1615801561029c576000805461ffff191661010117905580156102ae576000805461ff001916905550565b600054610100900460ff168061063b575060005460ff16155b6106575760405162461bcd60e51b815260040161018f906106a5565b600054610100900460ff16158015610679576000805461ffff19166101011790555b61029c336104fb565b600060208284031215610693578081fd5b813561069e8161074c565b9392505050565b6020808252602e908201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160408201526d191e481a5b9a5d1a585b1a5e995960921b606082015260800190565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b6000821982111561074757634e487b7160e01b81526011600452602481fd5b500190565b6001600160a01b03811681146102ae57600080fdfea26469706673582212209d575624978f1587a60baaaf3185655693db396e8e26251e40f337fa52271e8664736f6c63430008030033\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(EscrowUpgradeable))
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
    #[doc = "Returns signature for method `withdraw(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn withdraw(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([81, 207, 248, 217])
    }
    #[doc = "Returns signature for method `deposit(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn deposit(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([243, 64, 250, 1])
    }
    #[doc = "Returns signature for method `transferOwnership(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn transfer_ownership(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([242, 253, 227, 139])
    }
    #[doc = "Returns signature for method `depositsOf(address):(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn deposits_of(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::Address,),
      self::ethcontract::U256,
    > {
      self::ethcontract::contract::Signature::new([227, 169, 219, 26])
    }
    #[doc = "Returns signature for method `initialize()`."]
    #[allow(clippy::type_complexity)]
    pub fn initialize(&self) -> self::ethcontract::contract::Signature<(), ()> {
      self::ethcontract::contract::Signature::new([129, 41, 252, 28])
    }
    #[doc = "Returns signature for method `renounceOwnership()`."]
    #[allow(clippy::type_complexity)]
    pub fn renounce_ownership(&self) -> self::ethcontract::contract::Signature<(), ()> {
      self::ethcontract::contract::Signature::new([113, 80, 24, 166])
    }
    #[doc = "Returns signature for method `owner():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn owner(&self) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([141, 165, 203, 91])
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
    pub fn withdraw(
      &self,
      payee: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([81, 207, 248, 217], (payee,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn deposit(
      &self,
      payee: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([243, 64, 250, 1], (payee,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn transfer_ownership(
      &self,
      new_owner: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([242, 253, 227, 139], (new_owner,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn deposits_of(
      &self,
      payee: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([227, 169, 219, 26], (payee,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn initialize(&self) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([129, 41, 252, 28], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn renounce_ownership(&self) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([113, 80, 24, 166], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn owner(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([141, 165, 203, 91], ())
        .expect("generated call")
    }
  }
  impl std::ops::Deref for Contract {
    type Target = Methods;
    fn deref(&self) -> &Self::Target {
      &self.methods
    }
  }
  #[doc = r" Module containing all generated data models for this contract's"]
  #[doc = r" events."]
  pub mod event_data {
    use super::ethcontract;
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
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
          139, 224, 7, 156, 83, 22, 89, 20, 19, 68, 205, 31, 208, 164, 242, 132, 25, 73, 127, 151,
          34, 163, 218, 175, 227, 180, 24, 111, 107, 100, 87, 224,
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
        let (previous_owner, new_owner) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(OwnershipTransferred {
          previous_owner,
          new_owner,
        })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct Withdrawn {
      pub payee: self::ethcontract::Address,
      pub wei_amount: self::ethcontract::U256,
    }
    impl Withdrawn {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          112, 132, 245, 71, 102, 24, 216, 230, 11, 17, 239, 13, 125, 63, 6, 145, 70, 85, 173, 184,
          121, 62, 40, 255, 127, 1, 141, 76, 118, 213, 5, 213,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`Withdrawn(address,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "Withdrawn(address,uint256)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for Withdrawn {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (payee, wei_amount) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(Withdrawn { payee, wei_amount })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct Deposited {
      pub payee: self::ethcontract::Address,
      pub wei_amount: self::ethcontract::U256,
    }
    impl Deposited {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          45, 164, 102, 167, 178, 67, 4, 244, 126, 135, 250, 46, 30, 90, 129, 185, 131, 28, 229,
          79, 236, 25, 5, 92, 226, 119, 202, 47, 57, 186, 66, 196,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`Deposited(address,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "Deposited(address,uint256)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for Deposited {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (payee, wei_amount) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(Deposited { payee, wei_amount })
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
        self
          .instance
          .event(self::ethcontract::H256([
            139, 224, 7, 156, 83, 22, 89, 20, 19, 68, 205, 31, 208, 164, 242, 132, 25, 73, 127,
            151, 34, 163, 218, 175, 227, 180, 24, 111, 107, 100, 87, 224,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn withdrawn(&self) -> self::event_builders::WithdrawnBuilder {
      self::event_builders::WithdrawnBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            112, 132, 245, 71, 102, 24, 216, 230, 11, 17, 239, 13, 125, 63, 6, 145, 70, 85, 173,
            184, 121, 62, 40, 255, 127, 1, 141, 76, 118, 213, 5, 213,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn deposited(&self) -> self::event_builders::DepositedBuilder {
      self::event_builders::DepositedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            45, 164, 102, 167, 178, 67, 4, 244, 126, 135, 250, 46, 30, 90, 129, 185, 131, 28, 229,
            79, 236, 25, 5, 92, 226, 119, 202, 47, 57, 186, 66, 196,
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
    #[doc = "A builder for creating a filtered stream of `Withdrawn` events."]
    pub struct WithdrawnBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::Withdrawn>,
    );
    impl WithdrawnBuilder {
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
      #[doc = "Adds a filter for the payee event parameter."]
      pub fn payee(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::Withdrawn>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::Withdrawn>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `Deposited` events."]
    pub struct DepositedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::Deposited>,
    );
    impl DepositedBuilder {
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
      #[doc = "Adds a filter for the payee event parameter."]
      pub fn payee(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::Deposited>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::Deposited>,
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
    Deposited(self::event_data::Deposited),
    OwnershipTransferred(self::event_data::OwnershipTransferred),
    Withdrawn(self::event_data::Withdrawn),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([45 , 164 , 102 , 167 , 178 , 67 , 4 , 244 , 126 , 135 , 250 , 46 , 30 , 90 , 129 , 185 , 131 , 28 , 229 , 79 , 236 , 25 , 5 , 92 , 226 , 119 , 202 , 47 , 57 , 186 , 66 , 196]) => Ok (Event :: Deposited (log . clone () . decode (Contract :: raw_contract () . abi . event ("Deposited") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([139 , 224 , 7 , 156 , 83 , 22 , 89 , 20 , 19 , 68 , 205 , 31 , 208 , 164 , 242 , 132 , 25 , 73 , 127 , 151 , 34 , 163 , 218 , 175 , 227 , 180 , 24 , 111 , 107 , 100 , 87 , 224]) => Ok (Event :: OwnershipTransferred (log . clone () . decode (Contract :: raw_contract () . abi . event ("OwnershipTransferred") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([112 , 132 , 245 , 71 , 102 , 24 , 216 , 230 , 11 , 17 , 239 , 13 , 125 , 63 , 6 , 145 , 70 , 85 , 173 , 184 , 121 , 62 , 40 , 255 , 127 , 1 , 141 , 76 , 118 , 213 , 5 , 213]) => Ok (Event :: Withdrawn (log . clone () . decode (Contract :: raw_contract () . abi . event ("Withdrawn") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::escrow_upgradeable::Contract as EscrowUpgradeable;