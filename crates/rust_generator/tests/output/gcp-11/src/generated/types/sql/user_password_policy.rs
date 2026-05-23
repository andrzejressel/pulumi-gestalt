#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPasswordPolicy {
    /// Number of failed attempts allowed before the user get locked.
    #[builder(into)]
    pub r#allowed_failed_attempts: Option<i32>,
    /// If true, the check that will lock user after too many failed login attempts will be enabled.
    #[builder(into)]
    pub r#enable_failed_attempts_check: Option<bool>,
    /// If true, the user must specify the current password before changing the password. This flag is supported only for MySQL.
    #[builder(into)]
    pub r#enable_password_verification: Option<bool>,
    /// Password expiration duration with one week grace period.
    #[builder(into)]
    pub r#password_expiration_duration: Option<String>,
    #[builder(into)]
    pub r#statuses: Option<Vec<super::super::types::sql::UserPasswordPolicyStatus>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPasswordPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allowedFailedAttempts",
                    &self.r#allowed_failed_attempts,
                ),
                to_pulumi_object_field(
                    "enableFailedAttemptsCheck",
                    &self.r#enable_failed_attempts_check,
                ),
                to_pulumi_object_field(
                    "enablePasswordVerification",
                    &self.r#enable_password_verification,
                ),
                to_pulumi_object_field(
                    "passwordExpirationDuration",
                    &self.r#password_expiration_duration,
                ),
                to_pulumi_object_field(
                    "statuses",
                    &self.r#statuses,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("allowedFailedAttempts") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedFailedAttempts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_failed_attempts_check: {
                        let field_value = match fields_map.get("enableFailedAttemptsCheck") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableFailedAttemptsCheck' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_password_verification: {
                        let field_value = match fields_map.get("enablePasswordVerification") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePasswordVerification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password_expiration_duration: {
                        let field_value = match fields_map.get("passwordExpirationDuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'passwordExpirationDuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
