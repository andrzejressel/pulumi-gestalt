#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPrivateCloudManagementClusterAutoscalingSetting {
    /// The map with autoscaling policies applied to the cluster.
    /// The key is the identifier of the policy.
    /// It must meet the following requirements:
    ///  * Only contains 1-63 alphanumeric characters and hyphens
    ///  * Begins with an alphabetical character
    ///  * Ends with a non-hyphen character
    ///  * Not formatted as a UUID
    ///  * Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)
    /// 
    /// Currently the map must contain only one element
    /// that describes the autoscaling policy for compute nodes.
    #[builder(into)]
    #[serde(rename = "autoscalingPolicies")]
    pub r#autoscaling_policies: Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicy>,
    /// The minimum duration between consecutive autoscale operations.
    /// It starts once addition or removal of nodes is fully completed.
    /// Minimum cool down period is 30m.
    /// Cool down period must be in whole minutes (for example, 30m, 31m, 50m).
    /// Mandatory for successful addition of autoscaling settings in cluster.
    #[builder(into)]
    #[serde(rename = "coolDownPeriod")]
    pub r#cool_down_period: String,
    /// Maximum number of nodes of any type in a cluster.
    /// Mandatory for successful addition of autoscaling settings in cluster.
    #[builder(into)]
    #[serde(rename = "maxClusterNodeCount")]
    pub r#max_cluster_node_count: i32,
    /// Minimum number of nodes of any type in a cluster.
    /// Mandatory for successful addition of autoscaling settings in cluster.
    #[builder(into)]
    #[serde(rename = "minClusterNodeCount")]
    pub r#min_cluster_node_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPrivateCloudManagementClusterAutoscalingSetting {
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
                    "autoscaling_policies",
                    &self.r#autoscaling_policies,
                ),
                to_pulumi_object_field(
                    "cool_down_period",
                    &self.r#cool_down_period,
                ),
                to_pulumi_object_field(
                    "max_cluster_node_count",
                    &self.r#max_cluster_node_count,
                ),
                to_pulumi_object_field(
                    "min_cluster_node_count",
                    &self.r#min_cluster_node_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPrivateCloudManagementClusterAutoscalingSetting {
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
                    r#autoscaling_policies: {
                        let field_value = match fields_map.get("autoscaling_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cool_down_period: {
                        let field_value = match fields_map.get("cool_down_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'cool_down_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_cluster_node_count: {
                        let field_value = match fields_map.get("max_cluster_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_cluster_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_cluster_node_count: {
                        let field_value = match fields_map.get("min_cluster_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_cluster_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
