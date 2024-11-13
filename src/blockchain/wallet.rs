use hdwallet::{secp256k1::{PublicKey}, ExtendedPrivKey, ExtendedPubKey};
use sha2::{Digest, Sha256};
use chrono::Utc;
use bip39::{Mnemonic, Language};

use crate::blockchain::BlockTransaction;
#[derive(Debug)]
pub struct Wallet {
    id: String, // Derived from public key
    name: String,
    created_at: String,
    addresses: Vec<Address>,
    public_key: PublicKey // Used for testing, idea is not to store it in the future
}

#[derive(Debug)]
pub struct Address {
    address: String,
    created_at: String,
    transaction_history: Vec<Option<BlockTransaction>>
}

impl Wallet {
    pub fn new(name: String) -> Self {
        let created_at = Utc::now().to_rfc3339();
        let addresses = vec![];
        let public_key = Wallet::generate_public_key().unwrap();
        let id = "".to_string();
        Self {
            id,
            public_key,
            created_at,
            addresses,
            name,
        }
    }

    pub fn create_new_address(&mut self) {
        let address = Address::new();
        self.addresses.push(address);
    }

    // fn open_mnemonic_file() -> io::Result<Vec<String>> {
    //     let mnemonic_file_path = String::from("mnemonic.txt");
    //     let file = File::open(mnemonic_file_path)?;
    //     let reader = BufReader::new(file);

    //     let mut words: Vec<String> = Vec::new();

    //     for word in reader.lines() {
    //         let word = word?;
    //         words.push(word);
    //     }

    //     Ok(words)
    // }

    fn generate_public_key() -> Result<PublicKey, String> {
        // Generate a mnemonic and seed
        let mut rng = bip39::rand::thread_rng();
        let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 24).unwrap();
        let seed = mnemonic.to_seed("");  // Create the seed from the mnemonic
    
        // Attempt to create an ExtendedPrivKey from the seed
        let master_key = ExtendedPrivKey::with_seed(&seed);
    
        // Check if the master key creation succeeded
        if let Ok(master_key) = master_key {
            // Derive child private and public keys if master key is created successfully
            let child_priv_key = master_key.derive_private_key(hdwallet::KeyIndex::Normal(0)).unwrap();
            let child_pub_key = ExtendedPubKey::from_private_key(&child_priv_key);
    
            // Debug and print
            //dbg!("Master private key: ", master_key);
            //dbg!("Child public key: ", &child_pub_key);
            //println!("{}", mnemonic);
    
            // Return the derived public key
            return Ok(child_pub_key.public_key);
        }
    
        // If the master key creation failed, return an error
        Err("Cannot create master key from seed!".to_string())
    }
}

impl Address {
    pub fn new() -> Self {
        let created_at = Utc::now().to_rfc3339();
        let address = Self::generate_address();
        let transaction_history = vec![];

        Self {
            created_at,
            address,
            transaction_history
        }
    }

    fn generate_address() -> String {
        let combined_string = format!("");
        let mut hasher = Sha256::new();
        hasher.update(combined_string);
        let hash_result = hasher.finalize();
        let hash_result = format!("{:x}", hash_result);
        hash_result
    }
}
