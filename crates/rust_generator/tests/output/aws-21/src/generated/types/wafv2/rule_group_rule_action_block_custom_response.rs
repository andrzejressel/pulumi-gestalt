#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleActionBlockCustomResponse {
    /// References the response body that you want AWS WAF to return to the web request client. This must reference a `key` defined in a `custom_response_body` block of this resource.
    #[builder(into)]
    #[serde(rename = "customResponseBodyKey")]
    pub r#custom_response_body_key: Option<String>,
    /// The HTTP status code to return to the client.
    #[builder(into)]
    #[serde(rename = "responseCode")]
    pub r#response_code: i32,
    /// The `response_header` blocks used to define the HTTP response headers added to the response. See Custom HTTP Header below for details.
    #[builder(into)]
    #[serde(rename = "responseHeaders")]
    pub r#response_headers: Option<Vec<super::super::types::wafv2::RuleGroupRuleActionBlockCustomResponseResponseHeader>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleActionBlockCustomResponse {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("custom_response_body_key".to_string(), self.r#custom_response_body_key.to_pulumi_value().await);
            map.insert("response_code".to_string(), self.r#response_code.to_pulumi_value().await);
            map.insert("response_headers".to_string(), self.r#response_headers.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleActionBlockCustomResponse {
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
                    r#custom_response_body_key: {
                        let field_value = match fields_map.get("custom_response_body_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_response_body_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#response_code: {
                        let field_value = match fields_map.get("response_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#response_headers: {
                        let field_value = match fields_map.get("response_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::wafv2::RuleGroupRuleActionBlockCustomResponseResponseHeader>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
