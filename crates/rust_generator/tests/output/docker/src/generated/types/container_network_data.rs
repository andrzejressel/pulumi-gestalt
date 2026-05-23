#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerNetworkData {
    /// The network gateway of the container.
    #[builder(into)]
    pub r#gateway: Option<String>,
    /// The IPV6 address of the container.
    #[builder(into)]
    pub r#global_ipv_6_address: Option<String>,
    /// The IPV6 prefix length address of the container.
    #[builder(into)]
    pub r#global_ipv_6_prefix_length: Option<i32>,
    /// The IP address of the container.
    #[builder(into)]
    pub r#ip_address: Option<String>,
    /// The IP prefix length of the container.
    #[builder(into)]
    pub r#ip_prefix_length: Option<i32>,
    /// The IPV6 gateway of the container.
    #[builder(into)]
    pub r#ipv_6_gateway: Option<String>,
    /// The MAC address of the container.
    #[builder(into)]
    pub r#mac_address: Option<String>,
    /// The name of the network
    #[builder(into)]
    pub r#network_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContainerNetworkData {
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
                    "gateway",
                    &self.r#gateway,
                ),
                to_pulumi_object_field(
                    "globalIpv6Address",
                    &self.r#global_ipv_6_address,
                ),
                to_pulumi_object_field(
                    "globalIpv6PrefixLength",
                    &self.r#global_ipv_6_prefix_length,
                ),
                to_pulumi_object_field(
                    "ipAddress",
                    &self.r#ip_address,
                ),
                to_pulumi_object_field(
                    "ipPrefixLength",
                    &self.r#ip_prefix_length,
                ),
                to_pulumi_object_field(
                    "ipv6Gateway",
                    &self.r#ipv_6_gateway,
                ),
                to_pulumi_object_field(
                    "macAddress",
                    &self.r#mac_address,
                ),
                to_pulumi_object_field(
                    "networkName",
                    &self.r#network_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("globalIpv6Address") {
                            Some(value) => value,
                            None => bail!("Missing field 'globalIpv6Address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#global_ipv_6_prefix_length: {
                        let field_value = match fields_map.get("globalIpv6PrefixLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'globalIpv6PrefixLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_address: {
                        let field_value = match fields_map.get("ipAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_prefix_length: {
                        let field_value = match fields_map.get("ipPrefixLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipPrefixLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_gateway: {
                        let field_value = match fields_map.get("ipv6Gateway") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6Gateway' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mac_address: {
                        let field_value = match fields_map.get("macAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'macAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_name: {
                        let field_value = match fields_map.get("networkName") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
