use bitcoin::Address;
use bitcoin::Script;
use jsonrpc_client::ClientError;
use jsonrpc_client::RpcError;
use rpc;
use types::address::AddressInfoResult;
use BlockHash;
use TransactionId;

#[allow(unused_variables)]
pub trait BitcoinRpcApi: Send + Sync {
    // Order as per: https://bitcoin.org/en/developer-reference#rpcs

    fn add_multisig_address(
        &self,
        number_of_required_signatures: u32,
        participants: Vec<&Address>,
    ) -> Result<Result<rpc::MultiSigAddress, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: abandontransaction
    // TODO: addmultisigaddress
    // TODO: addnode
    // TODO: addwitnessaddress
    // TODO: backupwallet
    // TODO: bumpfee
    // TODO: clearbanned
    // TODO: createmultisig

    fn create_raw_transaction(
        &self,
        inputs: Vec<&rpc::NewTransactionInput>,
        output: &rpc::NewTransactionOutput,
    ) -> Result<Result<rpc::SerializedRawTransaction, RpcError>, ClientError> {
        unimplemented!()
    }

    fn decode_rawtransaction(
        &self,
        tx: rpc::SerializedRawTransaction,
    ) -> Result<Result<rpc::DecodedRawTransaction, RpcError>, ClientError> {
        unimplemented!()
    }

    fn decode_script(
        &self,
        script: Script,
    ) -> Result<Result<rpc::DecodedScript, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: disconnectnode

    fn dump_privkey(
        &self,
        address: &Address,
    ) -> Result<Result<rpc::PrivateKey, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: dumpwallet
    // TODO: encryptwallet
    // TODO: estimatefee
    // TODO: estimatepriority

    fn fund_raw_transaction(
        &self,
        tx: &rpc::SerializedRawTransaction,
        options: &rpc::FundingOptions,
    ) -> Result<Result<rpc::FundingResult, RpcError>, ClientError> {
        unimplemented!()
    }

    fn generate(
        &self,
        number_of_blocks: u32,
    ) -> Result<Result<Vec<BlockHash>, RpcError>, ClientError> {
        unimplemented!()
    }

    fn generate_to_address(
        &self,
        number_of_blocks: u32,
        address: &Address,
    ) -> Result<Result<Vec<BlockHash>, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: getaddednodeinfo

    fn get_address_info(
        &self,
        address: &Address,
    ) -> Result<Result<AddressInfoResult, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: getaddressesbylabel

    fn get_balance(&self) -> Result<Result<f32, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_best_block_hash(&self) -> Result<Result<BlockHash, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_block(
        &self,
        header_hash: &BlockHash,
    ) -> Result<Result<rpc::Block<TransactionId>, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO(evg): add verbosity param to get_block instead of separate methods?
    fn get_block_verbose(
        &self,
        header_hash: &BlockHash,
    ) -> Result<Result<rpc::Block<rpc::DecodedRawTransaction>, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_blockchain_info(&self) -> Result<Result<rpc::BlockchainInfo, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_block_count(&self) -> Result<Result<rpc::BlockHeight, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_block_hash(&self, height: u32) -> Result<Result<BlockHash, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: getblockheader
    // TODO: getblocktemplate
    // TODO: getchaintips
    // TODO: getconnectioncount
    // TODO: getdifficulty
    // TODO: getgenerate
    // TODO: gethashespersec
    // TODO: getinfo
    // TODO: getmemoryinfo
    // TODO: getmempoolancestors
    // TODO: getmempooldescendants
    // TODO: getmempoolentry
    // TODO: getmempoolinfo
    // TODO: getmininginfo
    // TODO: getnettotals
    // TODO: getnetworkhashesps
    // TODO: getnetworkinfo

    fn get_new_address(&self) -> Result<Result<Address, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: getpeerinfo
    // TODO: getrawchangeaddress
    // TODO: getrawmempool

    fn get_raw_transaction_serialized(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<rpc::SerializedRawTransaction, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_raw_transaction_verbose(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<rpc::VerboseRawTransaction, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: getreceivedbylabel
    // TODO: getreceivedbyaddress
    // TODO: gettxout
    // TODO: gettxoutsetinfo
    // TODO: getunconfirmedbalance
    // TODO: getwalletinfo
    // TODO: getwork
    // TODO: importaddress
    // TODO: importmulti
    // TODO: importprivkey
    // TODO: importprunedfunds
    // TODO: importwallet
    // TODO: keypoolrefill
    // TODO: invalidateblock
    // TODO: keypoolrefill
    // TODO: listlabels
    // TODO: listaddressgroupings
    // TODO: listbanned
    // TODO: listlockunspent
    // TODO: listreceivedbylabel
    // TODO: listreceivedbyaddress
    // TODO: listsinceblock
    // TODO: listtransactions

    fn list_unspent(
        &self,
        min_confirmations: rpc::TxOutConfirmations,
        max_confirmations: Option<u32>,
        recipients: Option<Vec<Address>>,
    ) -> Result<Result<Vec<rpc::UnspentTransactionOutput>, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: lockunspent
    // TODO: ping
    // TODO: preciousblock
    // TODO: prioritisetransaction
    // TODO: pruneblockchain
    // TODO: removeprunedfunds
    // TODO: sendmany

    fn send_raw_transaction(
        &self,
        tx_data: rpc::SerializedRawTransaction,
    ) -> Result<Result<TransactionId, RpcError>, ClientError> {
        unimplemented!()
    }

    fn send_to_address(
        &self,
        address: &Address,
        amount: f64,
    ) -> Result<Result<TransactionId, RpcError>, ClientError> {
        unimplemented!()
    }
    // TODO: setlabel
    // TODO: setban
    // TODO: setgenerate
    // TODO: setnetworkactive
    // TODO: settxfee
    // TODO: signmessage
    // TODO: signmessagewithprivkey

    fn sign_raw_transaction_with_key(
        &self,
        tx: &rpc::SerializedRawTransaction,
        private_keys: Option<Vec<&rpc::PrivateKey>>,
        dependencies: Option<Vec<&rpc::TransactionOutputDetail>>,
        signature_hash_type: Option<rpc::SigHashType>,
    ) -> Result<Result<rpc::SigningResult, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: signrawtransactionwithwallet
    // TODO: stop
    // TODO: submitblock

    fn validate_address(
        &self,
        address: &Address,
    ) -> Result<Result<rpc::AddressValidationResult, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: verifychain
    // TODO: verifymessage
    // TODO: verifytxoutproof
    // TODO: walletlock
    // TODO: walletpassphrase
    // TODO: walletpassphrasechange
}
