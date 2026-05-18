#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeployment {
    /// Required. The percentage based deployments that will occur as a part of a `Rollout`. List is expected in ascending order and each integer n is 0 <= n < 100.
    #[builder(into)]
    #[serde(rename = "percentages")]
    pub r#percentages: Vec<i32>,
    /// Optional. Configuration for the postdeploy job of the last phase. If this is not configured, postdeploy job will not be present.
    #[builder(into)]
    #[serde(rename = "postdeploy")]
    pub r#postdeploy: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeploymentPostdeploy>>,
    /// Optional. Configuration for the predeploy job of the first phase. If this is not configured, predeploy job will not be present.
    #[builder(into)]
    #[serde(rename = "predeploy")]
    pub r#predeploy: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeploymentPredeploy>>,
    /// Whether to run verify tests after each percentage deployment.
    #[builder(into)]
    #[serde(rename = "verify")]
    pub r#verify: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeployment {
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
                    "percentages",
                    &self.r#percentages,
                ),
                to_pulumi_object_field(
                    "postdeploy",
                    &self.r#postdeploy,
                ),
                to_pulumi_object_field(
                    "predeploy",
                    &self.r#predeploy,
                ),
                to_pulumi_object_field(
                    "verify",
                    &self.r#verify,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeployment {
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
                    r#percentages: {
                        let field_value = match fields_map.get("percentages") {
                            Some(value) => value,
                            None => bail!("Missing field 'percentages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#postdeploy: {
                        let field_value = match fields_map.get("postdeploy") {
                            Some(value) => value,
                            None => bail!("Missing field 'postdeploy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#predeploy: {
                        let field_value = match fields_map.get("predeploy") {
                            Some(value) => value,
                            None => bail!("Missing field 'predeploy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verify: {
                        let field_value = match fields_map.get("verify") {
                            Some(value) => value,
                            None => bail!("Missing field 'verify' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
