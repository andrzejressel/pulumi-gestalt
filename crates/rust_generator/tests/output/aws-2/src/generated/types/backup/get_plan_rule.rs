#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPlanRule {
    #[builder(into)]
    pub r#completion_window: i32,
    #[builder(into)]
    pub r#copy_actions: Vec<super::super::types::backup::GetPlanRuleCopyAction>,
    #[builder(into)]
    pub r#enable_continuous_backup: bool,
    #[builder(into)]
    pub r#lifecycles: Vec<super::super::types::backup::GetPlanRuleLifecycle>,
    #[builder(into)]
    pub r#recovery_point_tags: Option<std::collections::BTreeMap<String, String>>,
    #[builder(into)]
    pub r#rule_name: String,
    #[builder(into)]
    pub r#schedule: String,
    #[builder(into)]
    pub r#schedule_expression_timezone: String,
    #[builder(into)]
    pub r#start_window: i32,
    #[builder(into)]
    pub r#target_vault_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPlanRule {
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
                    "completionWindow",
                    &self.r#completion_window,
                ),
                to_pulumi_object_field(
                    "copyActions",
                    &self.r#copy_actions,
                ),
                to_pulumi_object_field(
                    "enableContinuousBackup",
                    &self.r#enable_continuous_backup,
                ),
                to_pulumi_object_field(
                    "lifecycles",
                    &self.r#lifecycles,
                ),
                to_pulumi_object_field(
                    "recoveryPointTags",
                    &self.r#recovery_point_tags,
                ),
                to_pulumi_object_field(
                    "ruleName",
                    &self.r#rule_name,
                ),
                to_pulumi_object_field(
                    "schedule",
                    &self.r#schedule,
                ),
                to_pulumi_object_field(
                    "scheduleExpressionTimezone",
                    &self.r#schedule_expression_timezone,
                ),
                to_pulumi_object_field(
                    "startWindow",
                    &self.r#start_window,
                ),
                to_pulumi_object_field(
                    "targetVaultName",
                    &self.r#target_vault_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPlanRule {
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
                    r#completion_window: {
                        let field_value = match fields_map.get("completionWindow") {
                            Some(value) => value,
                            None => bail!("Missing field 'completionWindow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#copy_actions: {
                        let field_value = match fields_map.get("copyActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'copyActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_continuous_backup: {
                        let field_value = match fields_map.get("enableContinuousBackup") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableContinuousBackup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycles: {
                        let field_value = match fields_map.get("lifecycles") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_point_tags: {
                        let field_value = match fields_map.get("recoveryPointTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoveryPointTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_name: {
                        let field_value = match fields_map.get("ruleName") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruleName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule: {
                        let field_value = match fields_map.get("schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule_expression_timezone: {
                        let field_value = match fields_map.get("scheduleExpressionTimezone") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduleExpressionTimezone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_window: {
                        let field_value = match fields_map.get("startWindow") {
                            Some(value) => value,
                            None => bail!("Missing field 'startWindow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_vault_name: {
                        let field_value = match fields_map.get("targetVaultName") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetVaultName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
