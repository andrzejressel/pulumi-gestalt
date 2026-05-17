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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "custom_connector".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_connector,
                )
                .await,
            );
            map.insert(
                "customer_profiles".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#customer_profiles,
                )
                .await,
            );
            map.insert(
                "event_bridge".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event_bridge,
                )
                .await,
            );
            map.insert(
                "honeycode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#honeycode,
                )
                .await,
            );
            map.insert(
                "lookout_metrics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lookout_metrics,
                )
                .await,
            );
            map.insert(
                "marketo".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#marketo,
                )
                .await,
            );
            map.insert(
                "redshift".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redshift,
                )
                .await,
            );
            map.insert(
                "s_3".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3,
                )
                .await,
            );
            map.insert(
                "salesforce".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#salesforce,
                )
                .await,
            );
            map.insert(
                "sapo_data".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sapo_data,
                )
                .await,
            );
            map.insert(
                "snowflake".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snowflake,
                )
                .await,
            );
            map.insert(
                "upsolver".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upsolver,
                )
                .await,
            );
            map.insert(
                "zendesk".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zendesk,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
