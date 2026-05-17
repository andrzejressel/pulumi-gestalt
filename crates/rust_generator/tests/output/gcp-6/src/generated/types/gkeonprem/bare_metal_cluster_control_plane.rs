#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterControlPlane {
    /// Customizes the default API server args. Only a subset of
    /// customized flags are supported. Please refer to the API server
    /// documentation below to know the exact format:
    /// https://kubernetes.io/docs/reference/command-line-tools-reference/kube-apiserver/
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "apiServerArgs")]
    pub r#api_server_args: Option<Vec<super::super::types::gkeonprem::BareMetalClusterControlPlaneApiServerArg>>,
    /// Configures the node pool running the control plane. If specified the corresponding NodePool will be created for the cluster's control plane. The NodePool will have the same name and namespace as the cluster.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "controlPlaneNodePoolConfig")]
    pub r#control_plane_node_pool_config: Box<super::super::types::gkeonprem::BareMetalClusterControlPlaneControlPlaneNodePoolConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BareMetalClusterControlPlane {
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
                    "api_server_args",
                    &self.r#api_server_args,
                ),
                to_pulumi_object_field(
                    "control_plane_node_pool_config",
                    &self.r#control_plane_node_pool_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BareMetalClusterControlPlane {
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
                    r#api_server_args: {
                        let field_value = match fields_map.get("api_server_args") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_server_args' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#control_plane_node_pool_config: {
                        let field_value = match fields_map.get("control_plane_node_pool_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'control_plane_node_pool_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
