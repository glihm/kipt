use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::types::FieldElement,
    providers::{
        jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider, SequencerGatewayProvider,
    },
    signers::{LocalWallet, SigningKey},
};
use url::Url;

use crate::error::{Error, KiptResult};

/// Setups a provider from the provided information.
///
/// # Arguments
///
/// * `url_network` - The RPC URL or network name. If it starts with "http", the
///   JSON RPC provider is used, else the gateway network if inferred.
pub async fn setup_provider(url_network: &str) -> KiptResult<AnyProvider> {
    let provider = if url_network.starts_with("http") {
        provider_from_url(url_network)?
    } else {
        provider_from_network(url_network)?
    };

    Ok(provider)
}

/// Setups an account from the given information.
///
/// # Arguments
///
/// * `url` - The RPC URL or network name. If it starts with "http", the
///   JSON RPC provider is used, else the gateway network if inferred.
/// * `account_address` - Address of the deployed account.
/// * `account_privkey` - Private key associated with the account to sign transactions.
pub async fn setup_account(
    url_network: &str,
    account_address: &str,
    account_privkey: &str,
) -> KiptResult<SingleOwnerAccount<AnyProvider, LocalWallet>> {
    let provider = if url_network.starts_with("http") {
        provider_from_url(url_network)?
    } else {
        provider_from_network(url_network)?
    };

    let chain_id = provider.chain_id().await?;

    let addr = FieldElement::from_hex_be(account_address)?;
    let key = FieldElement::from_hex_be(account_privkey)?;

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(key));

    let account =
        SingleOwnerAccount::new(provider, signer, addr, chain_id, ExecutionEncoding::Legacy);

    Ok(account)
}

/// Builds a provider from the given URL.
///
/// # Arguments
///
/// * `url` - The RPC URL to use for the provider.
fn provider_from_url(url: &str) -> KiptResult<AnyProvider> {
    let rpc_url =
        Url::parse(url).map_err(|_| Error::Other(format!("URL can't be parsed: {}", url)))?;

    Ok(AnyProvider::JsonRpcHttp(JsonRpcClient::new(
        HttpTransport::new(rpc_url),
    )))
}

/// Provider from network name.
///
/// # Arguments
///
/// * `network` - Name of the network to be resolved.
fn provider_from_network(network: &str) -> KiptResult<AnyProvider> {
    match network {
        "MAINNET" => Ok(AnyProvider::SequencerGateway(
            SequencerGatewayProvider::starknet_alpha_mainnet(),
        )),
        "GOERLI-1" => Ok(AnyProvider::SequencerGateway(
            SequencerGatewayProvider::starknet_alpha_goerli(),
        )),
        "GOERLI-2" => Ok(AnyProvider::SequencerGateway(
            SequencerGatewayProvider::starknet_alpha_goerli_2(),
        )),
        _ => Err(Error::Other(format!("Invalid network: {}", network))),
    }
}
