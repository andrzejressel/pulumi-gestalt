#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActionGroupAutomationRunbookReceiver {
    /// The automation account ID which holds this runbook and authenticates to Azure resources.
    #[builder(into)]
    pub r#automation_account_id: String,
    /// Indicates whether this instance is global runbook.
    #[builder(into)]
    pub r#is_global_runbook: bool,
    /// The name of the automation runbook receiver.
    #[builder(into)]
    pub r#name: String,
    /// The name for this runbook.
    #[builder(into)]
    pub r#runbook_name: String,
    /// The URI where webhooks should be sent.
    #[builder(into)]
    pub r#service_uri: String,
    /// Enables or disables the common alert schema.
    #[builder(into)]
    pub r#use_common_alert_schema: Option<bool>,
    /// The resource id for webhook linked to this runbook.
    #[builder(into)]
    pub r#webhook_resource_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ActionGroupAutomationRunbookReceiver {
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
                    "automationAccountId",
                    &self.r#automation_account_id,
                ),
                to_pulumi_object_field(
                    "isGlobalRunbook",
                    &self.r#is_global_runbook,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "runbookName",
                    &self.r#runbook_name,
                ),
                to_pulumi_object_field(
                    "serviceUri",
                    &self.r#service_uri,
                ),
                to_pulumi_object_field(
                    "useCommonAlertSchema",
                    &self.r#use_common_alert_schema,
                ),
                to_pulumi_object_field(
                    "webhookResourceId",
                    &self.r#webhook_resource_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("automationAccountId") {
                            Some(value) => value,
                            None => bail!("Missing field 'automationAccountId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_global_runbook: {
                        let field_value = match fields_map.get("isGlobalRunbook") {
                            Some(value) => value,
                            None => bail!("Missing field 'isGlobalRunbook' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("runbookName") {
                            Some(value) => value,
                            None => bail!("Missing field 'runbookName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_uri: {
                        let field_value = match fields_map.get("serviceUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_common_alert_schema: {
                        let field_value = match fields_map.get("useCommonAlertSchema") {
                            Some(value) => value,
                            None => bail!("Missing field 'useCommonAlertSchema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#webhook_resource_id: {
                        let field_value = match fields_map.get("webhookResourceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'webhookResourceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
