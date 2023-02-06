use vega_wallet_client::commands::{VoteSubmission, VoteValue};
use vega_wallet_client::WalletClient;

const WALLET_ADDRESS: &str = "http://localhost:1789";
const API_TOKEN: &str = "yf7loKt70Tgq4GXyoAcm68HUav5cwewbh9MYvvVDk4ARgyJD4CSl4cGtc6xmiJTA";

const PUB_KEY: &str = "6545621b8a3f398db322a4acc68c1b59fd284ab010e157e5aa887a6f55d94eba";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clt = WalletClient::new(WALLET_ADDRESS, API_TOKEN, PUB_KEY).await?;
    println!("{:?}", clt.list_keys().await?);

    let v = clt
        .sign(VoteSubmission {
            proposal_id: "6545621b8a3f398db322a4acc68c1b59fd284ab010e157e5aa887a6f55d94eba"
                .to_string(),
            value: VoteValue::No,
        })
        .await?;

    println!("{:?}", v);

    return Ok(());
}
