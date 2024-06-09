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

    let usd = "2"; // 5 USD
    let destination = "0x9918083A1564A7ec41ceDEAbFDc1Ce96a402970b";

    let res = exchange_client
        .withdraw_from_bridge(usd, destination, None)
        .await
        .unwrap();
    info!("Withdraw from bridge result: {res:?}");
}
