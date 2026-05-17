#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionLivenessCheck {
    /// Interval between health checks.
    #[builder(into)]
    #[serde(rename = "checkInterval")]
    pub r#check_interval: Option<String>,
    /// Number of consecutive failed checks required before considering the VM unhealthy. Default: 4.
    #[builder(into)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Option<f64>,
    /// Host header to send when performing a HTTP Readiness check. Example: "myapp.appspot.com"
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// The initial delay before starting to execute the checks. Default: "300s"
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "initialDelay")]
    pub r#initial_delay: Option<String>,
    /// The request path.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Number of consecutive successful checks required before considering the VM healthy. Default: 2.
    #[builder(into)]
    #[serde(rename = "successThreshold")]
    pub r#success_threshold: Option<f64>,
    /// Time before the check is considered failed. Default: "4s"
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleAppVersionLivenessCheck {
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
                    "check_interval",
                    &self.r#check_interval,
                ),
                to_pulumi_object_field(
                    "failure_threshold",
                    &self.r#failure_threshold,
                ),
                to_pulumi_object_field(
                    "host",
                    &self.r#host,
                ),
                to_pulumi_object_field(
                    "initial_delay",
                    &self.r#initial_delay,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "success_threshold",
                    &self.r#success_threshold,
                ),
                to_pulumi_object_field(
                    "timeout",
                    &self.r#timeout,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleAppVersionLivenessCheck {
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
                    r#check_interval: {
                        let field_value = match fields_map.get("check_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'check_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failure_threshold: {
                        let field_value = match fields_map.get("failure_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host: {
                        let field_value = match fields_map.get("host") {
                            Some(value) => value,
                            None => bail!("Missing field 'host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_delay: {
                        let field_value = match fields_map.get("initial_delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#success_threshold: {
                        let field_value = match fields_map.get("success_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout: {
                        let field_value = match fields_map.get("timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
