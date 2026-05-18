#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AzureNodePoolConfig {
    /// The OS image type to use on node pool instances.
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: Option<String>,
    /// Optional. The initial labels assigned to nodes of this node pool. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Proxy configuration for outbound HTTP(S) traffic.
    #[builder(into)]
    #[serde(rename = "proxyConfig")]
    pub r#proxy_config: Option<Box<super::super::types::container::AzureNodePoolConfigProxyConfig>>,
    /// Optional. Configuration related to the root volume provisioned for each node pool machine. When unspecified, it defaults to a 32-GiB Azure Disk.
    #[builder(into)]
    #[serde(rename = "rootVolume")]
    pub r#root_volume: Option<Box<super::super::types::container::AzureNodePoolConfigRootVolume>>,
    /// SSH configuration for how to access the node pool machines.
    #[builder(into)]
    #[serde(rename = "sshConfig")]
    pub r#ssh_config: Box<super::super::types::container::AzureNodePoolConfigSshConfig>,
    /// Optional. A set of tags to apply to all underlying Azure resources for this node pool. This currently only includes Virtual Machine Scale Sets. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// Optional. The Azure VM size name. Example: `Standard_DS2_v2`. See (/anthos/clusters/docs/azure/reference/supported-vms) for options. When unspecified, it defaults to `Standard_DS2_v2`.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AzureNodePoolConfig {
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
                    "image_type",
                    &self.r#image_type,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "proxy_config",
                    &self.r#proxy_config,
                ),
                to_pulumi_object_field(
                    "root_volume",
                    &self.r#root_volume,
                ),
                to_pulumi_object_field(
                    "ssh_config",
                    &self.r#ssh_config,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "vm_size",
                    &self.r#vm_size,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AzureNodePoolConfig {
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
                    r#image_type: {
                        let field_value = match fields_map.get("image_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_config: {
                        let field_value = match fields_map.get("proxy_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxy_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_volume: {
                        let field_value = match fields_map.get("root_volume") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_volume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_config: {
                        let field_value = match fields_map.get("ssh_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_size: {
                        let field_value = match fields_map.get("vm_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
