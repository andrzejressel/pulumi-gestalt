#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriberNotificationConfiguration {
    /// The configurations for HTTPS subscriber notification.
    #[builder(into)]
    #[serde(rename = "httpsNotificationConfiguration")]
    pub r#https_notification_configuration: Option<Box<super::super::types::securitylake::SubscriberNotificationConfigurationHttpsNotificationConfiguration>>,
    /// The configurations for SQS subscriber notification.
    /// There are no parameters within `sqs_notification_configuration`.
    #[builder(into)]
    #[serde(rename = "sqsNotificationConfiguration")]
    pub r#sqs_notification_configuration: Option<Box<super::super::types::securitylake::SubscriberNotificationConfigurationSqsNotificationConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriberNotificationConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("https_notification_configuration".to_string(), self.r#https_notification_configuration.to_pulumi_value().await);
            map.insert("sqs_notification_configuration".to_string(), self.r#sqs_notification_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriberNotificationConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#https_notification_configuration: {
                        let field_value = match fields_map.get("https_notification_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_notification_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securitylake::SubscriberNotificationConfigurationHttpsNotificationConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sqs_notification_configuration: {
                        let field_value = match fields_map.get("sqs_notification_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqs_notification_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securitylake::SubscriberNotificationConfigurationSqsNotificationConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
