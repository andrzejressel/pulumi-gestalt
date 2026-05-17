#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceAutoscalingConfigAutoscalingTargets {
    /// Specifies the target high priority cpu utilization percentage that the autoscaler
    /// should be trying to achieve for the instance.
    /// This number is on a scale from 0 (no utilization) to 100 (full utilization)..
    #[builder(into)]
    #[serde(rename = "highPriorityCpuUtilizationPercent")]
    pub r#high_priority_cpu_utilization_percent: Option<i32>,
    /// Specifies the target storage utilization percentage that the autoscaler
    /// should be trying to achieve for the instance.
    /// This number is on a scale from 0 (no utilization) to 100 (full utilization).
    #[builder(into)]
    #[serde(rename = "storageUtilizationPercent")]
    pub r#storage_utilization_percent: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceAutoscalingConfigAutoscalingTargets {
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
                "high_priority_cpu_utilization_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#high_priority_cpu_utilization_percent,
                )
                .await,
            );
            map.insert(
                "storage_utilization_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_utilization_percent,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceAutoscalingConfigAutoscalingTargets {
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
                    r#high_priority_cpu_utilization_percent: {
                        let field_value = match fields_map.get("high_priority_cpu_utilization_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'high_priority_cpu_utilization_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_utilization_percent: {
                        let field_value = match fields_map.get("storage_utilization_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_utilization_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
