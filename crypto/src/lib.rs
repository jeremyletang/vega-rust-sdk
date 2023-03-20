use crypto::Signer;
use errors::Error;
use prost::Message;
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256};
use vega_protobufs::vega::{
    api::v1::{
        core_service_client::CoreServiceClient, submit_raw_transaction_request,
        CheckTransactionRequest, LastBlockHeightRequest, SubmitTransactionRequest,
    },
    commands::v1::{
        input_data::Command, transaction::From as From_, InputData, ProofOfWork, Signature,
        Transaction, TxVersion,
    },
};

mod crypto;
pub mod errors;
pub mod pow;
pub mod slip10;

const CHAIN_ID_DELIMITER: char = 0 as char;
const SIGNATURE_ALGORITHM: &str = "vega/ed25519";

#[derive(Clone)]
pub struct Transact {
    signer: Signer,
    client: CoreServiceClient<tonic::transport::Channel>,
}

#[derive(Clone, Debug)]
pub enum Credentials<'s> {
    /// An hex encoded private key
    PrivateKey(&'s str),
    /// A mnemonic phrase and derivation count
    /// this is to be compatible with the the vega wallet
    /// standard derivation
    Mnemonic(&'s str, usize),
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct CheckTxResult {
    pub success: bool,
    pub code: u32,
    pub error: Option<String>,
    pub log: Option<String>,
    pub info: Option<String>,
    pub gas_wanted: i64,
    pub gas_used: i64,
}

#[derive(Clone, Debug)]
pub struct SendTxResult {
    pub success: bool,
    pub code: u32,
    pub error: Option<String>,
    pub hash: String,
}

impl Transact {
    pub async fn new<'s, D>(creds: Credentials<'s>, node_address: D) -> Result<Transact, Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
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

        let txid = random_hash();

        let (pow_nonce, _) = pow::solve(
            &res.get_ref().hash,
            &txid,
            res.get_ref().spam_pow_difficulty as usize,
        )?;

        let input_data = InputData {
            nonce: gen_nonce(),
            block_height: res.get_ref().height,
            command: Some(cmd.clone()),
        }
        .encode_to_vec();

        let signature = hex::encode(self.signer.sign(&build_signable_message(
            &input_data,
            &res.get_ref().chain_id,
        )));

        return Ok(Transaction {
            from: Some(From_::PubKey(hex::encode(self.signer.pubkey()))),
            version: TxVersion::V3.into(),
            input_data,
            signature: Some(Signature {
                value: signature,
                algo: SIGNATURE_ALGORITHM.into(),
                version: 1,
            }),
            pow: Some(ProofOfWork {
                tid: txid,
                nonce: pow_nonce,
            }),
        });
    }

    pub async fn send<P>(&mut self, p: P) -> Result<SendTxResult, Error>
    where
        P: Into<Payload>,
    {
        let tx = match p.into() {
            Payload::Command(c) => self.sign(&c).await?,
            Payload::Transaction(tx) => tx,
        };
        let resp = self
            .client
            .submit_transaction(SubmitTransactionRequest {
                tx: Some(tx),
                r#type: submit_raw_transaction_request::Type::Sync.into(),
            })
            .await?;

        let err = match resp.get_ref().success {
            true => None,
            false => Some(resp.get_ref().data.to_string()),
        };

        return Ok(SendTxResult {
            success: resp.get_ref().success,
            hash: resp.get_ref().tx_hash.clone(),
            code: resp.get_ref().code,
            error: err,
        });
    }

    pub async fn check<P>(&mut self, p: P) -> Result<CheckTxResult, Error>
    where
        P: Into<Payload>,
    {
        let tx = match p.into() {
            Payload::Command(c) => self.sign(&c).await?,
            Payload::Transaction(tx) => tx,
        };
        let resp = self
            .client
            .check_transaction(CheckTransactionRequest { tx: Some(tx) })
            .await?;

        let err = match resp.get_ref().success {
            true => None,
            false => Some(resp.get_ref().data.to_string()),
        };

        let info = match resp.get_ref().info.is_empty() {
            true => None,
            false => Some(resp.get_ref().info.to_string()),
        };

        let log = match resp.get_ref().log.is_empty() {
            true => None,
            false => Some(resp.get_ref().log.to_string()),
        };

        return Ok(CheckTxResult {
            success: resp.get_ref().success,
            code: resp.get_ref().code,
            gas_used: resp.get_ref().gas_used,
            gas_wanted: resp.get_ref().gas_wanted,
            error: err,
            info: info,
            log: log,
        });
    }

    /// The public key hex encoded
    pub fn public_key(&self) -> String {
        return hex::encode(self.signer.pubkey());
    }

    /// The secret key hex encoded
    pub fn secret_key(&self) -> String {
        return hex::encode(self.signer.secret());
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
