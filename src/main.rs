mod app;
mod err;

use app::init_app;
use lambda_http::{run, tracing, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(init_app()).await
}
