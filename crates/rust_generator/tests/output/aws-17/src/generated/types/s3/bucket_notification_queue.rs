#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketNotificationQueue {
    /// Specifies [event](http://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html#notification-how-to-event-types-and-destinations) for which to send notifications.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Vec<String>,
    /// Object key name prefix.
    #[builder(into)]
    #[serde(rename = "filterPrefix")]
    pub r#filter_prefix: Option<String>,
    /// Object key name suffix.
    #[builder(into)]
    #[serde(rename = "filterSuffix")]
    pub r#filter_suffix: Option<String>,
    /// Unique identifier for each of the notification configurations.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// SQS queue ARN.
    #[builder(into)]
    #[serde(rename = "queueArn")]
    pub r#queue_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketNotificationQueue {
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
                    "events",
                    &self.r#events,
                ),
                to_pulumi_object_field(
                    "filter_prefix",
                    &self.r#filter_prefix,
                ),
                to_pulumi_object_field(
                    "filter_suffix",
                    &self.r#filter_suffix,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "queue_arn",
                    &self.r#queue_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketNotificationQueue {
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
                    r#events: {
                        let field_value = match fields_map.get("events") {
                            Some(value) => value,
                            None => bail!("Missing field 'events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_prefix: {
                        let field_value = match fields_map.get("filter_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_suffix: {
                        let field_value = match fields_map.get("filter_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_arn: {
                        let field_value = match fields_map.get("queue_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
