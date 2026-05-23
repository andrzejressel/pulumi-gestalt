#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NextGenerationFirewallVirtualNetworkLocalRulestackNetworkProfile {
    /// Specifies a list of Azure Public IP Address IDs that can be used for Egress (Source) Network Address Translation.
    #[builder(into)]
    pub r#egress_nat_ip_address_ids: Option<Vec<String>>,
    #[builder(into)]
    pub r#egress_nat_ip_addresses: Option<Vec<String>>,
    /// Specifies a list of Azure Public IP Address IDs.
    #[builder(into)]
    pub r#public_ip_address_ids: Vec<String>,
    #[builder(into)]
    pub r#public_ip_addresses: Option<Vec<String>>,
    /// Specifies a list of trusted ranges to use for the Network.
    #[builder(into)]
    pub r#trusted_address_ranges: Option<Vec<String>>,
    /// A `vnet_configuration` block as defined below.
    #[builder(into)]
    pub r#vnet_configuration: Box<super::super::types::paloalto::NextGenerationFirewallVirtualNetworkLocalRulestackNetworkProfileVnetConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NextGenerationFirewallVirtualNetworkLocalRulestackNetworkProfile {
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
                    "egressNatIpAddressIds",
                    &self.r#egress_nat_ip_address_ids,
                ),
                to_pulumi_object_field(
                    "egressNatIpAddresses",
                    &self.r#egress_nat_ip_addresses,
                ),
                to_pulumi_object_field(
                    "publicIpAddressIds",
                    &self.r#public_ip_address_ids,
                ),
                to_pulumi_object_field(
                    "publicIpAddresses",
                    &self.r#public_ip_addresses,
                ),
                to_pulumi_object_field(
                    "trustedAddressRanges",
                    &self.r#trusted_address_ranges,
                ),
                to_pulumi_object_field(
                    "vnetConfiguration",
                    &self.r#vnet_configuration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NextGenerationFirewallVirtualNetworkLocalRulestackNetworkProfile {
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
                    r#egress_nat_ip_address_ids: {
                        let field_value = match fields_map.get("egressNatIpAddressIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'egressNatIpAddressIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#egress_nat_ip_addresses: {
                        let field_value = match fields_map.get("egressNatIpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'egressNatIpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_address_ids: {
                        let field_value = match fields_map.get("publicIpAddressIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddressIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_addresses: {
                        let field_value = match fields_map.get("publicIpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_address_ranges: {
                        let field_value = match fields_map.get("trustedAddressRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'trustedAddressRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_configuration: {
                        let field_value = match fields_map.get("vnetConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnetConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
