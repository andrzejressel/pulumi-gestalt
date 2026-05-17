#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActionGroupAutomationRunbookReceiver {
    /// The automation account ID which holds this runbook and authenticates to Azure resources.
    #[builder(into)]
    #[serde(rename = "automationAccountId")]
    pub r#automation_account_id: String,
    /// Indicates whether this instance is global runbook.
    #[builder(into)]
    #[serde(rename = "isGlobalRunbook")]
    pub r#is_global_runbook: bool,
    /// The name of the automation runbook receiver.
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
    /// Enables or disables the common alert schema.
    #[builder(into)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Option<bool>,
    /// The resource id for webhook linked to this runbook.
    #[builder(into)]
    #[serde(rename = "webhookResourceId")]
    pub r#webhook_resource_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ActionGroupAutomationRunbookReceiver {
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
                    "automation_account_id",
                    &self.r#automation_account_id,
                ),
                to_pulumi_object_field(
                    "is_global_runbook",
                    &self.r#is_global_runbook,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "runbook_name",
                    &self.r#runbook_name,
                ),
                to_pulumi_object_field(
                    "service_uri",
                    &self.r#service_uri,
                ),
                to_pulumi_object_field(
                    "use_common_alert_schema",
                    &self.r#use_common_alert_schema,
                ),
                to_pulumi_object_field(
                    "webhook_resource_id",
                    &self.r#webhook_resource_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ActionGroupAutomationRunbookReceiver {
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
