#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSet {
    /// The path of the account creation endpoint for your application. This is the page on your website that accepts the completed registration form for a new user. This page must accept POST requests.
    #[builder(into)]
    #[serde(rename = "creationPath")]
    pub r#creation_path: String,
    /// Whether or not to allow the use of regular expressions in the login page path.
    #[builder(into)]
    #[serde(rename = "enableRegexInPath")]
    pub r#enable_regex_in_path: Option<bool>,
    /// The path of the account registration endpoint for your application. This is the page on your website that presents the registration form to new users. This page must accept GET text/html requests.
    #[builder(into)]
    #[serde(rename = "registrationPagePath")]
    pub r#registration_page_path: String,
    /// The criteria for inspecting login requests, used by the ATP rule group to validate credentials usage. See `request_inspection` for more details.
    #[builder(into)]
    #[serde(rename = "requestInspection")]
    pub r#request_inspection: Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspection>,
    /// The criteria for inspecting responses to login requests, used by the ATP rule group to track login failure rates. Note that Response Inspection is available only on web ACLs that protect CloudFront distributions. See `response_inspection` for more details.
    #[builder(into)]
    #[serde(rename = "responseInspection")]
    pub r#response_inspection: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetResponseInspection>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSet {
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
                "creation_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#creation_path,
                )
                .await,
            );
            map.insert(
                "enable_regex_in_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_regex_in_path,
                )
                .await,
            );
            map.insert(
                "registration_page_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#registration_page_path,
                )
                .await,
            );
            map.insert(
                "request_inspection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_inspection,
                )
                .await,
            );
            map.insert(
                "response_inspection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#response_inspection,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSet {
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
                    r#creation_path: {
                        let field_value = match fields_map.get("creation_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'creation_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_regex_in_path: {
                        let field_value = match fields_map.get("enable_regex_in_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_regex_in_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registration_page_path: {
                        let field_value = match fields_map.get("registration_page_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'registration_page_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_inspection: {
                        let field_value = match fields_map.get("request_inspection") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_inspection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_inspection: {
                        let field_value = match fields_map.get("response_inspection") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_inspection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
