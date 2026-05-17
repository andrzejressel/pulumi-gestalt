#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPasswordPolicy {
    /// Number of failed attempts allowed before the user get locked.
    #[builder(into)]
    #[serde(rename = "allowedFailedAttempts")]
    pub r#allowed_failed_attempts: Option<i32>,
    /// If true, the check that will lock user after too many failed login attempts will be enabled.
    #[builder(into)]
    #[serde(rename = "enableFailedAttemptsCheck")]
    pub r#enable_failed_attempts_check: Option<bool>,
    /// If true, the user must specify the current password before changing the password. This flag is supported only for MySQL.
    #[builder(into)]
    #[serde(rename = "enablePasswordVerification")]
    pub r#enable_password_verification: Option<bool>,
    /// Password expiration duration with one week grace period.
    #[builder(into)]
    #[serde(rename = "passwordExpirationDuration")]
    pub r#password_expiration_duration: Option<String>,
    #[builder(into)]
    #[serde(rename = "statuses")]
    pub r#statuses: Option<Vec<super::super::types::sql::UserPasswordPolicyStatus>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPasswordPolicy {
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
                "allowed_failed_attempts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_failed_attempts,
                )
                .await,
            );
            map.insert(
                "enable_failed_attempts_check".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_failed_attempts_check,
                )
                .await,
            );
            map.insert(
                "enable_password_verification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_password_verification,
                )
                .await,
            );
            map.insert(
                "password_expiration_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#password_expiration_duration,
                )
                .await,
            );
            map.insert(
                "statuses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#statuses,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPasswordPolicy {
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
                    r#allowed_failed_attempts: {
                        let field_value = match fields_map.get("allowed_failed_attempts") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_failed_attempts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_failed_attempts_check: {
                        let field_value = match fields_map.get("enable_failed_attempts_check") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_failed_attempts_check' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_password_verification: {
                        let field_value = match fields_map.get("enable_password_verification") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_password_verification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password_expiration_duration: {
                        let field_value = match fields_map.get("password_expiration_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'password_expiration_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statuses: {
                        let field_value = match fields_map.get("statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
