#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentCoreNetworkConfiguration {
    /// List of strings containing Autonomous System Numbers (ASNs) to assign to Core Network Edges. By default, the core network automatically assigns an ASN for each Core Network Edge but you can optionally define the ASN in the edge-locations for each Region. The ASN uses an array of integer ranges only from `64512` to `65534` and `4200000000` to `4294967294` expressed as a string like `"64512-65534"`. No other ASN ranges can be used.
    #[builder(into)]
    #[serde(rename = "asnRanges")]
    pub r#asn_ranges: Vec<String>,
    /// A block value of AWS Region locations where you're creating Core Network Edges. Detailed below.
    #[builder(into)]
    #[serde(rename = "edgeLocations")]
    pub r#edge_locations: Vec<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentCoreNetworkConfigurationEdgeLocation>,
    /// The Classless Inter-Domain Routing (CIDR) block range used to create tunnels for AWS Transit Gateway Connect. The format is standard AWS CIDR range (for example, `10.0.1.0/24`). You can optionally define the inside CIDR in the Core Network Edges section per Region. The minimum is a `/24` for IPv4 or `/64` for IPv6. You can provide multiple `/24` subnets or a larger CIDR range. If you define a larger CIDR range, new Core Network Edges will be automatically assigned `/24` and `/64` subnets from the larger CIDR. an Inside CIDR block is required for attaching Connect attachments to a Core Network Edge.
    #[builder(into)]
    #[serde(rename = "insideCidrBlocks")]
    pub r#inside_cidr_blocks: Option<Vec<String>>,
    /// Indicates whether the core network forwards traffic over multiple equal-cost routes using VPN. The value can be either `true` or `false`. The default is `true`.
    #[builder(into)]
    #[serde(rename = "vpnEcmpSupport")]
    pub r#vpn_ecmp_support: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCoreNetworkPolicyDocumentCoreNetworkConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("asn_ranges".to_string(), self.r#asn_ranges.to_pulumi_value().await);
            map.insert("edge_locations".to_string(), self.r#edge_locations.to_pulumi_value().await);
            map.insert("inside_cidr_blocks".to_string(), self.r#inside_cidr_blocks.to_pulumi_value().await);
            map.insert("vpn_ecmp_support".to_string(), self.r#vpn_ecmp_support.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCoreNetworkPolicyDocumentCoreNetworkConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#asn_ranges: {
                        let field_value = match fields_map.get("asn_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'asn_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#edge_locations: {
                        let field_value = match fields_map.get("edge_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'edge_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentCoreNetworkConfigurationEdgeLocation> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#inside_cidr_blocks: {
                        let field_value = match fields_map.get("inside_cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'inside_cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vpn_ecmp_support: {
                        let field_value = match fields_map.get("vpn_ecmp_support") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpn_ecmp_support' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
