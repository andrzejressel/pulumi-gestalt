#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerNetworkData {
    /// The network gateway of the container.
    #[builder(into)]
    #[serde(rename = "gateway")]
    pub r#gateway: Option<String>,
    /// The IPV6 address of the container.
    #[builder(into)]
    #[serde(rename = "globalIpv6Address")]
    pub r#global_ipv_6_address: Option<String>,
    /// The IPV6 prefix length address of the container.
    #[builder(into)]
    #[serde(rename = "globalIpv6PrefixLength")]
    pub r#global_ipv_6_prefix_length: Option<i32>,
    /// The IP address of the container.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// The IP prefix length of the container.
    #[builder(into)]
    #[serde(rename = "ipPrefixLength")]
    pub r#ip_prefix_length: Option<i32>,
    /// The IPV6 gateway of the container.
    #[builder(into)]
    #[serde(rename = "ipv6Gateway")]
    pub r#ipv_6_gateway: Option<String>,
    /// The MAC address of the container.
    #[builder(into)]
    #[serde(rename = "macAddress")]
    pub r#mac_address: Option<String>,
    /// The name of the network
    #[builder(into)]
    #[serde(rename = "networkName")]
    pub r#network_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContainerNetworkData {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "gateway",
                    &self.r#gateway,
                ),
                to_pulumi_object_field(
                    "global_ipv_6_address",
                    &self.r#global_ipv_6_address,
                ),
                to_pulumi_object_field(
                    "global_ipv_6_prefix_length",
                    &self.r#global_ipv_6_prefix_length,
                ),
                to_pulumi_object_field(
                    "ip_address",
                    &self.r#ip_address,
                ),
                to_pulumi_object_field(
                    "ip_prefix_length",
                    &self.r#ip_prefix_length,
                ),
                to_pulumi_object_field(
                    "ipv_6_gateway",
                    &self.r#ipv_6_gateway,
                ),
                to_pulumi_object_field(
                    "mac_address",
                    &self.r#mac_address,
                ),
                to_pulumi_object_field(
                    "network_name",
                    &self.r#network_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContainerNetworkData {
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
                    r#gateway: {
                        let field_value = match fields_map.get("gateway") {
                            Some(value) => value,
                            None => bail!("Missing field 'gateway' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#global_ipv_6_address: {
                        let field_value = match fields_map.get("global_ipv_6_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'global_ipv_6_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#global_ipv_6_prefix_length: {
                        let field_value = match fields_map.get("global_ipv_6_prefix_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'global_ipv_6_prefix_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_address: {
                        let field_value = match fields_map.get("ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_prefix_length: {
                        let field_value = match fields_map.get("ip_prefix_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_prefix_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_gateway: {
                        let field_value = match fields_map.get("ipv_6_gateway") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_gateway' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mac_address: {
                        let field_value = match fields_map.get("mac_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'mac_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_name: {
                        let field_value = match fields_map.get("network_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
