#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTrafficManagerProfileMonitorConfig {
    /// One or more `custom_header` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "customHeaders")]
    pub r#custom_headers: Vec<super::super::types::network::GetTrafficManagerProfileMonitorConfigCustomHeader>,
    /// A list of status code ranges.
    #[builder(into)]
    #[serde(rename = "expectedStatusCodeRanges")]
    pub r#expected_status_code_ranges: Vec<String>,
    /// The interval used to check the endpoint health from a Traffic Manager probing agent.
    #[builder(into)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: i32,
    /// The path used by the monitoring checks.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// The port number used by the monitoring checks.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// The protocol used by the monitoring checks.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The amount of time the Traffic Manager probing agent should wait before considering that check a failure when a health check probe is sent to the endpoint.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: i32,
    /// The number of failures a Traffic Manager probing agent tolerates before marking that endpoint as unhealthy.
    #[builder(into)]
    #[serde(rename = "toleratedNumberOfFailures")]
    pub r#tolerated_number_of_failures: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTrafficManagerProfileMonitorConfig {
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
                "custom_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_headers,
                )
                .await,
            );
            map.insert(
                "expected_status_code_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expected_status_code_ranges,
                )
                .await,
            );
            map.insert(
                "interval_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#interval_in_seconds,
                )
                .await,
            );
            map.insert(
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
                )
                .await,
            );
            map.insert(
                "port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port,
                )
                .await,
            );
            map.insert(
                "protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol,
                )
                .await,
            );
            map.insert(
                "timeout_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_in_seconds,
                )
                .await,
            );
            map.insert(
                "tolerated_number_of_failures".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tolerated_number_of_failures,
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
                        let field_value = match fields_map.get("custom_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expected_status_code_ranges: {
                        let field_value = match fields_map.get("expected_status_code_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'expected_status_code_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval_in_seconds: {
                        let field_value = match fields_map.get("interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tolerated_number_of_failures: {
                        let field_value = match fields_map.get("tolerated_number_of_failures") {
                            Some(value) => value,
                            None => bail!("Missing field 'tolerated_number_of_failures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
