#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBrokerNodesNodeInfoList {
    /// Attached elastic network interface of the broker
    #[builder(into)]
    #[serde(rename = "attachedEniId")]
    pub r#attached_eni_id: String,
    /// ID of the broker
    #[builder(into)]
    #[serde(rename = "brokerId")]
    pub r#broker_id: f64,
    /// Client subnet to which this broker node belongs
    #[builder(into)]
    #[serde(rename = "clientSubnet")]
    pub r#client_subnet: String,
    /// The client virtual private cloud (VPC) IP address
    #[builder(into)]
    #[serde(rename = "clientVpcIpAddress")]
    pub r#client_vpc_ip_address: String,
    /// Set of endpoints for accessing the broker. This does not include ports
    #[builder(into)]
    #[serde(rename = "endpoints")]
    pub r#endpoints: Vec<String>,
    /// ARN of the node
    #[builder(into)]
    #[serde(rename = "nodeArn")]
    pub r#node_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBrokerNodesNodeInfoList {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("attached_eni_id".to_string(), self.r#attached_eni_id.to_pulumi_value().await);
            map.insert("broker_id".to_string(), self.r#broker_id.to_pulumi_value().await);
            map.insert("client_subnet".to_string(), self.r#client_subnet.to_pulumi_value().await);
            map.insert("client_vpc_ip_address".to_string(), self.r#client_vpc_ip_address.to_pulumi_value().await);
            map.insert("endpoints".to_string(), self.r#endpoints.to_pulumi_value().await);
            map.insert("node_arn".to_string(), self.r#node_arn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBrokerNodesNodeInfoList {
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
                    r#attached_eni_id: {
                        let field_value = match fields_map.get("attached_eni_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'attached_eni_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#broker_id: {
                        let field_value = match fields_map.get("broker_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'broker_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <f64 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#client_subnet: {
                        let field_value = match fields_map.get("client_subnet") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_subnet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#client_vpc_ip_address: {
                        let field_value = match fields_map.get("client_vpc_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_vpc_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#endpoints: {
                        let field_value = match fields_map.get("endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#node_arn: {
                        let field_value = match fields_map.get("node_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
