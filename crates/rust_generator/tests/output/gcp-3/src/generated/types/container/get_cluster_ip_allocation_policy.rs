#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterIpAllocationPolicy {
    /// AdditionalPodRangesConfig is the configuration for additional pod secondary ranges supporting the ClusterUpdate message.
    #[builder(into)]
    #[serde(rename = "additionalPodRangesConfigs")]
    pub r#additional_pod_ranges_configs: Vec<super::super::types::container::GetClusterIpAllocationPolicyAdditionalPodRangesConfig>,
    /// The IP address range for the cluster pod IPs. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use.
    #[builder(into)]
    #[serde(rename = "clusterIpv4CidrBlock")]
    pub r#cluster_ipv_4_cidr_block: String,
    /// The name of the existing secondary range in the cluster's subnetwork to use for pod IP addresses. Alternatively, cluster_ipv4_cidr_block can be used to automatically create a GKE-managed one.
    #[builder(into)]
    #[serde(rename = "clusterSecondaryRangeName")]
    pub r#cluster_secondary_range_name: String,
    /// Configuration for cluster level pod cidr overprovision. Default is disabled=false.
    #[builder(into)]
    #[serde(rename = "podCidrOverprovisionConfigs")]
    pub r#pod_cidr_overprovision_configs: Vec<super::super::types::container::GetClusterIpAllocationPolicyPodCidrOverprovisionConfig>,
    /// The IP address range of the services IPs in this cluster. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use.
    #[builder(into)]
    #[serde(rename = "servicesIpv4CidrBlock")]
    pub r#services_ipv_4_cidr_block: String,
    /// The name of the existing secondary range in the cluster's subnetwork to use for service ClusterIPs. Alternatively, services_ipv4_cidr_block can be used to automatically create a GKE-managed one.
    #[builder(into)]
    #[serde(rename = "servicesSecondaryRangeName")]
    pub r#services_secondary_range_name: String,
    /// The IP Stack type of the cluster. Choose between IPV4 and IPV4_IPV6. Default type is IPV4 Only if not set
    #[builder(into)]
    #[serde(rename = "stackType")]
    pub r#stack_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterIpAllocationPolicy {
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
                "additional_pod_ranges_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_pod_ranges_configs,
                )
                .await,
            );
            map.insert(
                "cluster_ipv_4_cidr_block".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_ipv_4_cidr_block,
                )
                .await,
            );
            map.insert(
                "cluster_secondary_range_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_secondary_range_name,
                )
                .await,
            );
            map.insert(
                "pod_cidr_overprovision_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pod_cidr_overprovision_configs,
                )
                .await,
            );
            map.insert(
                "services_ipv_4_cidr_block".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#services_ipv_4_cidr_block,
                )
                .await,
            );
            map.insert(
                "services_secondary_range_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#services_secondary_range_name,
                )
                .await,
            );
            map.insert(
                "stack_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stack_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterIpAllocationPolicy {
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
                    r#additional_pod_ranges_configs: {
                        let field_value = match fields_map.get("additional_pod_ranges_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_pod_ranges_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_ipv_4_cidr_block: {
                        let field_value = match fields_map.get("cluster_ipv_4_cidr_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_ipv_4_cidr_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_secondary_range_name: {
                        let field_value = match fields_map.get("cluster_secondary_range_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_secondary_range_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_cidr_overprovision_configs: {
                        let field_value = match fields_map.get("pod_cidr_overprovision_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_cidr_overprovision_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#services_ipv_4_cidr_block: {
                        let field_value = match fields_map.get("services_ipv_4_cidr_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'services_ipv_4_cidr_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#services_secondary_range_name: {
                        let field_value = match fields_map.get("services_secondary_range_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'services_secondary_range_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stack_type: {
                        let field_value = match fields_map.get("stack_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'stack_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
