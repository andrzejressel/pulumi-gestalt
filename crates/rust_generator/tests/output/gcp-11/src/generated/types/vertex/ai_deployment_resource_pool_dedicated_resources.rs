#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiDeploymentResourcePoolDedicatedResources {
    /// A list of the metric specifications that overrides a resource utilization metric.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autoscalingMetricSpecs")]
    pub r#autoscaling_metric_specs: Option<Vec<super::super::types::vertex::AiDeploymentResourcePoolDedicatedResourcesAutoscalingMetricSpec>>,
    /// The specification of a single machine used by the prediction
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "machineSpec")]
    pub r#machine_spec: Box<super::super::types::vertex::AiDeploymentResourcePoolDedicatedResourcesMachineSpec>,
    /// The maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, will use min_replica_count as the default value. The value of this field impacts the charge against Vertex CPU and GPU quotas. Specifically, you will be charged for max_replica_count * number of cores in the selected machine type) and (max_replica_count * number of GPUs per replica in the selected machine type).
    #[builder(into)]
    #[serde(rename = "maxReplicaCount")]
    pub r#max_replica_count: Option<i32>,
    /// The minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1. If traffic against the DeployedModel increases, it may dynamically be deployed onto more replicas, and as traffic decreases, some of these extra replicas may be freed.
    #[builder(into)]
    #[serde(rename = "minReplicaCount")]
    pub r#min_replica_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiDeploymentResourcePoolDedicatedResources {
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
                    "autoscaling_metric_specs",
                    &self.r#autoscaling_metric_specs,
                ),
                to_pulumi_object_field(
                    "machine_spec",
                    &self.r#machine_spec,
                ),
                to_pulumi_object_field(
                    "max_replica_count",
                    &self.r#max_replica_count,
                ),
                to_pulumi_object_field(
                    "min_replica_count",
                    &self.r#min_replica_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiDeploymentResourcePoolDedicatedResources {
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
                    r#autoscaling_metric_specs: {
                        let field_value = match fields_map.get("autoscaling_metric_specs") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling_metric_specs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_spec: {
                        let field_value = match fields_map.get("machine_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_replica_count: {
                        let field_value = match fields_map.get("max_replica_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_replica_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_replica_count: {
                        let field_value = match fields_map.get("min_replica_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_replica_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
