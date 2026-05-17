#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVpnGatewayBgpSettingInstance0BgpPeeringAddress {
    /// A list of custom BGP peering addresses to assigned to this instance.
    #[builder(into)]
    #[serde(rename = "customIps")]
    pub r#custom_ips: Vec<String>,
    /// The list of default BGP peering addresses which belong to the pre-defined VPN Gateway IP configuration.
    #[builder(into)]
    #[serde(rename = "defaultIps")]
    pub r#default_ips: Vec<String>,
    /// The pre-defined id of VPN Gateway IP Configuration.
    #[builder(into)]
    #[serde(rename = "ipConfigurationId")]
    pub r#ip_configuration_id: String,
    /// The list of tunnel public IP addresses which belong to the pre-defined VPN Gateway IP configuration.
    #[builder(into)]
    #[serde(rename = "tunnelIps")]
    pub r#tunnel_ips: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVpnGatewayBgpSettingInstance0BgpPeeringAddress {
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
                    "custom_ips",
                    &self.r#custom_ips,
                ),
                to_pulumi_object_field(
                    "default_ips",
                    &self.r#default_ips,
                ),
                to_pulumi_object_field(
                    "ip_configuration_id",
                    &self.r#ip_configuration_id,
                ),
                to_pulumi_object_field(
                    "tunnel_ips",
                    &self.r#tunnel_ips,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVpnGatewayBgpSettingInstance0BgpPeeringAddress {
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
                    r#custom_ips: {
                        let field_value = match fields_map.get("custom_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_ips: {
                        let field_value = match fields_map.get("default_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_configuration_id: {
                        let field_value = match fields_map.get("ip_configuration_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_configuration_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tunnel_ips: {
                        let field_value = match fields_map.get("tunnel_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'tunnel_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
