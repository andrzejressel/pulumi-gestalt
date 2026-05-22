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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "backup",
                    &self.r#backup,
                ),
                to_pulumi_object_field(
                    "policyType",
                    &self.r#policy_type,
                ),
                to_pulumi_object_field(
                    "retentionDaily",
                    &self.r#retention_daily,
                ),
                to_pulumi_object_field(
                    "retentionMonthly",
                    &self.r#retention_monthly,
                ),
                to_pulumi_object_field(
                    "retentionWeekly",
                    &self.r#retention_weekly,
                ),
                to_pulumi_object_field(
                    "retentionYearly",
                    &self.r#retention_yearly,
                ),
                to_pulumi_object_field(
                    "simpleRetention",
                    &self.r#simple_retention,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("policyType") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_daily: {
                        let field_value = match fields_map.get("retentionDaily") {
                            Some(value) => value,
                            None => bail!("Missing field 'retentionDaily' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_monthly: {
                        let field_value = match fields_map.get("retentionMonthly") {
                            Some(value) => value,
                            None => bail!("Missing field 'retentionMonthly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_weekly: {
                        let field_value = match fields_map.get("retentionWeekly") {
                            Some(value) => value,
                            None => bail!("Missing field 'retentionWeekly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_yearly: {
                        let field_value = match fields_map.get("retentionYearly") {
                            Some(value) => value,
                            None => bail!("Missing field 'retentionYearly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#simple_retention: {
                        let field_value = match fields_map.get("simpleRetention") {
                            Some(value) => value,
                            None => bail!("Missing field 'simpleRetention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
