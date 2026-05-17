#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPoolVerificationMessageTemplate {
    /// Default email option. Must be either `CONFIRM_WITH_CODE` or `CONFIRM_WITH_LINK`. Defaults to `CONFIRM_WITH_CODE`.
    #[builder(into)]
    #[serde(rename = "defaultEmailOption")]
    pub r#default_email_option: Option<String>,
    /// Email message template. Must contain the `{####}` placeholder. Conflicts with `email_verification_message` argument.
    #[builder(into)]
    #[serde(rename = "emailMessage")]
    pub r#email_message: Option<String>,
    /// Email message template for sending a confirmation link to the user, it must contain the `{##Click Here##}` placeholder.
    #[builder(into)]
    #[serde(rename = "emailMessageByLink")]
    pub r#email_message_by_link: Option<String>,
    /// Subject line for the email message template. Conflicts with `email_verification_subject` argument.
    #[builder(into)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: Option<String>,
    /// Subject line for the email message template for sending a confirmation link to the user.
    #[builder(into)]
    #[serde(rename = "emailSubjectByLink")]
    pub r#email_subject_by_link: Option<String>,
    /// SMS message template. Must contain the `{####}` placeholder. Conflicts with `sms_verification_message` argument.
    #[builder(into)]
    #[serde(rename = "smsMessage")]
    pub r#sms_message: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPoolVerificationMessageTemplate {
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
                "default_email_option".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_email_option,
                )
                .await,
            );
            map.insert(
                "email_message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_message,
                )
                .await,
            );
            map.insert(
                "email_message_by_link".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_message_by_link,
                )
                .await,
            );
            map.insert(
                "email_subject".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_subject,
                )
                .await,
            );
            map.insert(
                "email_subject_by_link".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_subject_by_link,
                )
                .await,
            );
            map.insert(
                "sms_message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sms_message,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPoolVerificationMessageTemplate {
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
                    r#default_email_option: {
                        let field_value = match fields_map.get("default_email_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_email_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_message: {
                        let field_value = match fields_map.get("email_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_message_by_link: {
                        let field_value = match fields_map.get("email_message_by_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_message_by_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_subject: {
                        let field_value = match fields_map.get("email_subject") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_subject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_subject_by_link: {
                        let field_value = match fields_map.get("email_subject_by_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_subject_by_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sms_message: {
                        let field_value = match fields_map.get("sms_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'sms_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
