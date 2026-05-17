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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "additional_endpoints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_endpoints,
                )
                .await,
            );
            map.insert(
                "api_enable_admin".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#api_enable_admin,
                )
                .await,
            );
            map.insert(
                "api_enable_debug".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#api_enable_debug,
                )
                .await,
            );
            map.insert(
                "consensus_client".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#consensus_client,
                )
                .await,
            );
            map.insert(
                "execution_client".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#execution_client,
                )
                .await,
            );
            map.insert(
                "geth_details".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#geth_details,
                )
                .await,
            );
            map.insert(
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
                )
                .await,
            );
            map.insert(
                "node_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_type,
                )
                .await,
            );
            map.insert(
                "validator_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#validator_config,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("additional_endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_enable_admin: {
                        let field_value = match fields_map.get("api_enable_admin") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_enable_admin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_enable_debug: {
                        let field_value = match fields_map.get("api_enable_debug") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_enable_debug' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consensus_client: {
                        let field_value = match fields_map.get("consensus_client") {
                            Some(value) => value,
                            None => bail!("Missing field 'consensus_client' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_client: {
                        let field_value = match fields_map.get("execution_client") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_client' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#geth_details: {
                        let field_value = match fields_map.get("geth_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'geth_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("node_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validator_config: {
                        let field_value = match fields_map.get("validator_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'validator_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
