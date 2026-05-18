#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobRetryConfig {
    /// The maximum amount of time to wait before retrying a job after it fails.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'.
    #[builder(into)]
    #[serde(rename = "maxBackoffDuration")]
    pub r#max_backoff_duration: Option<String>,
    /// The time between retries will double maxDoublings times.
    /// A job's retry interval starts at minBackoffDuration,
    /// then doubles maxDoublings times, then increases linearly,
    /// and finally retries retries at intervals of maxBackoffDuration up to retryCount times.
    #[builder(into)]
    #[serde(rename = "maxDoublings")]
    pub r#max_doublings: Option<i32>,
    /// The time limit for retrying a failed job, measured from time when an execution was first attempted.
    /// If specified with retryCount, the job will be retried until both limits are reached.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'.
    #[builder(into)]
    #[serde(rename = "maxRetryDuration")]
    pub r#max_retry_duration: Option<String>,
    /// The minimum amount of time to wait before retrying a job after it fails.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'.
    #[builder(into)]
    #[serde(rename = "minBackoffDuration")]
    pub r#min_backoff_duration: Option<String>,
    /// The number of attempts that the system will make to run a
    /// job using the exponential backoff procedure described by maxDoublings.
    /// Values greater than 5 and negative values are not allowed.
    #[builder(into)]
    #[serde(rename = "retryCount")]
    pub r#retry_count: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobRetryConfig {
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
                    "max_backoff_duration",
                    &self.r#max_backoff_duration,
                ),
                to_pulumi_object_field(
                    "max_doublings",
                    &self.r#max_doublings,
                ),
                to_pulumi_object_field(
                    "max_retry_duration",
                    &self.r#max_retry_duration,
                ),
                to_pulumi_object_field(
                    "min_backoff_duration",
                    &self.r#min_backoff_duration,
                ),
                to_pulumi_object_field(
                    "retry_count",
                    &self.r#retry_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobRetryConfig {
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
                    r#max_backoff_duration: {
                        let field_value = match fields_map.get("max_backoff_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_backoff_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_doublings: {
                        let field_value = match fields_map.get("max_doublings") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_doublings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_retry_duration: {
                        let field_value = match fields_map.get("max_retry_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_retry_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_backoff_duration: {
                        let field_value = match fields_map.get("min_backoff_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_backoff_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_count: {
                        let field_value = match fields_map.get("retry_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
