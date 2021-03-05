ZKSync under the hood

0. Precedure

   When users do a Layer2 transaction, there are should following precedures:

   Users must first register his public key first, WHY?

   Wallet packs the transaction, signs it.

   This should be wallet side or server API side, we need find its entry and how it did.

1. Witness

   Code in `zksync_witeness_generator`.

   It starts with a prometheus data exporter and a prover server.

   Then the prover server repeatedly check if database has available witness data, if it has, then it convert witness data into pub data, and then insert back into database. BTW, zksync use PostgreSQL as their database.

   Question:

   * what's prometheus data exporter? 
     * It's used for event monitoring and alerting. It monitor application's health.
   * what does `update_prover_job_queue` and `maintain` mean?

2. Prove

3. Submit

4. Verify

5. Finalize