#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolEmailConfiguration {
    /// - Configuration set used for sending emails.
    #[builder(into)]
    pub r#configuration_set: String,
    /// - Email sending account.
    #[builder(into)]
    pub r#email_sending_account: String,
    /// - Email sender address.
    #[builder(into)]
    pub r#from: String,
    /// - Reply-to email address.
    #[builder(into)]
    pub r#reply_to_email_address: String,
    /// - Source Amazon Resource Name (ARN) for emails.
    #[builder(into)]
    pub r#source_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserPoolEmailConfiguration {
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
                    "configurationSet",
                    &self.r#configuration_set,
                ),
                to_pulumi_object_field(
                    "emailSendingAccount",
                    &self.r#email_sending_account,
                ),
                to_pulumi_object_field(
                    "from",
                    &self.r#from,
                ),
                to_pulumi_object_field(
                    "replyToEmailAddress",
                    &self.r#reply_to_email_address,
                ),
                to_pulumi_object_field(
                    "sourceArn",
                    &self.r#source_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserPoolEmailConfiguration {
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
                        let field_value = match fields_map.get("configurationSet") {
                            Some(value) => value,
                            None => bail!("Missing field 'configurationSet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_sending_account: {
                        let field_value = match fields_map.get("emailSendingAccount") {
                            Some(value) => value,
                            None => bail!("Missing field 'emailSendingAccount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from: {
                        let field_value = match fields_map.get("from") {
                            Some(value) => value,
                            None => bail!("Missing field 'from' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reply_to_email_address: {
                        let field_value = match fields_map.get("replyToEmailAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'replyToEmailAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_arn: {
                        let field_value = match fields_map.get("sourceArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
