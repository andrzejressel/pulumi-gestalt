#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolEmailConfiguration {
    /// - Configuration set used for sending emails.
    #[builder(into)]
    #[serde(rename = "configurationSet")]
    pub r#configuration_set: String,
    /// - Email sending account.
    #[builder(into)]
    #[serde(rename = "emailSendingAccount")]
    pub r#email_sending_account: String,
    /// - Email sender address.
    #[builder(into)]
    #[serde(rename = "from")]
    pub r#from: String,
    /// - Reply-to email address.
    #[builder(into)]
    #[serde(rename = "replyToEmailAddress")]
    pub r#reply_to_email_address: String,
    /// - Source Amazon Resource Name (ARN) for emails.
    #[builder(into)]
    #[serde(rename = "sourceArn")]
    pub r#source_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserPoolEmailConfiguration {
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
                "configuration_set".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#configuration_set,
                )
                .await,
            );
            map.insert(
                "email_sending_account".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_sending_account,
                )
                .await,
            );
            map.insert(
                "from".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#from,
                )
                .await,
            );
            map.insert(
                "reply_to_email_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reply_to_email_address,
                )
                .await,
            );
            map.insert(
                "source_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_arn,
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
                    r#from: {
                        let field_value = match fields_map.get("from") {
                            Some(value) => value,
                            None => bail!("Missing field 'from' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
