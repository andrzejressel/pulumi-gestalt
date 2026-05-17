#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PointToPointVpnGatewayConnectionConfiguration {
    /// Should Internet Security be enabled to secure internet traffic? Changing this forces a new resource to be created. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "internetSecurityEnabled")]
    pub r#internet_security_enabled: Option<bool>,
    /// The Name which should be used for this Connection Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A `route` block as defined below.
    #[builder(into)]
    #[serde(rename = "route")]
    pub r#route: Option<Box<super::super::types::network::PointToPointVpnGatewayConnectionConfigurationRoute>>,
    /// A `vpn_client_address_pool` block as defined below.
    #[builder(into)]
    #[serde(rename = "vpnClientAddressPool")]
    pub r#vpn_client_address_pool: Box<super::super::types::network::PointToPointVpnGatewayConnectionConfigurationVpnClientAddressPool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PointToPointVpnGatewayConnectionConfiguration {
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
                "internet_security_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#internet_security_enabled,
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
                "route".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#route,
                )
                .await,
            );
            map.insert(
                "vpn_client_address_pool".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpn_client_address_pool,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PointToPointVpnGatewayConnectionConfiguration {
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
                    r#internet_security_enabled: {
                        let field_value = match fields_map.get("internet_security_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'internet_security_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#route: {
                        let field_value = match fields_map.get("route") {
                            Some(value) => value,
                            None => bail!("Missing field 'route' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_client_address_pool: {
                        let field_value = match fields_map.get("vpn_client_address_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpn_client_address_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
