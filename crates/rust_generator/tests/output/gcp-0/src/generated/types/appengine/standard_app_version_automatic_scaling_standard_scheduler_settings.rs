#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StandardAppVersionAutomaticScalingStandardSchedulerSettings {
    /// Maximum number of instances to run for this version. Set to zero to disable maxInstances configuration.
    #[builder(into)]
    #[serde(rename = "maxInstances")]
    pub r#max_instances: Option<i32>,
    /// Minimum number of instances to run for this version. Set to zero to disable minInstances configuration.
    #[builder(into)]
    #[serde(rename = "minInstances")]
    pub r#min_instances: Option<i32>,
    /// Target CPU utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value.
    #[builder(into)]
    #[serde(rename = "targetCpuUtilization")]
    pub r#target_cpu_utilization: Option<f64>,
    /// Target throughput utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value.
    #[builder(into)]
    #[serde(rename = "targetThroughputUtilization")]
    pub r#target_throughput_utilization: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StandardAppVersionAutomaticScalingStandardSchedulerSettings {
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
                "max_instances".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_instances,
                )
                .await,
            );
            map.insert(
                "min_instances".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_instances,
                )
                .await,
            );
            map.insert(
                "target_cpu_utilization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_cpu_utilization,
                )
                .await,
            );
            map.insert(
                "target_throughput_utilization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_throughput_utilization,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StandardAppVersionAutomaticScalingStandardSchedulerSettings {
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
                    r#max_instances: {
                        let field_value = match fields_map.get("max_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_instances: {
                        let field_value = match fields_map.get("min_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_cpu_utilization: {
                        let field_value = match fields_map.get("target_cpu_utilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_cpu_utilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_throughput_utilization: {
                        let field_value = match fields_map.get("target_throughput_utilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_throughput_utilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
