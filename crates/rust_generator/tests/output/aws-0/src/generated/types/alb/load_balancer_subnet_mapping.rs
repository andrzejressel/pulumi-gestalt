#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LoadBalancerSubnetMapping {
    /// Allocation ID of the Elastic IP address for an internet-facing load balancer.
    #[builder(into)]
    #[serde(rename = "allocationId")]
    pub r#allocation_id: Option<String>,
    /// IPv6 address. You associate IPv6 CIDR blocks with your VPC and choose the subnets where you launch both internet-facing and internal Application Load Balancers or Network Load Balancers.
    #[builder(into)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Option<String>,
    #[builder(into)]
    #[serde(rename = "outpostId")]
    pub r#outpost_id: Option<String>,
    /// Private IPv4 address for an internal load balancer.
    #[builder(into)]
    #[serde(rename = "privateIpv4Address")]
    pub r#private_ipv_4_address: Option<String>,
    /// ID of the subnet of which to attach to the load balancer. You can specify only one subnet per Availability Zone.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LoadBalancerSubnetMapping {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allocation_id".to_string(), self.r#allocation_id.to_pulumi_value().await);
            map.insert("ipv_6_address".to_string(), self.r#ipv_6_address.to_pulumi_value().await);
            map.insert("outpost_id".to_string(), self.r#outpost_id.to_pulumi_value().await);
            map.insert("private_ipv_4_address".to_string(), self.r#private_ipv_4_address.to_pulumi_value().await);
            map.insert("subnet_id".to_string(), self.r#subnet_id.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LoadBalancerSubnetMapping {
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
                    r#allocation_id: {
                        let field_value = match fields_map.get("allocation_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_address: {
                        let field_value = match fields_map.get("ipv_6_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#outpost_id: {
                        let field_value = match fields_map.get("outpost_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'outpost_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#private_ipv_4_address: {
                        let field_value = match fields_map.get("private_ipv_4_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ipv_4_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
