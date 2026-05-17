#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDestinationFlowConfig {
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
    /// This stores the information that is required to query a particular connector. See Destination Connector Properties for more information.
    #[builder(into)]
    #[serde(rename = "destinationConnectorProperties")]
    pub r#destination_connector_properties: Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorProperties>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowDestinationFlowConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "api_version",
                    &self.r#api_version,
                ),
                to_pulumi_object_field(
                    "connector_profile_name",
                    &self.r#connector_profile_name,
                ),
                to_pulumi_object_field(
                    "connector_type",
                    &self.r#connector_type,
                ),
                to_pulumi_object_field(
                    "destination_connector_properties",
                    &self.r#destination_connector_properties,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowDestinationFlowConfig {
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
                    r#destination_connector_properties: {
                        let field_value = match fields_map.get("destination_connector_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_connector_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
