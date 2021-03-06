#[allow(dead_code)]
pub mod enumerable_set_mock {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"EnumerableSetMock\",\"abi\":[{\"type\":\"function\",\"name\":\"add\",\"inputs\":[{\"name\":\"value\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"contains\",\"inputs\":[{\"name\":\"value\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"enumerate\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"length\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"remove\",\"inputs\":[{\"name\":\"value\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"get\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"TransactionResult\",\"inputs\":[{\"name\":\"result\",\"type\":\"bool\",\"indexed\":false}],\"anonymous\":false}],\"bytecode\":\"608060405234801561001057600080fd5b506107bc806100206000396000f3fe608060405234801561001057600080fd5b50600436106100625760003560e01c80630a3b0a4f146100675780631f7b6d32146100ab57806329092d0e146100c95780635dbe47e81461010d5780639507d39a14610169578063ff9f78b3146101d7575b600080fd5b6100a96004803603602081101561007d57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050610236565b005b6100b361028d565b6040518082815260200191505060405180910390f35b61010b600480360360208110156100df57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919050505061029e565b005b61014f6004803603602081101561012357600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff1690602001909291905050506102f5565b604051808215151515815260200191505060405180910390f35b6101956004803603602081101561017f57600080fd5b8101908080359060200190929190505050610312565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b6101df61032f565b6040518080602001828103825283818151815260200191508051906020019060200280838360005b83811015610222578082015181840152602081019050610207565b505050509050019250505060405180910390f35b600061024c82600061034090919063ffffffff16565b90507f5f2d66b608a7b4bfa43b7e9ba385022741152c97499b8a8abcd5c27258417b5a81604051808215151515815260200191505060405180910390a15050565b60006102996000610410565b905090565b60006102b482600061042190919063ffffffff16565b90507f5f2d66b608a7b4bfa43b7e9ba385022741152c97499b8a8abcd5c27258417b5a81604051808215151515815260200191505060405180910390a15050565b600061030b82600061060f90919063ffffffff16565b9050919050565b600061032882600061065e90919063ffffffff16565b9050919050565b606061033b60006106a2565b905090565b600061034c838361060f565b61040557826001018290806001815401808255809150509060018203906000526020600020016000909192909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508360000160008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055506001905061040a565b600090505b92915050565b600081600101805490509050919050565b600061042d838361060f565b1561060457600060018460000160008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020540390506000600185600101805490500390508181146105725760008560010182815481106104a257fe5b9060005260206000200160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050808660010184815481106104df57fe5b9060005260206000200160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550600183018660000160008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550505b8460000160008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009055846001018054806105c457fe5b6001900381819060005260206000200160006101000a81549073ffffffffffffffffffffffffffffffffffffffff02191690559055600192505050610609565b600090505b92915050565b6000808360000160008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020541415905092915050565b600082600101828154811061066f57fe5b9060005260206000200160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905092915050565b60608082600101805490506040519080825280602002602001820160405280156106db5781602001602082028038833980820191505090505b50905060005b836001018054905081101561077d578360010181815481106106ff57fe5b9060005260206000200160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1682828151811061073657fe5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505080806001019150506106e1565b508091505091905056fea265627a7a72315820dfd0f3c01c3e856cfd35eb926aabdc7e3385520eb246e5ef37a81d672d7e7de964736f6c63430005110032\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(EnumerableSetMock))
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
    #[doc = "Returns signature for method `add(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn add(&self) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([10, 59, 10, 79])
    }
    #[doc = "Returns signature for method `contains(address):(bool)`."]
    #[allow(clippy::type_complexity)]
    pub fn contains(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), bool> {
      self::ethcontract::contract::Signature::new([93, 190, 71, 232])
    }
    #[doc = "Returns signature for method `enumerate():(address[])`."]
    #[allow(clippy::type_complexity)]
    pub fn enumerate(
      &self,
    ) -> self::ethcontract::contract::Signature<(), Vec<self::ethcontract::Address>> {
      self::ethcontract::contract::Signature::new([255, 159, 120, 179])
    }
    #[doc = "Returns signature for method `length():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn length(&self) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([31, 123, 109, 50])
    }
    #[doc = "Returns signature for method `remove(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn remove(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([41, 9, 45, 14])
    }
    #[doc = "Returns signature for method `get(uint256):(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn get(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::U256,),
      self::ethcontract::Address,
    > {
      self::ethcontract::contract::Signature::new([149, 7, 211, 154])
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
    pub fn add(
      &self,
      value: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([10, 59, 10, 79], (value,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn contains(
      &self,
      value: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([93, 190, 71, 232], (value,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn enumerate(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<Vec<self::ethcontract::Address>> {
      self
        .instance
        .view_method([255, 159, 120, 179], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn length(&self) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([31, 123, 109, 50], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn remove(
      &self,
      value: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([41, 9, 45, 14], (value,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get(
      &self,
      index: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([149, 7, 211, 154], (index,))
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
    pub struct TransactionResult {
      pub result: bool,
    }
    impl TransactionResult {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          95, 45, 102, 182, 8, 167, 180, 191, 164, 59, 126, 155, 163, 133, 2, 39, 65, 21, 44, 151,
          73, 155, 138, 138, 188, 213, 194, 114, 88, 65, 123, 90,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`TransactionResult(bool)`"]
      pub fn abi_signature() -> &'static str {
        "TransactionResult(bool)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for TransactionResult {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (result,) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(TransactionResult { result })
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
    pub fn transaction_result(&self) -> self::event_builders::TransactionResultBuilder {
      self::event_builders::TransactionResultBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            95, 45, 102, 182, 8, 167, 180, 191, 164, 59, 126, 155, 163, 133, 2, 39, 65, 21, 44,
            151, 73, 155, 138, 138, 188, 213, 194, 114, 88, 65, 123, 90,
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
    #[doc = "A builder for creating a filtered stream of `TransactionResult` events."]
    pub struct TransactionResultBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::TransactionResult>,
    );
    impl TransactionResultBuilder {
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
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::TransactionResult>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::TransactionResult>,
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
    TransactionResult(self::event_data::TransactionResult),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([95 , 45 , 102 , 182 , 8 , 167 , 180 , 191 , 164 , 59 , 126 , 155 , 163 , 133 , 2 , 39 , 65 , 21 , 44 , 151 , 73 , 155 , 138 , 138 , 188 , 213 , 194 , 114 , 88 , 65 , 123 , 90]) => Ok (Event :: TransactionResult (log . clone () . decode (Contract :: raw_contract () . abi . event ("TransactionResult") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::enumerable_set_mock::Contract as EnumerableSetMock;
