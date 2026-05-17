#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterControlPlaneNode {
    /// AutoResizeConfig provides auto resizing configurations.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autoResizeConfig")]
    pub r#auto_resize_config: Option<Box<super::super::types::gkeonprem::VMwareClusterControlPlaneNodeAutoResizeConfig>>,
    /// The number of CPUs for each admin cluster node that serve as control planes
    /// for this VMware User Cluster. (default: 4 CPUs)
    #[builder(into)]
    #[serde(rename = "cpus")]
    pub r#cpus: Option<i32>,
    /// The megabytes of memory for each admin cluster node that serves as a
    /// control plane for this VMware User Cluster (default: 8192 MB memory).
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Option<i32>,
    /// The number of control plane nodes for this VMware User Cluster.
    /// (default: 1 replica).
    #[builder(into)]
    #[serde(rename = "replicas")]
    pub r#replicas: Option<i32>,
    /// (Output)
    /// Vsphere-specific config.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vsphereConfigs")]
    pub r#vsphere_configs: Option<Vec<super::super::types::gkeonprem::VMwareClusterControlPlaneNodeVsphereConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VMwareClusterControlPlaneNode {
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
                    "auto_resize_config",
                    &self.r#auto_resize_config,
                ),
                to_pulumi_object_field(
                    "cpus",
                    &self.r#cpus,
                ),
                to_pulumi_object_field(
                    "memory",
                    &self.r#memory,
                ),
                to_pulumi_object_field(
                    "replicas",
                    &self.r#replicas,
                ),
                to_pulumi_object_field(
                    "vsphere_configs",
                    &self.r#vsphere_configs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VMwareClusterControlPlaneNode {
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
                    r#auto_resize_config: {
                        let field_value = match fields_map.get("auto_resize_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_resize_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpus: {
                        let field_value = match fields_map.get("cpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory: {
                        let field_value = match fields_map.get("memory") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replicas: {
                        let field_value = match fields_map.get("replicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vsphere_configs: {
                        let field_value = match fields_map.get("vsphere_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'vsphere_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
