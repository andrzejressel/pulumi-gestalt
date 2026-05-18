#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetesGatewayServiceMesh {
    /// Required. Name of the Kubernetes Deployment whose traffic is managed by the specified HTTPRoute and Service.
    #[builder(into)]
    #[serde(rename = "deployment")]
    pub r#deployment: String,
    /// Required. Name of the Gateway API HTTPRoute.
    #[builder(into)]
    #[serde(rename = "httpRoute")]
    pub r#http_route: String,
    /// Optional. The label to use when selecting Pods for the Deployment and Service resources. This label must already be present in both resources.
    #[builder(into)]
    #[serde(rename = "podSelectorLabel")]
    pub r#pod_selector_label: Option<String>,
    /// Optional. Route destinations allow configuring the Gateway API HTTPRoute to be deployed to additional clusters. This option is available for multi-cluster service mesh set ups that require the route to exist in the clusters that call the service. If unspecified, the HTTPRoute will only be deployed to the Target cluster.
    #[builder(into)]
    #[serde(rename = "routeDestinations")]
    pub r#route_destinations: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetesGatewayServiceMeshRouteDestinations>>,
    /// Optional. The time to wait for route updates to propagate. The maximum configurable time is 3 hours, in seconds format. If unspecified, there is no wait time.
    #[builder(into)]
    #[serde(rename = "routeUpdateWaitTime")]
    pub r#route_update_wait_time: Option<String>,
    /// Required. Name of the Kubernetes Service.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: String,
    /// Optional. The amount of time to migrate traffic back from the canary Service to the original Service during the stable phase deployment. If specified, must be between 15s and 3600s. If unspecified, there is no cutback time.
    #[builder(into)]
    #[serde(rename = "stableCutbackDuration")]
    pub r#stable_cutback_duration: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetesGatewayServiceMesh {
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
                    "deployment",
                    &self.r#deployment,
                ),
                to_pulumi_object_field(
                    "http_route",
                    &self.r#http_route,
                ),
                to_pulumi_object_field(
                    "pod_selector_label",
                    &self.r#pod_selector_label,
                ),
                to_pulumi_object_field(
                    "route_destinations",
                    &self.r#route_destinations,
                ),
                to_pulumi_object_field(
                    "route_update_wait_time",
                    &self.r#route_update_wait_time,
                ),
                to_pulumi_object_field(
                    "service",
                    &self.r#service,
                ),
                to_pulumi_object_field(
                    "stable_cutback_duration",
                    &self.r#stable_cutback_duration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetesGatewayServiceMesh {
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
                    r#deployment: {
                        let field_value = match fields_map.get("deployment") {
                            Some(value) => value,
                            None => bail!("Missing field 'deployment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_route: {
                        let field_value = match fields_map.get("http_route") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_route' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_selector_label: {
                        let field_value = match fields_map.get("pod_selector_label") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_selector_label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_destinations: {
                        let field_value = match fields_map.get("route_destinations") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_destinations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_update_wait_time: {
                        let field_value = match fields_map.get("route_update_wait_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_update_wait_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service: {
                        let field_value = match fields_map.get("service") {
                            Some(value) => value,
                            None => bail!("Missing field 'service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stable_cutback_duration: {
                        let field_value = match fields_map.get("stable_cutback_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'stable_cutback_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
