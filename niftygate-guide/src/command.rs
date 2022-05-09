use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use structopt::StructOpt;

fn we_can_do_it() {
  println!("\nGood news! There's a preset for that:")
}

fn roll_your_own() {
  println!("\nOh no! You'll have to roll your own contract for that.")
}

fn suggest_custom_mintable_with_operators() {
  roll_your_own();
  println!(
    "
  You can still leverage OpenZeppelin though!

  If you extend the {erc777} contract, add the {access_control} library, and 
  implement minting, you should be most of the way to what you need!

  Read more:

    {erc777} Specification: {spec}
    {erc777} Token API: {erc777_api}
    {access_control} API: {access_control_api}
    ",
    access_control = style("AccessControl").bold().white().on_black(),
    access_control_api =
      style("https://docs.openzeppelin.com/contracts/3.x/api/access#AccessControl").underlined(),
    erc777 = style("ERC777").bold().blue(),
    erc777_api = style("https://docs.openzeppelin.com/contracts/3.x/api/token/erc777").underlined(),
    spec = style("https://eips.ethereum.org/EIPS/eip-777").underlined(),
  );
}

fn read_more(preset: &str, erc: &str, api: &str, spec: &str) {
  println!(
    "
  Read more:

    {erc} Tokens: {spec}
    {preset} API: {api}
    ",
    api = style(api).underlined(),
    erc = style(erc).bold().blue(),
    preset = style(preset).bold().white().on_black(),
    spec = style(spec).underlined(),
  )
}

fn deploy_with(command: &str) {
  println!(
    "
  Deploy with:

    {command}

  (check --help for options)
    ",
    command = style(command).bold().white().on_black(),
  )
}

fn openzeppelin_erc20_preset_fixed_supply() {
  we_can_do_it();
  println!(
    "
  {preset} (from OpenZeppelin)

  This is an {erc} token, including:
    - Preminted initial supply
    - Ability for {holders} to {burn} (destroy) their tokens
    - No access control mechanism (for {minting}/{pausing}) and hence no governance
    ",
    burn = style("burn").bright().red().on_black(),
    erc = style("ERC20").bold().blue(),
    holders = style("holders").bold().yellow().on_black(),
    minting = style("minting").bright().green().on_black(),
    pausing = style("pausing").bright().cyan().on_black(),
    preset = style("ERC20PresetFixedSupply").bold().white().on_black(),
  );

  read_more(
    "ERC20PresetFixedSupply",
    "ERC20",
    "https://docs.openzeppelin.com/contracts/3.x/api/presets#ERC20PresetFixedSupply",
    "https://eips.ethereum.org/EIPS/eip-20",
  );

  deploy_with("niftygate contract deploy ERC20PresetFixedSupply");
}

fn openzeppelin_preset_erc20_preset_minter_pauser() {
  we_can_do_it();
  println!(
    "
  {preset} (from OpenZeppelin)

  This is an {erc} token, including:
    - ability for {holders} to {burn} (destroy) their tokens
    - a {minter} role that allows for token {minting} (creation)
    - a {pauser} role that allows to {stop} all token transfers

  The account that deploys the contract will be granted the {minter} 
  and {pauser} roles, as well as the default {admin} role, which will 
  let it grant both {minter} and {pauser} roles to other accounts.
    ",
    admin = style("admin").bold().magenta().on_black(),
    burn = style("burn").bright().red().on_black(),
    erc = style("ERC20").bold().blue(),
    holders = style("holders").bold().yellow().on_black(),
    minter = style("minter").bold().green().on_black(),
    minting = style("minting").bright().green().on_black(),
    pauser = style("pauser").bold().cyan().on_black(),
    preset = style("ERC20PresetMinterPauser").bold().white().on_black(),
    stop = style("stop").cyan().on_black(),
  );

  read_more(
    "ERC20PresetMinterPauser",
    "ERC20",
    "https://docs.openzeppelin.com/contracts/3.x/api/presets#ERC20PresetMinterPauser",
    "https://eips.ethereum.org/EIPS/eip-20",
  );

  deploy_with("niftygate contract deploy ERC20PresetMinterPauser");
}

fn openzeppelin_preset_erc721_preset_minter_pauser_auto_id() {
  we_can_do_it();
  println!(
    "
  {preset} (from OpenZeppelin)

  This is an {erc} token, including:
    - ability for {holders} to {burn} (destroy) their tokens
    - a {minter} role that allows for token {minting} (creation)
    - a {pauser} role that allows to {stop} all token transfers
    - token ID and URI autogeneration

  The account that deploys the contract will be granted the {minter} 
  and {pauser} roles, as well as the default {admin} role, which will 
  let it grant both {minter} and {pauser} roles to other accounts.
    ",
    admin = style("admin").bold().magenta().on_black(),
    burn = style("burn").bright().red().on_black(),
    erc = style("ERC721").bold().blue(),
    holders = style("holders").bold().yellow().on_black(),
    minter = style("minter").bold().green().on_black(),
    minting = style("minting").bright().green().on_black(),
    pauser = style("pauser").bold().cyan().on_black(),
    preset = style("ERC721PresetMinterPauserAutoId")
      .bold()
      .white()
      .on_black(),
    stop = style("stop").cyan().on_black(),
  );

  read_more(
    "ERC721PresetMinterPauserAutoId",
    "ERC721",
    "https://docs.openzeppelin.com/contracts/3.x/api/presets#ERC721PresetMinterPauserAutoId",
    "https://eips.ethereum.org/EIPS/eip-721",
  );

  deploy_with("niftygate contract deploy ERC721PresetMinterPauserAutoId");
}

fn openzeppelin_preset_erc777_preset_fixed_supply() {
  we_can_do_it();
  println!(
    "
  {preset} (from OpenZeppelin)

  This is an {erc} token, including:
    - Preminted initial supply
    - No access control mechanism (for {minting}/{pausing}) and hence no governance
    ",
    preset = style("ERC777PresetFixedSupply").bold().white().on_black(),
    erc = style("ERC777").bold().blue(),
    minting = style("minting").bright().green().on_black(),
    pausing = style("pausing").bright().cyan().on_black(),
  );

  read_more(
    "ERC777PresetFixedSupply",
    "ERC777",
    "https://docs.openzeppelin.com/contracts/3.x/api/presets#ERC777PresetFixedSupply",
    "https://eips.ethereum.org/EIPS/eip-777",
  );

  deploy_with("niftygate contract deploy ERC777PresetFixedSupply");
}

fn openzeppelin_preset_erc1155_preset_fixed_supply() {
  we_can_do_it();
  println!(
    "
  {preset} (from OpenZeppelin)

  This is an {erc} token, including:
  - ability for {holders} to {burn} (destroy) their tokens
  - a {minter} role that allows for token {minting} (creation)
  - a {pauser} role that allows to {stop} all token transfers

  The account that deploys the contract will be granted the {minter} 
  and {pauser} roles, as well as the default {admin} role, which will 
  let it grant both {minter} and {pauser} roles to other accounts.
    ",
    preset = style("ERC1155PresetFixedSupply").bold().white().on_black(),
    erc = style("ERC1155").bold().blue(),
    burn = style("burn").bright().red().on_black(),
    minting = style("minting").bright().green().on_black(),
    holders = style("holders").bold().yellow().on_black(),
    minter = style("minter").bold().green().on_black(),
    pauser = style("pauser").bold().white().on_black(),
    admin = style("admin").bold().magenta().on_black(),
    stop = style("stop").red().on_black(),
  );

  read_more(
    "ERC1155PresetFixedSupply",
    "ERC1155",
    "https://docs.openzeppelin.com/contracts/3.x/api/presets#ERC1155PresetMinterPauser",
    "https://eips.ethereum.org/EIPS/eip-1155",
  );

  deploy_with("niftygate contract deploy ERC1155PresetFixedSupply")
}

fn choose_a_contract() -> Result<()> {
  let theme = ColorfulTheme::default();
  let fungible = Select::with_theme(&theme)
    .with_prompt("Are tokens fungible?")
    .item("Yes, any token is equivalent to any other token (like points).")
    .item("No, each token is unique (like subscriptions).")
    .default(0)
    .interact()?
    .eq(&0);

  let mintable = if fungible {
    Confirm::with_theme(&theme)
      .with_prompt("Can the token supply be increased after deployment?")
      .interact()?
  } else {
    false
  };

  let operators = if fungible {
    Confirm::with_theme(&theme)
      .with_prompt(
        "Do you require operators (special accounts that can transfer on behalf of token holders)?",
      )
      .interact()?
  } else {
    false
  };

  let bundles = if !fungible {
    Confirm::with_theme(&theme)
      .with_prompt("Do you need to bundle multiple tokens types under a single contract?")
      .interact()?
  } else {
    false
  };

  match fungible {
    false => match bundles {
      false => openzeppelin_preset_erc721_preset_minter_pauser_auto_id(),
      true => openzeppelin_preset_erc1155_preset_fixed_supply(),
    },
    true => match (mintable, operators) {
      (false, false) => openzeppelin_erc20_preset_fixed_supply(),
      (false, true) => openzeppelin_preset_erc777_preset_fixed_supply(),
      (true, false) => openzeppelin_preset_erc20_preset_minter_pauser(),
      (true, true) => suggest_custom_mintable_with_operators(),
    },
  }

  Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Interactive Guides")]
pub enum Command {
  #[structopt(about = "Helps you decide which contract to use.")]
  ChooseAContract,
}

impl Command {
  pub fn execute(self) -> Result<()> {
    match self {
      Self::ChooseAContract => choose_a_contract(),
    }
  }
}
