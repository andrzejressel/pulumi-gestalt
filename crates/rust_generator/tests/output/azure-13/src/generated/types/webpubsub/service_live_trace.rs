#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceLiveTrace {
    /// Whether the log category `ConnectivityLogs` is enabled? Defaults to `true`
    #[builder(into)]
    #[serde(rename = "connectivityLogsEnabled")]
    pub r#connectivity_logs_enabled: Option<bool>,
    /// Whether the live trace is enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Whether the log category `HttpRequestLogs` is enabled? Defaults to `true`
    #[builder(into)]
    #[serde(rename = "httpRequestLogsEnabled")]
    pub r#http_request_logs_enabled: Option<bool>,
    /// Whether the log category `MessagingLogs` is enabled? Defaults to `true`
    #[builder(into)]
    #[serde(rename = "messagingLogsEnabled")]
    pub r#messaging_logs_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceLiveTrace {
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
                    "connectivity_logs_enabled",
                    &self.r#connectivity_logs_enabled,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "http_request_logs_enabled",
                    &self.r#http_request_logs_enabled,
                ),
                to_pulumi_object_field(
                    "messaging_logs_enabled",
                    &self.r#messaging_logs_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceLiveTrace {
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
                    r#connectivity_logs_enabled: {
                        let field_value = match fields_map.get("connectivity_logs_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectivity_logs_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_request_logs_enabled: {
                        let field_value = match fields_map.get("http_request_logs_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_request_logs_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#messaging_logs_enabled: {
                        let field_value = match fields_map.get("messaging_logs_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'messaging_logs_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
