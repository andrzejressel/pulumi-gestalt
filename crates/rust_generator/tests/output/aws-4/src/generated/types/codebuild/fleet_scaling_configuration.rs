#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetScalingConfiguration {
    #[builder(into)]
    #[serde(rename = "desiredCapacity")]
    pub r#desired_capacity: Option<i32>,
    /// Maximum number of instances in the ﬂeet when auto-scaling.
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Option<i32>,
    /// Scaling type for a compute fleet. Valid value: `TARGET_TRACKING_SCALING`.
    #[builder(into)]
    #[serde(rename = "scalingType")]
    pub r#scaling_type: Option<String>,
    /// Configuration block. Detailed below.
    #[builder(into)]
    #[serde(rename = "targetTrackingScalingConfigs")]
    pub r#target_tracking_scaling_configs: Option<Vec<super::super::types::codebuild::FleetScalingConfigurationTargetTrackingScalingConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetScalingConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("desired_capacity".to_string(), self.r#desired_capacity.to_pulumi_value().await);
            map.insert("max_capacity".to_string(), self.r#max_capacity.to_pulumi_value().await);
            map.insert("scaling_type".to_string(), self.r#scaling_type.to_pulumi_value().await);
            map.insert("target_tracking_scaling_configs".to_string(), self.r#target_tracking_scaling_configs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetScalingConfiguration {
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
                    r#desired_capacity: {
                        let field_value = match fields_map.get("desired_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_capacity: {
                        let field_value = match fields_map.get("max_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scaling_type: {
                        let field_value = match fields_map.get("scaling_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaling_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target_tracking_scaling_configs: {
                        let field_value = match fields_map.get("target_tracking_scaling_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_tracking_scaling_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::codebuild::FleetScalingConfigurationTargetTrackingScalingConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
