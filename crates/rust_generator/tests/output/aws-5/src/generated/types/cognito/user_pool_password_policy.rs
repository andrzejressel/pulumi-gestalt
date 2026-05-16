#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPoolPasswordPolicy {
    /// Minimum length of the password policy that you have set.
    #[builder(into)]
    #[serde(rename = "minimumLength")]
    pub r#minimum_length: Option<i32>,
    /// Number of previous passwords that you want Amazon Cognito to restrict each user from reusing. Users can't set a password that matches any of number of previous passwords specified by this argument. A value of 0 means that password history is not enforced. Valid values are between 0 and 24.
    /// 
    /// **Note:** This argument requires advanced security features to be active in the user pool.
    #[builder(into)]
    #[serde(rename = "passwordHistorySize")]
    pub r#password_history_size: Option<i32>,
    /// Whether you have required users to use at least one lowercase letter in their password.
    #[builder(into)]
    #[serde(rename = "requireLowercase")]
    pub r#require_lowercase: Option<bool>,
    /// Whether you have required users to use at least one number in their password.
    #[builder(into)]
    #[serde(rename = "requireNumbers")]
    pub r#require_numbers: Option<bool>,
    /// Whether you have required users to use at least one symbol in their password.
    #[builder(into)]
    #[serde(rename = "requireSymbols")]
    pub r#require_symbols: Option<bool>,
    /// Whether you have required users to use at least one uppercase letter in their password.
    #[builder(into)]
    #[serde(rename = "requireUppercase")]
    pub r#require_uppercase: Option<bool>,
    /// In the password policy you have set, refers to the number of days a temporary password is valid. If the user does not sign-in during this time, their password will need to be reset by an administrator.
    #[builder(into)]
    #[serde(rename = "temporaryPasswordValidityDays")]
    pub r#temporary_password_validity_days: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPoolPasswordPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("minimum_length".to_string(), self.r#minimum_length.to_pulumi_value().await);
            map.insert("password_history_size".to_string(), self.r#password_history_size.to_pulumi_value().await);
            map.insert("require_lowercase".to_string(), self.r#require_lowercase.to_pulumi_value().await);
            map.insert("require_numbers".to_string(), self.r#require_numbers.to_pulumi_value().await);
            map.insert("require_symbols".to_string(), self.r#require_symbols.to_pulumi_value().await);
            map.insert("require_uppercase".to_string(), self.r#require_uppercase.to_pulumi_value().await);
            map.insert("temporary_password_validity_days".to_string(), self.r#temporary_password_validity_days.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPoolPasswordPolicy {
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
                    r#minimum_length: {
                        let field_value = match fields_map.get("minimum_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#password_history_size: {
                        let field_value = match fields_map.get("password_history_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'password_history_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#require_lowercase: {
                        let field_value = match fields_map.get("require_lowercase") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_lowercase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#require_numbers: {
                        let field_value = match fields_map.get("require_numbers") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_numbers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#require_symbols: {
                        let field_value = match fields_map.get("require_symbols") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_symbols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#require_uppercase: {
                        let field_value = match fields_map.get("require_uppercase") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_uppercase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#temporary_password_validity_days: {
                        let field_value = match fields_map.get("temporary_password_validity_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'temporary_password_validity_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
