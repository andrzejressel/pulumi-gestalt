#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscaleSettingNotificationEmail {
    /// Specifies a list of custom email addresses to which the email notifications will be sent.
    #[builder(into)]
    #[serde(rename = "customEmails")]
    pub r#custom_emails: Option<Vec<String>>,
    /// Should email notifications be sent to the subscription administrator? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendToSubscriptionAdministrator")]
    pub r#send_to_subscription_administrator: Option<bool>,
    /// Should email notifications be sent to the subscription co-administrator? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendToSubscriptionCoAdministrator")]
    pub r#send_to_subscription_co_administrator: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscaleSettingNotificationEmail {
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
                    "custom_emails",
                    &self.r#custom_emails,
                ),
                to_pulumi_object_field(
                    "send_to_subscription_administrator",
                    &self.r#send_to_subscription_administrator,
                ),
                to_pulumi_object_field(
                    "send_to_subscription_co_administrator",
                    &self.r#send_to_subscription_co_administrator,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutoscaleSettingNotificationEmail {
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
                    r#custom_emails: {
                        let field_value = match fields_map.get("custom_emails") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_emails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_to_subscription_administrator: {
                        let field_value = match fields_map.get("send_to_subscription_administrator") {
                            Some(value) => value,
                            None => bail!("Missing field 'send_to_subscription_administrator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_to_subscription_co_administrator: {
                        let field_value = match fields_map.get("send_to_subscription_co_administrator") {
                            Some(value) => value,
                            None => bail!("Missing field 'send_to_subscription_co_administrator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
