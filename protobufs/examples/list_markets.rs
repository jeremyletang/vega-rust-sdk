use vega_protobufs::datanode::api::v2::{
    trading_data_service_client::TradingDataServiceClient, ListMarketsRequest,
};

const NODE_ADDRESS: &str = "tcp://n10.testnet.vega.xyz:3007";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TradingDataServiceClient::connect(NODE_ADDRESS).await?;

    let resp = client
        .list_markets(ListMarketsRequest { pagination: None })
        .await?;

    for mkt in resp.get_ref().markets.as_ref().unwrap().edges.iter() {
        println!("{:?}", mkt.node.as_ref().unwrap());
    }

    return Ok(());
}
