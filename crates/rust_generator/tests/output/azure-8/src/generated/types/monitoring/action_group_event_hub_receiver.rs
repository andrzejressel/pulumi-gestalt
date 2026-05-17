#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActionGroupEventHubReceiver {
    /// The name of the specific Event Hub queue.
    #[builder(into)]
    #[serde(rename = "eventHubName")]
    pub r#event_hub_name: String,
    /// The namespace name of the Event Hub.
    #[builder(into)]
    #[serde(rename = "eventHubNamespace")]
    pub r#event_hub_namespace: String,
    /// The name of the EventHub Receiver, must be unique within action group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The ID for the subscription containing this Event Hub. Default to the subscription ID of the Action Group.
    #[builder(into)]
    #[serde(rename = "subscriptionId")]
    pub r#subscription_id: Option<String>,
    /// The Tenant ID for the subscription containing this Event Hub.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
    /// Indicates whether to use common alert schema.
    #[builder(into)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ActionGroupEventHubReceiver {
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
                "event_hub_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event_hub_name,
                )
                .await,
            );
            map.insert(
                "event_hub_namespace".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event_hub_namespace,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "subscription_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subscription_id,
                )
                .await,
            );
            map.insert(
                "tenant_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tenant_id,
                )
                .await,
            );
            map.insert(
                "use_common_alert_schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_common_alert_schema,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ActionGroupEventHubReceiver {
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
                    r#event_hub_name: {
                        let field_value = match fields_map.get("event_hub_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_hub_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_hub_namespace: {
                        let field_value = match fields_map.get("event_hub_namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_hub_namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subscription_id: {
                        let field_value = match fields_map.get("subscription_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subscription_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tenant_id: {
                        let field_value = match fields_map.get("tenant_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenant_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_common_alert_schema: {
                        let field_value = match fields_map.get("use_common_alert_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_common_alert_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
