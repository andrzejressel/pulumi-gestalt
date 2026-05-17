#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourcePolicySnapshotSchedulePolicy {
    /// Retention policy applied to snapshots created by this resource policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Option<Box<super::super::types::compute::ResourcePolicySnapshotSchedulePolicyRetentionPolicy>>,
    /// Contains one of an `hourlySchedule`, `dailySchedule`, or `weeklySchedule`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<super::super::types::compute::ResourcePolicySnapshotSchedulePolicySchedule>,
    /// Properties with which the snapshots are created, such as labels.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "snapshotProperties")]
    pub r#snapshot_properties: Option<Box<super::super::types::compute::ResourcePolicySnapshotSchedulePolicySnapshotProperties>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResourcePolicySnapshotSchedulePolicy {
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
                "retention_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_policy,
                )
                .await,
            );
            map.insert(
                "schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schedule,
                )
                .await,
            );
            map.insert(
                "snapshot_properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snapshot_properties,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResourcePolicySnapshotSchedulePolicy {
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
                    r#retention_policy: {
                        let field_value = match fields_map.get("retention_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#snapshot_properties: {
                        let field_value = match fields_map.get("snapshot_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
