use base64;
use bitcoin::Address;
use bitcoin::Script;
use jsonrpc_client::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    ClientError, HTTPClient, JsonRpcVersion, RpcClient, RpcError, RpcRequest,
};
use rpc;
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Debug;
use types::address::AddressInfoResult;
use BitcoinRpcApi;
use BlockHash;
use TransactionId;

struct RetryConfig {
    max_retries: u32,
    interval: u64,
}

pub struct BitcoinCoreClient {
    client: RpcClient,
    retry_config: Option<RetryConfig>,
}

#[allow(dead_code)]
impl BitcoinCoreClient {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Basic {}",
                base64::encode(&format!("{}:{}", username, password))
            ))
            .unwrap(),
        );

        let client = HTTPClient::builder()
            .default_headers(headers)
            .build()
            .expect("unable to create HTTP client");

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
    ) -> Result<Result<R, RpcError>, ClientError>
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
    ) -> Result<Result<rpc::MultiSigAddress, RpcError>, ClientError> {
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
        inputs: Vec<&rpc::NewTransactionInput>,
        output: &rpc::NewTransactionOutput,
    ) -> Result<Result<rpc::SerializedRawTransaction, RpcError>, ClientError> {
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
        tx: rpc::SerializedRawTransaction,
    ) -> Result<Result<rpc::DecodedRawTransaction, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "decoderawtransaction",
            tx,
        ))
    }

    fn decode_script(
        &self,
        script: Script,
    ) -> Result<Result<rpc::DecodedScript, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "decodescript",
            script,
        ))
    }

    fn dump_privkey(
        &self,
        address: &Address,
    ) -> Result<Result<rpc::PrivateKey, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "dumpprivkey",
            address,
        ))
    }

    fn fund_raw_transaction(
        &self,
        tx: &rpc::SerializedRawTransaction,
        options: &rpc::FundingOptions,
    ) -> Result<Result<rpc::FundingResult, RpcError>, ClientError> {
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
    ) -> Result<Result<Vec<BlockHash>, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "generate",
            number_of_blocks,
        ))
    }

    fn generate_to_address(
        &self,
        number_of_blocks: u32,
        address: &Address,
    ) -> Result<Result<Vec<BlockHash>, RpcError>, ClientError> {
        self.send(&RpcRequest::new2(
            JsonRpcVersion::V1,
            "42",
            "generatetoaddress",
            number_of_blocks,
            address,
        ))
    }

    fn get_address_info(
        &self,
        address: &Address,
    ) -> Result<Result<AddressInfoResult, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "getaddressinfo",
            address,
        ))
    }

    fn get_balance(&self) -> Result<Result<f32, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(JsonRpcVersion::V1, "42", "getbalance"))
    }

    fn get_best_block_hash(&self) -> Result<Result<BlockHash, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "42",
            "getbestblockhash",
        ))
    }

    fn get_block(
        &self,
        header_hash: &BlockHash,
    ) -> Result<Result<rpc::Block<TransactionId>, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "getblock",
            header_hash,
        ))
    }

    fn get_block_verbose(
        &self,
        header_hash: &BlockHash,
    ) -> Result<Result<rpc::Block<rpc::DecodedRawTransaction>, RpcError>, ClientError> {
        self.send(&RpcRequest::new2(
            JsonRpcVersion::V1,
            "42",
            "getblock",
            header_hash,
            2,
        ))
    }

    fn get_blockchain_info(&self) -> Result<Result<rpc::BlockchainInfo, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "42",
            "getblockchaininfo",
        ))
    }

    fn get_block_count(&self) -> Result<Result<rpc::BlockHeight, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(JsonRpcVersion::V1, "42", "getblockcount"))
    }

    fn get_block_hash(&self, height: u32) -> Result<Result<BlockHash, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "42",
            "getblockhash",
            height,
        ))
    }

    fn get_new_address(&self) -> Result<Result<Address, RpcError>, ClientError> {
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
    ) -> Result<Result<rpc::SerializedRawTransaction, RpcError>, ClientError> {
        self.get_raw_transaction(tx, false)
    }

    fn get_raw_transaction_verbose(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<rpc::VerboseRawTransaction, RpcError>, ClientError> {
        self.get_raw_transaction(tx, true)
    }

    fn list_unspent(
        &self,
        min_confirmations: rpc::TxOutConfirmations,
        max_confirmations: Option<u32>,
        recipients: Option<Vec<Address>>,
    ) -> Result<Result<Vec<rpc::UnspentTransactionOutput>, RpcError>, ClientError> {
        use rpc::TxOutConfirmations::*;

        let min_confirmations = match min_confirmations {
            Unconfirmed => 0,
            AtLeast(number) => number,
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
        tx_data: rpc::SerializedRawTransaction,
    ) -> Result<Result<TransactionId, RpcError>, ClientError> {
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
    ) -> Result<Result<TransactionId, RpcError>, ClientError> {
        self.send(&RpcRequest::new2(
            JsonRpcVersion::V1,
            "42",
            "sendtoaddress",
            address,
            amount,
        ))
    }

    fn sign_raw_transaction_with_key(
        &self,
        tx: &rpc::SerializedRawTransaction,
        private_keys: Option<Vec<&rpc::PrivateKey>>,
        dependencies: Option<Vec<&rpc::TransactionOutputDetail>>,
        signature_hash_type: Option<rpc::SigHashType>,
    ) -> Result<Result<rpc::SigningResult, RpcError>, ClientError> {
        self.send(&RpcRequest::new4(
            JsonRpcVersion::V1,
            "42",
            "signrawtransactionwithkey",
            tx,
            private_keys,
            dependencies,
            signature_hash_type,
        ))
    }

    fn validate_address(
        &self,
        address: &Address,
    ) -> Result<Result<rpc::AddressValidationResult, RpcError>, ClientError> {
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
    ) -> Result<Result<R, RpcError>, ClientError> {
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
