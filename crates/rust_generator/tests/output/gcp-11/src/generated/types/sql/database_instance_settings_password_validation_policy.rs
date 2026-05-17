#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseInstanceSettingsPasswordValidationPolicy {
    /// Checks if the password is a combination of lowercase, uppercase, numeric, and non-alphanumeric characters.
    #[builder(into)]
    #[serde(rename = "complexity")]
    pub r#complexity: Option<String>,
    /// Prevents the use of the username in the password.
    #[builder(into)]
    #[serde(rename = "disallowUsernameSubstring")]
    pub r#disallow_username_substring: Option<bool>,
    /// Enables or disable the password validation policy.
    #[builder(into)]
    #[serde(rename = "enablePasswordPolicy")]
    pub r#enable_password_policy: bool,
    /// Specifies the minimum number of characters that the password must have.
    #[builder(into)]
    #[serde(rename = "minLength")]
    pub r#min_length: Option<i32>,
    /// Specifies the minimum duration after which you can change the password.
    #[builder(into)]
    #[serde(rename = "passwordChangeInterval")]
    pub r#password_change_interval: Option<String>,
    /// Specifies the number of previous passwords that you can't reuse.
    #[builder(into)]
    #[serde(rename = "reuseInterval")]
    pub r#reuse_interval: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseInstanceSettingsPasswordValidationPolicy {
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
                "complexity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#complexity,
                )
                .await,
            );
            map.insert(
                "disallow_username_substring".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disallow_username_substring,
                )
                .await,
            );
            map.insert(
                "enable_password_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_password_policy,
                )
                .await,
            );
            map.insert(
                "min_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_length,
                )
                .await,
            );
            map.insert(
                "password_change_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#password_change_interval,
                )
                .await,
            );
            map.insert(
                "reuse_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reuse_interval,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseInstanceSettingsPasswordValidationPolicy {
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
                    r#complexity: {
                        let field_value = match fields_map.get("complexity") {
                            Some(value) => value,
                            None => bail!("Missing field 'complexity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disallow_username_substring: {
                        let field_value = match fields_map.get("disallow_username_substring") {
                            Some(value) => value,
                            None => bail!("Missing field 'disallow_username_substring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_password_policy: {
                        let field_value = match fields_map.get("enable_password_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_password_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_length: {
                        let field_value = match fields_map.get("min_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password_change_interval: {
                        let field_value = match fields_map.get("password_change_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'password_change_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reuse_interval: {
                        let field_value = match fields_map.get("reuse_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'reuse_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
