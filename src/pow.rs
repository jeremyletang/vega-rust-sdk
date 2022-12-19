use crate::Error;
use sha3::{Digest, Sha3_256};

const BLOCK_HASH_LEN: usize = 64;
const MAX_DIFFICULTY: usize = 256;
const MAX_NONCE: u64 = u64::MAX;
const PREFIX: &str = "Vega_SPAM_PoW";

pub fn solve(block_hash: &str, tx_id: &str, difficulty: usize) -> Result<(u64, Vec<u8>), Error> {
    if difficulty > MAX_DIFFICULTY {
        return Err(Error::InvalidDifficulty);
    }
    if tx_id.len() == 0 {
        return Err(Error::EmptyTxId);
    }
    if block_hash.len() != BLOCK_HASH_LEN {
        return Err(Error::InvalidBlockHash);
    }

    let mut nonce: u64 = 0;
    let mut hash = vec![];
    let mut hasher = Sha3_256::new();
    while nonce < MAX_NONCE {
        hasher.update(prepare_message(&block_hash, &tx_id, nonce));
        hash = hasher.finalize_reset().to_vec();
        if count_leading_zeroes(&hash) >= difficulty {
            break;
        }
        nonce += 1;
    }

    return Ok((nonce, hash));
}

fn count_leading_zeroes(h: &[u8]) -> usize {
    match h {
        [] => 0,
        [head, tail @ ..] => match zeroes(*head) {
            8 => 8 + count_leading_zeroes(tail),
            n => n,
        },
    }
}

fn zeroes(b: u8) -> usize {
    match b {
        0 => 8,
        _ => zeroes(b >> 1) - 1,
    }
}

fn prepare_message(block_hash: &str, tx_id: &str, nonce: u64) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    out.extend_from_slice(PREFIX.as_bytes());
    out.extend_from_slice(block_hash.as_bytes());
    out.extend_from_slice(tx_id.as_bytes());
    out.extend_from_slice(&nonce.to_be_bytes());
    return out;
}

// #[derive(Debug)]
// pub enum Error {

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "pow error: {}", self.desc())
//     }
// }

// impl StdError for Error {}

// impl Error {
//     pub fn desc(&self) -> String {
//         use Error::*;
//         match self {
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn test_pow() {
        let block_hash = "2FB2146FC01F21D358323174BAA230E7DE61C0F150B7FBC415C896B0C23E50FF";
        let tx_id = "2E7A16D9EF690F0D2BEED115FBA13BA2AAA16C8F971910AD88C72B9DB010C7D4";

        let (nonce, _) = super::solve(block_hash, tx_id, 2).unwrap();
        assert_eq!(nonce, 4);
    }

    #[test]
    fn test_zeroes() {
        let zs = super::zeroes(0);
        assert_eq!(zs, 8);
        let zs = super::zeroes(1);
        assert_eq!(zs, 7);
        let zs = super::zeroes(10);
        assert_eq!(zs, 4);
        let zs = super::zeroes(50);
        assert_eq!(zs, 2);
        let zs = super::zeroes(120);
        assert_eq!(zs, 1);
        let zs = super::zeroes(150);
        assert_eq!(zs, 0);
    }
    #[test]
    fn test_count_leading_zeroes() {
        let zs = super::count_leading_zeroes(&[0, 0, 0]);
        assert_eq!(zs, 24);
        let zs = super::count_leading_zeroes(&[150, 0, 0]);
        assert_eq!(zs, 0);
        let zs = super::count_leading_zeroes(&[0, 1, 0]);
        assert_eq!(zs, 15);
    }
}
