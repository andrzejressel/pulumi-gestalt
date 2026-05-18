#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRoleManagementPolicyNotificationRuleEligibleActivationApproverNotification {
    /// A list of additional email addresses that will receive these notifications.
    #[builder(into)]
    #[serde(rename = "additionalRecipients")]
    pub r#additional_recipients: Vec<String>,
    /// (Boolean) Should the default recipients receive these notifications.
    #[builder(into)]
    #[serde(rename = "defaultRecipients")]
    pub r#default_recipients: bool,
    /// (String) What level of notifications should be sent. Either `All` or `Critical`.
    #[builder(into)]
    #[serde(rename = "notificationLevel")]
    pub r#notification_level: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRoleManagementPolicyNotificationRuleEligibleActivationApproverNotification {
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
                    "additional_recipients",
                    &self.r#additional_recipients,
                ),
                to_pulumi_object_field(
                    "default_recipients",
                    &self.r#default_recipients,
                ),
                to_pulumi_object_field(
                    "notification_level",
                    &self.r#notification_level,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRoleManagementPolicyNotificationRuleEligibleActivationApproverNotification {
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
                    r#additional_recipients: {
                        let field_value = match fields_map.get("additional_recipients") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_recipients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_recipients: {
                        let field_value = match fields_map.get("default_recipients") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_recipients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_level: {
                        let field_value = match fields_map.get("notification_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
