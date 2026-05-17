#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterLoadBalancerManualLbConfig {
    /// NodePort for control plane service. The Kubernetes API server in the admin
    /// cluster is implemented as a Service of type NodePort (ex. 30968).
    #[builder(into)]
    #[serde(rename = "controlPlaneNodePort")]
    pub r#control_plane_node_port: Option<i32>,
    /// NodePort for ingress service's http. The ingress service in the admin
    /// cluster is implemented as a Service of type NodePort (ex. 32527).
    #[builder(into)]
    #[serde(rename = "ingressHttpNodePort")]
    pub r#ingress_http_node_port: Option<i32>,
    /// NodePort for ingress service's https. The ingress service in the admin
    /// cluster is implemented as a Service of type NodePort (ex. 30139).
    #[builder(into)]
    #[serde(rename = "ingressHttpsNodePort")]
    pub r#ingress_https_node_port: Option<i32>,
    /// NodePort for konnectivity server service running as a sidecar in each
    /// kube-apiserver pod (ex. 30564).
    #[builder(into)]
    #[serde(rename = "konnectivityServerNodePort")]
    pub r#konnectivity_server_node_port: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VMwareClusterLoadBalancerManualLbConfig {
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
                    "control_plane_node_port",
                    &self.r#control_plane_node_port,
                ),
                to_pulumi_object_field(
                    "ingress_http_node_port",
                    &self.r#ingress_http_node_port,
                ),
                to_pulumi_object_field(
                    "ingress_https_node_port",
                    &self.r#ingress_https_node_port,
                ),
                to_pulumi_object_field(
                    "konnectivity_server_node_port",
                    &self.r#konnectivity_server_node_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VMwareClusterLoadBalancerManualLbConfig {
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
                    r#control_plane_node_port: {
                        let field_value = match fields_map.get("control_plane_node_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'control_plane_node_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_http_node_port: {
                        let field_value = match fields_map.get("ingress_http_node_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_http_node_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_https_node_port: {
                        let field_value = match fields_map.get("ingress_https_node_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_https_node_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#konnectivity_server_node_port: {
                        let field_value = match fields_map.get("konnectivity_server_node_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'konnectivity_server_node_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
