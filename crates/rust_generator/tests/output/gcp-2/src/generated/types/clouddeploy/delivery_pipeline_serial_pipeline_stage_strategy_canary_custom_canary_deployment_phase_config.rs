#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeploymentPhaseConfig {
    /// Required. Percentage deployment for the phase.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: i32,
    /// Required. The ID to assign to the `Rollout` phase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^a-z?$`.
    #[builder(into)]
    #[serde(rename = "phaseId")]
    pub r#phase_id: String,
    /// Optional. Configuration for the postdeploy job of this phase. If this is not configured, postdeploy job will not be present for this phase.
    #[builder(into)]
    #[serde(rename = "postdeploy")]
    pub r#postdeploy: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeploymentPhaseConfigPostdeploy>>,
    /// Optional. Configuration for the predeploy job of this phase. If this is not configured, predeploy job will not be present for this phase.
    #[builder(into)]
    #[serde(rename = "predeploy")]
    pub r#predeploy: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeploymentPhaseConfigPredeploy>>,
    /// Skaffold profiles to use when rendering the manifest for this phase. These are in addition to the profiles list specified in the `DeliveryPipeline` stage.
    #[builder(into)]
    #[serde(rename = "profiles")]
    pub r#profiles: Option<Vec<String>>,
    /// Whether to run verify tests after the deployment.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "verify")]
    pub r#verify: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeploymentPhaseConfig {
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
                    "percentage",
                    &self.r#percentage,
                ),
                to_pulumi_object_field(
                    "phase_id",
                    &self.r#phase_id,
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
                    "profiles",
                    &self.r#profiles,
                ),
                to_pulumi_object_field(
                    "verify",
                    &self.r#verify,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeploymentPhaseConfig {
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
                    r#percentage: {
                        let field_value = match fields_map.get("percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phase_id: {
                        let field_value = match fields_map.get("phase_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'phase_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#profiles: {
                        let field_value = match fields_map.get("profiles") {
                            Some(value) => value,
                            None => bail!("Missing field 'profiles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
