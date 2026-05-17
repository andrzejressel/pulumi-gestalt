#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventSubscriptionWebhookEndpoint {
    /// The Azure Active Directory Application ID or URI to get the access token that will be included as the bearer token in delivery requests.
    #[builder(into)]
    #[serde(rename = "activeDirectoryAppIdOrUri")]
    pub r#active_directory_app_id_or_uri: Option<String>,
    /// The Azure Active Directory Tenant ID to get the access token that will be included as the bearer token in delivery requests.
    #[builder(into)]
    #[serde(rename = "activeDirectoryTenantId")]
    pub r#active_directory_tenant_id: Option<String>,
    /// The base url of the webhook where the Event Subscription will receive events.
    #[builder(into)]
    #[serde(rename = "baseUrl")]
    pub r#base_url: Option<String>,
    /// Maximum number of events per batch.
    #[builder(into)]
    #[serde(rename = "maxEventsPerBatch")]
    pub r#max_events_per_batch: Option<i32>,
    /// Preferred batch size in Kilobytes.
    #[builder(into)]
    #[serde(rename = "preferredBatchSizeInKilobytes")]
    pub r#preferred_batch_size_in_kilobytes: Option<i32>,
    /// Specifies the url of the webhook where the Event Subscription will receive events.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventSubscriptionWebhookEndpoint {
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
                    "active_directory_app_id_or_uri",
                    &self.r#active_directory_app_id_or_uri,
                ),
                to_pulumi_object_field(
                    "active_directory_tenant_id",
                    &self.r#active_directory_tenant_id,
                ),
                to_pulumi_object_field(
                    "base_url",
                    &self.r#base_url,
                ),
                to_pulumi_object_field(
                    "max_events_per_batch",
                    &self.r#max_events_per_batch,
                ),
                to_pulumi_object_field(
                    "preferred_batch_size_in_kilobytes",
                    &self.r#preferred_batch_size_in_kilobytes,
                ),
                to_pulumi_object_field(
                    "url",
                    &self.r#url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventSubscriptionWebhookEndpoint {
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
                    r#active_directory_app_id_or_uri: {
                        let field_value = match fields_map.get("active_directory_app_id_or_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directory_app_id_or_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#active_directory_tenant_id: {
                        let field_value = match fields_map.get("active_directory_tenant_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directory_tenant_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#base_url: {
                        let field_value = match fields_map.get("base_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
