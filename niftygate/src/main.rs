use niftygate::WrappedResult;

#[async_std::main]
async fn main() -> WrappedResult<()> {
  niftygate::command::run().await
}
