#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterNetworkConfig {
    /// Enables the use of advanced Anthos networking features, such as Bundled
    /// Load Balancing with BGP or the egress NAT gateway.
    /// Setting configuration for advanced networking features will automatically
    /// set this flag.
    #[builder(into)]
    #[serde(rename = "advancedNetworking")]
    pub r#advanced_networking: Option<bool>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "islandModeCidr")]
    pub r#island_mode_cidr: Option<Box<super::super::types::gkeonprem::BareMetalClusterNetworkConfigIslandModeCidr>>,
    /// Configuration for multiple network interfaces.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "multipleNetworkInterfacesConfig")]
    pub r#multiple_network_interfaces_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterNetworkConfigMultipleNetworkInterfacesConfig>>,
    /// Configuration for SR-IOV.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "srIovConfig")]
    pub r#sr_iov_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterNetworkConfigSrIovConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BareMetalClusterNetworkConfig {
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
                "advanced_networking".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advanced_networking,
                )
                .await,
            );
            map.insert(
                "island_mode_cidr".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#island_mode_cidr,
                )
                .await,
            );
            map.insert(
                "multiple_network_interfaces_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#multiple_network_interfaces_config,
                )
                .await,
            );
            map.insert(
                "sr_iov_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sr_iov_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BareMetalClusterNetworkConfig {
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
                    r#advanced_networking: {
                        let field_value = match fields_map.get("advanced_networking") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_networking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#island_mode_cidr: {
                        let field_value = match fields_map.get("island_mode_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'island_mode_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiple_network_interfaces_config: {
                        let field_value = match fields_map.get("multiple_network_interfaces_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiple_network_interfaces_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sr_iov_config: {
                        let field_value = match fields_map.get("sr_iov_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'sr_iov_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
