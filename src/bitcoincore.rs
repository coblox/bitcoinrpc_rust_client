use jsonrpc_client::{
    header::{Authorization, Basic, Headers},
    HTTPClient, HTTPError, JsonRpcVersion, RpcClient, RpcError, RpcRequest,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Debug;
use types::{Address, *};
use BitcoinRpcApi;

struct RetryConfig {
    max_retries: u32,
    interval: u64,
}

pub struct BitcoinCoreClient {
    client: RpcClient,
    retry_config: Option<RetryConfig>,
}

pub enum TxOutConfirmations {
    Unconfirmed,
    AtLeast(i32),
}

#[allow(dead_code)]
impl BitcoinCoreClient {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: username.to_string(),
            password: Some(password.to_string()),
        }));

        let client = HTTPClient::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let rpc_client = RpcClient::new(client, url);

        BitcoinCoreClient {
            client: rpc_client,
            retry_config: Some(RetryConfig {
                max_retries: 10,
                interval: 500,
            }),
        }
    }

    fn get_raw_transaction<R: Debug>(
        &self,
        tx: &TransactionId,
        verbose: bool,
    ) -> Result<Result<R, RpcError>, HTTPError>
    where
        R: DeserializeOwned,
    {
        self.send(&RpcRequest::new2(
            JsonRpcVersion::V1,
            "42",
            "getrawtransaction",
            tx,
            verbose,
        ))
    }
}

impl BitcoinRpcApi for BitcoinCoreClient {
    // Order as per: https://bitcoin.org/en/developer-reference#rpcs

    fn add_multisig_address(
        &self,
        number_of_required_signatures: u32,
        participants: Vec<&Address>,
    ) -> Result<Result<MultiSigAddress, RpcError>, HTTPError> {
        self.send(&RpcRequest::new2(
            JsonRpcVersion::V1,
            "42",
            "addmultisigaddress",
            number_of_required_signatures,
            participants,
        ))
    }

    fn create_raw_transaction(
        &self,
        inputs: Vec<&NewTransactionInput>,
        output: &NewTransactionOutput,
    ) -> Result<Result<SerializedRawTransaction, RpcError>, HTTPError> {
        self.send(&RpcRequest::new2(
            JsonRpcVersion::V1,
            "42",
            "createrawtransaction",
            inputs,
            output,
        ))
    }

    fn decode_rawtransaction(
        &self,
        tx: SerializedRawTransaction,
    ) -> Result<Result<DecodedRawTransaction, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "decoderawtransaction",
            tx,
        ))
    }

    fn decode_script(
        &self,
        script: RedeemScript,
    ) -> Result<Result<DecodedScript, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "decodescript",
            script,
        ))
    }

    fn dump_privkey(&self, address: &Address) -> Result<Result<PrivateKey, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "dumpprivkey",
            address,
        ))
    }

    fn fund_raw_transaction(
        &self,
        tx: &SerializedRawTransaction,
        options: &FundingOptions,
    ) -> Result<Result<FundingResult, RpcError>, HTTPError> {
        self.send(&RpcRequest::new2(
            JsonRpcVersion::V1,
            "42",
            "fundrawtransaction",
            tx,
            options,
        ))
    }

    fn generate(
        &self,
        number_of_blocks: u32,
    ) -> Result<Result<Vec<BlockHash>, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "generate",
            number_of_blocks,
        ))
    }

    fn get_account(&self, address: &Address) -> Result<Result<Account, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "getaccount",
            address,
        ))
    }

    fn get_best_block_hash(&self) -> Result<Result<BlockHash, RpcError>, HTTPError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "42",
            "getbestblockhash",
        ))
    }

    fn get_block(&self, header_hash: &BlockHash) -> Result<Result<Block, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "getblock",
            header_hash,
        ))
    }

    fn get_blockchain_info(&self) -> Result<Result<BlockchainInfo, RpcError>, HTTPError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "42",
            "getblockchaininfo",
        ))
    }

    fn get_block_count(&self) -> Result<Result<BlockHeight, RpcError>, HTTPError> {
        self.send(&RpcRequest::new0(JsonRpcVersion::V1, "42", "getblockcount"))
    }

    fn get_block_hash(&self, height: u32) -> Result<Result<BlockHash, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "getblockhash",
            height,
        ))
    }

    fn get_new_address(&self) -> Result<Result<Address, RpcError>, HTTPError> {
        self.send(&RpcRequest::new2(
            JsonRpcVersion::V1,
            "42",
            "getnewaddress",
            "",
            "bech32",
        ))
    }

    fn get_raw_transaction_serialized(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<SerializedRawTransaction, RpcError>, HTTPError> {
        self.get_raw_transaction(tx, false)
    }

    fn get_raw_transaction_verbose(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<VerboseRawTransaction, RpcError>, HTTPError> {
        self.get_raw_transaction(tx, true)
    }

    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<Transaction, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "gettransaction",
            tx,
        ))
    }

    fn list_unspent(
        &self,
        min_confirmations: TxOutConfirmations,
        max_confirmations: Option<u32>,
        recipients: Option<Vec<Address>>,
    ) -> Result<Result<Vec<UnspentTransactionOutput>, RpcError>, HTTPError> {
        let min_confirmations = match min_confirmations {
            TxOutConfirmations::Unconfirmed => 0,
            TxOutConfirmations::AtLeast(number) => number,
        };

        self.send(&RpcRequest::new3(
            JsonRpcVersion::V1,
            "42",
            "listunspent",
            min_confirmations,
            max_confirmations,
            recipients,
        ))
    }

    fn send_raw_transaction(
        &self,
        tx_data: SerializedRawTransaction,
    ) -> Result<Result<TransactionId, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "sendrawtransaction",
            tx_data,
        ))
    }

    fn send_to_address(
        &self,
        address: &Address,
        amount: f64,
    ) -> Result<Result<TransactionId, RpcError>, HTTPError> {
        self.send(&RpcRequest::new2(
            JsonRpcVersion::V1,
            "42",
            "sendtoaddress",
            address,
            amount,
        ))
    }

    fn sign_raw_transaction(
        &self,
        tx: &SerializedRawTransaction,
        dependencies: Option<Vec<&TransactionOutputDetail>>,
        private_keys: Option<Vec<&PrivateKey>>,
        signature_hash_type: Option<SigHashType>,
    ) -> Result<Result<SigningResult, RpcError>, HTTPError> {
        self.send(&RpcRequest::new4(
            JsonRpcVersion::V1,
            "42",
            "signrawtransaction",
            tx,
            dependencies,
            private_keys,
            signature_hash_type,
        ))
    }

    fn validate_address(
        &self,
        address: &Address,
    ) -> Result<Result<AddressValidationResult, RpcError>, HTTPError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "validateaddress",
            address,
        ))
    }
}

impl BitcoinCoreClient {
    fn send<R: DeserializeOwned + Debug, P: Serialize + Debug>(
        &self,
        request: &RpcRequest<P>,
    ) -> Result<Result<R, RpcError>, HTTPError> {
        if let Some(ref config) = self.retry_config {
            for i in 0..config.max_retries {
                let result = self.client.send::<R, P>(request);

                match result {
                    Ok(Err(ref rpc_error)) if rpc_error.code == -28 => {
                        info!("Bitcoind is still starting up. Request will be retried in {} milliseconds. ({}/{}) ", config.interval, i, config.max_retries);

                        ::std::thread::sleep(::std::time::Duration::from_millis(config.interval));
                        continue;
                    }
                    _ => return result,
                }
            }
        }
        self.client.send(request)
    }
}
