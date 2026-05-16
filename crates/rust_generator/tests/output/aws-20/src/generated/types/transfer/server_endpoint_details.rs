#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerEndpointDetails {
    /// A list of address allocation IDs that are required to attach an Elastic IP address to your SFTP server's endpoint. This property can only be used when `endpoint_type` is set to `VPC`.
    #[builder(into)]
    #[serde(rename = "addressAllocationIds")]
    pub r#address_allocation_ids: Option<Vec<String>>,
    /// A list of security groups IDs that are available to attach to your server's endpoint. If no security groups are specified, the VPC's default security groups are automatically assigned to your endpoint. This property can only be used when `endpoint_type` is set to `VPC`.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// A list of subnet IDs that are required to host your SFTP server endpoint in your VPC. This property can only be used when `endpoint_type` is set to `VPC`.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Option<Vec<String>>,
    /// The ID of the VPC endpoint. This property can only be used when `endpoint_type` is set to `VPC_ENDPOINT`
    #[builder(into)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Option<String>,
    /// The VPC ID of the virtual private cloud in which the SFTP server's endpoint will be hosted. This property can only be used when `endpoint_type` is set to `VPC`.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServerEndpointDetails {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("address_allocation_ids".to_string(), self.r#address_allocation_ids.to_pulumi_value().await);
            map.insert("security_group_ids".to_string(), self.r#security_group_ids.to_pulumi_value().await);
            map.insert("subnet_ids".to_string(), self.r#subnet_ids.to_pulumi_value().await);
            map.insert("vpc_endpoint_id".to_string(), self.r#vpc_endpoint_id.to_pulumi_value().await);
            map.insert("vpc_id".to_string(), self.r#vpc_id.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServerEndpointDetails {
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
                    r#address_allocation_ids: {
                        let field_value = match fields_map.get("address_allocation_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_allocation_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
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
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vpc_endpoint_id: {
                        let field_value = match fields_map.get("vpc_endpoint_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_endpoint_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vpc_id: {
                        let field_value = match fields_map.get("vpc_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
