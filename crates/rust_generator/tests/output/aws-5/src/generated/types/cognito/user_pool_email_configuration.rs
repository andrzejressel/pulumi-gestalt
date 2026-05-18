#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPoolEmailConfiguration {
    /// Email configuration set name from SES.
    #[builder(into)]
    #[serde(rename = "configurationSet")]
    pub r#configuration_set: Option<String>,
    /// Email delivery method to use. `COGNITO_DEFAULT` for the default email functionality built into Cognito or `DEVELOPER` to use your Amazon SES configuration. Required to be `DEVELOPER` if `from_email_address` is set.
    #[builder(into)]
    #[serde(rename = "emailSendingAccount")]
    pub r#email_sending_account: Option<String>,
    /// Sender’s email address or sender’s display name with their email address (e.g., `john@example.com`, `John Smith <john@example.com>` or `\"John Smith Ph.D.\" <john@example.com>`). Escaped double quotes are required around display names that contain certain characters as specified in [RFC 5322](https://tools.ietf.org/html/rfc5322).
    #[builder(into)]
    #[serde(rename = "fromEmailAddress")]
    pub r#from_email_address: Option<String>,
    /// REPLY-TO email address.
    #[builder(into)]
    #[serde(rename = "replyToEmailAddress")]
    pub r#reply_to_email_address: Option<String>,
    /// ARN of the SES verified email identity to use. Required if `email_sending_account` is set to `DEVELOPER`.
    #[builder(into)]
    #[serde(rename = "sourceArn")]
    pub r#source_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPoolEmailConfiguration {
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
                    "configuration_set",
                    &self.r#configuration_set,
                ),
                to_pulumi_object_field(
                    "email_sending_account",
                    &self.r#email_sending_account,
                ),
                to_pulumi_object_field(
                    "from_email_address",
                    &self.r#from_email_address,
                ),
                to_pulumi_object_field(
                    "reply_to_email_address",
                    &self.r#reply_to_email_address,
                ),
                to_pulumi_object_field(
                    "source_arn",
                    &self.r#source_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPoolEmailConfiguration {
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
                    r#configuration_set: {
                        let field_value = match fields_map.get("configuration_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'configuration_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_sending_account: {
                        let field_value = match fields_map.get("email_sending_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_sending_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_email_address: {
                        let field_value = match fields_map.get("from_email_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_email_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reply_to_email_address: {
                        let field_value = match fields_map.get("reply_to_email_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'reply_to_email_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_arn: {
                        let field_value = match fields_map.get("source_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
