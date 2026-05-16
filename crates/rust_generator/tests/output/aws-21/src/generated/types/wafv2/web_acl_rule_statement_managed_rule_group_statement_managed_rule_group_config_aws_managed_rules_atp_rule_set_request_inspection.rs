#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspection {
    /// Details about your login page password field. See `password_field` for more details.
    #[builder(into)]
    #[serde(rename = "passwordField")]
    pub r#password_field: Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspectionPasswordField>,
    /// The payload type for your login endpoint, either JSON or form encoded.
    #[builder(into)]
    #[serde(rename = "payloadType")]
    pub r#payload_type: String,
    /// Details about your login page username field. See `username_field` for more details.
    #[builder(into)]
    #[serde(rename = "usernameField")]
    pub r#username_field: Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspectionUsernameField>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspection {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("password_field".to_string(), self.r#password_field.to_pulumi_value().await);
            map.insert("payload_type".to_string(), self.r#payload_type.to_pulumi_value().await);
            map.insert("username_field".to_string(), self.r#username_field.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspection {
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
                    r#password_field: {
                        let field_value = match fields_map.get("password_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'password_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspectionPasswordField> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#payload_type: {
                        let field_value = match fields_map.get("payload_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'payload_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#username_field: {
                        let field_value = match fields_map.get("username_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'username_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetRequestInspectionUsernameField> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
