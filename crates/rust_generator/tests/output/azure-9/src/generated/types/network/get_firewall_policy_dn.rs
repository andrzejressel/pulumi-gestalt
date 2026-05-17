#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFirewallPolicyDn {
    #[builder(into)]
    #[serde(rename = "networkRuleFqdnEnabled")]
    pub r#network_rule_fqdn_enabled: bool,
    #[builder(into)]
    #[serde(rename = "proxyEnabled")]
    pub r#proxy_enabled: bool,
    #[builder(into)]
    #[serde(rename = "servers")]
    pub r#servers: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFirewallPolicyDn {
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
                "network_rule_fqdn_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_rule_fqdn_enabled,
                )
                .await,
            );
            map.insert(
                "proxy_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#proxy_enabled,
                )
                .await,
            );
            map.insert(
                "servers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#servers,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFirewallPolicyDn {
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
                    r#network_rule_fqdn_enabled: {
                        let field_value = match fields_map.get("network_rule_fqdn_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_rule_fqdn_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_enabled: {
                        let field_value = match fields_map.get("proxy_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxy_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#servers: {
                        let field_value = match fields_map.get("servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
