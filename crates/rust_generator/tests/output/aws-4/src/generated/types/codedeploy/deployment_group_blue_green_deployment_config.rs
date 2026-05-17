#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentGroupBlueGreenDeploymentConfig {
    /// Information about the action to take when newly provisioned instances are ready to receive traffic in a blue/green deployment (documented below).
    #[builder(into)]
    #[serde(rename = "deploymentReadyOption")]
    pub r#deployment_ready_option: Option<Box<super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfigDeploymentReadyOption>>,
    /// Information about how instances are provisioned for a replacement environment in a blue/green deployment (documented below).
    #[builder(into)]
    #[serde(rename = "greenFleetProvisioningOption")]
    pub r#green_fleet_provisioning_option: Option<Box<super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfigGreenFleetProvisioningOption>>,
    /// Information about whether to terminate instances in the original fleet during a blue/green deployment (documented below).
    /// 
    /// _Only one `blue_green_deployment_config` is allowed_.
    #[builder(into)]
    #[serde(rename = "terminateBlueInstancesOnDeploymentSuccess")]
    pub r#terminate_blue_instances_on_deployment_success: Option<Box<super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfigTerminateBlueInstancesOnDeploymentSuccess>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeploymentGroupBlueGreenDeploymentConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "deployment_ready_option",
                    &self.r#deployment_ready_option,
                ),
                to_pulumi_object_field(
                    "green_fleet_provisioning_option",
                    &self.r#green_fleet_provisioning_option,
                ),
                to_pulumi_object_field(
                    "terminate_blue_instances_on_deployment_success",
                    &self.r#terminate_blue_instances_on_deployment_success,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeploymentGroupBlueGreenDeploymentConfig {
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
                    r#deployment_ready_option: {
                        let field_value = match fields_map.get("deployment_ready_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'deployment_ready_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#green_fleet_provisioning_option: {
                        let field_value = match fields_map.get("green_fleet_provisioning_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'green_fleet_provisioning_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#terminate_blue_instances_on_deployment_success: {
                        let field_value = match fields_map.get("terminate_blue_instances_on_deployment_success") {
                            Some(value) => value,
                            None => bail!("Missing field 'terminate_blue_instances_on_deployment_success' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
