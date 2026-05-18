#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ComputeClusterScaleSettings {
    /// Maximum node count. Changing this forces a new Machine Learning Compute Cluster to be created.
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: i32,
    /// Minimal node count. Changing this forces a new Machine Learning Compute Cluster to be created.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: i32,
    /// Node Idle Time Before Scale Down: defines the time until the compute is shutdown when it has gone into Idle state. Is defined according to W3C XML schema standard for duration. Changing this forces a new Machine Learning Compute Cluster to be created.
    #[builder(into)]
    #[serde(rename = "scaleDownNodesAfterIdleDuration")]
    pub r#scale_down_nodes_after_idle_duration: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ComputeClusterScaleSettings {
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
                    "max_node_count",
                    &self.r#max_node_count,
                ),
                to_pulumi_object_field(
                    "min_node_count",
                    &self.r#min_node_count,
                ),
                to_pulumi_object_field(
                    "scale_down_nodes_after_idle_duration",
                    &self.r#scale_down_nodes_after_idle_duration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ComputeClusterScaleSettings {
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
                    r#scale_down_nodes_after_idle_duration: {
                        let field_value = match fields_map.get("scale_down_nodes_after_idle_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_down_nodes_after_idle_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
