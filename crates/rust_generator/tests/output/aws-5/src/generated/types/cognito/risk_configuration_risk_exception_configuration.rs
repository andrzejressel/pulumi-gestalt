#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RiskConfigurationRiskExceptionConfiguration {
    /// Overrides the risk decision to always block the pre-authentication requests.
    /// The IP range is in CIDR notation, a compact representation of an IP address and its routing prefix.
    /// Can contain a maximum of 200 items.
    #[builder(into)]
    #[serde(rename = "blockedIpRangeLists")]
    pub r#blocked_ip_range_lists: Option<Vec<String>>,
    /// Risk detection isn't performed on the IP addresses in this range list.
    /// The IP range is in CIDR notation.
    /// Can contain a maximum of 200 items.
    #[builder(into)]
    #[serde(rename = "skippedIpRangeLists")]
    pub r#skipped_ip_range_lists: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RiskConfigurationRiskExceptionConfiguration {
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
                "blocked_ip_range_lists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#blocked_ip_range_lists,
                )
                .await,
            );
            map.insert(
                "skipped_ip_range_lists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#skipped_ip_range_lists,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RiskConfigurationRiskExceptionConfiguration {
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
                    r#blocked_ip_range_lists: {
                        let field_value = match fields_map.get("blocked_ip_range_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'blocked_ip_range_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skipped_ip_range_lists: {
                        let field_value = match fields_map.get("skipped_ip_range_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'skipped_ip_range_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
