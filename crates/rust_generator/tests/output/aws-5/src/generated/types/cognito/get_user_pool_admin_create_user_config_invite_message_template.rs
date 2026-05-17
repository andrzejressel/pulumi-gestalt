#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolAdminCreateUserConfigInviteMessageTemplate {
    /// - Email message content.
    #[builder(into)]
    #[serde(rename = "emailMessage")]
    pub r#email_message: String,
    /// - Email message subject.
    #[builder(into)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: String,
    /// - SMS message content.
    #[builder(into)]
    #[serde(rename = "smsMessage")]
    pub r#sms_message: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserPoolAdminCreateUserConfigInviteMessageTemplate {
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
                "email_message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_message,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserPoolAdminCreateUserConfigInviteMessageTemplate {
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
                    r#email_message: {
                        let field_value = match fields_map.get("email_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
