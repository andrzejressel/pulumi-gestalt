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
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "https_notification_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#https_notification_configuration,
                )
                .await,
            );
            map.insert(
                "sqs_notification_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sqs_notification_configuration,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriberNotificationConfiguration {
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
                    r#https_notification_configuration: {
                        let field_value = match fields_map.get("https_notification_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_notification_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sqs_notification_configuration: {
                        let field_value = match fields_map.get("sqs_notification_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqs_notification_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
