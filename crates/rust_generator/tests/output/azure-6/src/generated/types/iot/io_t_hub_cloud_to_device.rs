#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IoTHubCloudToDevice {
    /// The default time to live for cloud-to-device messages, specified as an [ISO 8601 timespan duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This value must be between 1 minute and 48 hours. Defaults to `PT1H`.
    #[builder(into)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Option<String>,
    /// A `feedback` block as defined below.
    #[builder(into)]
    #[serde(rename = "feedbacks")]
    pub r#feedbacks: Option<Vec<super::super::types::iot::IoTHubCloudToDeviceFeedback>>,
    /// The maximum delivery count for cloud-to-device per-device queues. This value must be between `1` and `100`. Defaults to `10`.
    #[builder(into)]
    #[serde(rename = "maxDeliveryCount")]
    pub r#max_delivery_count: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IoTHubCloudToDevice {
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
                    "default_ttl",
                    &self.r#default_ttl,
                ),
                to_pulumi_object_field(
                    "feedbacks",
                    &self.r#feedbacks,
                ),
                to_pulumi_object_field(
                    "max_delivery_count",
                    &self.r#max_delivery_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IoTHubCloudToDevice {
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
                    r#default_ttl: {
                        let field_value = match fields_map.get("default_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#feedbacks: {
                        let field_value = match fields_map.get("feedbacks") {
                            Some(value) => value,
                            None => bail!("Missing field 'feedbacks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_delivery_count: {
                        let field_value = match fields_map.get("max_delivery_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_delivery_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
