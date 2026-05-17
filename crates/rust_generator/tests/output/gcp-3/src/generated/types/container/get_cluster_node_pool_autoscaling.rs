#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodePoolAutoscaling {
    /// Location policy specifies the algorithm used when scaling-up the node pool. "BALANCED" - Is a best effort policy that aims to balance the sizes of available zones. "ANY" - Instructs the cluster autoscaler to prioritize utilization of unused reservations, and reduces preemption risk for Spot VMs.
    #[builder(into)]
    #[serde(rename = "locationPolicy")]
    pub r#location_policy: String,
    /// Maximum number of nodes per zone in the node pool. Must be >= min_node_count. Cannot be used with total limits.
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: i32,
    /// Minimum number of nodes per zone in the node pool. Must be >=0 and <= max_node_count. Cannot be used with total limits.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: i32,
    /// Maximum number of all nodes in the node pool. Must be >= total_min_node_count. Cannot be used with per zone limits.
    #[builder(into)]
    #[serde(rename = "totalMaxNodeCount")]
    pub r#total_max_node_count: i32,
    /// Minimum number of all nodes in the node pool. Must be >=0 and <= total_max_node_count. Cannot be used with per zone limits.
    #[builder(into)]
    #[serde(rename = "totalMinNodeCount")]
    pub r#total_min_node_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterNodePoolAutoscaling {
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
                    "location_policy",
                    &self.r#location_policy,
                ),
                to_pulumi_object_field(
                    "max_node_count",
                    &self.r#max_node_count,
                ),
                to_pulumi_object_field(
                    "min_node_count",
                    &self.r#min_node_count,
                ),
                to_pulumi_object_field(
                    "total_max_node_count",
                    &self.r#total_max_node_count,
                ),
                to_pulumi_object_field(
                    "total_min_node_count",
                    &self.r#total_min_node_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterNodePoolAutoscaling {
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
                    r#location_policy: {
                        let field_value = match fields_map.get("location_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'location_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_node_count: {
                        let field_value = match fields_map.get("max_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_node_count: {
                        let field_value = match fields_map.get("min_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_max_node_count: {
                        let field_value = match fields_map.get("total_max_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_max_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_min_node_count: {
                        let field_value = match fields_map.get("total_min_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_min_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
