mod generated;

pub mod crowdsale {
  pub use crate::openzeppelin::contracts_legacy::generated::crowdsale::*;

  pub mod distribution {
    pub use crate::openzeppelin::contracts_legacy::generated::finalizable_crowdsale::*;
    pub use crate::openzeppelin::contracts_legacy::generated::post_delivery_crowdsale::*;
    pub use crate::openzeppelin::contracts_legacy::generated::refundable_crowdsale::*;
    pub use crate::openzeppelin::contracts_legacy::generated::refundable_post_delivery_crowdsale::*;
  }

  pub mod emission {
    pub use crate::openzeppelin::contracts_legacy::generated::allowance_crowdsale::*;
    pub use crate::openzeppelin::contracts_legacy::generated::minted_crowdsale::*;
  }

  pub mod price {
    pub use crate::openzeppelin::contracts_legacy::generated::increasing_price_crowdsale::*;
  }

  pub mod validation {
    pub use crate::openzeppelin::contracts_legacy::generated::capped_crowdsale::*;
    pub use crate::openzeppelin::contracts_legacy::generated::individually_capped_crowdsale::*;
    pub use crate::openzeppelin::contracts_legacy::generated::pausable_crowdsale::*;
    pub use crate::openzeppelin::contracts_legacy::generated::timed_crowdsale::*;
    pub use crate::openzeppelin::contracts_legacy::generated::whitelist_crowdsale::*;
  }
}

pub mod example {
  pub use crate::openzeppelin::contracts_legacy::generated::sample_crowdsale::*;
  pub use crate::openzeppelin::contracts_legacy::generated::sample_crowdsale_token::*;
  pub use crate::openzeppelin::contracts_legacy::generated::simple_token::*;
}
