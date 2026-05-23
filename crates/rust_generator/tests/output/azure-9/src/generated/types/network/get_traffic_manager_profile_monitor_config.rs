#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTrafficManagerProfileMonitorConfig {
    /// One or more `custom_header` blocks as defined below.
    #[builder(into)]
    pub r#custom_headers: Vec<super::super::types::network::GetTrafficManagerProfileMonitorConfigCustomHeader>,
    /// A list of status code ranges.
    #[builder(into)]
    pub r#expected_status_code_ranges: Vec<String>,
    /// The interval used to check the endpoint health from a Traffic Manager probing agent.
    #[builder(into)]
    pub r#interval_in_seconds: i32,
    /// The path used by the monitoring checks.
    #[builder(into)]
    pub r#path: String,
    /// The port number used by the monitoring checks.
    #[builder(into)]
    pub r#port: i32,
    /// The protocol used by the monitoring checks.
    #[builder(into)]
    pub r#protocol: String,
    /// The amount of time the Traffic Manager probing agent should wait before considering that check a failure when a health check probe is sent to the endpoint.
    #[builder(into)]
    pub r#timeout_in_seconds: i32,
    /// The number of failures a Traffic Manager probing agent tolerates before marking that endpoint as unhealthy.
    #[builder(into)]
    pub r#tolerated_number_of_failures: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTrafficManagerProfileMonitorConfig {
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
                    "customHeaders",
                    &self.r#custom_headers,
                ),
                to_pulumi_object_field(
                    "expectedStatusCodeRanges",
                    &self.r#expected_status_code_ranges,
                ),
                to_pulumi_object_field(
                    "intervalInSeconds",
                    &self.r#interval_in_seconds,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "timeoutInSeconds",
                    &self.r#timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "toleratedNumberOfFailures",
                    &self.r#tolerated_number_of_failures,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTrafficManagerProfileMonitorConfig {
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
                    r#custom_headers: {
                        let field_value = match fields_map.get("customHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'customHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expected_status_code_ranges: {
                        let field_value = match fields_map.get("expectedStatusCodeRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'expectedStatusCodeRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval_in_seconds: {
                        let field_value = match fields_map.get("intervalInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'intervalInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tolerated_number_of_failures: {
                        let field_value = match fields_map.get("toleratedNumberOfFailures") {
                            Some(value) => value,
                            None => bail!("Missing field 'toleratedNumberOfFailures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
