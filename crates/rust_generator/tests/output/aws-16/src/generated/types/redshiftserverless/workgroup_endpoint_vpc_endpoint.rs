#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkgroupEndpointVpcEndpoint {
    /// The network interfaces of the endpoint.. See `Network Interface` below.
    #[builder(into)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Option<Vec<super::super::types::redshiftserverless::WorkgroupEndpointVpcEndpointNetworkInterface>>,
    /// The DNS address of the VPC endpoint.
    #[builder(into)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Option<String>,
    /// The port that Amazon Redshift Serverless listens on.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkgroupEndpointVpcEndpoint {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("network_interfaces".to_string(), self.r#network_interfaces.to_pulumi_value().await);
            map.insert("vpc_endpoint_id".to_string(), self.r#vpc_endpoint_id.to_pulumi_value().await);
            map.insert("vpc_id".to_string(), self.r#vpc_id.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkgroupEndpointVpcEndpoint {
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
                    r#network_interfaces: {
                        let field_value = match fields_map.get("network_interfaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_interfaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::redshiftserverless::WorkgroupEndpointVpcEndpointNetworkInterface>> as FromPulumiValue>::from_pulumi_value(field_value)?
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
