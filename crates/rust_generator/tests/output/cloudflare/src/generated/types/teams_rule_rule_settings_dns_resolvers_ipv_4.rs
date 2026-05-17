#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TeamsRuleRuleSettingsDnsResolversIpv4 {
    /// The IPv4 or IPv6 address of the upstream resolver.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: String,
    /// A port number to use for the upstream resolver. Defaults to `53`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Whether to connect to this resolver over a private network. Must be set when `vnet_id` is set.
    #[builder(into)]
    #[serde(rename = "routeThroughPrivateNetwork")]
    pub r#route_through_private_network: Option<bool>,
    /// specify a virtual network for this resolver. Uses default virtual network id if omitted.
    #[builder(into)]
    #[serde(rename = "vnetId")]
    pub r#vnet_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TeamsRuleRuleSettingsDnsResolversIpv4 {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip,
                )
                .await,
            );
            map.insert(
                "port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port,
                )
                .await,
            );
            map.insert(
                "route_through_private_network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#route_through_private_network,
                )
                .await,
            );
            map.insert(
                "vnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vnet_id,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TeamsRuleRuleSettingsDnsResolversIpv4 {
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
                    r#ip: {
                        let field_value = match fields_map.get("ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_through_private_network: {
                        let field_value = match fields_map.get("route_through_private_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_through_private_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_id: {
                        let field_value = match fields_map.get("vnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
