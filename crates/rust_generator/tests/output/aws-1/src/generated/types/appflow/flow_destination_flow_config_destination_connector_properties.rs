#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDestinationFlowConfigDestinationConnectorProperties {
    /// Properties that are required to query the custom Connector. See Custom Connector Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "customConnector")]
    pub r#custom_connector: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector>>,
    /// Properties that are required to query Amazon Connect Customer Profiles. See Customer Profiles Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "customerProfiles")]
    pub r#customer_profiles: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles>>,
    /// Properties that are required to query Amazon EventBridge. See Generic Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "eventBridge")]
    pub r#event_bridge: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridge>>,
    /// Properties that are required to query Amazon Honeycode. See Generic Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "honeycode")]
    pub r#honeycode: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycode>>,
    #[builder(into)]
    #[serde(rename = "lookoutMetrics")]
    pub r#lookout_metrics: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesLookoutMetrics>>,
    /// Properties that are required to query Marketo. See Generic Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "marketo")]
    pub r#marketo: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesMarketo>>,
    /// Properties that are required to query Amazon Redshift. See Redshift Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "redshift")]
    pub r#redshift: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesRedshift>>,
    /// Properties that are required to query Amazon S3. See S3 Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesS3>>,
    /// Properties that are required to query Salesforce. See Salesforce Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "salesforce")]
    pub r#salesforce: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforce>>,
    /// Properties that are required to query SAPOData. See SAPOData Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "sapoData")]
    pub r#sapo_data: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSapoData>>,
    /// Properties that are required to query Snowflake. See Snowflake Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "snowflake")]
    pub r#snowflake: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake>>,
    /// Properties that are required to query Upsolver. See Upsolver Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "upsolver")]
    pub r#upsolver: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolver>>,
    /// Properties that are required to query Zendesk. See Zendesk Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "zendesk")]
    pub r#zendesk: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowDestinationFlowConfigDestinationConnectorProperties {
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
                    "custom_connector",
                    &self.r#custom_connector,
                ),
                to_pulumi_object_field(
                    "customer_profiles",
                    &self.r#customer_profiles,
                ),
                to_pulumi_object_field(
                    "event_bridge",
                    &self.r#event_bridge,
                ),
                to_pulumi_object_field(
                    "honeycode",
                    &self.r#honeycode,
                ),
                to_pulumi_object_field(
                    "lookout_metrics",
                    &self.r#lookout_metrics,
                ),
                to_pulumi_object_field(
                    "marketo",
                    &self.r#marketo,
                ),
                to_pulumi_object_field(
                    "redshift",
                    &self.r#redshift,
                ),
                to_pulumi_object_field(
                    "s_3",
                    &self.r#s_3,
                ),
                to_pulumi_object_field(
                    "salesforce",
                    &self.r#salesforce,
                ),
                to_pulumi_object_field(
                    "sapo_data",
                    &self.r#sapo_data,
                ),
                to_pulumi_object_field(
                    "snowflake",
                    &self.r#snowflake,
                ),
                to_pulumi_object_field(
                    "upsolver",
                    &self.r#upsolver,
                ),
                to_pulumi_object_field(
                    "zendesk",
                    &self.r#zendesk,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowDestinationFlowConfigDestinationConnectorProperties {
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
                    r#custom_connector: {
                        let field_value = match fields_map.get("custom_connector") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_connector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_profiles: {
                        let field_value = match fields_map.get("customer_profiles") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_profiles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_bridge: {
                        let field_value = match fields_map.get("event_bridge") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_bridge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#honeycode: {
                        let field_value = match fields_map.get("honeycode") {
                            Some(value) => value,
                            None => bail!("Missing field 'honeycode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lookout_metrics: {
                        let field_value = match fields_map.get("lookout_metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'lookout_metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#marketo: {
                        let field_value = match fields_map.get("marketo") {
                            Some(value) => value,
                            None => bail!("Missing field 'marketo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redshift: {
                        let field_value = match fields_map.get("redshift") {
                            Some(value) => value,
                            None => bail!("Missing field 'redshift' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3: {
                        let field_value = match fields_map.get("s_3") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#salesforce: {
                        let field_value = match fields_map.get("salesforce") {
                            Some(value) => value,
                            None => bail!("Missing field 'salesforce' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sapo_data: {
                        let field_value = match fields_map.get("sapo_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'sapo_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snowflake: {
                        let field_value = match fields_map.get("snowflake") {
                            Some(value) => value,
                            None => bail!("Missing field 'snowflake' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upsolver: {
                        let field_value = match fields_map.get("upsolver") {
                            Some(value) => value,
                            None => bail!("Missing field 'upsolver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zendesk: {
                        let field_value = match fields_map.get("zendesk") {
                            Some(value) => value,
                            None => bail!("Missing field 'zendesk' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
