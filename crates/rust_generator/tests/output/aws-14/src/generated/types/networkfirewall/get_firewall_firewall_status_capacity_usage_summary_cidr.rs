#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFirewallFirewallStatusCapacityUsageSummaryCidr {
    /// Available number of CIDR blocks available for use by the IP set references in a firewall.
    #[builder(into)]
    #[serde(rename = "availableCidrCount")]
    pub r#available_cidr_count: i32,
    /// The list of IP set references used by a firewall.
    #[builder(into)]
    #[serde(rename = "ipSetReferences")]
    pub r#ip_set_references: Vec<super::super::types::networkfirewall::GetFirewallFirewallStatusCapacityUsageSummaryCidrIpSetReference>,
    /// Number of CIDR blocks used by the IP set references in a firewall.
    #[builder(into)]
    #[serde(rename = "utilizedCidrCount")]
    pub r#utilized_cidr_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFirewallFirewallStatusCapacityUsageSummaryCidr {
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
                "available_cidr_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#available_cidr_count,
                )
                .await,
            );
            map.insert(
                "ip_set_references".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_set_references,
                )
                .await,
            );
            map.insert(
                "utilized_cidr_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#utilized_cidr_count,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFirewallFirewallStatusCapacityUsageSummaryCidr {
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
                    r#available_cidr_count: {
                        let field_value = match fields_map.get("available_cidr_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_cidr_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_set_references: {
                        let field_value = match fields_map.get("ip_set_references") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_set_references' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#utilized_cidr_count: {
                        let field_value = match fields_map.get("utilized_cidr_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'utilized_cidr_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
