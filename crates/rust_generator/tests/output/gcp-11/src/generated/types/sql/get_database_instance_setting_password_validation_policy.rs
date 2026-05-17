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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "complexity",
                    &self.r#complexity,
                ),
                to_pulumi_object_field(
                    "disallow_username_substring",
                    &self.r#disallow_username_substring,
                ),
                to_pulumi_object_field(
                    "enable_password_policy",
                    &self.r#enable_password_policy,
                ),
                to_pulumi_object_field(
                    "min_length",
                    &self.r#min_length,
                ),
                to_pulumi_object_field(
                    "password_change_interval",
                    &self.r#password_change_interval,
                ),
                to_pulumi_object_field(
                    "reuse_interval",
                    &self.r#reuse_interval,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
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
