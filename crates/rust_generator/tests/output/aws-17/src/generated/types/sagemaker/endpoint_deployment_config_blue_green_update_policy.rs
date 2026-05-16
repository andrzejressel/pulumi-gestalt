#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeploymentConfigBlueGreenUpdatePolicy {
    #[builder(into)]
    #[serde(rename = "maximumExecutionTimeoutInSeconds")]
    pub r#maximum_execution_timeout_in_seconds: Option<i32>,
    /// Additional waiting time in seconds after the completion of an endpoint deployment before terminating the old endpoint fleet. Default is `0`. Valid values are between `0` and `3600`.
    #[builder(into)]
    #[serde(rename = "terminationWaitInSeconds")]
    pub r#termination_wait_in_seconds: Option<i32>,
    /// Defines the traffic routing strategy to shift traffic from the old fleet to the new fleet during an endpoint deployment. See Traffic Routing Configuration.
    #[builder(into)]
    #[serde(rename = "trafficRoutingConfiguration")]
    pub r#traffic_routing_configuration: Box<super::super::types::sagemaker::EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointDeploymentConfigBlueGreenUpdatePolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("maximum_execution_timeout_in_seconds".to_string(), self.r#maximum_execution_timeout_in_seconds.to_pulumi_value().await);
            map.insert("termination_wait_in_seconds".to_string(), self.r#termination_wait_in_seconds.to_pulumi_value().await);
            map.insert("traffic_routing_configuration".to_string(), self.r#traffic_routing_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointDeploymentConfigBlueGreenUpdatePolicy {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#maximum_execution_timeout_in_seconds: {
                        let field_value = match fields_map.get("maximum_execution_timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_execution_timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#termination_wait_in_seconds: {
                        let field_value = match fields_map.get("termination_wait_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'termination_wait_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#traffic_routing_configuration: {
                        let field_value = match fields_map.get("traffic_routing_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'traffic_routing_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::sagemaker::EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
