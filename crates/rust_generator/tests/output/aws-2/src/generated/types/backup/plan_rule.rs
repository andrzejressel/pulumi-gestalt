#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PlanRule {
    /// The amount of time in minutes AWS Backup attempts a backup before canceling the job and returning an error.
    #[builder(into)]
    #[serde(rename = "completionWindow")]
    pub r#completion_window: Option<i32>,
    /// Configuration block(s) with copy operation settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "copyActions")]
    pub r#copy_actions: Option<Vec<super::super::types::backup::PlanRuleCopyAction>>,
    /// Enable continuous backups for supported resources.
    #[builder(into)]
    #[serde(rename = "enableContinuousBackup")]
    pub r#enable_continuous_backup: Option<bool>,
    /// The lifecycle defines when a protected resource is transitioned to cold storage and when it expires.  Fields documented below.
    #[builder(into)]
    #[serde(rename = "lifecycle")]
    pub r#lifecycle: Option<Box<super::super::types::backup::PlanRuleLifecycle>>,
    /// Metadata that you can assign to help organize the resources that you create.
    #[builder(into)]
    #[serde(rename = "recoveryPointTags")]
    pub r#recovery_point_tags: Option<std::collections::HashMap<String, String>>,
    /// An display name for a backup rule.
    #[builder(into)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: String,
    /// A CRON expression specifying when AWS Backup initiates a backup job.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<String>,
    /// The timezone in which the schedule expression is set. Default value: `"Etc/UTC"`.
    #[builder(into)]
    #[serde(rename = "scheduleExpressionTimezone")]
    pub r#schedule_expression_timezone: Option<String>,
    /// The amount of time in minutes before beginning a backup.
    #[builder(into)]
    #[serde(rename = "startWindow")]
    pub r#start_window: Option<i32>,
    /// The name of a logical container where backups are stored.
    #[builder(into)]
    #[serde(rename = "targetVaultName")]
    pub r#target_vault_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PlanRule {
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
                    "completion_window",
                    &self.r#completion_window,
                ),
                to_pulumi_object_field(
                    "copy_actions",
                    &self.r#copy_actions,
                ),
                to_pulumi_object_field(
                    "enable_continuous_backup",
                    &self.r#enable_continuous_backup,
                ),
                to_pulumi_object_field(
                    "lifecycle",
                    &self.r#lifecycle,
                ),
                to_pulumi_object_field(
                    "recovery_point_tags",
                    &self.r#recovery_point_tags,
                ),
                to_pulumi_object_field(
                    "rule_name",
                    &self.r#rule_name,
                ),
                to_pulumi_object_field(
                    "schedule",
                    &self.r#schedule,
                ),
                to_pulumi_object_field(
                    "schedule_expression_timezone",
                    &self.r#schedule_expression_timezone,
                ),
                to_pulumi_object_field(
                    "start_window",
                    &self.r#start_window,
                ),
                to_pulumi_object_field(
                    "target_vault_name",
                    &self.r#target_vault_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PlanRule {
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
                        let field_value = match fields_map.get("completion_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'completion_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#copy_actions: {
                        let field_value = match fields_map.get("copy_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_continuous_backup: {
                        let field_value = match fields_map.get("enable_continuous_backup") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_continuous_backup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle: {
                        let field_value = match fields_map.get("lifecycle") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_point_tags: {
                        let field_value = match fields_map.get("recovery_point_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_point_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_name: {
                        let field_value = match fields_map.get("rule_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("schedule_expression_timezone") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_expression_timezone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_window: {
                        let field_value = match fields_map.get("start_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_vault_name: {
                        let field_value = match fields_map.get("target_vault_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_vault_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
