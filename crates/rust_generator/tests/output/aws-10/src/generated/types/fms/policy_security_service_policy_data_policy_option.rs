#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicySecurityServicePolicyDataPolicyOption {
    /// Defines the deployment model to use for the firewall policy. Documented below.
    #[builder(into)]
    #[serde(rename = "networkFirewallPolicy")]
    pub r#network_firewall_policy: Option<Box<super::super::types::fms::PolicySecurityServicePolicyDataPolicyOptionNetworkFirewallPolicy>>,
    #[builder(into)]
    #[serde(rename = "thirdPartyFirewallPolicy")]
    pub r#third_party_firewall_policy: Option<Box<super::super::types::fms::PolicySecurityServicePolicyDataPolicyOptionThirdPartyFirewallPolicy>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicySecurityServicePolicyDataPolicyOption {
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
                "network_firewall_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_firewall_policy,
                )
                .await,
            );
            map.insert(
                "third_party_firewall_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#third_party_firewall_policy,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicySecurityServicePolicyDataPolicyOption {
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
                    r#network_firewall_policy: {
                        let field_value = match fields_map.get("network_firewall_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_firewall_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#third_party_firewall_policy: {
                        let field_value = match fields_map.get("third_party_firewall_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'third_party_firewall_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
