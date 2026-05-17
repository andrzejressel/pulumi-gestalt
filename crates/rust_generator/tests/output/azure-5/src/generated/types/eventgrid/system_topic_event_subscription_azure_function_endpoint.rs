#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SystemTopicEventSubscriptionAzureFunctionEndpoint {
    /// Specifies the ID of the Function where the Event Subscription will receive events. This must be the functions ID in format {function_app.id}/functions/{name}.
    #[builder(into)]
    #[serde(rename = "functionId")]
    pub r#function_id: String,
    /// Maximum number of events per batch.
    #[builder(into)]
    #[serde(rename = "maxEventsPerBatch")]
    pub r#max_events_per_batch: Option<i32>,
    /// Preferred batch size in Kilobytes.
    #[builder(into)]
    #[serde(rename = "preferredBatchSizeInKilobytes")]
    pub r#preferred_batch_size_in_kilobytes: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SystemTopicEventSubscriptionAzureFunctionEndpoint {
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
                    "function_id",
                    &self.r#function_id,
                ),
                to_pulumi_object_field(
                    "max_events_per_batch",
                    &self.r#max_events_per_batch,
                ),
                to_pulumi_object_field(
                    "preferred_batch_size_in_kilobytes",
                    &self.r#preferred_batch_size_in_kilobytes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SystemTopicEventSubscriptionAzureFunctionEndpoint {
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
                    r#function_id: {
                        let field_value = match fields_map.get("function_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'function_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_events_per_batch: {
                        let field_value = match fields_map.get("max_events_per_batch") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_events_per_batch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preferred_batch_size_in_kilobytes: {
                        let field_value = match fields_map.get("preferred_batch_size_in_kilobytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferred_batch_size_in_kilobytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
