#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionEventTrigger {
    /// Criteria used to filter events.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "eventFilters")]
    pub r#event_filters: Option<Vec<super::super::types::cloudfunctionsv2::FunctionEventTriggerEventFilter>>,
    /// Required. The type of event to observe.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: Option<String>,
    /// The name of a Pub/Sub topic in the same project that will be used
    /// as the transport topic for the event delivery.
    #[builder(into)]
    #[serde(rename = "pubsubTopic")]
    pub r#pubsub_topic: Option<String>,
    /// Describes the retry policy in case of function's execution failure.
    /// Retried execution is charged as any other execution.
    /// Possible values are: `RETRY_POLICY_UNSPECIFIED`, `RETRY_POLICY_DO_NOT_RETRY`, `RETRY_POLICY_RETRY`.
    #[builder(into)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Option<String>,
    /// Optional. The email of the trigger's service account. The service account
    /// must have permission to invoke Cloud Run services. If empty, defaults to the
    /// Compute Engine default service account: {project_number}-compute@developer.gserviceaccount.com.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Option<String>,
    /// (Output)
    /// Output only. The resource name of the Eventarc trigger.
    #[builder(into)]
    #[serde(rename = "trigger")]
    pub r#trigger: Option<String>,
    /// The region that the trigger will be in. The trigger will only receive
    /// events originating in this region. It can be the same
    /// region as the function, a different region or multi-region, or the global
    /// region. If not provided, defaults to the same region as the function.
    #[builder(into)]
    #[serde(rename = "triggerRegion")]
    pub r#trigger_region: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FunctionEventTrigger {
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
                    "event_filters",
                    &self.r#event_filters,
                ),
                to_pulumi_object_field(
                    "event_type",
                    &self.r#event_type,
                ),
                to_pulumi_object_field(
                    "pubsub_topic",
                    &self.r#pubsub_topic,
                ),
                to_pulumi_object_field(
                    "retry_policy",
                    &self.r#retry_policy,
                ),
                to_pulumi_object_field(
                    "service_account_email",
                    &self.r#service_account_email,
                ),
                to_pulumi_object_field(
                    "trigger",
                    &self.r#trigger,
                ),
                to_pulumi_object_field(
                    "trigger_region",
                    &self.r#trigger_region,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FunctionEventTrigger {
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
                    r#event_filters: {
                        let field_value = match fields_map.get("event_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_type: {
                        let field_value = match fields_map.get("event_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#retry_policy: {
                        let field_value = match fields_map.get("retry_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_email: {
                        let field_value = match fields_map.get("service_account_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trigger: {
                        let field_value = match fields_map.get("trigger") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trigger_region: {
                        let field_value = match fields_map.get("trigger_region") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger_region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
