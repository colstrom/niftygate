use dialoguer::theme::Theme;
use dialoguer::{MultiSelect, Select};
use humansize::file_size_opts::{self, FileSizeOpts};
use humansize::FileSize;
use structopt::clap::arg_enum;

arg_enum! {
  #[derive(Debug)]
  pub enum PrettySizeUnits {
    Binary,
    Decimal,
    Conventional,
  }
}

impl Default for PrettySizeUnits {
  fn default() -> Self {
    Self::Binary
  }
}

impl From<&PrettySizeUnits> for FileSizeOpts {
  fn from(units: &PrettySizeUnits) -> Self {
    match units {
      PrettySizeUnits::Binary => file_size_opts::BINARY,
      PrettySizeUnits::Decimal => file_size_opts::DECIMAL,
      PrettySizeUnits::Conventional => file_size_opts::CONVENTIONAL,
    }
  }
}

pub(crate) fn pretty_size(size: u64, units: &PrettySizeUnits) -> String {
  let opts: FileSizeOpts = units.into();
  size
    .file_size(opts)
    .expect("something has gone horribly wrong, and an unsigned integer is negative")
}

#[derive(Debug, thiserror::Error)]
pub enum MenuError {
  #[error("default item must be among selectable items (this is a bug).")]
  UnselectableDefault,
  #[error("the index of one or more selected items is out of bounds (this is a bug).")]
  SelectionIndexOutOfBounds,
  #[error("io error: {0}")]
  IOError(#[from] std::io::Error),
}

#[derive(Debug, Default)]
pub struct UserInput<T>
where
  T: Theme,
{
  theme: T,
}

impl<T> UserInput<T>
where
  T: Theme + Default,
{
  pub fn new() -> Self {
    let theme = T::default();
    Self { theme }
  }

  fn select<Prompt>(&self, prompt: Prompt) -> Select
  where
    Prompt: Into<String>,
  {
    let mut menu = Select::with_theme(&self.theme);
    menu.with_prompt(prompt);
    menu
  }

  fn multi_select<Prompt>(&self, prompt: Prompt) -> MultiSelect
  where
    Prompt: Into<String>,
  {
    let mut menu = MultiSelect::with_theme(&self.theme);
    menu.with_prompt(prompt);
    menu
  }

  pub fn select_one<'item, Prompt, Item>(
    &self,
    prompt: Prompt,
    items: &'item [Item],
    default: Option<&Item>,
  ) -> Result<&'item Item, MenuError>
  where
    Prompt: Into<String>,
    Item: PartialEq + ToString,
  {
    let mut menu = self.select(prompt);
    menu.items(items);

    if let Some(default) = default {
      match items.iter().position(|item| item.eq(default)) {
        Some(val) => {
          menu.default(val);
        }
        None => return Err(MenuError::UnselectableDefault),
      }
    }

    let selection = menu.interact()?;
    match items.get(selection) {
      None => Err(MenuError::SelectionIndexOutOfBounds),
      Some(selected) => Ok(selected),
    }
  }

  pub fn select_multiple<'item, Prompt, Item>(
    &self,
    prompt: Prompt,
    items: &'item [Item],
  ) -> Result<Vec<&'item Item>, MenuError>
  where
    Prompt: Into<String>,
    Item: ToString,
  {
    let mut menu = self.multi_select(prompt);
    menu.items(items);

    let mut selected = Vec::new();

    for selection in menu.interact()? {
      match items.get(selection) {
        None => return Err(MenuError::SelectionIndexOutOfBounds),
        Some(item) => selected.push(item),
      }
    }

    Ok(selected)
  }

  pub fn select_multiple_as<'item, DisplayFn, Prompt, Item, Label>(
    &self,
    display: DisplayFn,
    prompt: Prompt,
    items: &'item [Item],
  ) -> Result<Vec<&'item Item>, MenuError>
  where
    DisplayFn: Fn(&'item Item) -> Label,
    Prompt: Into<String>,
    Label: ToString,
  {
    let mut menu = self.multi_select(prompt);
    let labels = items
      .iter()
      .map(display)
      .map(|label| label.to_string())
      .collect::<Vec<String>>();
    menu.items(&labels);

    let mut selected = Vec::new();

    for selection in menu.interact()? {
      match items.get(selection) {
        None => return Err(MenuError::SelectionIndexOutOfBounds),
        Some(item) => selected.push(item),
      }
    }

    Ok(selected)
  }
}
