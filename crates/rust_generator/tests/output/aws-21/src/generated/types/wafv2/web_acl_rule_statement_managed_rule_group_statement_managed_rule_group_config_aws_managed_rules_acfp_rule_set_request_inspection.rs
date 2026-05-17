#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspection {
    /// The names of the fields in the request payload that contain your customer's primary physical address. See `address_fields` for more details.
    #[builder(into)]
    #[serde(rename = "addressFields")]
    pub r#address_fields: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionAddressFields>>,
    /// The name of the field in the request payload that contains your customer's email. See `email_field` for more details.
    #[builder(into)]
    #[serde(rename = "emailField")]
    pub r#email_field: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionEmailField>>,
    /// Details about your login page password field. See `password_field` for more details.
    #[builder(into)]
    #[serde(rename = "passwordField")]
    pub r#password_field: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionPasswordField>>,
    /// The payload type for your login endpoint, either JSON or form encoded.
    #[builder(into)]
    #[serde(rename = "payloadType")]
    pub r#payload_type: String,
    /// The names of the fields in the request payload that contain your customer's primary phone number. See `phone_number_fields` for more details.
    #[builder(into)]
    #[serde(rename = "phoneNumberFields")]
    pub r#phone_number_fields: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionPhoneNumberFields>>,
    /// Details about your login page username field. See `username_field` for more details.
    #[builder(into)]
    #[serde(rename = "usernameField")]
    pub r#username_field: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionUsernameField>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspection {
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
                "address_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_fields,
                )
                .await,
            );
            map.insert(
                "email_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_field,
                )
                .await,
            );
            map.insert(
                "password_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#password_field,
                )
                .await,
            );
            map.insert(
                "payload_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#payload_type,
                )
                .await,
            );
            map.insert(
                "phone_number_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#phone_number_fields,
                )
                .await,
            );
            map.insert(
                "username_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#username_field,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspection {
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
                    r#address_fields: {
                        let field_value = match fields_map.get("address_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_field: {
                        let field_value = match fields_map.get("email_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password_field: {
                        let field_value = match fields_map.get("password_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'password_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#payload_type: {
                        let field_value = match fields_map.get("payload_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'payload_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phone_number_fields: {
                        let field_value = match fields_map.get("phone_number_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'phone_number_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username_field: {
                        let field_value = match fields_map.get("username_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'username_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
