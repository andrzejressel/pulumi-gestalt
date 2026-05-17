#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetActionGroupAutomationRunbookReceiver {
    /// The automation account ID which holds this runbook and authenticates to Azure resources.
    #[builder(into)]
    #[serde(rename = "automationAccountId")]
    pub r#automation_account_id: String,
    /// Indicates whether this instance is global runbook.
    #[builder(into)]
    #[serde(rename = "isGlobalRunbook")]
    pub r#is_global_runbook: bool,
    /// Specifies the name of the Action Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name for this runbook.
    #[builder(into)]
    #[serde(rename = "runbookName")]
    pub r#runbook_name: String,
    /// The URI where webhooks should be sent.
    #[builder(into)]
    #[serde(rename = "serviceUri")]
    pub r#service_uri: String,
    /// Indicates whether to use common alert schema.
    #[builder(into)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: bool,
    /// The resource id for webhook linked to this runbook.
    #[builder(into)]
    #[serde(rename = "webhookResourceId")]
    pub r#webhook_resource_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetActionGroupAutomationRunbookReceiver {
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
                "automation_account_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#automation_account_id,
                )
                .await,
            );
            map.insert(
                "is_global_runbook".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_global_runbook,
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
                "runbook_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#runbook_name,
                )
                .await,
            );
            map.insert(
                "service_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_uri,
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
            map.insert(
                "webhook_resource_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#webhook_resource_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetActionGroupAutomationRunbookReceiver {
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
                    r#automation_account_id: {
                        let field_value = match fields_map.get("automation_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'automation_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_global_runbook: {
                        let field_value = match fields_map.get("is_global_runbook") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_global_runbook' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#runbook_name: {
                        let field_value = match fields_map.get("runbook_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'runbook_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_uri: {
                        let field_value = match fields_map.get("service_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#webhook_resource_id: {
                        let field_value = match fields_map.get("webhook_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'webhook_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
