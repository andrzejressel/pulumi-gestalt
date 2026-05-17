#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterDefaultNodePoolLinuxOsConfig {
    /// Specifies the size of the swap file on each node in MB.
    #[builder(into)]
    #[serde(rename = "swapFileSizeMb")]
    pub r#swap_file_size_mb: Option<i32>,
    /// A `sysctl_config` block as defined below.
    #[builder(into)]
    #[serde(rename = "sysctlConfig")]
    pub r#sysctl_config: Option<Box<super::super::types::containerservice::KubernetesClusterDefaultNodePoolLinuxOsConfigSysctlConfig>>,
    /// specifies the defrag configuration for Transparent Huge Page. Possible values are `always`, `defer`, `defer+madvise`, `madvise` and `never`.
    #[builder(into)]
    #[serde(rename = "transparentHugePageDefrag")]
    pub r#transparent_huge_page_defrag: Option<String>,
    /// Specifies the Transparent Huge Page enabled configuration. Possible values are `always`, `madvise` and `never`.
    #[builder(into)]
    #[serde(rename = "transparentHugePageEnabled")]
    pub r#transparent_huge_page_enabled: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterDefaultNodePoolLinuxOsConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "swap_file_size_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#swap_file_size_mb,
                )
                .await,
            );
            map.insert(
                "sysctl_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sysctl_config,
                )
                .await,
            );
            map.insert(
                "transparent_huge_page_defrag".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transparent_huge_page_defrag,
                )
                .await,
            );
            map.insert(
                "transparent_huge_page_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transparent_huge_page_enabled,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterDefaultNodePoolLinuxOsConfig {
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
