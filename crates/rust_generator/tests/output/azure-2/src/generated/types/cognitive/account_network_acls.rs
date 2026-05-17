#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountNetworkAcls {
    /// The Default Action to use when no rules match from `ip_rules` / `virtual_network_rules`. Possible values are `Allow` and `Deny`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: String,
    /// One or more IP Addresses, or CIDR Blocks which should be able to access the Cognitive Account.
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<String>>,
    /// A `virtual_network_rules` block as defined below.
    #[builder(into)]
    #[serde(rename = "virtualNetworkRules")]
    pub r#virtual_network_rules: Option<Vec<super::super::types::cognitive::AccountNetworkAclsVirtualNetworkRule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountNetworkAcls {
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
                "default_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_action,
                )
                .await,
            );
            map.insert(
                "ip_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_rules,
                )
                .await,
            );
            map.insert(
                "virtual_network_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_network_rules,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountNetworkAcls {
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
                    r#default_action: {
                        let field_value = match fields_map.get("default_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_rules: {
                        let field_value = match fields_map.get("ip_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_rules: {
                        let field_value = match fields_map.get("virtual_network_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_network_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
