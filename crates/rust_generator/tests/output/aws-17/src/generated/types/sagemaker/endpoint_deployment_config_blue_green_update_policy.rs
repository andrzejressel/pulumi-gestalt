#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeploymentConfigBlueGreenUpdatePolicy {
    #[builder(into)]
    pub r#maximum_execution_timeout_in_seconds: Option<i32>,
    /// Additional waiting time in seconds after the completion of an endpoint deployment before terminating the old endpoint fleet. Default is `0`. Valid values are between `0` and `3600`.
    #[builder(into)]
    pub r#termination_wait_in_seconds: Option<i32>,
    /// Defines the traffic routing strategy to shift traffic from the old fleet to the new fleet during an endpoint deployment. See Traffic Routing Configuration.
    #[builder(into)]
    pub r#traffic_routing_configuration: Box<super::super::types::sagemaker::EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointDeploymentConfigBlueGreenUpdatePolicy {
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
                    "maximumExecutionTimeoutInSeconds",
                    &self.r#maximum_execution_timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "terminationWaitInSeconds",
                    &self.r#termination_wait_in_seconds,
                ),
                to_pulumi_object_field(
                    "trafficRoutingConfiguration",
                    &self.r#traffic_routing_configuration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointDeploymentConfigBlueGreenUpdatePolicy {
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
                    r#maximum_execution_timeout_in_seconds: {
                        let field_value = match fields_map.get("maximumExecutionTimeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumExecutionTimeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#termination_wait_in_seconds: {
                        let field_value = match fields_map.get("terminationWaitInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'terminationWaitInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#traffic_routing_configuration: {
                        let field_value = match fields_map.get("trafficRoutingConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'trafficRoutingConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
