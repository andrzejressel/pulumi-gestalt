#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterPrivateClusterConfig {
    /// When `true`, the cluster's private
    /// endpoint is used as the cluster endpoint and access through the public endpoint
    /// is disabled. When `false`, either endpoint can be used. This field only applies
    /// to private clusters, when `enable_private_nodes` is `true`.
    #[builder(into)]
    #[serde(rename = "enablePrivateEndpoint")]
    pub r#enable_private_endpoint: Option<bool>,
    /// Enables the private cluster feature,
    /// creating a private endpoint on the cluster. In a private cluster, nodes only
    /// have RFC 1918 private addresses and communicate with the master's private
    /// endpoint via private networking.
    #[builder(into)]
    #[serde(rename = "enablePrivateNodes")]
    pub r#enable_private_nodes: Option<bool>,
    /// Controls cluster master global
    /// access settings. If unset, the provider will no longer manage this field and will
    /// not modify the previously-set value. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "masterGlobalAccessConfig")]
    pub r#master_global_access_config: Option<Box<super::super::types::container::ClusterPrivateClusterConfigMasterGlobalAccessConfig>>,
    /// The IP range in CIDR notation to use for
    /// the hosted master network. This range will be used for assigning private IP
    /// addresses to the cluster master(s) and the ILB VIP. This range must not overlap
    /// with any other ranges in use within the cluster's network, and it must be a /28
    /// subnet. See [Private Cluster Limitations](https://cloud.google.com/kubernetes-engine/docs/how-to/private-clusters#req_res_lim)
    /// for more details. This field only applies to private clusters, when
    /// `enable_private_nodes` is `true`.
    #[builder(into)]
    #[serde(rename = "masterIpv4CidrBlock")]
    pub r#master_ipv_4_cidr_block: Option<String>,
    /// The name of the peering between this cluster and the Google owned VPC.
    #[builder(into)]
    #[serde(rename = "peeringName")]
    pub r#peering_name: Option<String>,
    /// The internal IP address of this cluster's master endpoint.
    #[builder(into)]
    #[serde(rename = "privateEndpoint")]
    pub r#private_endpoint: Option<String>,
    /// Subnetwork in cluster's network where master's endpoint will be provisioned.
    #[builder(into)]
    #[serde(rename = "privateEndpointSubnetwork")]
    pub r#private_endpoint_subnetwork: Option<String>,
    /// The external IP address of this cluster's master endpoint.
    /// 
    /// !> The Google provider is unable to validate certain configurations of
    /// `private_cluster_config` when `enable_private_nodes` is `false`. It's
    /// recommended that you omit the block entirely if the field is not set to `true`.
    #[builder(into)]
    #[serde(rename = "publicEndpoint")]
    pub r#public_endpoint: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterPrivateClusterConfig {
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
                "enable_private_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_private_endpoint,
                )
                .await,
            );
            map.insert(
                "enable_private_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_private_nodes,
                )
                .await,
            );
            map.insert(
                "master_global_access_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#master_global_access_config,
                )
                .await,
            );
            map.insert(
                "master_ipv_4_cidr_block".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#master_ipv_4_cidr_block,
                )
                .await,
            );
            map.insert(
                "peering_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#peering_name,
                )
                .await,
            );
            map.insert(
                "private_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_endpoint,
                )
                .await,
            );
            map.insert(
                "private_endpoint_subnetwork".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_endpoint_subnetwork,
                )
                .await,
            );
            map.insert(
                "public_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_endpoint,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterPrivateClusterConfig {
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
                    r#enable_private_endpoint: {
                        let field_value = match fields_map.get("enable_private_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_private_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_private_nodes: {
                        let field_value = match fields_map.get("enable_private_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_private_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_global_access_config: {
                        let field_value = match fields_map.get("master_global_access_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_global_access_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_ipv_4_cidr_block: {
                        let field_value = match fields_map.get("master_ipv_4_cidr_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_ipv_4_cidr_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peering_name: {
                        let field_value = match fields_map.get("peering_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'peering_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoint: {
                        let field_value = match fields_map.get("private_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoint_subnetwork: {
                        let field_value = match fields_map.get("private_endpoint_subnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_endpoint_subnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_endpoint: {
                        let field_value = match fields_map.get("public_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
