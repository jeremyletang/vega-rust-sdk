//use protos::vega::commands::v1::Transaction;
use crypto::Signer;
use errors::Error;
use prost::Message;
use protos::vega::{
    api::v1::{
        core_service_client::CoreServiceClient, CheckTransactionRequest, LastBlockHeightRequest,
        SubmitTransactionRequest,
    },
    commands::v1::{
        input_data::Command, transaction::From as From_, InputData, ProofOfWork, Signature,
        Transaction,
    },
};
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256};

mod crypto;
mod errors;
pub mod pow;
pub mod protos;
pub mod slip10;

const CHAIN_ID_DELIMITER: char = 0 as char;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct Transact {
    signer: Signer,
    client: CoreServiceClient<tonic::transport::Channel>,
}

pub enum Credentials<'s> {
    /// An hex encoded private key
    PrivateKey(&'s str),
    /// A mnemonic phrase and derivation count
    /// this is to be compatible with the the vega wallet
    /// standard derivation
    Mnemonic(&'s str, usize),
}

pub enum Payload {
    Command(Command),
    Transaction(Transaction),
}

impl From<Command> for Payload {
    fn from(c: Command) -> Self {
        Payload::Command(c)
    }
}

impl From<Transaction> for Payload {
    fn from(t: Transaction) -> Self {
        Payload::Transaction(t)
    }
}

impl Transact {
    pub async fn new<'s, D>(creds: Credentials<'s>, node_address: D) -> Result<Transact, Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let signer = match creds {
            Credentials::PrivateKey(secret) => crypto::Signer::from_secret_key(secret)?,
            Credentials::Mnemonic(mnemonic, derivations) => {
                crypto::Signer::from_mnemonic(mnemonic, derivations)?
            }
        };

        let client = CoreServiceClient::connect(node_address).await?;
        return Ok(Transact { signer, client });
    }

    pub async fn sign(&mut self, cmd: &Command) -> Result<Transaction, Error> {
        // first get the block infos
        let res = self
            .client
            .last_block_height(LastBlockHeightRequest {})
            .await?;

        let chain_id = res.get_ref().chain_id.clone();
        let nonce = gen_nonce();
        let block_height = res.get_ref().height;
        let block_hash = res.get_ref().hash.clone();
        let difficulty = res.get_ref().spam_pow_difficulty;
        let txid = random_hash();
        let (pow_nonce, _) = pow::solve(&block_hash, &txid, difficulty as usize)?;

        let input_data = InputData {
            nonce,
            block_height,
            command: Some(cmd.clone()),
        }
        .encode_to_vec();

        let signature = hex::encode(
            self.signer
                .sign(&build_signable_message(&input_data, &chain_id)),
        );

        return Ok(Transaction {
            from: Some(From_::PubKey(hex::encode(self.signer.pubkey()))),
            version: 3,
            input_data,
            signature: Some(Signature {
                value: signature,
                algo: "vega/ed25519".into(),
                version: 1,
            }),
            pow: Some(ProofOfWork {
                tid: txid,
                nonce: pow_nonce,
            }),
        });
    }

    pub async fn send<P>(&mut self, p: P) -> Result<(), Error>
    where
        P: Into<Payload>,
    {
        let tx = match p.into() {
            Payload::Command(c) => self.sign(&c).await?,
            Payload::Transaction(tx) => tx,
        };
        let res = self
            .client
            .submit_transaction(SubmitTransactionRequest {
                tx: Some(tx),
                r#type: 2,
            })
            .await?;
        return Ok(());
    }

    pub async fn check<P>(&mut self, p: P) -> Result<(), Error>
    where
        P: Into<Payload>,
    {
        let tx = match p.into() {
            Payload::Command(c) => self.sign(&c).await?,
            Payload::Transaction(tx) => tx,
        };
        let res = self
            .client
            .check_transaction(CheckTransactionRequest { tx: Some(tx) })
            .await?;
        return Ok(());
    }
}

fn build_signable_message(input_data: &[u8], chain_id: &str) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    out.extend_from_slice(chain_id.as_bytes());
    out.extend_from_slice(&[CHAIN_ID_DELIMITER as u8]);
    out.extend_from_slice(input_data);
    return out;
}

fn gen_nonce() -> u64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..u64::MAX);
}

fn random_hash() -> String {
    let msg = thread_rng()
        .sample_iter::<u8, _>(rand::distributions::Standard)
        .take(10)
        .collect::<Vec<u8>>();
    let mut hasher = Sha3_256::new();
    hasher.update(msg);
    let h = hasher.finalize().to_vec();
    return hex::encode(h).to_uppercase();
}
