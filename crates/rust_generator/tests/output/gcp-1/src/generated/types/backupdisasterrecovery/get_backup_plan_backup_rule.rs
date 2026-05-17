#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackupPlanBackupRule {
    /// Configures the duration for which backup data will be kept. The value should be greater than or equal to minimum enforced retention of the backup vault.
    #[builder(into)]
    #[serde(rename = "backupRetentionDays")]
    pub r#backup_retention_days: i32,
    /// The unique ID of this 'BackupRule'. The 'rule_id' is unique per 'BackupPlan'.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: String,
    /// StandardSchedule defines a schedule that runs within the confines of a defined window of days.
    #[builder(into)]
    #[serde(rename = "standardSchedules")]
    pub r#standard_schedules: Vec<super::super::types::backupdisasterrecovery::GetBackupPlanBackupRuleStandardSchedule>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBackupPlanBackupRule {
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
                "backup_retention_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup_retention_days,
                )
                .await,
            );
            map.insert(
                "rule_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_id,
                )
                .await,
            );
            map.insert(
                "standard_schedules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#standard_schedules,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBackupPlanBackupRule {
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
                    r#backup_retention_days: {
                        let field_value = match fields_map.get("backup_retention_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_retention_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_id: {
                        let field_value = match fields_map.get("rule_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#standard_schedules: {
                        let field_value = match fields_map.get("standard_schedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'standard_schedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
