#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigSignIn {
    /// Whether to allow more than one account to have the same email.
    #[builder(into)]
    #[serde(rename = "allowDuplicateEmails")]
    pub r#allow_duplicate_emails: Option<bool>,
    /// Configuration options related to authenticating an anonymous user.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "anonymous")]
    pub r#anonymous: Option<Box<super::super::types::identityplatform::ConfigSignInAnonymous>>,
    /// Configuration options related to authenticating a user by their email address.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<Box<super::super::types::identityplatform::ConfigSignInEmail>>,
    /// (Output)
    /// Output only. Hash config information.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hashConfigs")]
    pub r#hash_configs: Option<Vec<super::super::types::identityplatform::ConfigSignInHashConfig>>,
    /// Configuration options related to authenticated a user by their phone number.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Option<Box<super::super::types::identityplatform::ConfigSignInPhoneNumber>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigSignIn {
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
                    "allow_duplicate_emails",
                    &self.r#allow_duplicate_emails,
                ),
                to_pulumi_object_field(
                    "anonymous",
                    &self.r#anonymous,
                ),
                to_pulumi_object_field(
                    "email",
                    &self.r#email,
                ),
                to_pulumi_object_field(
                    "hash_configs",
                    &self.r#hash_configs,
                ),
                to_pulumi_object_field(
                    "phone_number",
                    &self.r#phone_number,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigSignIn {
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
                    r#allow_duplicate_emails: {
                        let field_value = match fields_map.get("allow_duplicate_emails") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_duplicate_emails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#anonymous: {
                        let field_value = match fields_map.get("anonymous") {
                            Some(value) => value,
                            None => bail!("Missing field 'anonymous' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email: {
                        let field_value = match fields_map.get("email") {
                            Some(value) => value,
                            None => bail!("Missing field 'email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hash_configs: {
                        let field_value = match fields_map.get("hash_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'hash_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phone_number: {
                        let field_value = match fields_map.get("phone_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'phone_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
