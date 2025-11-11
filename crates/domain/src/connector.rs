use crate::{FieldName, NodeValue, ResourceFields};
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
}

#[derive(Builder)]
pub struct RegisterResourceRequest {
    pub name: String,
    pub r#type: String,
    pub object: HashMap<FieldName, NodeValue>,
    pub version: String,
}

#[derive(Builder)]
pub struct RegisterResourceResult {
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
    outputs: HashMap<FieldName, NodeValue>,
}

impl RegisterOutputsRequest {
    pub fn new(outputs: HashMap<FieldName, NodeValue>) -> Self {
        Self { outputs }
    }
}
