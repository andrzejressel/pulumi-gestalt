use crate::{FieldName, NodeValue, ResourceFields};
use anyhow;
use async_trait::async_trait;
use bon::Builder;
use std::collections::HashMap;

#[cfg(feature = "test-utils")]
use mockall;

#[cfg_attr(feature = "test-utils", mockall::automock)]
#[async_trait]
pub trait PulumiConnector: Send + Sync {
    async fn register_resource(&self, req: RegisterResourceRequest) -> RegisterResourceResult;
    async fn resource_invoke(&self, req: ResourceInvokeRequest) -> ResourceInvokeResult;
    async fn register_outputs(&self, req: RegisterOutputsRequest) -> ();
    async fn require_pulumi_version(&self, version_range: &str) -> anyhow::Result<()>;
}

#[derive(Builder)]
pub struct RegisterResourceRequest {
    pub name: String,
    pub r#type: String,
    pub object: HashMap<FieldName, NodeValue>,
    pub version: String,
    pub provider: Option<String>,
}

#[derive(Builder)]
pub struct RegisterResourceResult {
    pub urn: NodeValue,
    pub id: NodeValue,
    pub fields: ResourceFields,
}

#[derive(Builder)]
pub struct ResourceInvokeRequest {
    pub object: HashMap<FieldName, NodeValue>,
    pub version: String,
    pub token: String,
}

#[derive(Builder)]
pub struct ResourceInvokeResult {
    pub fields: ResourceFields,
}

#[derive(Builder, Debug, PartialEq)]
pub struct RegisterOutputsRequest {
    pub outputs: HashMap<FieldName, NodeValue>,
}

impl RegisterOutputsRequest {
    pub fn new(outputs: HashMap<FieldName, NodeValue>) -> Self {
        Self { outputs }
    }
}
