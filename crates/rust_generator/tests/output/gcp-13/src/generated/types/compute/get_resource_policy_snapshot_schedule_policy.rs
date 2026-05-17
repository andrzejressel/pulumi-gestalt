#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResourcePolicySnapshotSchedulePolicy {
    /// Retention policy applied to snapshots created by this resource policy.
    #[builder(into)]
    #[serde(rename = "retentionPolicies")]
    pub r#retention_policies: Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicyRetentionPolicy>,
    /// Contains one of an 'hourlySchedule', 'dailySchedule', or 'weeklySchedule'.
    #[builder(into)]
    #[serde(rename = "schedules")]
    pub r#schedules: Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicySchedule>,
    /// Properties with which the snapshots are created, such as labels.
    #[builder(into)]
    #[serde(rename = "snapshotProperties")]
    pub r#snapshot_properties: Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicySnapshotProperty>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetResourcePolicySnapshotSchedulePolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "retention_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_policies,
                )
                .await,
            );
            map.insert(
                "schedules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schedules,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetResourcePolicySnapshotSchedulePolicy {
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
                    r#retention_policies: {
                        let field_value = match fields_map.get("retention_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedules: {
                        let field_value = match fields_map.get("schedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
