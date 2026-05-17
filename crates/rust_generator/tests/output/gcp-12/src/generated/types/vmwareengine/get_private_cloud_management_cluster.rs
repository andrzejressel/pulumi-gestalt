#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPrivateCloudManagementCluster {
    /// Configuration of the autoscaling applied to this cluster
    /// Private cloud must have a minimum of 3 nodes to add autoscale settings
    #[builder(into)]
    #[serde(rename = "autoscalingSettings")]
    pub r#autoscaling_settings: Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterAutoscalingSetting>,
    /// The user-provided identifier of the new Cluster. The identifier must meet the following requirements:
    ///   * Only contains 1-63 alphanumeric characters and hyphens
    ///   * Begins with an alphabetical character
    ///   * Ends with a non-hyphen character
    ///   * Not formatted as a UUID
    ///   * Complies with RFC 1034 (https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)
    #[builder(into)]
    #[serde(rename = "clusterId")]
    pub r#cluster_id: String,
    /// The map of cluster node types in this cluster,
    /// where the key is canonical identifier of the node type (corresponds to the NodeType).
    #[builder(into)]
    #[serde(rename = "nodeTypeConfigs")]
    pub r#node_type_configs: Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterNodeTypeConfig>,
    /// The stretched cluster configuration for the private cloud.
    #[builder(into)]
    #[serde(rename = "stretchedClusterConfigs")]
    pub r#stretched_cluster_configs: Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterStretchedClusterConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPrivateCloudManagementCluster {
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
                    "autoscaling_settings",
                    &self.r#autoscaling_settings,
                ),
                to_pulumi_object_field(
                    "cluster_id",
                    &self.r#cluster_id,
                ),
                to_pulumi_object_field(
                    "node_type_configs",
                    &self.r#node_type_configs,
                ),
                to_pulumi_object_field(
                    "stretched_cluster_configs",
                    &self.r#stretched_cluster_configs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPrivateCloudManagementCluster {
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
                    r#autoscaling_settings: {
                        let field_value = match fields_map.get("autoscaling_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_id: {
                        let field_value = match fields_map.get("cluster_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_type_configs: {
                        let field_value = match fields_map.get("node_type_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_type_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stretched_cluster_configs: {
                        let field_value = match fields_map.get("stretched_cluster_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'stretched_cluster_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
