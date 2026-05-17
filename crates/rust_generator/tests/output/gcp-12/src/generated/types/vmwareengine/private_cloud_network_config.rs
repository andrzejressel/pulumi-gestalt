#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PrivateCloudNetworkConfig {
    /// (Output)
    /// DNS Server IP of the Private Cloud.
    #[builder(into)]
    #[serde(rename = "dnsServerIp")]
    pub r#dns_server_ip: Option<String>,
    /// Management CIDR used by VMware management appliances.
    #[builder(into)]
    #[serde(rename = "managementCidr")]
    pub r#management_cidr: String,
    /// (Output)
    /// The IP address layout version of the management IP address range.
    /// Possible versions include:
    /// * managementIpAddressLayoutVersion=1: Indicates the legacy IP address layout used by some existing private clouds. This is no longer supported for new private clouds
    /// as it does not support all features.
    /// * managementIpAddressLayoutVersion=2: Indicates the latest IP address layout
    /// used by all newly created private clouds. This version supports all current features.
    #[builder(into)]
    #[serde(rename = "managementIpAddressLayoutVersion")]
    pub r#management_ip_address_layout_version: Option<i32>,
    /// The relative resource name of the VMware Engine network attached to the private cloud.
    /// Specify the name in the following form: projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
    /// where {project} can either be a project number or a project ID.
    #[builder(into)]
    #[serde(rename = "vmwareEngineNetwork")]
    pub r#vmware_engine_network: Option<String>,
    /// (Output)
    /// The canonical name of the VMware Engine network in
    /// the form: projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
    #[builder(into)]
    #[serde(rename = "vmwareEngineNetworkCanonical")]
    pub r#vmware_engine_network_canonical: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PrivateCloudNetworkConfig {
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
                "dns_server_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_server_ip,
                )
                .await,
            );
            map.insert(
                "management_cidr".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#management_cidr,
                )
                .await,
            );
            map.insert(
                "management_ip_address_layout_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#management_ip_address_layout_version,
                )
                .await,
            );
            map.insert(
                "vmware_engine_network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vmware_engine_network,
                )
                .await,
            );
            map.insert(
                "vmware_engine_network_canonical".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vmware_engine_network_canonical,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PrivateCloudNetworkConfig {
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
                    r#dns_server_ip: {
                        let field_value = match fields_map.get("dns_server_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_server_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#management_cidr: {
                        let field_value = match fields_map.get("management_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'management_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#management_ip_address_layout_version: {
                        let field_value = match fields_map.get("management_ip_address_layout_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'management_ip_address_layout_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vmware_engine_network: {
                        let field_value = match fields_map.get("vmware_engine_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmware_engine_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vmware_engine_network_canonical: {
                        let field_value = match fields_map.get("vmware_engine_network_canonical") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmware_engine_network_canonical' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
