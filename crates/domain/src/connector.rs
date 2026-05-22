use crate::{FieldName, PulumiValue, ResourceFields};
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
    pub object: HashMap<FieldName, PulumiValue>,
    pub version: String,
    pub provider: Option<String>,
}

#[derive(Builder)]
pub struct RegisterResourceResult {
    pub urn: PulumiValue,
    pub id: PulumiValue,
    pub fields: ResourceFields,
}

#[derive(Builder)]
pub struct ResourceInvokeRequest {
    pub object: HashMap<FieldName, PulumiValue>,
    pub version: String,
    pub token: String,
}

#[derive(Builder)]
pub struct ResourceInvokeResult {
    pub fields: ResourceFields,
}

#[derive(Builder, Debug, PartialEq)]
pub struct RegisterOutputsRequest {
    pub outputs: HashMap<FieldName, PulumiValue>,
}

impl RegisterOutputsRequest {
    pub fn new(outputs: HashMap<FieldName, PulumiValue>) -> Self {
        Self { outputs }
    }
}
