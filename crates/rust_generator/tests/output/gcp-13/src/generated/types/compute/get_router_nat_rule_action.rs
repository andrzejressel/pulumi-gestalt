#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouterNatRuleAction {
    /// A list of URLs of the IP resources used for this NAT rule.
    /// These IP addresses must be valid static external IP addresses assigned to the project.
    /// This field is used for public NAT.
    #[builder(into)]
    #[serde(rename = "sourceNatActiveIps")]
    pub r#source_nat_active_ips: Vec<String>,
    /// A list of URLs of the subnetworks used as source ranges for this NAT Rule.
    /// These subnetworks must have purpose set to PRIVATE_NAT.
    /// This field is used for private NAT.
    #[builder(into)]
    #[serde(rename = "sourceNatActiveRanges")]
    pub r#source_nat_active_ranges: Vec<String>,
    /// A list of URLs of the IP resources to be drained.
    /// These IPs must be valid static external IPs that have been assigned to the NAT.
    /// These IPs should be used for updating/patching a NAT rule only.
    /// This field is used for public NAT.
    #[builder(into)]
    #[serde(rename = "sourceNatDrainIps")]
    pub r#source_nat_drain_ips: Vec<String>,
    /// A list of URLs of subnetworks representing source ranges to be drained.
    /// This is only supported on patch/update, and these subnetworks must have previously been used as active ranges in this NAT Rule.
    /// This field is used for private NAT.
    #[builder(into)]
    #[serde(rename = "sourceNatDrainRanges")]
    pub r#source_nat_drain_ranges: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouterNatRuleAction {
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
                "source_nat_active_ips".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_nat_active_ips,
                )
                .await,
            );
            map.insert(
                "source_nat_active_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_nat_active_ranges,
                )
                .await,
            );
            map.insert(
                "source_nat_drain_ips".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_nat_drain_ips,
                )
                .await,
            );
            map.insert(
                "source_nat_drain_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_nat_drain_ranges,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouterNatRuleAction {
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
                    r#source_nat_active_ips: {
                        let field_value = match fields_map.get("source_nat_active_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_nat_active_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_nat_active_ranges: {
                        let field_value = match fields_map.get("source_nat_active_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_nat_active_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_nat_drain_ips: {
                        let field_value = match fields_map.get("source_nat_drain_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_nat_drain_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_nat_drain_ranges: {
                        let field_value = match fields_map.get("source_nat_drain_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_nat_drain_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
