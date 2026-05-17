#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppServiceLogs {
    /// An `application_logs` block as defined below.
    #[builder(into)]
    #[serde(rename = "applicationLogs")]
    pub r#application_logs: Option<Box<super::super::types::appservice::AppServiceLogsApplicationLogs>>,
    /// Should `Detailed error messages` be enabled on this App Service? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "detailedErrorMessagesEnabled")]
    pub r#detailed_error_messages_enabled: Option<bool>,
    /// Should `Failed request tracing` be enabled on this App Service? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "failedRequestTracingEnabled")]
    pub r#failed_request_tracing_enabled: Option<bool>,
    /// An `http_logs` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpLogs")]
    pub r#http_logs: Option<Box<super::super::types::appservice::AppServiceLogsHttpLogs>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppServiceLogs {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "application_logs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_logs,
                )
                .await,
            );
            map.insert(
                "detailed_error_messages_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#detailed_error_messages_enabled,
                )
                .await,
            );
            map.insert(
                "failed_request_tracing_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failed_request_tracing_enabled,
                )
                .await,
            );
            map.insert(
                "http_logs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_logs,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppServiceLogs {
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
                    r#application_logs: {
                        let field_value = match fields_map.get("application_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detailed_error_messages_enabled: {
                        let field_value = match fields_map.get("detailed_error_messages_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'detailed_error_messages_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failed_request_tracing_enabled: {
                        let field_value = match fields_map.get("failed_request_tracing_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'failed_request_tracing_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_logs: {
                        let field_value = match fields_map.get("http_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
