#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscalerAutoscalingPolicyMetric {
    /// A filter string to be used as the filter string for
    /// a Stackdriver Monitoring TimeSeries.list API call.
    /// This filter is used to select a specific TimeSeries for
    /// the purpose of autoscaling and to determine whether the metric
    /// is exporting per-instance or per-group data.
    /// You can only use the AND operator for joining selectors.
    /// You can only use direct equality comparison operator (=) without
    /// any functions for each selector.
    /// You can specify the metric in both the filter string and in the
    /// metric field. However, if specified in both places, the metric must
    /// be identical.
    /// The monitored resource type determines what kind of values are
    /// expected for the metric. If it is a gce_instance, the autoscaler
    /// expects the metric to include a separate TimeSeries for each
    /// instance in a group. In such a case, you cannot filter on resource
    /// labels.
    /// If the resource type is any other value, the autoscaler expects
    /// this metric to contain values that apply to the entire autoscaled
    /// instance group and resource label filtering can be performed to
    /// point autoscaler at the correct TimeSeries to scale upon.
    /// This is called a per-group metric for the purpose of autoscaling.
    /// If not specified, the type defaults to gce_instance.
    /// You should provide a filter that is selective enough to pick just
    /// one TimeSeries for the autoscaled group or for each of the instances
    /// (if you are using gce_instance resource type). If multiple
    /// TimeSeries are returned upon the query execution, the autoscaler
    /// will sum their respective values to obtain its scaling value.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<String>,
    /// The identifier (type) of the Stackdriver Monitoring metric.
    /// The metric cannot have negative values.
    /// The metric must have a value type of INT64 or DOUBLE.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// If scaling is based on a per-group metric value that represents the
    /// total amount of work to be done or resource usage, set this value to
    /// an amount assigned for a single instance of the scaled group.
    /// The autoscaler will keep the number of instances proportional to the
    /// value of this metric, the metric itself should not change value due
    /// to group resizing.
    /// For example, a good metric to use with the target is
    /// `pubsub.googleapis.com/subscription/num_undelivered_messages`
    /// or a custom metric exporting the total number of requests coming to
    /// your instances.
    /// A bad example would be a metric exporting an average or median
    /// latency, since this value can't include a chunk assignable to a
    /// single instance, it could be better used with utilization_target
    /// instead.
    #[builder(into)]
    #[serde(rename = "singleInstanceAssignment")]
    pub r#single_instance_assignment: Option<f64>,
    /// The target value of the metric that autoscaler should
    /// maintain. This must be a positive value. A utilization
    /// metric scales number of virtual machines handling requests
    /// to increase or decrease proportionally to the metric.
    /// For example, a good metric to use as a utilizationTarget is
    /// www.googleapis.com/compute/instance/network/received_bytes_count.
    /// The autoscaler will work to keep this value constant for each
    /// of the instances.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<f64>,
    /// Defines how target utilization value is expressed for a
    /// Stackdriver Monitoring metric.
    /// Possible values are: `GAUGE`, `DELTA_PER_SECOND`, `DELTA_PER_MINUTE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscalerAutoscalingPolicyMetric {
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
                    "filter",
                    &self.r#filter,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "single_instance_assignment",
                    &self.r#single_instance_assignment,
                ),
                to_pulumi_object_field(
                    "target",
                    &self.r#target,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutoscalerAutoscalingPolicyMetric {
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
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#single_instance_assignment: {
                        let field_value = match fields_map.get("single_instance_assignment") {
                            Some(value) => value,
                            None => bail!("Missing field 'single_instance_assignment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
