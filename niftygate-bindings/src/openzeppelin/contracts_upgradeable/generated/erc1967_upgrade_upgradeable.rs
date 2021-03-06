#[allow(dead_code)]
pub mod erc1967_upgrade_upgradeable {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"ERC1967UpgradeUpgradeable\",\"abi\":[{\"type\":\"event\",\"name\":\"Upgraded\",\"inputs\":[{\"name\":\"implementation\",\"type\":\"address\",\"indexed\":true}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"AdminChanged\",\"inputs\":[{\"name\":\"previousAdmin\",\"type\":\"address\",\"indexed\":false},{\"name\":\"newAdmin\",\"type\":\"address\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"BeaconUpgraded\",\"inputs\":[{\"name\":\"beacon\",\"type\":\"address\",\"indexed\":true}],\"anonymous\":false}],\"bytecode\":\"\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(ERC1967UpgradeUpgradeable))
        .field(&self.address())
        .finish()
    }
  }
  #[derive(Clone)]
  struct Methods {
    instance: self::ethcontract::dyns::DynInstance,
  }
  #[derive(Clone, Copy)]
  struct Signatures;
  #[doc = r" Module containing all generated data models for this contract's"]
  #[doc = r" events."]
  pub mod event_data {
    use super::ethcontract;
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct Upgraded {
      pub implementation: self::ethcontract::Address,
    }
    impl Upgraded {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          188, 124, 215, 90, 32, 238, 39, 253, 154, 222, 186, 179, 32, 65, 247, 85, 33, 77, 188,
          107, 255, 169, 12, 192, 34, 91, 57, 218, 46, 92, 45, 59,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`Upgraded(address)`"]
      pub fn abi_signature() -> &'static str {
        "Upgraded(address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for Upgraded {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (implementation,) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(Upgraded { implementation })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct AdminChanged {
      pub previous_admin: self::ethcontract::Address,
      pub new_admin: self::ethcontract::Address,
    }
    impl AdminChanged {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          126, 100, 77, 121, 66, 47, 23, 192, 30, 72, 148, 181, 244, 245, 136, 211, 49, 235, 250,
          40, 101, 61, 66, 174, 131, 45, 197, 158, 56, 201, 121, 143,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`AdminChanged(address,address)`"]
      pub fn abi_signature() -> &'static str {
        "AdminChanged(address,address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for AdminChanged {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (previous_admin, new_admin) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(AdminChanged {
          previous_admin,
          new_admin,
        })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct BeaconUpgraded {
      pub beacon: self::ethcontract::Address,
    }
    impl BeaconUpgraded {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          28, 243, 176, 58, 108, 241, 159, 162, 186, 186, 77, 241, 72, 233, 220, 171, 237, 234,
          127, 138, 92, 7, 132, 14, 32, 126, 92, 8, 155, 233, 93, 62,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`BeaconUpgraded(address)`"]
      pub fn abi_signature() -> &'static str {
        "BeaconUpgraded(address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for BeaconUpgraded {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (beacon,) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(BeaconUpgraded { beacon })
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
    pub fn upgraded(&self) -> self::event_builders::UpgradedBuilder {
      self::event_builders::UpgradedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            188, 124, 215, 90, 32, 238, 39, 253, 154, 222, 186, 179, 32, 65, 247, 85, 33, 77, 188,
            107, 255, 169, 12, 192, 34, 91, 57, 218, 46, 92, 45, 59,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn admin_changed(&self) -> self::event_builders::AdminChangedBuilder {
      self::event_builders::AdminChangedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            126, 100, 77, 121, 66, 47, 23, 192, 30, 72, 148, 181, 244, 245, 136, 211, 49, 235, 250,
            40, 101, 61, 66, 174, 131, 45, 197, 158, 56, 201, 121, 143,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn beacon_upgraded(&self) -> self::event_builders::BeaconUpgradedBuilder {
      self::event_builders::BeaconUpgradedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            28, 243, 176, 58, 108, 241, 159, 162, 186, 186, 77, 241, 72, 233, 220, 171, 237, 234,
            127, 138, 92, 7, 132, 14, 32, 126, 92, 8, 155, 233, 93, 62,
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
    #[doc = "A builder for creating a filtered stream of `Upgraded` events."]
    pub struct UpgradedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::Upgraded>,
    );
    impl UpgradedBuilder {
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
      #[doc = "Adds a filter for the implementation event parameter."]
      pub fn implementation(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::Upgraded>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::Upgraded>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `AdminChanged` events."]
    pub struct AdminChangedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::AdminChanged>,
    );
    impl AdminChangedBuilder {
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
        std::vec::Vec<self::ethcontract::Event<self::event_data::AdminChanged>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::AdminChanged>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `BeaconUpgraded` events."]
    pub struct BeaconUpgradedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::BeaconUpgraded>,
    );
    impl BeaconUpgradedBuilder {
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
      #[doc = "Adds a filter for the beacon event parameter."]
      pub fn beacon(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::BeaconUpgraded>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::BeaconUpgraded>,
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
    AdminChanged(self::event_data::AdminChanged),
    BeaconUpgraded(self::event_data::BeaconUpgraded),
    Upgraded(self::event_data::Upgraded),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([126 , 100 , 77 , 121 , 66 , 47 , 23 , 192 , 30 , 72 , 148 , 181 , 244 , 245 , 136 , 211 , 49 , 235 , 250 , 40 , 101 , 61 , 66 , 174 , 131 , 45 , 197 , 158 , 56 , 201 , 121 , 143]) => Ok (Event :: AdminChanged (log . clone () . decode (Contract :: raw_contract () . abi . event ("AdminChanged") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([28 , 243 , 176 , 58 , 108 , 241 , 159 , 162 , 186 , 186 , 77 , 241 , 72 , 233 , 220 , 171 , 237 , 234 , 127 , 138 , 92 , 7 , 132 , 14 , 32 , 126 , 92 , 8 , 155 , 233 , 93 , 62]) => Ok (Event :: BeaconUpgraded (log . clone () . decode (Contract :: raw_contract () . abi . event ("BeaconUpgraded") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([188 , 124 , 215 , 90 , 32 , 238 , 39 , 253 , 154 , 222 , 186 , 179 , 32 , 65 , 247 , 85 , 33 , 77 , 188 , 107 , 255 , 169 , 12 , 192 , 34 , 91 , 57 , 218 , 46 , 92 , 45 , 59]) => Ok (Event :: Upgraded (log . clone () . decode (Contract :: raw_contract () . abi . event ("Upgraded") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::erc1967_upgrade_upgradeable::Contract as ERC1967UpgradeUpgradeable;
