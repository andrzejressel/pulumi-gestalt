#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceNetwork {
    /// The network connect mode of the Filestore instance.
    /// If not provided, the connect mode defaults to
    /// DIRECT_PEERING.
    /// Default value is `DIRECT_PEERING`.
    /// Possible values are: `DIRECT_PEERING`, `PRIVATE_SERVICE_ACCESS`.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "connectMode")]
    pub r#connect_mode: Option<String>,
    /// (Output)
    /// A list of IPv4 or IPv6 addresses.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Option<Vec<String>>,
    /// IP versions for which the instance has
    /// IP addresses assigned.
    /// Each value may be one of: `ADDRESS_MODE_UNSPECIFIED`, `MODE_IPV4`, `MODE_IPV6`.
    #[builder(into)]
    #[serde(rename = "modes")]
    pub r#modes: Vec<String>,
    /// The name of the GCE VPC network to which the
    /// instance is connected.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
    /// A /29 CIDR block that identifies the range of IP
    /// addresses reserved for this instance.
    #[builder(into)]
    #[serde(rename = "reservedIpRange")]
    pub r#reserved_ip_range: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceNetwork {
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
                    "connect_mode",
                    &self.r#connect_mode,
                ),
                to_pulumi_object_field(
                    "ip_addresses",
                    &self.r#ip_addresses,
                ),
                to_pulumi_object_field(
                    "modes",
                    &self.r#modes,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "reserved_ip_range",
                    &self.r#reserved_ip_range,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceNetwork {
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
                    r#connect_mode: {
                        let field_value = match fields_map.get("connect_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'connect_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_addresses: {
                        let field_value = match fields_map.get("ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#modes: {
                        let field_value = match fields_map.get("modes") {
                            Some(value) => value,
                            None => bail!("Missing field 'modes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reserved_ip_range: {
                        let field_value = match fields_map.get("reserved_ip_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'reserved_ip_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
