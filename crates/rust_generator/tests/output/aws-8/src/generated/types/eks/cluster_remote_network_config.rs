#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterRemoteNetworkConfig {
    /// Configuration block with remote node network configuration for EKS Hybrid Nodes. Detailed below.
    #[builder(into)]
    #[serde(rename = "remoteNodeNetworks")]
    pub r#remote_node_networks: Box<super::super::types::eks::ClusterRemoteNetworkConfigRemoteNodeNetworks>,
    /// Configuration block with remote pod network configuration for EKS Hybrid Nodes. Detailed below.
    #[builder(into)]
    #[serde(rename = "remotePodNetworks")]
    pub r#remote_pod_networks: Option<Box<super::super::types::eks::ClusterRemoteNetworkConfigRemotePodNetworks>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterRemoteNetworkConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "remote_node_networks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#remote_node_networks,
                )
                .await,
            );
            map.insert(
                "remote_pod_networks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#remote_pod_networks,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterRemoteNetworkConfig {
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
                    r#remote_node_networks: {
                        let field_value = match fields_map.get("remote_node_networks") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_node_networks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_pod_networks: {
                        let field_value = match fields_map.get("remote_pod_networks") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_pod_networks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
