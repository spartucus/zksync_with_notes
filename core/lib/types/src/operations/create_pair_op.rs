use crate::tx::TxSignature;
use crate::CreatePair;
use crate::{AccountId, Address, Nonce};
use anyhow::{ensure, format_err};
use serde::{Deserialize, Serialize};
use zksync_crypto::params::{ACCOUNT_ID_BIT_WIDTH, CHUNK_BYTES};
use zksync_crypto::primitives::FromBytes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePairOp {
    pub tx: CreatePair,
    pub creator: AccountId,
    pub token_a: AccountId,
    pub token_b: AccountId,
    pub token_liquidity: AccountId,
}

impl CreatePairOp {
    pub const CHUNKS: usize = 6;
    pub const OP_CODE: u8 = 0x08;

    pub(crate) fn get_public_data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(Self::OP_CODE); // opcode
        data.extend_from_slice(&self.creator.to_be_bytes());
        data.extend_from_slice(&self.token_a.to_be_bytes());
        data.extend_from_slice(&self.token_b.to_be_bytes());
        data.extend_from_slice(&self.token_liquidity.to_be_bytes());
        data.resize(Self::CHUNKS * CHUNK_BYTES, 0x00);
        data
    }

    pub fn from_public_data(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(
            bytes.len() == Self::CHUNKS * CHUNK_BYTES,
            "Wrong bytes length for transfer pubdata"
        );
    }
}
