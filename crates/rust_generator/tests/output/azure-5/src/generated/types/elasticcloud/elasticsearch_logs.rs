#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ElasticsearchLogs {
    /// A list of `filtering_tag` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "filteringTags")]
    pub r#filtering_tags: Option<Vec<super::super::types::elasticcloud::ElasticsearchLogsFilteringTag>>,
    /// Specifies if the Azure Activity Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendActivityLogs")]
    pub r#send_activity_logs: Option<bool>,
    /// Specifies if the AzureAD Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendAzureadLogs")]
    pub r#send_azuread_logs: Option<bool>,
    /// Specifies if the Azure Subscription Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendSubscriptionLogs")]
    pub r#send_subscription_logs: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ElasticsearchLogs {
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
                    "filtering_tags",
                    &self.r#filtering_tags,
                ),
                to_pulumi_object_field(
                    "send_activity_logs",
                    &self.r#send_activity_logs,
                ),
                to_pulumi_object_field(
                    "send_azuread_logs",
                    &self.r#send_azuread_logs,
                ),
                to_pulumi_object_field(
                    "send_subscription_logs",
                    &self.r#send_subscription_logs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ElasticsearchLogs {
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
                    r#filtering_tags: {
                        let field_value = match fields_map.get("filtering_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'filtering_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_activity_logs: {
                        let field_value = match fields_map.get("send_activity_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'send_activity_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_azuread_logs: {
                        let field_value = match fields_map.get("send_azuread_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'send_azuread_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_subscription_logs: {
                        let field_value = match fields_map.get("send_subscription_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'send_subscription_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
