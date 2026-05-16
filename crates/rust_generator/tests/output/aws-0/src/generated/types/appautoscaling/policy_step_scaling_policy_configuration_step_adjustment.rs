#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyStepScalingPolicyConfigurationStepAdjustment {
    /// Lower bound for the difference between the alarm threshold and the CloudWatch metric. Without a value, AWS will treat this bound as negative infinity.
    #[builder(into)]
    #[serde(rename = "metricIntervalLowerBound")]
    pub r#metric_interval_lower_bound: Option<String>,
    /// Upper bound for the difference between the alarm threshold and the CloudWatch metric. Without a value, AWS will treat this bound as infinity. The upper bound must be greater than the lower bound.
    #[builder(into)]
    #[serde(rename = "metricIntervalUpperBound")]
    pub r#metric_interval_upper_bound: Option<String>,
    /// Number of members by which to scale, when the adjustment bounds are breached. A positive value scales up. A negative value scales down.
    #[builder(into)]
    #[serde(rename = "scalingAdjustment")]
    pub r#scaling_adjustment: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyStepScalingPolicyConfigurationStepAdjustment {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("metric_interval_lower_bound".to_string(), self.r#metric_interval_lower_bound.to_pulumi_value().await);
            map.insert("metric_interval_upper_bound".to_string(), self.r#metric_interval_upper_bound.to_pulumi_value().await);
            map.insert("scaling_adjustment".to_string(), self.r#scaling_adjustment.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyStepScalingPolicyConfigurationStepAdjustment {
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
                    r#metric_interval_lower_bound: {
                        let field_value = match fields_map.get("metric_interval_lower_bound") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_interval_lower_bound' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#metric_interval_upper_bound: {
                        let field_value = match fields_map.get("metric_interval_upper_bound") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_interval_upper_bound' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scaling_adjustment: {
                        let field_value = match fields_map.get("scaling_adjustment") {
                            Some(value) => value,
                            None => bail!("Missing field 'scaling_adjustment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
