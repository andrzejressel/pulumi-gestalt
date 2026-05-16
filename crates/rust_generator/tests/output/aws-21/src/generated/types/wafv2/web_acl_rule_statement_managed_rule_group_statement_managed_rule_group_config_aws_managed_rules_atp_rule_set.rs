#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSet {
    /// Whether or not to allow the use of regular expressions in the login page path.
    #[builder(into)]
    #[serde(rename = "enableRegexInPath")]
    pub r#enable_regex_in_path: Option<bool>,
    /// The path of the login endpoint for your application.
    #[builder(into)]
    #[serde(rename = "loginPath")]
    pub r#login_path: String,
    /// The criteria for inspecting login requests, used by the ATP rule group to validate credentials usage. See `request_inspection` for more details.
    #[builder(into)]
    #[serde(rename = "requestInspection")]
    pub r#request_inspection: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspection>>,
    /// The criteria for inspecting responses to login requests, used by the ATP rule group to track login failure rates. Note that Response Inspection is available only on web ACLs that protect CloudFront distributions. See `response_inspection` for more details.
    #[builder(into)]
    #[serde(rename = "responseInspection")]
    pub r#response_inspection: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspection>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSet {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("enable_regex_in_path".to_string(), self.r#enable_regex_in_path.to_pulumi_value().await);
            map.insert("login_path".to_string(), self.r#login_path.to_pulumi_value().await);
            map.insert("request_inspection".to_string(), self.r#request_inspection.to_pulumi_value().await);
            map.insert("response_inspection".to_string(), self.r#response_inspection.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSet {
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
                    r#enable_regex_in_path: {
                        let field_value = match fields_map.get("enable_regex_in_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_regex_in_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#login_path: {
                        let field_value = match fields_map.get("login_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'login_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#request_inspection: {
                        let field_value = match fields_map.get("request_inspection") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_inspection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspection>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#response_inspection: {
                        let field_value = match fields_map.get("response_inspection") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_inspection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspection>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
