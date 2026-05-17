#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CapacityProviderAutoScalingGroupProvider {
    /// ARN of the associated auto scaling group.
    #[builder(into)]
    #[serde(rename = "autoScalingGroupArn")]
    pub r#auto_scaling_group_arn: String,
    /// Enables or disables a graceful shutdown of instances without disturbing workloads. Valid values are `ENABLED` and `DISABLED`. The default value is `ENABLED` when a capacity provider is created.
    #[builder(into)]
    #[serde(rename = "managedDraining")]
    pub r#managed_draining: Option<String>,
    /// Configuration block defining the parameters of the auto scaling. Detailed below.
    #[builder(into)]
    #[serde(rename = "managedScaling")]
    pub r#managed_scaling: Option<Box<super::super::types::ecs::CapacityProviderAutoScalingGroupProviderManagedScaling>>,
    /// Enables or disables container-aware termination of instances in the auto scaling group when scale-in happens. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "managedTerminationProtection")]
    pub r#managed_termination_protection: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CapacityProviderAutoScalingGroupProvider {
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
                    "auto_scaling_group_arn",
                    &self.r#auto_scaling_group_arn,
                ),
                to_pulumi_object_field(
                    "managed_draining",
                    &self.r#managed_draining,
                ),
                to_pulumi_object_field(
                    "managed_scaling",
                    &self.r#managed_scaling,
                ),
                to_pulumi_object_field(
                    "managed_termination_protection",
                    &self.r#managed_termination_protection,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CapacityProviderAutoScalingGroupProvider {
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
                    r#auto_scaling_group_arn: {
                        let field_value = match fields_map.get("auto_scaling_group_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_scaling_group_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_draining: {
                        let field_value = match fields_map.get("managed_draining") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_draining' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_scaling: {
                        let field_value = match fields_map.get("managed_scaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_scaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_termination_protection: {
                        let field_value = match fields_map.get("managed_termination_protection") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_termination_protection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
