#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelVpc {
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "networkInterfaceIds")]
    pub r#network_interface_ids: Option<Vec<String>>,
    /// List of public address allocation ids to associate with ENIs that will be created in Output VPC. Must specify one for SINGLE_PIPELINE, two for STANDARD channels.
    #[builder(into)]
    #[serde(rename = "publicAddressAllocationIds")]
    pub r#public_address_allocation_ids: Vec<String>,
    /// A list of up to 5 EC2 VPC security group IDs to attach to the Output VPC network interfaces. If none are specified then the VPC default security group will be used.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// A list of VPC subnet IDs from the same VPC. If STANDARD channel, subnet IDs must be mapped to two unique availability zones (AZ).
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelVpc {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("availability_zones".to_string(), self.r#availability_zones.to_pulumi_value().await);
            map.insert("network_interface_ids".to_string(), self.r#network_interface_ids.to_pulumi_value().await);
            map.insert("public_address_allocation_ids".to_string(), self.r#public_address_allocation_ids.to_pulumi_value().await);
            map.insert("security_group_ids".to_string(), self.r#security_group_ids.to_pulumi_value().await);
            map.insert("subnet_ids".to_string(), self.r#subnet_ids.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelVpc {
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
                    r#availability_zones: {
                        let field_value = match fields_map.get("availability_zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#network_interface_ids: {
                        let field_value = match fields_map.get("network_interface_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_interface_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#public_address_allocation_ids: {
                        let field_value = match fields_map.get("public_address_allocation_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_address_allocation_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#security_group_ids: {
                        let field_value = match fields_map.get("security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#subnet_ids: {
                        let field_value = match fields_map.get("subnet_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
