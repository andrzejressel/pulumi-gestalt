#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowSourceFlowConfig {
    /// API version that the destination connector uses.
    #[builder(into)]
    #[serde(rename = "apiVersion")]
    pub r#api_version: Option<String>,
    /// Name of the connector profile. This name must be unique for each connector profile in the AWS account.
    #[builder(into)]
    #[serde(rename = "connectorProfileName")]
    pub r#connector_profile_name: Option<String>,
    /// Type of connector, such as Salesforce, Amplitude, and so on. Valid values are `Salesforce`, `Singular`, `Slack`, `Redshift`, `S3`, `Marketo`, `Googleanalytics`, `Zendesk`, `Servicenow`, `Datadog`, `Trendmicro`, `Snowflake`, `Dynatrace`, `Infornexus`, `Amplitude`, `Veeva`, `EventBridge`, `LookoutMetrics`, `Upsolver`, `Honeycode`, `CustomerProfiles`, `SAPOData`, and `CustomConnector`.
    #[builder(into)]
    #[serde(rename = "connectorType")]
    pub r#connector_type: String,
    /// Defines the configuration for a scheduled incremental data pull. If a valid configuration is provided, the fields specified in the configuration are used when querying for the incremental data pull. See Incremental Pull Config for more details.
    #[builder(into)]
    #[serde(rename = "incrementalPullConfig")]
    pub r#incremental_pull_config: Option<Box<super::super::types::appflow::FlowSourceFlowConfigIncrementalPullConfig>>,
    /// Information that is required to query a particular source connector. See Source Connector Properties for details.
    #[builder(into)]
    #[serde(rename = "sourceConnectorProperties")]
    pub r#source_connector_properties: Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorProperties>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowSourceFlowConfig {
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
                "api_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#api_version,
                )
                .await,
            );
            map.insert(
                "connector_profile_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connector_profile_name,
                )
                .await,
            );
            map.insert(
                "connector_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connector_type,
                )
                .await,
            );
            map.insert(
                "incremental_pull_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#incremental_pull_config,
                )
                .await,
            );
            map.insert(
                "source_connector_properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_connector_properties,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowSourceFlowConfig {
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
                    r#api_version: {
                        let field_value = match fields_map.get("api_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connector_profile_name: {
                        let field_value = match fields_map.get("connector_profile_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'connector_profile_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connector_type: {
                        let field_value = match fields_map.get("connector_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'connector_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#incremental_pull_config: {
                        let field_value = match fields_map.get("incremental_pull_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'incremental_pull_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_connector_properties: {
                        let field_value = match fields_map.get("source_connector_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_connector_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
