#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPolicyAlertStrategy {
    /// If an alert policy that was active has no data for this long, any open incidents will close.
    #[builder(into)]
    #[serde(rename = "autoClose")]
    pub r#auto_close: Option<String>,
    /// Control over how the notification channels in `notification_channels`
    /// are notified when this alert fires, on a per-channel basis.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "notificationChannelStrategies")]
    pub r#notification_channel_strategies: Option<Vec<super::super::types::monitoring::AlertPolicyAlertStrategyNotificationChannelStrategy>>,
    /// Control when notifications will be sent out.
    /// Each value may be one of: `NOTIFICATION_PROMPT_UNSPECIFIED`, `OPENED`, `CLOSED`.
    #[builder(into)]
    #[serde(rename = "notificationPrompts")]
    pub r#notification_prompts: Option<Vec<String>>,
    /// Required for alert policies with a LogMatch condition.
    /// This limit is not implemented for alert policies that are not log-based.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "notificationRateLimit")]
    pub r#notification_rate_limit: Option<Box<super::super::types::monitoring::AlertPolicyAlertStrategyNotificationRateLimit>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertPolicyAlertStrategy {
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
                "auto_close".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_close,
                )
                .await,
            );
            map.insert(
                "notification_channel_strategies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notification_channel_strategies,
                )
                .await,
            );
            map.insert(
                "notification_prompts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notification_prompts,
                )
                .await,
            );
            map.insert(
                "notification_rate_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notification_rate_limit,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertPolicyAlertStrategy {
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
                    r#auto_close: {
                        let field_value = match fields_map.get("auto_close") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_close' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_channel_strategies: {
                        let field_value = match fields_map.get("notification_channel_strategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_channel_strategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_prompts: {
                        let field_value = match fields_map.get("notification_prompts") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_prompts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_rate_limit: {
                        let field_value = match fields_map.get("notification_rate_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_rate_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
