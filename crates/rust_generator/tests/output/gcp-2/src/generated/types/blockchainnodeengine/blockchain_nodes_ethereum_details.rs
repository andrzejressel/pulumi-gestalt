#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BlockchainNodesEthereumDetails {
    /// (Output)
    /// User-provided key-value pairs
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "additionalEndpoints")]
    pub r#additional_endpoints: Option<Vec<super::super::types::blockchainnodeengine::BlockchainNodesEthereumDetailsAdditionalEndpoint>>,
    /// Enables JSON-RPC access to functions in the admin namespace. Defaults to false.
    #[builder(into)]
    #[serde(rename = "apiEnableAdmin")]
    pub r#api_enable_admin: Option<bool>,
    /// Enables JSON-RPC access to functions in the debug namespace. Defaults to false.
    #[builder(into)]
    #[serde(rename = "apiEnableDebug")]
    pub r#api_enable_debug: Option<bool>,
    /// The consensus client
    /// Possible values are: `CONSENSUS_CLIENT_UNSPECIFIED`, `LIGHTHOUSE`.
    #[builder(into)]
    #[serde(rename = "consensusClient")]
    pub r#consensus_client: Option<String>,
    /// The execution client
    /// Possible values are: `EXECUTION_CLIENT_UNSPECIFIED`, `GETH`, `ERIGON`.
    #[builder(into)]
    #[serde(rename = "executionClient")]
    pub r#execution_client: Option<String>,
    /// User-provided key-value pairs
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gethDetails")]
    pub r#geth_details: Option<Box<super::super::types::blockchainnodeengine::BlockchainNodesEthereumDetailsGethDetails>>,
    /// The Ethereum environment being accessed.
    /// Possible values are: `MAINNET`, `TESTNET_GOERLI_PRATER`, `TESTNET_SEPOLIA`.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// The type of Ethereum node.
    /// Possible values are: `LIGHT`, `FULL`, `ARCHIVE`.
    #[builder(into)]
    #[serde(rename = "nodeType")]
    pub r#node_type: Option<String>,
    /// Configuration for validator-related parameters on the beacon client, and for any managed validator client.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "validatorConfig")]
    pub r#validator_config: Option<Box<super::super::types::blockchainnodeengine::BlockchainNodesEthereumDetailsValidatorConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BlockchainNodesEthereumDetails {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "additionalEndpoints",
                    &self.r#additional_endpoints,
                ),
                to_pulumi_object_field(
                    "apiEnableAdmin",
                    &self.r#api_enable_admin,
                ),
                to_pulumi_object_field(
                    "apiEnableDebug",
                    &self.r#api_enable_debug,
                ),
                to_pulumi_object_field(
                    "consensusClient",
                    &self.r#consensus_client,
                ),
                to_pulumi_object_field(
                    "executionClient",
                    &self.r#execution_client,
                ),
                to_pulumi_object_field(
                    "gethDetails",
                    &self.r#geth_details,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "nodeType",
                    &self.r#node_type,
                ),
                to_pulumi_object_field(
                    "validatorConfig",
                    &self.r#validator_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BlockchainNodesEthereumDetails {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#additional_endpoints: {
                        let field_value = match fields_map.get("additionalEndpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalEndpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_enable_admin: {
                        let field_value = match fields_map.get("apiEnableAdmin") {
                            Some(value) => value,
                            None => bail!("Missing field 'apiEnableAdmin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_enable_debug: {
                        let field_value = match fields_map.get("apiEnableDebug") {
                            Some(value) => value,
                            None => bail!("Missing field 'apiEnableDebug' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consensus_client: {
                        let field_value = match fields_map.get("consensusClient") {
                            Some(value) => value,
                            None => bail!("Missing field 'consensusClient' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_client: {
                        let field_value = match fields_map.get("executionClient") {
                            Some(value) => value,
                            None => bail!("Missing field 'executionClient' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#geth_details: {
                        let field_value = match fields_map.get("gethDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'gethDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_type: {
                        let field_value = match fields_map.get("nodeType") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validator_config: {
                        let field_value = match fields_map.get("validatorConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'validatorConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
