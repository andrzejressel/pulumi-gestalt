#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementRateBasedStatementScopeDownStatementXssMatchStatementFieldToMatchCookiesMatchPattern {
    /// An empty configuration block that is used for inspecting all headers.
    #[builder(into)]
    #[serde(rename = "all")]
    pub r#all: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatementXssMatchStatementFieldToMatchCookiesMatchPatternAll>>,
    #[builder(into)]
    #[serde(rename = "excludedCookies")]
    pub r#excluded_cookies: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "includedCookies")]
    pub r#included_cookies: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementRateBasedStatementScopeDownStatementXssMatchStatementFieldToMatchCookiesMatchPattern {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("all".to_string(), self.r#all.to_pulumi_value().await);
            map.insert("excluded_cookies".to_string(), self.r#excluded_cookies.to_pulumi_value().await);
            map.insert("included_cookies".to_string(), self.r#included_cookies.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementRateBasedStatementScopeDownStatementXssMatchStatementFieldToMatchCookiesMatchPattern {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#all: {
                        let field_value = match fields_map.get("all") {
                            Some(value) => value,
                            None => bail!("Missing field 'all' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatementXssMatchStatementFieldToMatchCookiesMatchPatternAll>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#excluded_cookies: {
                        let field_value = match fields_map.get("excluded_cookies") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_cookies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#included_cookies: {
                        let field_value = match fields_map.get("included_cookies") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_cookies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
