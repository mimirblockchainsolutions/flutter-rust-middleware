use error::Error;
use mimir_crypto::secp256k1::{Public, Secret, Address, Signer};
use mimir_common::types::{Bytes, H256};
use crypto::symmetriccipher::Decryptor;
use crypto::blockmodes::CtrMode;
use crypto::aessafe::AesSafe128Encryptor;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use mimir_crypto::keccak256::Keccak256;
use ring;
use rand;

#[derive(Debug, Serialize, Deserialize)]
pub struct Keys {
    public: Public,
    pub secret: Secret,
    address: Address,
}

#[derive(Deserialize, Debug)]
pub struct Wallet {
    crypto: Crypto,
}

#[derive(Deserialize, Debug)]
struct Crypto {
    ciphertext: H256,
    cipherparams: Cypherparams,
    kdf: String,
    kdfparams: Kdfparams,
    mac: H256,
}

#[derive(Deserialize, Debug)]
struct Cypherparams {
    iv: Bytes,
}

#[derive(Deserialize, Debug)]
struct Kdfparams {
    c: u32,
    dklen: u32,
    prf: String,
    salt: H256,
}

pub fn keygen() -> Keys {
    let signer: Signer = rand::random();
    Keys {
        public: signer.public(),
        secret: signer.secret(),
        address: signer.address(),
    }
}

pub fn decrypt_wallet(wallet: Wallet, password: String) -> Result<Secret, Error> {
    let mut derived_key = [0u8; 32];
    ring::pbkdf2::derive(
        &ring::digest::SHA256,
        wallet.crypto.kdfparams.c,
        &wallet.crypto.kdfparams.salt,
        password.as_ref(),
        &mut derived_key[..],
    );
    let right_bits = &derived_key[0..16];
    let left_bits = &derived_key[16..32];
    right_bits.to_vec();
    left_bits.to_vec();

    let mut mac_buff = vec![0u8; 16 + wallet.crypto.ciphertext.len()];
    mac_buff[0..16].copy_from_slice(left_bits);
    mac_buff[16..wallet.crypto.ciphertext.len() + 16].copy_from_slice(&wallet.crypto.ciphertext);

    let mac = Keccak256::hash(&mac_buff);

    assert_eq!(&mac, &*wallet.crypto.mac);
    let mut encryptor = CtrMode::new(
        AesSafe128Encryptor::new(&left_bits),
        wallet.crypto.cipherparams.iv.into(),
    );
    let mut decrypted = [0u8; 32];
    encryptor
        .decrypt(
            &mut RefReadBuffer::new(&wallet.crypto.ciphertext),
            &mut RefWriteBuffer::new(&mut decrypted),
            true,
        )
        .map_err(|err| Error::message("decryption failed"))?;
    Ok(decrypted.into())
}

pub fn create_signer(secret: Secret) -> Result<Signer, Error> {
    Signer::new(secret).map_err(|err| err.into())
}

pub fn keys_to_signer(keys: Keys) -> Result<Signer, Error> {
    Signer::new(keys.secret).map_err(|err| err.into())
}
