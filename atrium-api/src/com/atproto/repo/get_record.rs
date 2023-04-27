// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.repo.getRecord` namespace.

/// Get a record.
#[async_trait::async_trait]
pub trait GetRecord: crate::xrpc::XrpcClient {
    async fn get_record(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "com.atproto.repo.getRecord",
            Some(params),
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    /// The CID of the version of the record. If not specified, then return the most recent version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    /// The NSID of the record collection.
    pub collection: String,
    /// The handle or DID of the repo.
    pub repo: String,
    /// The key of the record.
    pub rkey: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    pub uri: String,
    pub value: crate::records::Record,
}

pub enum Error {
}