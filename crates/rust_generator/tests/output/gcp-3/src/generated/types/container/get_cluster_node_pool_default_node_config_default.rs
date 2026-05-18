#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodePoolDefaultNodeConfigDefault {
    /// Parameters for containerd configuration.
    #[builder(into)]
    #[serde(rename = "containerdConfigs")]
    pub r#containerd_configs: Vec<super::super::types::container::GetClusterNodePoolDefaultNodeConfigDefaultContainerdConfig>,
    /// GCFS configuration for this node.
    #[builder(into)]
    #[serde(rename = "gcfsConfigs")]
    pub r#gcfs_configs: Vec<super::super::types::container::GetClusterNodePoolDefaultNodeConfigDefaultGcfsConfig>,
    /// Controls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`.
    #[builder(into)]
    #[serde(rename = "insecureKubeletReadonlyPortEnabled")]
    pub r#insecure_kubelet_readonly_port_enabled: String,
    /// Type of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT.
    #[builder(into)]
    #[serde(rename = "loggingVariant")]
    pub r#logging_variant: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterNodePoolDefaultNodeConfigDefault {
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
                    "containerd_configs",
                    &self.r#containerd_configs,
                ),
                to_pulumi_object_field(
                    "gcfs_configs",
                    &self.r#gcfs_configs,
                ),
                to_pulumi_object_field(
                    "insecure_kubelet_readonly_port_enabled",
                    &self.r#insecure_kubelet_readonly_port_enabled,
                ),
                to_pulumi_object_field(
                    "logging_variant",
                    &self.r#logging_variant,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterNodePoolDefaultNodeConfigDefault {
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
                    r#containerd_configs: {
                        let field_value = match fields_map.get("containerd_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerd_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcfs_configs: {
                        let field_value = match fields_map.get("gcfs_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcfs_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#insecure_kubelet_readonly_port_enabled: {
                        let field_value = match fields_map.get("insecure_kubelet_readonly_port_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'insecure_kubelet_readonly_port_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging_variant: {
                        let field_value = match fields_map.get("logging_variant") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging_variant' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
