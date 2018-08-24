use bitcoincore::TxOutConfirmations;
use jsonrpc::HTTPError;
use jsonrpc::RpcError;
use types::*;
use BitcoinRpcApi;
use NewTransactionOutput;

pub struct BitcoinStubClient {}

impl BitcoinStubClient {
    pub fn new() -> Self {
        Self {}
    }
}

#[allow(unused_variables)]
impl BitcoinRpcApi for BitcoinStubClient {
    fn add_multisig_address(
        &self,
        number_of_required_signatures: u32,
        participants: Vec<&Address>,
    ) -> Result<Result<MultiSigAddress, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn create_raw_transaction(
        &self,
        inputs: Vec<&NewTransactionInput>,
        output: &NewTransactionOutput,
    ) -> Result<Result<SerializedRawTransaction, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn decode_rawtransaction(
        &self,
        tx: SerializedRawTransaction,
    ) -> Result<Result<DecodedRawTransaction, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn decode_script(
        &self,
        script: RedeemScript,
    ) -> Result<Result<DecodedScript, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn dump_privkey(&self, address: &Address) -> Result<Result<PrivateKey, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn fund_raw_transaction(
        &self,
        tx: &SerializedRawTransaction,
        options: &FundingOptions,
    ) -> Result<Result<FundingResult, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn generate(
        &self,
        number_of_blocks: u32,
    ) -> Result<Result<Vec<BlockHash>, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn get_account(&self, address: &Address) -> Result<Result<Account, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn get_block(&self, header_hash: &BlockHash) -> Result<Result<Block, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn get_blockchain_info(&self) -> Result<Result<BlockchainInfo, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn get_block_count(&self) -> Result<Result<BlockHeight, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn get_new_address(&self) -> Result<Result<Address, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn get_raw_transaction_serialized(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<SerializedRawTransaction, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn get_raw_transaction_verbose(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<VerboseRawTransaction, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<Transaction, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn list_unspent(
        &self,
        min_confirmations: TxOutConfirmations,
        max_confirmations: Option<u32>,
        recipients: Option<Vec<Address>>,
    ) -> Result<Result<Vec<UnspentTransactionOutput>, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn send_raw_transaction(
        &self,
        tx_data: SerializedRawTransaction,
    ) -> Result<Result<TransactionId, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn send_to_address(
        &self,
        address: &Address,
        amount: f64,
    ) -> Result<Result<TransactionId, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn sign_raw_transaction(
        &self,
        tx: &SerializedRawTransaction,
        dependencies: Option<Vec<&TransactionOutputDetail>>,
        private_keys: Option<Vec<&PrivateKey>>,
        signature_hash_type: Option<SigHashType>,
    ) -> Result<Result<SigningResult, RpcError>, HTTPError> {
        unimplemented!()
    }

    fn validate_address(
        &self,
        address: &Address,
    ) -> Result<Result<AddressValidationResult, RpcError>, HTTPError> {
        unimplemented!()
    }
}
