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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("completion_window".to_string(), self.r#completion_window.to_pulumi_value().await);
            map.insert("copy_actions".to_string(), self.r#copy_actions.to_pulumi_value().await);
            map.insert("enable_continuous_backup".to_string(), self.r#enable_continuous_backup.to_pulumi_value().await);
            map.insert("lifecycle".to_string(), self.r#lifecycle.to_pulumi_value().await);
            map.insert("recovery_point_tags".to_string(), self.r#recovery_point_tags.to_pulumi_value().await);
            map.insert("rule_name".to_string(), self.r#rule_name.to_pulumi_value().await);
            map.insert("schedule".to_string(), self.r#schedule.to_pulumi_value().await);
            map.insert("schedule_expression_timezone".to_string(), self.r#schedule_expression_timezone.to_pulumi_value().await);
            map.insert("start_window".to_string(), self.r#start_window.to_pulumi_value().await);
            map.insert("target_vault_name".to_string(), self.r#target_vault_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PlanRule {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#completion_window: {
                        let field_value = match fields_map.get("completion_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'completion_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#copy_actions: {
                        let field_value = match fields_map.get("copy_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::backup::PlanRuleCopyAction>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_continuous_backup: {
                        let field_value = match fields_map.get("enable_continuous_backup") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_continuous_backup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lifecycle: {
                        let field_value = match fields_map.get("lifecycle") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::backup::PlanRuleLifecycle>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#recovery_point_tags: {
                        let field_value = match fields_map.get("recovery_point_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_point_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rule_name: {
                        let field_value = match fields_map.get("rule_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schedule: {
                        let field_value = match fields_map.get("schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schedule_expression_timezone: {
                        let field_value = match fields_map.get("schedule_expression_timezone") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_expression_timezone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#start_window: {
                        let field_value = match fields_map.get("start_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target_vault_name: {
                        let field_value = match fields_map.get("target_vault_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_vault_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
