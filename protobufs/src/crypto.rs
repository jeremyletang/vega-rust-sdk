use crate::errors::Error;
use crate::slip10::{self, Node};
use bip39::{Language, Mnemonic};
use ed25519_compact;
use sha3::{Digest, Sha3_256};

const MAGIC_NB: u32 = 1789;
const ORGIN_INDEX: u32 = slip10::FIRST_HARDENED_INDEX + MAGIC_NB;

#[derive(Clone)]
pub struct Signer {
    secret: ed25519_compact::SecretKey,
    pubkey: ed25519_compact::PublicKey,
}

impl Signer {
    pub fn from_secret_key(secret: &str) -> Result<Signer, Error> {
        let bytes = hex::decode(secret)?;
        let secret = ed25519_compact::SecretKey::from_slice(&*bytes)?;
        return Ok(Signer {
            pubkey: secret.public_key(),
            secret,
        });
    }
    pub fn from_mnemonic(mnemonic: &str, derivations: usize) -> Result<Signer, Error> {
        let m = Mnemonic::parse_in(Language::English, mnemonic)?;
        let node = Node::new_master_node(&m.to_seed(""))?
            .derive(ORGIN_INDEX)?
            .derive(slip10::FIRST_HARDENED_INDEX)?
            .derive(slip10::FIRST_HARDENED_INDEX + derivations as u32)?;
        let (p, s) = node.keypair();
        return Ok(Signer {
            secret: s.bytes,
            pubkey: p.bytes,
        });
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(message);
        let h = hasher.finalize().to_vec();
        return self.secret.sign(h, None).to_vec();
    }

    pub fn pubkey(&self) -> &[u8] {
        return &*self.pubkey;
    }

    pub fn secret(&self) -> &[u8] {
        return &*self.secret;
    }
}

#[cfg(test)]
mod tests {
    // extern crate test;
    use super::*;
    use hex;

    #[test]
    fn test_derive() {
        let mnemomnic = "another deal useless giraffe quarter glimpse blur civil reflect jelly quit endorse engage slender energy scare ask suggest toe spirit leaf seed unveil million";
        let pubkey = "053a10c3e8aa92bcfae80b61845a23a4dfc88d94a31570e3c494da9f43b64ca0";
        let secretkey = "e70da3716e54cfe4cbed58b584b85095bb4a8257a4b39ec91b491f29526430b6053a10c3e8aa92bcfae80b61845a23a4dfc88d94a31570e3c494da9f43b64ca0";

        let s = Signer::from_mnemonic(mnemomnic, 10);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(pubkey, hex::encode(s.pubkey()));
        assert_eq!(secretkey, hex::encode(s.secret()));

        let s2 = Signer::from_secret_key(&*hex::encode(s.secret()));
        assert!(s2.is_ok());
        let s2 = s2.unwrap();
        assert_eq!(pubkey, hex::encode(s2.pubkey()));
        assert_eq!(secretkey, hex::encode(s2.secret()));
    }
}
