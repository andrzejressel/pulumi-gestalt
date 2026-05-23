#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateContainerLivenessProbe {
    /// The number of consecutive failures required to consider this probe as failed. Possible values are between `1` and `10`. Defaults to `3`.
    #[builder(into)]
    pub r#failure_count_threshold: Option<i32>,
    /// A `header` block as detailed below.
    #[builder(into)]
    pub r#headers: Option<Vec<super::super::types::containerapp::JobTemplateContainerLivenessProbeHeader>>,
    /// The probe hostname. Defaults to the pod IP address. Setting a value for `Host` in `headers` can be used to override this for `HTTP` and `HTTPS` type probes.
    #[builder(into)]
    pub r#host: Option<String>,
    /// The time in seconds to wait after the container has started before the probe is started.
    #[builder(into)]
    pub r#initial_delay: Option<i32>,
    /// How often, in seconds, the probe should run. Possible values are in the range `1` - `240`. Defaults to `10`.
    #[builder(into)]
    pub r#interval_seconds: Option<i32>,
    /// The URI to use with the `host` for http type probes. Not valid for `TCP` type probes. Defaults to `/`.
    #[builder(into)]
    pub r#path: Option<String>,
    /// The port number on which to connect. Possible values are between `1` and `65535`.
    #[builder(into)]
    pub r#port: i32,
    /// The time in seconds after the container is sent the termination signal before the process if forcibly killed.
    #[builder(into)]
    pub r#termination_grace_period_seconds: Option<i32>,
    /// Time in seconds after which the probe times out. Possible values are in the range `1` - `240`. Defaults to `1`.
    #[builder(into)]
    pub r#timeout: Option<i32>,
    /// Type of probe. Possible values are `TCP`, `HTTP`, and `HTTPS`.
    #[builder(into)]
    pub r#transport: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateContainerLivenessProbe {
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
                    "failureCountThreshold",
                    &self.r#failure_count_threshold,
                ),
                to_pulumi_object_field(
                    "headers",
                    &self.r#headers,
                ),
                to_pulumi_object_field(
                    "host",
                    &self.r#host,
                ),
                to_pulumi_object_field(
                    "initialDelay",
                    &self.r#initial_delay,
                ),
                to_pulumi_object_field(
                    "intervalSeconds",
                    &self.r#interval_seconds,
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
                    "terminationGracePeriodSeconds",
                    &self.r#termination_grace_period_seconds,
                ),
                to_pulumi_object_field(
                    "timeout",
                    &self.r#timeout,
                ),
                to_pulumi_object_field(
                    "transport",
                    &self.r#transport,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateContainerLivenessProbe {
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
                    r#failure_count_threshold: {
                        let field_value = match fields_map.get("failureCountThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'failureCountThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("initialDelay") {
                            Some(value) => value,
                            None => bail!("Missing field 'initialDelay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval_seconds: {
                        let field_value = match fields_map.get("intervalSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'intervalSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#termination_grace_period_seconds: {
                        let field_value = match fields_map.get("terminationGracePeriodSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'terminationGracePeriodSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#transport: {
                        let field_value = match fields_map.get("transport") {
                            Some(value) => value,
                            None => bail!("Missing field 'transport' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
