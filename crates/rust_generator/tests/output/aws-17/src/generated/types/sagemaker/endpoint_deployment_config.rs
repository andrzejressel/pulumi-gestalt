#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeploymentConfig {
    /// Automatic rollback configuration for handling endpoint deployment failures and recovery. See Auto Rollback Configuration.
    #[builder(into)]
    #[serde(rename = "autoRollbackConfiguration")]
    pub r#auto_rollback_configuration: Option<Box<super::super::types::sagemaker::EndpointDeploymentConfigAutoRollbackConfiguration>>,
    /// Update policy for a blue/green deployment. If this update policy is specified, SageMaker creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips traffic to the new fleet according to the specified traffic routing configuration. Only one update policy should be used in the deployment configuration. If no update policy is specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting by default. See Blue Green Update Config.
    #[builder(into)]
    #[serde(rename = "blueGreenUpdatePolicy")]
    pub r#blue_green_update_policy: Option<Box<super::super::types::sagemaker::EndpointDeploymentConfigBlueGreenUpdatePolicy>>,
    /// Specifies a rolling deployment strategy for updating a SageMaker endpoint. See Rolling Update Policy.
    #[builder(into)]
    #[serde(rename = "rollingUpdatePolicy")]
    pub r#rolling_update_policy: Option<Box<super::super::types::sagemaker::EndpointDeploymentConfigRollingUpdatePolicy>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointDeploymentConfig {
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
                    "auto_rollback_configuration",
                    &self.r#auto_rollback_configuration,
                ),
                to_pulumi_object_field(
                    "blue_green_update_policy",
                    &self.r#blue_green_update_policy,
                ),
                to_pulumi_object_field(
                    "rolling_update_policy",
                    &self.r#rolling_update_policy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointDeploymentConfig {
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
                    r#auto_rollback_configuration: {
                        let field_value = match fields_map.get("auto_rollback_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_rollback_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#blue_green_update_policy: {
                        let field_value = match fields_map.get("blue_green_update_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'blue_green_update_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rolling_update_policy: {
                        let field_value = match fields_map.get("rolling_update_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'rolling_update_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
