use vega_protobufs::protos::vega::commands::v1::{input_data::Command, VoteSubmission};
use vega_protobufs::{Credentials, Transact};

// const MNEMONIC: &str = "another deal useless giraffe quarter glimpse blur civil reflect jelly quit endorse engage slender energy scare ask suggest toe spirit leaf seed unveil million";
// const DERIVATIONS: usize = 42;

const PRIVKEY: &str = "e70da3716e54cfe4cbed58b584b85095bb4a8257a4b39ec91b491f29526430b6053a10c3e8aa92bcfae80b61845a23a4dfc88d94a31570e3c494da9f43b64ca0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut t = Transact::new(
        Credentials::PrivateKey(PRIVKEY),
        "tcp://n10.testnet.vega.xyz:3002",
    )
    .await?;

    let tx = t
        .sign(&Command::VoteSubmission(VoteSubmission {
            proposal_id: "7e2847d30ef2d4858f0f098c4251c789ad63ac9644e610ddb1cb014334a01ca6"
                .to_string(),
            value: 1,
        }))
        .await?;

    println!("{:?}", tx);

    let res = t.send(tx).await?;

    println!("{:?}", res);

    return Ok(());
}
