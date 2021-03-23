ZKSync under the hood

0. Precedure

   When users do a Layer2 transaction, there are should following precedures:

   Users must first register his public key first, WHY? // The L1 address and L2 address (aka public key hash) are
   different.

   Wallet packs the transaction, signs it.

   This should be wallet side or server API side, we need find its entry and how it did.

   Question:

   - what's commitment and proof in ZKRollup?
   - what's state hash and verify key?
   - when transaction done on Layer2, the balance updated immediately, does this mean are there no double-spent issues?
   - what's witness and pub data?
   - Is pub data same as parameters?
   - What're the differences between `CircuitAccount` in `core/lib/crypto/src/circuit/account.rs` and
     `AccountWitness`/`AccountContent` in `core/lib/circuit/src/account.rs`?
   - If we write Rust circuits, how do we write same logic code of solidity?
   - And how could we test circuit to see if it is right?
   - when does `synthesize` be called?
   - when does `SRS/CRS` generated?
   - when to `proving`?
   - when to `verify`?

1. Witness

   Code in `zksync_witeness_generator`.

   It starts with a prometheus data exporter and a prover server.

   Then the prover server repeatedly check if database has available witness data, if it has, then it convert witness
   data into pub data, and then insert back into database. BTW, zksync use PostgreSQL as their database.

   Question:

   - what's prometheus data exporter?
     - It's used for event monitoring and alerting. It monitor application's health.
   - what does `update_prover_job_queue` and `maintain` mean?

2. Prove

   It is done by layer2 prover server.

3. Submit

   It is done by layer2 eth sender server.

4. Verify

   It is done on the layer1 contract side.

5. Finalize

   It is done on the layer1 contract side.

6. TODO:

   There are several structs that we need to define:

   - witness
   - operation in types
   - tx in types
   - account
