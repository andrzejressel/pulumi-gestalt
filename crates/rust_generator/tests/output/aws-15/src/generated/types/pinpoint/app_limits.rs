#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppLimits {
    /// The maximum number of messages that the campaign can send daily.
    #[builder(into)]
    #[serde(rename = "daily")]
    pub r#daily: Option<i32>,
    /// The length of time (in seconds) that the campaign can run before it ends and message deliveries stop. This duration begins at the scheduled start time for the campaign. The minimum value is 60.
    #[builder(into)]
    #[serde(rename = "maximumDuration")]
    pub r#maximum_duration: Option<i32>,
    /// The number of messages that the campaign can send per second. The minimum value is 50, and the maximum is 20000.
    #[builder(into)]
    #[serde(rename = "messagesPerSecond")]
    pub r#messages_per_second: Option<i32>,
    /// The maximum total number of messages that the campaign can send.
    #[builder(into)]
    #[serde(rename = "total")]
    pub r#total: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppLimits {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "daily",
                    &self.r#daily,
                ),
                to_pulumi_object_field(
                    "maximum_duration",
                    &self.r#maximum_duration,
                ),
                to_pulumi_object_field(
                    "messages_per_second",
                    &self.r#messages_per_second,
                ),
                to_pulumi_object_field(
                    "total",
                    &self.r#total,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppLimits {
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
                    r#daily: {
                        let field_value = match fields_map.get("daily") {
                            Some(value) => value,
                            None => bail!("Missing field 'daily' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_duration: {
                        let field_value = match fields_map.get("maximum_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#messages_per_second: {
                        let field_value = match fields_map.get("messages_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'messages_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total: {
                        let field_value = match fields_map.get("total") {
                            Some(value) => value,
                            None => bail!("Missing field 'total' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
