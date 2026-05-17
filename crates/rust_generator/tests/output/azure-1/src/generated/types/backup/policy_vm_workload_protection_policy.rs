#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyVmWorkloadProtectionPolicy {
    /// A `backup` block as defined below.
    #[builder(into)]
    #[serde(rename = "backup")]
    pub r#backup: Box<super::super::types::backup::PolicyVmWorkloadProtectionPolicyBackup>,
    /// The type of the VM Workload Backup Policy. Possible values are `Differential`, `Full`, `Incremental` and `Log`.
    #[builder(into)]
    #[serde(rename = "policyType")]
    pub r#policy_type: String,
    /// A `retention_daily` block as defined below.
    #[builder(into)]
    #[serde(rename = "retentionDaily")]
    pub r#retention_daily: Option<Box<super::super::types::backup::PolicyVmWorkloadProtectionPolicyRetentionDaily>>,
    /// A `retention_monthly` block as defined below.
    #[builder(into)]
    #[serde(rename = "retentionMonthly")]
    pub r#retention_monthly: Option<Box<super::super::types::backup::PolicyVmWorkloadProtectionPolicyRetentionMonthly>>,
    /// A `retention_weekly` block as defined below.
    #[builder(into)]
    #[serde(rename = "retentionWeekly")]
    pub r#retention_weekly: Option<Box<super::super::types::backup::PolicyVmWorkloadProtectionPolicyRetentionWeekly>>,
    /// A `retention_yearly` block as defined below.
    #[builder(into)]
    #[serde(rename = "retentionYearly")]
    pub r#retention_yearly: Option<Box<super::super::types::backup::PolicyVmWorkloadProtectionPolicyRetentionYearly>>,
    /// A `simple_retention` block as defined below.
    #[builder(into)]
    #[serde(rename = "simpleRetention")]
    pub r#simple_retention: Option<Box<super::super::types::backup::PolicyVmWorkloadProtectionPolicySimpleRetention>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyVmWorkloadProtectionPolicy {
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
                "backup".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backup,
                )
                .await,
            );
            map.insert(
                "policy_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_type,
                )
                .await,
            );
            map.insert(
                "retention_daily".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_daily,
                )
                .await,
            );
            map.insert(
                "retention_monthly".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_monthly,
                )
                .await,
            );
            map.insert(
                "retention_weekly".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_weekly,
                )
                .await,
            );
            map.insert(
                "retention_yearly".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_yearly,
                )
                .await,
            );
            map.insert(
                "simple_retention".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#simple_retention,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyVmWorkloadProtectionPolicy {
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
                    r#backup: {
                        let field_value = match fields_map.get("backup") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_type: {
                        let field_value = match fields_map.get("policy_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_daily: {
                        let field_value = match fields_map.get("retention_daily") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_daily' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_monthly: {
                        let field_value = match fields_map.get("retention_monthly") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_monthly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_weekly: {
                        let field_value = match fields_map.get("retention_weekly") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_weekly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_yearly: {
                        let field_value = match fields_map.get("retention_yearly") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_yearly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#simple_retention: {
                        let field_value = match fields_map.get("simple_retention") {
                            Some(value) => value,
                            None => bail!("Missing field 'simple_retention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
