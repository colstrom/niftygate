#[allow(dead_code)]
pub mod i_compound_timelock_upgradeable {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"ICompoundTimelockUpgradeable\",\"abi\":[{\"type\":\"function\",\"name\":\"admin\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"MINIMUM_DELAY\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pendingAdmin\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"MAXIMUM_DELAY\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"cancelTransaction\",\"inputs\":[{\"name\":\"target\",\"type\":\"address\"},{\"name\":\"value\",\"type\":\"uint256\"},{\"name\":\"signature\",\"type\":\"string\"},{\"name\":\"data\",\"type\":\"bytes\"},{\"name\":\"eta\",\"type\":\"uint256\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"acceptAdmin\",\"inputs\":[],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"GRACE_PERIOD\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"delay\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"executeTransaction\",\"inputs\":[{\"name\":\"target\",\"type\":\"address\"},{\"name\":\"value\",\"type\":\"uint256\"},{\"name\":\"signature\",\"type\":\"string\"},{\"name\":\"data\",\"type\":\"bytes\"},{\"name\":\"eta\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\"}],\"constant\":false,\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"queuedTransactions\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setDelay\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"queueTransaction\",\"inputs\":[{\"name\":\"target\",\"type\":\"address\"},{\"name\":\"value\",\"type\":\"uint256\"},{\"name\":\"signature\",\"type\":\"string\"},{\"name\":\"data\",\"type\":\"bytes\"},{\"name\":\"eta\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setPendingAdmin\",\"inputs\":[{\"name\":\"\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"receive\"}],\"bytecode\":\"\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(ICompoundTimelockUpgradeable))
        .field(&self.address())
        .finish()
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
    #[doc = "Returns signature for method `admin():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn admin(&self) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([248, 81, 164, 64])
    }
    #[doc = "Returns signature for method `MINIMUM_DELAY():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn minimum_delay(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([177, 180, 58, 229])
    }
    #[doc = "Returns signature for method `pendingAdmin():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn pending_admin(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([38, 120, 34, 71])
    }
    #[doc = "Returns signature for method `MAXIMUM_DELAY():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn maximum_delay(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([125, 100, 95, 171])
    }
    #[doc = "Returns signature for method `cancelTransaction(address,uint256,string,bytes,uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn cancel_transaction(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::Address,
        self::ethcontract::U256,
        String,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
        self::ethcontract::U256,
      ),
      (),
    > {
      self::ethcontract::contract::Signature::new([89, 31, 205, 254])
    }
    #[doc = "Returns signature for method `acceptAdmin()`."]
    #[allow(clippy::type_complexity)]
    pub fn accept_admin(&self) -> self::ethcontract::contract::Signature<(), ()> {
      self::ethcontract::contract::Signature::new([14, 24, 182, 129])
    }
    #[doc = "Returns signature for method `GRACE_PERIOD():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn grace_period(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([193, 162, 135, 226])
    }
    #[doc = "Returns signature for method `delay():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn delay(&self) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([106, 66, 184, 248])
    }
    #[doc = "Returns signature for method `executeTransaction(address,uint256,string,bytes,uint256):(bytes)`."]
    #[allow(clippy::type_complexity)]
    pub fn execute_transaction(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::Address,
        self::ethcontract::U256,
        String,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
        self::ethcontract::U256,
      ),
      self::ethcontract::tokens::Bytes<Vec<u8>>,
    > {
      self::ethcontract::contract::Signature::new([8, 37, 243, 143])
    }
    #[doc = "Returns signature for method `queuedTransactions(bytes32):(bool)`."]
    #[allow(clippy::type_complexity)]
    pub fn queued_transactions(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::tokens::Bytes<[u8; 32]>,), bool>
    {
      self::ethcontract::contract::Signature::new([242, 176, 101, 55])
    }
    #[doc = "Returns signature for method `setDelay(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn set_delay(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::U256,), ()> {
      self::ethcontract::contract::Signature::new([225, 119, 36, 110])
    }
    #[doc = "Returns signature for method `queueTransaction(address,uint256,string,bytes,uint256):(bytes32)`."]
    #[allow(clippy::type_complexity)]
    pub fn queue_transaction(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::Address,
        self::ethcontract::U256,
        String,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
        self::ethcontract::U256,
      ),
      self::ethcontract::tokens::Bytes<[u8; 32]>,
    > {
      self::ethcontract::contract::Signature::new([58, 102, 249, 1])
    }
    #[doc = "Returns signature for method `setPendingAdmin(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn set_pending_admin(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([77, 209, 139, 245])
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
    pub fn admin(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([248, 81, 164, 64], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn minimum_delay(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([177, 180, 58, 229], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn pending_admin(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([38, 120, 34, 71], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn maximum_delay(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([125, 100, 95, 171], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn cancel_transaction(
      &self,
      target: self::ethcontract::Address,
      value: self::ethcontract::U256,
      signature: String,
      data: self::ethcontract::tokens::Bytes<Vec<u8>>,
      eta: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([89, 31, 205, 254], (target, value, signature, data, eta))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn accept_admin(&self) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([14, 24, 182, 129], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn grace_period(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([193, 162, 135, 226], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn delay(&self) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([106, 66, 184, 248], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn execute_transaction(
      &self,
      target: self::ethcontract::Address,
      value: self::ethcontract::U256,
      signature: String,
      data: self::ethcontract::tokens::Bytes<Vec<u8>>,
      eta: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::tokens::Bytes<Vec<u8>>> {
      self
        .instance
        .method([8, 37, 243, 143], (target, value, signature, data, eta))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn queued_transactions(
      &self,
      p0: self::ethcontract::tokens::Bytes<[u8; 32]>,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([242, 176, 101, 55], (p0,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn set_delay(
      &self,
      p0: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([225, 119, 36, 110], (p0,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn queue_transaction(
      &self,
      target: self::ethcontract::Address,
      value: self::ethcontract::U256,
      signature: String,
      data: self::ethcontract::tokens::Bytes<Vec<u8>>,
      eta: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 32]>> {
      self
        .instance
        .method([58, 102, 249, 1], (target, value, signature, data, eta))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn set_pending_admin(
      &self,
      p0: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([77, 209, 139, 245], (p0,))
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
      self
        .raw_instance()
        .fallback(data)
        .expect("generated fallback method")
    }
  }
}
pub use self::i_compound_timelock_upgradeable::Contract as ICompoundTimelockUpgradeable;
