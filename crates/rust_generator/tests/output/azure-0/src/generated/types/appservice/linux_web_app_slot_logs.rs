#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxWebAppSlotLogs {
    /// A `application_logs` block as defined above.
    #[builder(into)]
    #[serde(rename = "applicationLogs")]
    pub r#application_logs: Option<Box<super::super::types::appservice::LinuxWebAppSlotLogsApplicationLogs>>,
    /// Should detailed error messages be enabled?
    #[builder(into)]
    #[serde(rename = "detailedErrorMessages")]
    pub r#detailed_error_messages: Option<bool>,
    /// Should the failed request tracing be enabled?
    #[builder(into)]
    #[serde(rename = "failedRequestTracing")]
    pub r#failed_request_tracing: Option<bool>,
    /// An `http_logs` block as defined above.
    #[builder(into)]
    #[serde(rename = "httpLogs")]
    pub r#http_logs: Option<Box<super::super::types::appservice::LinuxWebAppSlotLogsHttpLogs>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxWebAppSlotLogs {
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
                    "application_logs",
                    &self.r#application_logs,
                ),
                to_pulumi_object_field(
                    "detailed_error_messages",
                    &self.r#detailed_error_messages,
                ),
                to_pulumi_object_field(
                    "failed_request_tracing",
                    &self.r#failed_request_tracing,
                ),
                to_pulumi_object_field(
                    "http_logs",
                    &self.r#http_logs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxWebAppSlotLogs {
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
                    r#detailed_error_messages: {
                        let field_value = match fields_map.get("detailed_error_messages") {
                            Some(value) => value,
                            None => bail!("Missing field 'detailed_error_messages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failed_request_tracing: {
                        let field_value = match fields_map.get("failed_request_tracing") {
                            Some(value) => value,
                            None => bail!("Missing field 'failed_request_tracing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
