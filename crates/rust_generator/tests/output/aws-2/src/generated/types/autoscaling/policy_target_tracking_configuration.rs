#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyTargetTrackingConfiguration {
    /// Customized metric. Conflicts with `predefined_metric_specification`.
    #[builder(into)]
    #[serde(rename = "customizedMetricSpecification")]
    pub r#customized_metric_specification: Option<Box<super::super::types::autoscaling::PolicyTargetTrackingConfigurationCustomizedMetricSpecification>>,
    /// Whether scale in by the target tracking policy is disabled.
    #[builder(into)]
    #[serde(rename = "disableScaleIn")]
    pub r#disable_scale_in: Option<bool>,
    /// Predefined metric. Conflicts with `customized_metric_specification`.
    #[builder(into)]
    #[serde(rename = "predefinedMetricSpecification")]
    pub r#predefined_metric_specification: Option<Box<super::super::types::autoscaling::PolicyTargetTrackingConfigurationPredefinedMetricSpecification>>,
    /// Target value for the metric.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyTargetTrackingConfiguration {
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
                    "customized_metric_specification",
                    &self.r#customized_metric_specification,
                ),
                to_pulumi_object_field(
                    "disable_scale_in",
                    &self.r#disable_scale_in,
                ),
                to_pulumi_object_field(
                    "predefined_metric_specification",
                    &self.r#predefined_metric_specification,
                ),
                to_pulumi_object_field(
                    "target_value",
                    &self.r#target_value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyTargetTrackingConfiguration {
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
                    r#customized_metric_specification: {
                        let field_value = match fields_map.get("customized_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'customized_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_scale_in: {
                        let field_value = match fields_map.get("disable_scale_in") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_scale_in' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#predefined_metric_specification: {
                        let field_value = match fields_map.get("predefined_metric_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'predefined_metric_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_value: {
                        let field_value = match fields_map.get("target_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
