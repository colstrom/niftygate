#[allow(dead_code)]
pub mod gsn_recipient_signature_mock {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"GSNRecipientSignatureMock\",\"abi\":[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"trustedSigner\",\"type\":\"address\"}]},{\"type\":\"function\",\"name\":\"preRelayedCall\",\"inputs\":[{\"name\":\"context\",\"type\":\"bytes\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"mockFunction\",\"inputs\":[],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"postRelayedCall\",\"inputs\":[{\"name\":\"context\",\"type\":\"bytes\"},{\"name\":\"success\",\"type\":\"bool\"},{\"name\":\"actualCharge\",\"type\":\"uint256\"},{\"name\":\"preRetVal\",\"type\":\"bytes32\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"relayHubVersion\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"string\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"acceptRelayedCall\",\"inputs\":[{\"name\":\"relay\",\"type\":\"address\"},{\"name\":\"from\",\"type\":\"address\"},{\"name\":\"encodedFunction\",\"type\":\"bytes\"},{\"name\":\"transactionFee\",\"type\":\"uint256\"},{\"name\":\"gasPrice\",\"type\":\"uint256\"},{\"name\":\"gasLimit\",\"type\":\"uint256\"},{\"name\":\"nonce\",\"type\":\"uint256\"},{\"name\":\"approvalData\",\"type\":\"bytes\"},{\"name\":\"\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"},{\"name\":\"\",\"type\":\"bytes\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getHubAddr\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"MockFunctionCalled\",\"inputs\":[],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RelayHubChanged\",\"inputs\":[{\"name\":\"oldRelayHub\",\"type\":\"address\",\"indexed\":true},{\"name\":\"newRelayHub\",\"type\":\"address\",\"indexed\":true}],\"anonymous\":false}],\"bytecode\":\"608060405273d216153c06e857cd7f72665e0af1d7d82172f4946000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555034801561006457600080fd5b50604051610c68380380610c688339818101604052602081101561008757600080fd5b810190808051906020019092919050505080600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16141561011f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526039815260200180610c2f6039913960400191505060405180910390fd5b80600160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050610abe806101716000396000f3fe608060405234801561001057600080fd5b50600436106100625760003560e01c80633e6fec041461006757806374e861d61461007157806380274db7146100bb57806383947ea014610148578063ad61ccd514610309578063e06e0e221461038c575b600080fd5b61006f610425565b005b610079610453565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b610132600480360360208110156100d157600080fd5b81019080803590602001906401000000008111156100ee57600080fd5b82018360208201111561010057600080fd5b8035906020019184600183028401116401000000008311171561012257600080fd5b909192939192939050505061047c565b6040518082815260200191505060405180910390f35b610287600480360361012081101561015f57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803590602001906401000000008111156101bc57600080fd5b8201836020820111156101ce57600080fd5b803590602001918460018302840111640100000000831117156101f057600080fd5b9091929391929390803590602001909291908035906020019092919080359060200190929190803590602001909291908035906020019064010000000081111561023957600080fd5b82018360208201111561024b57600080fd5b8035906020019184600183028401116401000000008311171561026d57600080fd5b90919293919293908035906020019092919050505061055e565b6040518083815260200180602001828103825283818151815260200191508051906020019080838360005b838110156102cd5780820151818401526020810190506102b2565b50505050905090810190601f1680156102fa5780820380516001836020036101000a031916815260200191505b50935050505060405180910390f35b610311610788565b6040518080602001828103825283818151815260200191508051906020019080838360005b83811015610351578082015181840152602081019050610336565b50505050905090810190601f16801561037e5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b610423600480360360808110156103a257600080fd5b81019080803590602001906401000000008111156103bf57600080fd5b8201836020820111156103d157600080fd5b803590602001918460018302840111640100000000831117156103f357600080fd5b909192939192939080351515906020019092919080359060200190929190803590602001909291905050506107c5565b005b7f52c66ed6ec9ca819cba26fe2b2650059270d8981b295af300187a964f54a8c2360405160405180910390a1565b60008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b6000610486610453565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610509576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526024815260200180610a666024913960400191505060405180910390fd5b61055683838080601f016020809104026020016040519081016040528093929190818152602001838380828437600081840152601f19601f820116905080830192505050505050506108a7565b905092915050565b60006060808d8d8d8d8d8d8d8d610573610453565b30604051602001808b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1660601b81526014018a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1660601b8152601401898980828437808301925050508781526020018681526020018581526020018481526020018373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1660601b81526014018273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1660601b81526014019a50505050505050505050506040516020818303038152906040529050600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1661073087878080601f016020809104026020016040519081016040528093929190818152602001838380828437600081840152601f19601f8201169050808301925050505050505061072284805190602001206108ae565b61090690919063ffffffff16565b73ffffffffffffffffffffffffffffffffffffffff16141561075e57610754610a0a565b9250925050610778565b61077260008081111561076d57fe5b610a2e565b92509250505b9b509b9950505050505050505050565b60606040518060400160405280600581526020017f312e302e30000000000000000000000000000000000000000000000000000000815250905090565b6107cd610453565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610850576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526024815260200180610a666024913960400191505060405180910390fd5b6108a085858080601f016020809104026020016040519081016040528093929190818152602001838380828437600081840152601f19601f82011690508083019250505050505050848484610a4f565b5050505050565b6000919050565b60008160405160200180807f19457468657265756d205369676e6564204d6573736167653a0a333200000000815250601c01828152602001915050604051602081830303815290604052805190602001209050919050565b6000604182511461091a5760009050610a04565b60008060006020850151925060408501519150606085015160001a90507f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08260001c111561096e5760009350505050610a04565b601b8160ff16141580156109865750601c8160ff1614155b156109975760009350505050610a04565b60018682858560405160008152602001604052604051808581526020018460ff1660ff1681526020018381526020018281526020019450505050506020604051602081039080840390855afa1580156109f4573d6000803e3d6000fd5b5050506020604051035193505050505b92915050565b60006060610a2660405180602001604052806000815250610a55565b915091509091565b6000606082600b016040518060200160405280600081525091509150915091565b50505050565b600060606000839150915091509156fe47534e526563697069656e743a2063616c6c6572206973206e6f742052656c6179487562a265627a7a72315820e3262a80e775000a0e0fb7b1aa8f28e139f1c926b8005fb35dd748496e1c57a364736f6c6343000511003247534e526563697069656e745369676e61747572653a2074727573746564207369676e657220697320746865207a65726f2061646472657373\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(GSNRecipientSignatureMock))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = "Generated by `ethcontract`"]
    #[allow(clippy::too_many_arguments)]
    pub fn builder<F, B, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      trusted_signer: self::ethcontract::Address,
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
      DeployBuilder::new(web3, bytecode, (trusted_signer,)).expect("valid deployment args")
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
    #[doc = "Returns signature for method `preRelayedCall(bytes):(bytes32)`."]
    #[allow(clippy::type_complexity)]
    pub fn pre_relayed_call(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::tokens::Bytes<Vec<u8>>,),
      self::ethcontract::tokens::Bytes<[u8; 32]>,
    > {
      self::ethcontract::contract::Signature::new([128, 39, 77, 183])
    }
    #[doc = "Returns signature for method `mockFunction()`."]
    #[allow(clippy::type_complexity)]
    pub fn mock_function(&self) -> self::ethcontract::contract::Signature<(), ()> {
      self::ethcontract::contract::Signature::new([62, 111, 236, 4])
    }
    #[doc = "Returns signature for method `postRelayedCall(bytes,bool,uint256,bytes32)`."]
    #[allow(clippy::type_complexity)]
    pub fn post_relayed_call(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::tokens::Bytes<Vec<u8>>,
        bool,
        self::ethcontract::U256,
        self::ethcontract::tokens::Bytes<[u8; 32]>,
      ),
      (),
    > {
      self::ethcontract::contract::Signature::new([224, 110, 14, 34])
    }
    #[doc = "Returns signature for method `relayHubVersion():(string)`."]
    #[allow(clippy::type_complexity)]
    pub fn relay_hub_version(&self) -> self::ethcontract::contract::Signature<(), String> {
      self::ethcontract::contract::Signature::new([173, 97, 204, 213])
    }
    #[doc = "Returns signature for method `acceptRelayedCall(address,address,bytes,uint256,uint256,uint256,uint256,bytes,uint256):(uint256,bytes)`."]
    #[allow(clippy::type_complexity)]
    pub fn accept_relayed_call(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::Address,
        self::ethcontract::Address,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
        self::ethcontract::U256,
      ),
      (
        self::ethcontract::U256,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
      ),
    > {
      self::ethcontract::contract::Signature::new([131, 148, 126, 160])
    }
    #[doc = "Returns signature for method `getHubAddr():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn get_hub_addr(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([116, 232, 97, 214])
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
    pub fn pre_relayed_call(
      &self,
      context: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 32]>> {
      self
        .instance
        .method([128, 39, 77, 183], (context,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn mock_function(&self) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([62, 111, 236, 4], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn post_relayed_call(
      &self,
      context: self::ethcontract::tokens::Bytes<Vec<u8>>,
      success: bool,
      actual_charge: self::ethcontract::U256,
      pre_ret_val: self::ethcontract::tokens::Bytes<[u8; 32]>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method(
          [224, 110, 14, 34],
          (context, success, actual_charge, pre_ret_val),
        )
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn relay_hub_version(&self) -> self::ethcontract::dyns::DynViewMethodBuilder<String> {
      self
        .instance
        .view_method([173, 97, 204, 213], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn accept_relayed_call(
      &self,
      relay: self::ethcontract::Address,
      from: self::ethcontract::Address,
      encoded_function: self::ethcontract::tokens::Bytes<Vec<u8>>,
      transaction_fee: self::ethcontract::U256,
      gas_price: self::ethcontract::U256,
      gas_limit: self::ethcontract::U256,
      nonce: self::ethcontract::U256,
      approval_data: self::ethcontract::tokens::Bytes<Vec<u8>>,
      p8: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<(
      self::ethcontract::U256,
      self::ethcontract::tokens::Bytes<Vec<u8>>,
    )> {
      self
        .instance
        .view_method(
          [131, 148, 126, 160],
          (
            relay,
            from,
            encoded_function,
            transaction_fee,
            gas_price,
            gas_limit,
            nonce,
            approval_data,
            p8,
          ),
        )
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_hub_addr(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([116, 232, 97, 214], ())
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
    pub struct MockFunctionCalled();
    impl MockFunctionCalled {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          82, 198, 110, 214, 236, 156, 168, 25, 203, 162, 111, 226, 178, 101, 0, 89, 39, 13, 137,
          129, 178, 149, 175, 48, 1, 135, 169, 100, 245, 74, 140, 35,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`MockFunctionCalled()`"]
      pub fn abi_signature() -> &'static str {
        "MockFunctionCalled()"
      }
    }
    impl self::ethcontract::tokens::Tokenize for MockFunctionCalled {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let () = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(MockFunctionCalled())
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct RelayHubChanged {
      pub old_relay_hub: self::ethcontract::Address,
      pub new_relay_hub: self::ethcontract::Address,
    }
    impl RelayHubChanged {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          185, 248, 75, 142, 101, 22, 75, 20, 67, 154, 227, 98, 13, 240, 164, 216, 120, 109, 137,
          105, 150, 192, 40, 43, 104, 63, 157, 140, 8, 240, 70, 232,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`RelayHubChanged(address,address)`"]
      pub fn abi_signature() -> &'static str {
        "RelayHubChanged(address,address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for RelayHubChanged {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (old_relay_hub, new_relay_hub) =
          self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(RelayHubChanged {
          old_relay_hub,
          new_relay_hub,
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
    pub fn mock_function_called(&self) -> self::event_builders::MockFunctionCalledBuilder {
      self::event_builders::MockFunctionCalledBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            82, 198, 110, 214, 236, 156, 168, 25, 203, 162, 111, 226, 178, 101, 0, 89, 39, 13, 137,
            129, 178, 149, 175, 48, 1, 135, 169, 100, 245, 74, 140, 35,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn relay_hub_changed(&self) -> self::event_builders::RelayHubChangedBuilder {
      self::event_builders::RelayHubChangedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            185, 248, 75, 142, 101, 22, 75, 20, 67, 154, 227, 98, 13, 240, 164, 216, 120, 109, 137,
            105, 150, 192, 40, 43, 104, 63, 157, 140, 8, 240, 70, 232,
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
    #[doc = "A builder for creating a filtered stream of `MockFunctionCalled` events."]
    pub struct MockFunctionCalledBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::MockFunctionCalled>,
    );
    impl MockFunctionCalledBuilder {
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
        std::vec::Vec<self::ethcontract::Event<self::event_data::MockFunctionCalled>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::MockFunctionCalled>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `RelayHubChanged` events."]
    pub struct RelayHubChangedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::RelayHubChanged>,
    );
    impl RelayHubChangedBuilder {
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
      #[doc = "Adds a filter for the oldRelayHub event parameter."]
      pub fn old_relay_hub(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the newRelayHub event parameter."]
      pub fn new_relay_hub(
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
        std::vec::Vec<self::ethcontract::Event<self::event_data::RelayHubChanged>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::RelayHubChanged>,
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
    MockFunctionCalled(self::event_data::MockFunctionCalled),
    RelayHubChanged(self::event_data::RelayHubChanged),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([82 , 198 , 110 , 214 , 236 , 156 , 168 , 25 , 203 , 162 , 111 , 226 , 178 , 101 , 0 , 89 , 39 , 13 , 137 , 129 , 178 , 149 , 175 , 48 , 1 , 135 , 169 , 100 , 245 , 74 , 140 , 35]) => Ok (Event :: MockFunctionCalled (log . clone () . decode (Contract :: raw_contract () . abi . event ("MockFunctionCalled") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([185 , 248 , 75 , 142 , 101 , 22 , 75 , 20 , 67 , 154 , 227 , 98 , 13 , 240 , 164 , 216 , 120 , 109 , 137 , 105 , 150 , 192 , 40 , 43 , 104 , 63 , 157 , 140 , 8 , 240 , 70 , 232]) => Ok (Event :: RelayHubChanged (log . clone () . decode (Contract :: raw_contract () . abi . event ("RelayHubChanged") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::gsn_recipient_signature_mock::Contract as GSNRecipientSignatureMock;
