use super::Storage;

pub struct Progress<S>
where
  S: Storage,
{
  pub storage: S,
}

impl<S> Progress<S>
where
  S: Storage,
{
  pub fn new(storage: S) -> Self {
    Self { storage }
  }
}

impl<S> Storage for Progress<S>
where
  S: Storage,
{
  type Path = <S as Storage>::Path;
  type Error = <S as Storage>::Error;

  fn root(&self) -> &Self::Path {
    self.storage.root()
  }

  // fn reader(&self, path: impl AsRef<Utf8Path>) -> Result<Option<Box<dyn Read>>, Self::Error> {
  //   self
  //     .storage
  //     .reader(path)
  //     .map(|reader| reader.map(|reader| ))
  // }

  // fn writer(&self, path: impl AsRef<Utf8Path>) -> Result<Box<dyn Write>, Self::Error> {
  //   self.storage.writer(path).map(|writer| writer.progress())
  // }
}
