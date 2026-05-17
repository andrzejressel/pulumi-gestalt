#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstanceSettingPasswordValidationPolicy {
    /// Password complexity.
    #[builder(into)]
    #[serde(rename = "complexity")]
    pub r#complexity: String,
    /// Disallow username as a part of the password.
    #[builder(into)]
    #[serde(rename = "disallowUsernameSubstring")]
    pub r#disallow_username_substring: bool,
    /// Whether the password policy is enabled or not.
    #[builder(into)]
    #[serde(rename = "enablePasswordPolicy")]
    pub r#enable_password_policy: bool,
    /// Minimum number of characters allowed.
    #[builder(into)]
    #[serde(rename = "minLength")]
    pub r#min_length: i32,
    /// Minimum interval after which the password can be changed. This flag is only supported for PostgresSQL.
    #[builder(into)]
    #[serde(rename = "passwordChangeInterval")]
    pub r#password_change_interval: String,
    /// Number of previous passwords that cannot be reused.
    #[builder(into)]
    #[serde(rename = "reuseInterval")]
    pub r#reuse_interval: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabaseInstanceSettingPasswordValidationPolicy {
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabaseInstanceSettingPasswordValidationPolicy {
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
