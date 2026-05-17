#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPolicyMysqlFlexibleServerRetentionRuleCriteria {
    /// Possible values are `AllBackup`, `FirstOfDay`, `FirstOfWeek`, `FirstOfMonth` and `FirstOfYear`. These values mean the first successful backup of the day/week/month/year. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "absoluteCriteria")]
    pub r#absolute_criteria: Option<String>,
    /// Possible values are `Monday`, `Tuesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Option<Vec<String>>,
    /// Possible values are `January`, `February`, `March`, `April`, `May`, `June`, `July`, `August`, `September`, `October`, `November` and `December`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "monthsOfYears")]
    pub r#months_of_years: Option<Vec<String>>,
    /// Specifies a list of backup times for backup in the `RFC3339` format. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "scheduledBackupTimes")]
    pub r#scheduled_backup_times: Option<Vec<String>>,
    /// Possible values are `First`, `Second`, `Third`, `Fourth` and `Last`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "weeksOfMonths")]
    pub r#weeks_of_months: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupPolicyMysqlFlexibleServerRetentionRuleCriteria {
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
                "absolute_criteria".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#absolute_criteria,
                )
                .await,
            );
            map.insert(
                "days_of_weeks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#days_of_weeks,
                )
                .await,
            );
            map.insert(
                "months_of_years".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#months_of_years,
                )
                .await,
            );
            map.insert(
                "scheduled_backup_times".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scheduled_backup_times,
                )
                .await,
            );
            map.insert(
                "weeks_of_months".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weeks_of_months,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackupPolicyMysqlFlexibleServerRetentionRuleCriteria {
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
                    r#absolute_criteria: {
                        let field_value = match fields_map.get("absolute_criteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'absolute_criteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_of_weeks: {
                        let field_value = match fields_map.get("days_of_weeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'days_of_weeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#months_of_years: {
                        let field_value = match fields_map.get("months_of_years") {
                            Some(value) => value,
                            None => bail!("Missing field 'months_of_years' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scheduled_backup_times: {
                        let field_value = match fields_map.get("scheduled_backup_times") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduled_backup_times' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weeks_of_months: {
                        let field_value = match fields_map.get("weeks_of_months") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeks_of_months' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
