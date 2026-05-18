#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceAutoscalingConfig {
    /// Asymmetric autoscaling options for specific replicas.
    #[builder(into)]
    #[serde(rename = "asymmetricAutoscalingOptions")]
    pub r#asymmetric_autoscaling_options: Vec<super::super::types::spanner::GetInstanceAutoscalingConfigAsymmetricAutoscalingOption>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events. Users can define the minimum and
    /// maximum compute capacity allocated to the instance, and the autoscaler will
    /// only scale within that range. Users can either use nodes or processing
    /// units to specify the limits, but should use the same unit to set both the
    /// min_limit and max_limit.
    #[builder(into)]
    #[serde(rename = "autoscalingLimits")]
    pub r#autoscaling_limits: Vec<super::super::types::spanner::GetInstanceAutoscalingConfigAutoscalingLimit>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    #[builder(into)]
    #[serde(rename = "autoscalingTargets")]
    pub r#autoscaling_targets: Vec<super::super::types::spanner::GetInstanceAutoscalingConfigAutoscalingTarget>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInstanceAutoscalingConfig {
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
                    "asymmetric_autoscaling_options",
                    &self.r#asymmetric_autoscaling_options,
                ),
                to_pulumi_object_field(
                    "autoscaling_limits",
                    &self.r#autoscaling_limits,
                ),
                to_pulumi_object_field(
                    "autoscaling_targets",
                    &self.r#autoscaling_targets,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInstanceAutoscalingConfig {
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
                    r#asymmetric_autoscaling_options: {
                        let field_value = match fields_map.get("asymmetric_autoscaling_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'asymmetric_autoscaling_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autoscaling_limits: {
                        let field_value = match fields_map.get("autoscaling_limits") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling_limits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autoscaling_targets: {
                        let field_value = match fields_map.get("autoscaling_targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling_targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
