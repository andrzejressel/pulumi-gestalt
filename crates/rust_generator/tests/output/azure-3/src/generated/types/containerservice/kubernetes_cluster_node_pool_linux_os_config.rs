#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterNodePoolLinuxOsConfig {
    /// Specifies the size of swap file on each node in MB. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "swapFileSizeMb")]
    pub r#swap_file_size_mb: Option<i32>,
    /// A `sysctl_config` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sysctlConfig")]
    pub r#sysctl_config: Option<Box<super::super::types::containerservice::KubernetesClusterNodePoolLinuxOsConfigSysctlConfig>>,
    /// specifies the defrag configuration for Transparent Huge Page. Possible values are `always`, `defer`, `defer+madvise`, `madvise` and `never`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "transparentHugePageDefrag")]
    pub r#transparent_huge_page_defrag: Option<String>,
    /// Specifies the Transparent Huge Page enabled configuration. Possible values are `always`, `madvise` and `never`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "transparentHugePageEnabled")]
    pub r#transparent_huge_page_enabled: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterNodePoolLinuxOsConfig {
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
                    "swap_file_size_mb",
                    &self.r#swap_file_size_mb,
                ),
                to_pulumi_object_field(
                    "sysctl_config",
                    &self.r#sysctl_config,
                ),
                to_pulumi_object_field(
                    "transparent_huge_page_defrag",
                    &self.r#transparent_huge_page_defrag,
                ),
                to_pulumi_object_field(
                    "transparent_huge_page_enabled",
                    &self.r#transparent_huge_page_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterNodePoolLinuxOsConfig {
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
                    r#swap_file_size_mb: {
                        let field_value = match fields_map.get("swap_file_size_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'swap_file_size_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sysctl_config: {
                        let field_value = match fields_map.get("sysctl_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'sysctl_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transparent_huge_page_defrag: {
                        let field_value = match fields_map.get("transparent_huge_page_defrag") {
                            Some(value) => value,
                            None => bail!("Missing field 'transparent_huge_page_defrag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transparent_huge_page_enabled: {
                        let field_value = match fields_map.get("transparent_huge_page_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'transparent_huge_page_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
