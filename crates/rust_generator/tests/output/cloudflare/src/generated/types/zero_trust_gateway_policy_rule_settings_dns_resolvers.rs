#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustGatewayPolicyRuleSettingsDnsResolvers {
    /// IPv4 resolvers.
    #[builder(into)]
    #[serde(rename = "ipv4s")]
    pub r#ipv_4_s: Option<Vec<super::types::ZeroTrustGatewayPolicyRuleSettingsDnsResolversIpv4>>,
    /// IPv6 resolvers.
    #[builder(into)]
    #[serde(rename = "ipv6s")]
    pub r#ipv_6_s: Option<Vec<super::types::ZeroTrustGatewayPolicyRuleSettingsDnsResolversIpv6>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustGatewayPolicyRuleSettingsDnsResolvers {
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
                "ipv_4_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_4_s,
                )
                .await,
            );
            map.insert(
                "ipv_6_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_s,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustGatewayPolicyRuleSettingsDnsResolvers {
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
                    r#ipv_4_s: {
                        let field_value = match fields_map.get("ipv_4_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_4_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_s: {
                        let field_value = match fields_map.get("ipv_6_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
