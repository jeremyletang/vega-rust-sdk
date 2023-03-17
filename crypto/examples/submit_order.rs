use sha3::{Digest, Sha3_256};
use vega_crypto::{Credentials, Transact};
use vega_protobufs::vega::{
    commands::v1::{input_data::Command, OrderSubmission},
    order::{TimeInForce, Type},
    Side,
};

const MNEMONIC: &str = "another deal useless giraffe quarter glimpse blur civil reflect jelly quit endorse engage slender energy scare ask suggest toe spirit leaf seed unveil million";
// const DERIVATIONS: usize = 42;

// const PRIVKEY: &str = "e70da3716e54cfe4cbed58b584b85095bb4a8257a4b39ec91b491f29526430b6053a10c3e8aa92bcfae80b61845a23a4dfc88d94a31570e3c494da9f43b64ca0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut t = Transact::new(
        Credentials::Mnemonic(MNEMONIC, 2),
        "tcp://n10.testnet.vega.xyz:3002",
    )
    .await?;

    let order = Command::OrderSubmission(OrderSubmission {
        expires_at: 0,
        market_id: "10c7d40afd910eeac0c2cad186d79cb194090d5d5f13bd31e14c49fd1bded7e2".to_string(),
        pegged_order: None,
        price: "700000".to_string(),
        size: 10,
        reference: "justin".to_string(),
        side: Side::Sell.into(),
        time_in_force: TimeInForce::Gtc.into(),
        r#type: Type::Limit.into(),
    });

    let tx = t.sign(&order).await?;

    println!("{:?}", tx);

    let signature = hex::decode(tx.signature.as_ref().unwrap().value.clone()).unwrap();

    let res = t.send(tx).await?;

    println!("{:?}", res);

    let mut hasher = Sha3_256::new();
    hasher.update(signature);
    let h = hasher.finalize().to_vec();
    let order_id = hex::encode(h);

    println!("order id: {}", order_id);

    return Ok(());
}
