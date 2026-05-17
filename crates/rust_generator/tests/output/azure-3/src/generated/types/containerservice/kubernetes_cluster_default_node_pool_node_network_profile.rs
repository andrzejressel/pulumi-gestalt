#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterDefaultNodePoolNodeNetworkProfile {
    /// One or more `allowed_host_ports` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "allowedHostPorts")]
    pub r#allowed_host_ports: Option<Vec<super::super::types::containerservice::KubernetesClusterDefaultNodePoolNodeNetworkProfileAllowedHostPort>>,
    /// A list of Application Security Group IDs which should be associated with this Node Pool.
    #[builder(into)]
    #[serde(rename = "applicationSecurityGroupIds")]
    pub r#application_security_group_ids: Option<Vec<String>>,
    /// Specifies a mapping of tags to the instance-level public IPs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "nodePublicIpTags")]
    pub r#node_public_ip_tags: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterDefaultNodePoolNodeNetworkProfile {
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
                "allowed_host_ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_host_ports,
                )
                .await,
            );
            map.insert(
                "application_security_group_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_security_group_ids,
                )
                .await,
            );
            map.insert(
                "node_public_ip_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_public_ip_tags,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterDefaultNodePoolNodeNetworkProfile {
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
                    r#allowed_host_ports: {
                        let field_value = match fields_map.get("allowed_host_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_host_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_security_group_ids: {
                        let field_value = match fields_map.get("application_security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_public_ip_tags: {
                        let field_value = match fields_map.get("node_public_ip_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_public_ip_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
