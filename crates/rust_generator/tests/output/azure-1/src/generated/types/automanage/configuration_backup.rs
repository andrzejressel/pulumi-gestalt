#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationBackup {
    /// The retention range in days of the backup policy. Defaults to `5`.
    #[builder(into)]
    #[serde(rename = "instantRpRetentionRangeInDays")]
    pub r#instant_rp_retention_range_in_days: Option<i32>,
    /// The name of the backup policy.
    #[builder(into)]
    #[serde(rename = "policyName")]
    pub r#policy_name: Option<String>,
    /// A `retention_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Option<Box<super::super::types::automanage::ConfigurationBackupRetentionPolicy>>,
    /// A `schedule_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "schedulePolicy")]
    pub r#schedule_policy: Option<Box<super::super::types::automanage::ConfigurationBackupSchedulePolicy>>,
    /// The timezone of the backup policy. Defaults to `UTC`.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationBackup {
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
                    "instantRpRetentionRangeInDays",
                    &self.r#instant_rp_retention_range_in_days,
                ),
                to_pulumi_object_field(
                    "policyName",
                    &self.r#policy_name,
                ),
                to_pulumi_object_field(
                    "retentionPolicy",
                    &self.r#retention_policy,
                ),
                to_pulumi_object_field(
                    "schedulePolicy",
                    &self.r#schedule_policy,
                ),
                to_pulumi_object_field(
                    "timeZone",
                    &self.r#time_zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationBackup {
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
                    r#instant_rp_retention_range_in_days: {
                        let field_value = match fields_map.get("instantRpRetentionRangeInDays") {
                            Some(value) => value,
                            None => bail!("Missing field 'instantRpRetentionRangeInDays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_name: {
                        let field_value = match fields_map.get("policyName") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_policy: {
                        let field_value = match fields_map.get("retentionPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'retentionPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule_policy: {
                        let field_value = match fields_map.get("schedulePolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedulePolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("timeZone") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeZone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
