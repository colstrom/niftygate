#[allow(dead_code)]
pub mod i_access_control_upgradeable {
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
          # [allow (unused_mut)] let mut artifact = Artifact :: from_json ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"IAccessControlUpgradeable\",\n  \"sourceName\": \"contracts/access/AccessControlUpgradeable.sol\",\n  \"abi\": [\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"getRoleAdmin\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"grantRole\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"hasRole\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"renounceRole\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"revokeRole\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    }\n  ],\n  \"bytecode\": \"0x\",\n  \"deployedBytecode\": \"0x\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("valid artifact JSON") ;
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
      f.debug_tuple(stringify!(IAccessControlUpgradeable))
        .field(&self.address())
        .finish()
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
    pub fn revoke_role(
      &self,
      role: self::ethcontract::tokens::Bytes<[u8; 32]>,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([213, 71, 116, 31], (role, account))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn grant_role(
      &self,
      role: self::ethcontract::tokens::Bytes<[u8; 32]>,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([47, 47, 241, 93], (role, account))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn renounce_role(
      &self,
      role: self::ethcontract::tokens::Bytes<[u8; 32]>,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([54, 86, 138, 190], (role, account))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn has_role(
      &self,
      role: self::ethcontract::tokens::Bytes<[u8; 32]>,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([145, 209, 72, 84], (role, account))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_role_admin(
      &self,
      role: self::ethcontract::tokens::Bytes<[u8; 32]>,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 32]>>
    {
      self
        .instance
        .view_method([36, 138, 156, 163], (role,))
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
pub use self::i_access_control_upgradeable::Contract as IAccessControlUpgradeable;