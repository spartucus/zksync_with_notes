// External deps
use num::ToPrimitive;
use zksync_crypto::franklin_crypto::{
    bellman::pairing::{
        bn256::{Bn256, Fr},
        ff::{Field, PrimeField},
    },
    rescue::RescueEngine,
};
// Workspace deps
use zksync_crypto::{
    circuit::{
        account::CircuitAccountTree,
        utils::{append_be_fixed_width, le_bit_vector_into_field_element},
    },
    params::{
        account_tree_depth, ACCOUNT_ID_BIT_WIDTH, AMOUNT_EXPONENT_BIT_WIDTH,
        AMOUNT_MANTISSA_BIT_WIDTH, CHUNK_BIT_WIDTH, FEE_EXPONENT_BIT_WIDTH, FEE_MANTISSA_BIT_WIDTH,
        NEW_PUBKEY_HASH_WIDTH, NONCE_BIT_WIDTH, TOKEN_BIT_WIDTH, TX_TYPE_BIT_WIDTH,
    },
    primitives::FloatConversions,
};
use zksync_types::operations::CreatePairOp;
// Local deps
use crate::{
    operation::{Operation, OperationArguments, OperationBranch, OperationBranchWitness},
    utils::resize_grow_only,
    witness::{
        utils::{apply_leaf_operation, get_audits, SigDataInput},
        Witness,
    },
};

pub struct CreatePairWitness<E: RescueEngine> {
    // TODO:
}

impl Witness for CreatePairWitness<Bn256> {
    type OperationType = CreatePairOp;
    type CalculateOpsInput = SigDataInput;

    fn apply_tx(tree: &mut CircuitAccountTree, transfer: &CreatePairOp) -> Self {}

    fn get_pubdata(&self) -> Vec<bool> {}

    fn get_offset_commitment_data(&self) -> Vec<bool> {}

    fn calculate_operations(&self, input: Self::CalculateOpsInput) -> Vec<Operation<Bn256>> {}
}
