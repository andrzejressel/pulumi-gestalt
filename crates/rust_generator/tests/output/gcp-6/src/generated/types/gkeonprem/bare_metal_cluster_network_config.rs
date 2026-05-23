#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterNetworkConfig {
    /// Enables the use of advanced Anthos networking features, such as Bundled
    /// Load Balancing with BGP or the egress NAT gateway.
    /// Setting configuration for advanced networking features will automatically
    /// set this flag.
    #[builder(into)]
    pub r#advanced_networking: Option<bool>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    pub r#island_mode_cidr: Option<Box<super::super::types::gkeonprem::BareMetalClusterNetworkConfigIslandModeCidr>>,
    /// Configuration for multiple network interfaces.
    /// Structure is documented below.
    #[builder(into)]
    pub r#multiple_network_interfaces_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterNetworkConfigMultipleNetworkInterfacesConfig>>,
    /// Configuration for SR-IOV.
    /// Structure is documented below.
    #[builder(into)]
    pub r#sr_iov_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterNetworkConfigSrIovConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BareMetalClusterNetworkConfig {
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
                    "advancedNetworking",
                    &self.r#advanced_networking,
                ),
                to_pulumi_object_field(
                    "islandModeCidr",
                    &self.r#island_mode_cidr,
                ),
                to_pulumi_object_field(
                    "multipleNetworkInterfacesConfig",
                    &self.r#multiple_network_interfaces_config,
                ),
                to_pulumi_object_field(
                    "srIovConfig",
                    &self.r#sr_iov_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("advancedNetworking") {
                            Some(value) => value,
                            None => bail!("Missing field 'advancedNetworking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#island_mode_cidr: {
                        let field_value = match fields_map.get("islandModeCidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'islandModeCidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiple_network_interfaces_config: {
                        let field_value = match fields_map.get("multipleNetworkInterfacesConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'multipleNetworkInterfacesConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sr_iov_config: {
                        let field_value = match fields_map.get("srIovConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'srIovConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
