use ethers::signers::LocalWallet;
use hyperliquid_rust_sdk::{BaseUrl, ExchangeClient};
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: LocalWallet = "6a8ecb51ace4f8fac7a5329ad0278e1278f54202365b0a637b97916e3304762a"
        .parse()
        .unwrap();

    let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Mainnet), None, None)
        .await
        .unwrap();

    let amount = "2"; // 1 USD
    let destination = "0x0D1d9635D0640821d15e323ac8AdADfA9c111414";

    let res = exchange_client
        .usdc_transfer(amount, destination, None)
        .await
        .unwrap();
    info!("Usdc transfer result: {res:?}");
}
