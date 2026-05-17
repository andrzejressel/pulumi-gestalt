#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineCondition {
    /// Details around the Pipeline's overall status.
    #[builder(into)]
    #[serde(rename = "pipelineReadyConditions")]
    pub r#pipeline_ready_conditions: Option<Vec<super::super::types::clouddeploy::DeliveryPipelineConditionPipelineReadyCondition>>,
    /// Details around targets enumerated in the pipeline.
    #[builder(into)]
    #[serde(rename = "targetsPresentConditions")]
    pub r#targets_present_conditions: Option<Vec<super::super::types::clouddeploy::DeliveryPipelineConditionTargetsPresentCondition>>,
    /// Details on the whether the targets enumerated in the pipeline are of the same type.
    #[builder(into)]
    #[serde(rename = "targetsTypeConditions")]
    pub r#targets_type_conditions: Option<Vec<super::super::types::clouddeploy::DeliveryPipelineConditionTargetsTypeCondition>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeliveryPipelineCondition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "pipeline_ready_conditions",
                    &self.r#pipeline_ready_conditions,
                ),
                to_pulumi_object_field(
                    "targets_present_conditions",
                    &self.r#targets_present_conditions,
                ),
                to_pulumi_object_field(
                    "targets_type_conditions",
                    &self.r#targets_type_conditions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeliveryPipelineCondition {
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
                    r#pipeline_ready_conditions: {
                        let field_value = match fields_map.get("pipeline_ready_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'pipeline_ready_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#targets_present_conditions: {
                        let field_value = match fields_map.get("targets_present_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'targets_present_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#targets_type_conditions: {
                        let field_value = match fields_map.get("targets_type_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'targets_type_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
