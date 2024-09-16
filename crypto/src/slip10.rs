use ed25519_compact::{KeyPair, Seed};
use hmac::{Hmac, Mac};
use regex::Regex;
use sha2::Sha512;
use std::error::Error as StdError;
use std::fmt;

pub const FIRST_HARDENED_INDEX: u32 = 0x80000000;
const SEED_MODIFIER: &str = "ed25519 seed";

#[derive(Debug)]
pub enum Error {
    InvalidPath,
    NoPublicDerivation,
    HmacError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuration error: {}", self.desc())
    }
}

impl StdError for Error {}

impl Error {
    pub fn desc(&self) -> String {
        use Error::*;
        match self {
            InvalidPath => "invalid derivation path".into(),
            NoPublicDerivation => "no public derivation allowed with ed25519".into(),
            HmacError(e) => format!("hmac error: {}", e),
        }
    }
}

pub struct Node {
    pub chain_code: Vec<u8>,
    pub key: Vec<u8>,
}

pub struct PublicKey {
    pub bytes: ed25519_compact::PublicKey,
}

impl PublicKey {
    pub fn bytes(&self) -> &[u8] {
        &*self.bytes
    }
}

pub struct PrivateKey {
    pub bytes: ed25519_compact::SecretKey,
}

impl PrivateKey {
    pub fn bytes(&self) -> &[u8] {
        &*self.bytes
    }

    pub fn seed(&self) -> Vec<u8> {
        return (*self.bytes.seed()).into();
    }
}

impl Node {
    pub fn derive_for_path(path: &str, seed: &[u8]) -> Result<Node, Error> {
        if !is_valid_path(path) {
            return Err(Error::InvalidPath);
        }

        let mut node = Node::new_master_node(seed)?;
        for s in path
            .split("/")
            .collect::<Vec<&str>>()
            .splice(1.., Vec::new())
        {
            // try to convert to uint + check overflow
            match s.trim_end_matches("'").parse::<u32>() {
                Ok(val) => {
                    let i = val + FIRST_HARDENED_INDEX;
                    node = node.derive(i)?;
                }
                Err(_) => return Err(Error::InvalidPath),
            }
        }

        return Ok(node);
    }

    // NewMasterNode generates a new master key from seed.
    pub fn new_master_node(seed: &[u8]) -> Result<Node, Error> {
        let mut hasher = match Hmac::<Sha512>::new_from_slice(SEED_MODIFIER.as_bytes()) {
            Ok(hasher) => hasher,
            Err(e) => return Err(Error::HmacError(e.to_string())),
        };

        hasher.update(seed);
        return Ok(Node::from_hash(&(hasher.finalize().into_bytes())));
    }

    pub fn from_hash(hash: &[u8]) -> Node {
        let mut key = vec![0; 32];
        key.clone_from_slice(&hash[..32]);
        let mut chain_code = vec![0; 32];
        chain_code.clone_from_slice(&hash[32..]);
        return Node { chain_code, key };
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend_from_slice(&self.key);
        out.extend_from_slice(&self.chain_code);
        return out;
    }

    pub fn derive(&self, i: u32) -> Result<Node, Error> {
        // no public derivation for ed25519
        if i < FIRST_HARDENED_INDEX {
            return Err(Error::NoPublicDerivation);
        }

        let mut bytes = vec![0x0; 1];
        bytes.extend_from_slice(&self.key);
        bytes.extend_from_slice(&i.to_be_bytes());

        let mut hasher = match Hmac::<Sha512>::new_from_slice(&self.chain_code) {
            Ok(hasher) => hasher,
            Err(e) => return Err(Error::HmacError(e.to_string())),
        };

        hasher.update(&bytes);
        return Ok(Node::from_hash(&(hasher.finalize().into_bytes())));
    }

    pub fn keypair(&self) -> (PublicKey, PrivateKey) {
        let seed = Seed::from_slice(&self.key).unwrap();
        let kp = KeyPair::from_seed(seed);
        return (PublicKey { bytes: kp.pk }, PrivateKey { bytes: kp.sk });
    }

    // https://github.com/satoshilabs/slips/blob/master/slip-0010/testvectors.py#L64
    pub fn public_key_with_prefix(&self) -> Vec<u8> {
        let (pkey, _) = self.keypair();
        let mut out = vec![0x00; 1];
        out.extend_from_slice(&pkey.bytes());
        return out;
    }

    pub fn raw_seed(&self) -> &[u8] {
        return &self.key;
    }

    // PrivateKey returns private key seed bytes
    pub fn private_key(&self) -> PrivateKey {
        let (_, prv) = self.keypair();
        return prv;
    }
}

fn is_valid_path(path: &str) -> bool {
    let re = Regex::new(r"^m(/[0-9]+')*$").unwrap();
    if !re.is_match(path) {
        return false;
    }

    for s in path
        .split("/")
        .collect::<Vec<&str>>()
        .splice(1.., Vec::new())
    {
        // try to convert to uint + check overflow
        if s.trim_end_matches("'").parse::<u32>().is_err() {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    // extern crate test;
    use super::*;
    // use test::Bencher;

    struct Args<'l> {
        path: &'l str,
        seed: &'l [u8],
    }
    struct Test<'l> {
        args: Args<'l>,
        want_priv: &'l str,
        want_pub: &'l str,
        want_err: bool,
    }

    #[test]
    fn test_derive_for_path() {
        let seed = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed).unwrap();
        let tests = vec![
            Test {
                args: Args {
                    path: "m",
                    seed: &seed_bytes,
                },
                want_priv: "2b4be7f19ee27bbf30c667b642d5f4aa69fd169872f8fc3059c08ebae2eb19e7",
                want_pub: "00a4b2856bfec510abab89753fac1ac0e1112364e7d250545963f135f2a33188ed",

                want_err: false,
            },
            Test {
                args: Args {
                    path: "m/0'",
                    seed: &seed_bytes,
                },
                want_priv: "68e0fe46dfb67e368c75379acec591dad19df3cde26e63b93a8e704f1dade7a3",
                want_pub: "008c8a13df77a28f3445213a0f432fde644acaa215fc72dcdf300d5efaa85d350c",
                want_err: false,
            },
            Test {
                args: Args {
                    path: "m/0'/1'",
                    seed: &seed_bytes,
                },
                want_priv: "b1d0bad404bf35da785a64ca1ac54b2617211d2777696fbffaf208f746ae84f2",
                want_pub: "001932a5270f335bed617d5b935c80aedb1a35bd9fc1e31acafd5372c30f5c1187",
                want_err: false,
            },
            Test {
                args: Args {
                    path: "m/0'/1'/2'",
                    seed: &seed_bytes,
                },
                want_priv: "92a5b23c0b8a99e37d07df3fb9966917f5d06e02ddbd909c7e184371463e9fc9",
                want_pub: "00ae98736566d30ed0e9d2f4486a64bc95740d89c7db33f52121f8ea8f76ff0fc1",

                want_err: false,
            },
            Test {
                args: Args {
                    path: "m/0'/1'/2'/2'",
                    seed: &seed_bytes,
                },
                want_priv: "30d1dc7e5fc04c31219ab25a27ae00b50f6fd66622f6e9c913253d6511d1e662",
                want_pub: "008abae2d66361c879b900d204ad2cc4984fa2aa344dd7ddc46007329ac76c429c",
                want_err: false,
            },
            Test {
                args: Args {
                    path: "m/0'/1'/2'/2'/1000000000'",
                    seed: &seed_bytes,
                },
                want_priv: "8f94d394a8e8fd6b1bc2f3f49f5c47e385281d5c17e65324b0f62483e37e8793",
                want_pub: "003c24da049451555d51a7014a37337aa4e12d41e485abccfa46b47dfb2af54b7a",
                want_err: false,
            },
            Test {
                args: Args {
                    path: "m/0",
                    seed: &seed_bytes,
                },
                want_priv: "",
                want_pub: "",
                want_err: true,
            },
        ];

        for t in tests.iter() {
            // let node = Node::new_master_node(&t.args.seed);
            let node = Node::derive_for_path(t.args.path, t.args.seed);
            assert_eq!(node.is_ok(), !t.want_err);
            if t.want_err {
                continue;
            }
            let node = node.unwrap();

            let privk = node.private_key().seed();
            assert_eq!(t.want_priv, hex::encode(&privk));

            let pubk = node.public_key_with_prefix();
            assert_eq!(t.want_pub, hex::encode(&pubk));
        }
    }

    #[test]
    fn test_is_valid_path() {
        assert!(is_valid_path("m/0'/2147483647'/1'/2147483646'/2'"));
    }

    // #[bench]
    // fn derive_keys(b: &mut Bencher) {
    //     let seed = "000102030405060708090a0b0c0d0e0f";
    //     let seed_bytes = hex::decode(seed).unwrap();
    //     let node = Node::new_master_node(&seed_bytes).unwrap();
    //     let mut i = 0;

    //     b.iter(|| {
    //         let n = node.derive(FIRST_HARDENED_INDEX + i);
    //         assert!(n.is_ok());
    //         i += 1;
    //     });
    // }
}
