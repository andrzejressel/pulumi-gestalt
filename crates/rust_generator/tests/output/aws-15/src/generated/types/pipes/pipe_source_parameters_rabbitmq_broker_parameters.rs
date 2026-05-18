#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeSourceParametersRabbitmqBrokerParameters {
    /// The maximum number of records to include in each batch. Maximum value of 10000.
    #[builder(into)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Option<i32>,
    /// The credentials needed to access the resource. Detailed below.
    #[builder(into)]
    #[serde(rename = "credentials")]
    pub r#credentials: Box<super::super::types::pipes::PipeSourceParametersRabbitmqBrokerParametersCredentials>,
    /// The maximum length of a time to wait for events. Maximum value of 300.
    #[builder(into)]
    #[serde(rename = "maximumBatchingWindowInSeconds")]
    pub r#maximum_batching_window_in_seconds: Option<i32>,
    /// The name of the destination queue to consume. Maximum length of 1000.
    #[builder(into)]
    #[serde(rename = "queueName")]
    pub r#queue_name: String,
    /// The name of the virtual host associated with the source broker. Maximum length of 200.
    #[builder(into)]
    #[serde(rename = "virtualHost")]
    pub r#virtual_host: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeSourceParametersRabbitmqBrokerParameters {
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
                    "batch_size",
                    &self.r#batch_size,
                ),
                to_pulumi_object_field(
                    "credentials",
                    &self.r#credentials,
                ),
                to_pulumi_object_field(
                    "maximum_batching_window_in_seconds",
                    &self.r#maximum_batching_window_in_seconds,
                ),
                to_pulumi_object_field(
                    "queue_name",
                    &self.r#queue_name,
                ),
                to_pulumi_object_field(
                    "virtual_host",
                    &self.r#virtual_host,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeSourceParametersRabbitmqBrokerParameters {
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
                    r#batch_size: {
                        let field_value = match fields_map.get("batch_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credentials: {
                        let field_value = match fields_map.get("credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_batching_window_in_seconds: {
                        let field_value = match fields_map.get("maximum_batching_window_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_batching_window_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_name: {
                        let field_value = match fields_map.get("queue_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_host: {
                        let field_value = match fields_map.get("virtual_host") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
