#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfiguration {
    /// Batch size for the first step to turn on traffic on the new endpoint fleet. Value must be less than or equal to 50% of the variant's total instance count. See Canary Size.
    #[builder(into)]
    #[serde(rename = "canarySize")]
    pub r#canary_size: Option<Box<super::super::types::sagemaker::EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfigurationCanarySize>>,
    /// Batch size for each step to turn on traffic on the new endpoint fleet. Value must be 10-50% of the variant's total instance count. See Linear Step Size.
    #[builder(into)]
    #[serde(rename = "linearStepSize")]
    pub r#linear_step_size: Option<Box<super::super::types::sagemaker::EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfigurationLinearStepSize>>,
    /// Traffic routing strategy type. Valid values are: `ALL_AT_ONCE`, `CANARY`, and `LINEAR`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The waiting time (in seconds) between incremental steps to turn on traffic on the new endpoint fleet. Valid values are between `0` and `3600`.
    #[builder(into)]
    #[serde(rename = "waitIntervalInSeconds")]
    pub r#wait_interval_in_seconds: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfiguration {
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
                    "canary_size",
                    &self.r#canary_size,
                ),
                to_pulumi_object_field(
                    "linear_step_size",
                    &self.r#linear_step_size,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
                to_pulumi_object_field(
                    "wait_interval_in_seconds",
                    &self.r#wait_interval_in_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfiguration {
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
                    r#canary_size: {
                        let field_value = match fields_map.get("canary_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'canary_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linear_step_size: {
                        let field_value = match fields_map.get("linear_step_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'linear_step_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wait_interval_in_seconds: {
                        let field_value = match fields_map.get("wait_interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'wait_interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
