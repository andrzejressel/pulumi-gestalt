#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnSiteLink {
    /// A `bgp` block as defined above.
    /// 
    /// > **NOTE:** The `link.bgp` has to be set when the `address_cidrs` isn't specified.
    #[builder(into)]
    #[serde(rename = "bgp")]
    pub r#bgp: Option<Box<super::super::types::network::VpnSiteLinkBgp>>,
    /// The FQDN of this VPN Site Link.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Option<String>,
    /// The ID of the VPN Site Link.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The IP address of this VPN Site Link.
    /// 
    /// > **NOTE:** Either `fqdn` or `ip_address` should be specified.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// The name which should be used for this VPN Site Link.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name of the physical link at the VPN Site. Example: `ATT`, `Verizon`.
    #[builder(into)]
    #[serde(rename = "providerName")]
    pub r#provider_name: Option<String>,
    /// The speed of the VPN device at the branch location in unit of mbps. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "speedInMbps")]
    pub r#speed_in_mbps: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpnSiteLink {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "bgp".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bgp,
                )
                .await,
            );
            map.insert(
                "fqdn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fqdn,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "ip_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_address,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "provider_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provider_name,
                )
                .await,
            );
            map.insert(
                "speed_in_mbps".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#speed_in_mbps,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VpnSiteLink {
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
                    r#bgp: {
                        let field_value = match fields_map.get("bgp") {
                            Some(value) => value,
                            None => bail!("Missing field 'bgp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fqdn: {
                        let field_value = match fields_map.get("fqdn") {
                            Some(value) => value,
                            None => bail!("Missing field 'fqdn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provider_name: {
                        let field_value = match fields_map.get("provider_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'provider_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#speed_in_mbps: {
                        let field_value = match fields_map.get("speed_in_mbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'speed_in_mbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
