#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPolicyAlertStrategyNotificationChannelStrategy {
    /// The notification channels that these settings apply to. Each of these
    /// correspond to the name field in one of the NotificationChannel objects
    /// referenced in the notification_channels field of this AlertPolicy. The format is
    /// `projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]`
    #[builder(into)]
    #[serde(rename = "notificationChannelNames")]
    pub r#notification_channel_names: Option<Vec<String>>,
    /// The frequency at which to send reminder notifications for open incidents.
    #[builder(into)]
    #[serde(rename = "renotifyInterval")]
    pub r#renotify_interval: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertPolicyAlertStrategyNotificationChannelStrategy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "notification_channel_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notification_channel_names,
                )
                .await,
            );
            map.insert(
                "renotify_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#renotify_interval,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertPolicyAlertStrategyNotificationChannelStrategy {
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
                    r#notification_channel_names: {
                        let field_value = match fields_map.get("notification_channel_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_channel_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#renotify_interval: {
                        let field_value = match fields_map.get("renotify_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'renotify_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
