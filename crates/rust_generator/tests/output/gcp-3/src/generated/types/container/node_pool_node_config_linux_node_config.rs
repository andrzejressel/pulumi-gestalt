#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodePoolNodeConfigLinuxNodeConfig {
    /// cgroupMode specifies the cgroup mode to be used on the node.
    #[builder(into)]
    #[serde(rename = "cgroupMode")]
    pub r#cgroup_mode: Option<String>,
    /// Amounts for 2M and 1G hugepages.
    #[builder(into)]
    #[serde(rename = "hugepagesConfig")]
    pub r#hugepages_config: Option<Box<super::super::types::container::NodePoolNodeConfigLinuxNodeConfigHugepagesConfig>>,
    /// The Linux kernel parameters to be applied to the nodes and all pods running on the nodes.
    #[builder(into)]
    #[serde(rename = "sysctls")]
    pub r#sysctls: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NodePoolNodeConfigLinuxNodeConfig {
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
                "cgroup_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cgroup_mode,
                )
                .await,
            );
            map.insert(
                "hugepages_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hugepages_config,
                )
                .await,
            );
            map.insert(
                "sysctls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sysctls,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NodePoolNodeConfigLinuxNodeConfig {
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
                    r#cgroup_mode: {
                        let field_value = match fields_map.get("cgroup_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'cgroup_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hugepages_config: {
                        let field_value = match fields_map.get("hugepages_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'hugepages_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sysctls: {
                        let field_value = match fields_map.get("sysctls") {
                            Some(value) => value,
                            None => bail!("Missing field 'sysctls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
