#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscalingPolicyBasicAlgorithmYarnConfig {
    /// Timeout for YARN graceful decommissioning of Node Managers. Specifies the
    /// duration to wait for jobs to complete before forcefully removing workers
    /// (and potentially interrupting jobs). Only applicable to downscaling operations.
    /// Bounds: [0s, 1d].
    #[builder(into)]
    #[serde(rename = "gracefulDecommissionTimeout")]
    pub r#graceful_decommission_timeout: String,
    /// Fraction of average pending memory in the last cooldown period for which to
    /// remove workers. A scale-down factor of 1 will result in scaling down so that there
    /// is no available memory remaining after the update (more aggressive scaling).
    /// A scale-down factor of 0 disables removing workers, which can be beneficial for
    /// autoscaling a single job.
    /// Bounds: [0.0, 1.0].
    #[builder(into)]
    #[serde(rename = "scaleDownFactor")]
    pub r#scale_down_factor: f64,
    /// Minimum scale-down threshold as a fraction of total cluster size before scaling occurs.
    /// For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must
    /// recommend at least a 2 worker scale-down for the cluster to scale. A threshold of 0
    /// means the autoscaler will scale down on any recommended change.
    /// Bounds: [0.0, 1.0]. Default: 0.0.
    #[builder(into)]
    #[serde(rename = "scaleDownMinWorkerFraction")]
    pub r#scale_down_min_worker_fraction: Option<f64>,
    /// Fraction of average pending memory in the last cooldown period for which to
    /// add workers. A scale-up factor of 1.0 will result in scaling up so that there
    /// is no pending memory remaining after the update (more aggressive scaling).
    /// A scale-up factor closer to 0 will result in a smaller magnitude of scaling up
    /// (less aggressive scaling).
    /// Bounds: [0.0, 1.0].
    #[builder(into)]
    #[serde(rename = "scaleUpFactor")]
    pub r#scale_up_factor: f64,
    /// Minimum scale-up threshold as a fraction of total cluster size before scaling
    /// occurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler
    /// must recommend at least a 2-worker scale-up for the cluster to scale. A threshold of
    /// 0 means the autoscaler will scale up on any recommended change.
    /// Bounds: [0.0, 1.0]. Default: 0.0.
    #[builder(into)]
    #[serde(rename = "scaleUpMinWorkerFraction")]
    pub r#scale_up_min_worker_fraction: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscalingPolicyBasicAlgorithmYarnConfig {
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
                    "graceful_decommission_timeout",
                    &self.r#graceful_decommission_timeout,
                ),
                to_pulumi_object_field(
                    "scale_down_factor",
                    &self.r#scale_down_factor,
                ),
                to_pulumi_object_field(
                    "scale_down_min_worker_fraction",
                    &self.r#scale_down_min_worker_fraction,
                ),
                to_pulumi_object_field(
                    "scale_up_factor",
                    &self.r#scale_up_factor,
                ),
                to_pulumi_object_field(
                    "scale_up_min_worker_fraction",
                    &self.r#scale_up_min_worker_fraction,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutoscalingPolicyBasicAlgorithmYarnConfig {
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
                    r#graceful_decommission_timeout: {
                        let field_value = match fields_map.get("graceful_decommission_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'graceful_decommission_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_factor: {
                        let field_value = match fields_map.get("scale_down_factor") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_down_factor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_down_min_worker_fraction: {
                        let field_value = match fields_map.get("scale_down_min_worker_fraction") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_down_min_worker_fraction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_up_factor: {
                        let field_value = match fields_map.get("scale_up_factor") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_up_factor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_up_min_worker_fraction: {
                        let field_value = match fields_map.get("scale_up_min_worker_fraction") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_up_min_worker_fraction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
