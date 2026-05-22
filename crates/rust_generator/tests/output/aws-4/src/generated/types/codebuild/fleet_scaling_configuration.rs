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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "desiredCapacity",
                    &self.r#desired_capacity,
                ),
                to_pulumi_object_field(
                    "maxCapacity",
                    &self.r#max_capacity,
                ),
                to_pulumi_object_field(
                    "scalingType",
                    &self.r#scaling_type,
                ),
                to_pulumi_object_field(
                    "targetTrackingScalingConfigs",
                    &self.r#target_tracking_scaling_configs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetScalingConfiguration {
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
                    r#desired_capacity: {
                        let field_value = match fields_map.get("desiredCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'desiredCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_capacity: {
                        let field_value = match fields_map.get("maxCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scaling_type: {
                        let field_value = match fields_map.get("scalingType") {
                            Some(value) => value,
                            None => bail!("Missing field 'scalingType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_tracking_scaling_configs: {
                        let field_value = match fields_map.get("targetTrackingScalingConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetTrackingScalingConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
