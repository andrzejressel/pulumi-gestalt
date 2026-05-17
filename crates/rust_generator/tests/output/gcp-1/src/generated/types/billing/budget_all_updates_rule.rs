#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetAllUpdatesRule {
    /// Boolean. When set to true, disables default notifications sent
    /// when a threshold is exceeded. Default recipients are
    /// those with Billing Account Administrators and Billing
    /// Account Users IAM roles for the target account.
    #[builder(into)]
    #[serde(rename = "disableDefaultIamRecipients")]
    pub r#disable_default_iam_recipients: Option<bool>,
    /// When set to true, and when the budget has a single project configured,
    /// notifications will be sent to project level recipients of that project.
    /// This field will be ignored if the budget has multiple or no project configured.
    /// Currently, project level recipients are the users with Owner role on a cloud project.
    #[builder(into)]
    #[serde(rename = "enableProjectLevelRecipients")]
    pub r#enable_project_level_recipients: Option<bool>,
    /// The full resource name of a monitoring notification
    /// channel in the form
    /// projects/{project_id}/notificationChannels/{channel_id}.
    /// A maximum of 5 channels are allowed.
    #[builder(into)]
    #[serde(rename = "monitoringNotificationChannels")]
    pub r#monitoring_notification_channels: Option<Vec<String>>,
    /// The name of the Cloud Pub/Sub topic where budget related
    /// messages will be published, in the form
    /// projects/{project_id}/topics/{topic_id}. Updates are sent
    /// at regular intervals to the topic.
    #[builder(into)]
    #[serde(rename = "pubsubTopic")]
    pub r#pubsub_topic: Option<String>,
    /// The schema version of the notification. Only "1.0" is
    /// accepted. It represents the JSON schema as defined in
    /// https://cloud.google.com/billing/docs/how-to/budgets#notification_format.
    #[builder(into)]
    #[serde(rename = "schemaVersion")]
    pub r#schema_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetAllUpdatesRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "disable_default_iam_recipients",
                    &self.r#disable_default_iam_recipients,
                ),
                to_pulumi_object_field(
                    "enable_project_level_recipients",
                    &self.r#enable_project_level_recipients,
                ),
                to_pulumi_object_field(
                    "monitoring_notification_channels",
                    &self.r#monitoring_notification_channels,
                ),
                to_pulumi_object_field(
                    "pubsub_topic",
                    &self.r#pubsub_topic,
                ),
                to_pulumi_object_field(
                    "schema_version",
                    &self.r#schema_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetAllUpdatesRule {
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
                    r#disable_default_iam_recipients: {
                        let field_value = match fields_map.get("disable_default_iam_recipients") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_default_iam_recipients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_project_level_recipients: {
                        let field_value = match fields_map.get("enable_project_level_recipients") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_project_level_recipients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitoring_notification_channels: {
                        let field_value = match fields_map.get("monitoring_notification_channels") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitoring_notification_channels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pubsub_topic: {
                        let field_value = match fields_map.get("pubsub_topic") {
                            Some(value) => value,
                            None => bail!("Missing field 'pubsub_topic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_version: {
                        let field_value = match fields_map.get("schema_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
