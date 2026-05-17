#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetRollingUpgradePolicy {
    /// The maximum percent of total virtual machine instances that will be upgraded simultaneously by the rolling upgrade in one batch. As this is a maximum, unhealthy instances in previous or future batches can cause the percentage of instances in a batch to decrease to ensure higher reliability. Defaults to `20`.
    #[builder(into)]
    #[serde(rename = "maxBatchInstancePercent")]
    pub r#max_batch_instance_percent: Option<i32>,
    /// The maximum percentage of the total virtual machine instances in the scale set that can be simultaneously unhealthy, either as a result of being upgraded, or by being found in an unhealthy state by the virtual machine health checks before the rolling upgrade aborts. This constraint will be checked prior to starting any batch. Defaults to `20`.
    #[builder(into)]
    #[serde(rename = "maxUnhealthyInstancePercent")]
    pub r#max_unhealthy_instance_percent: Option<i32>,
    /// The maximum percentage of upgraded virtual machine instances that can be found to be in an unhealthy state. This check will happen after each batch is upgraded. If this percentage is ever exceeded, the rolling update aborts. Defaults to `20`.
    #[builder(into)]
    #[serde(rename = "maxUnhealthyUpgradedInstancePercent")]
    pub r#max_unhealthy_upgraded_instance_percent: Option<i32>,
    /// The wait time between completing the update for all virtual machines in one batch and starting the next batch. The time duration should be specified in ISO 8601 format for duration (<https://en.wikipedia.org/wiki/ISO_8601#Durations>). Defaults to `PT0S` seconds represented as `PT0S`.
    #[builder(into)]
    #[serde(rename = "pauseTimeBetweenBatches")]
    pub r#pause_time_between_batches: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScaleSetRollingUpgradePolicy {
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
                    "max_batch_instance_percent",
                    &self.r#max_batch_instance_percent,
                ),
                to_pulumi_object_field(
                    "max_unhealthy_instance_percent",
                    &self.r#max_unhealthy_instance_percent,
                ),
                to_pulumi_object_field(
                    "max_unhealthy_upgraded_instance_percent",
                    &self.r#max_unhealthy_upgraded_instance_percent,
                ),
                to_pulumi_object_field(
                    "pause_time_between_batches",
                    &self.r#pause_time_between_batches,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScaleSetRollingUpgradePolicy {
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
                    r#max_batch_instance_percent: {
                        let field_value = match fields_map.get("max_batch_instance_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_batch_instance_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_unhealthy_instance_percent: {
                        let field_value = match fields_map.get("max_unhealthy_instance_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_unhealthy_instance_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_unhealthy_upgraded_instance_percent: {
                        let field_value = match fields_map.get("max_unhealthy_upgraded_instance_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_unhealthy_upgraded_instance_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pause_time_between_batches: {
                        let field_value = match fields_map.get("pause_time_between_batches") {
                            Some(value) => value,
                            None => bail!("Missing field 'pause_time_between_batches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
