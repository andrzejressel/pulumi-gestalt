#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateContainerStartupProbe {
    /// The number of consecutive failures required to consider this probe as failed. Possible values are between `1` and `10`. Defaults to `3`.
    #[builder(into)]
    #[serde(rename = "failureCountThreshold")]
    pub r#failure_count_threshold: Option<i32>,
    /// A `header` block as detailed below.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::containerapp::JobTemplateContainerStartupProbeHeader>>,
    /// The value for the host header which should be sent with this probe. If unspecified, the IP Address of the Pod is used as the host header. Setting a value for `Host` in `headers` can be used to override this for `HTTP` and `HTTPS` type probes.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// The number of seconds elapsed after the container has started before the probe is initiated. Possible values are between `0` and `60`. Defaults to `0` seconds.
    #[builder(into)]
    #[serde(rename = "initialDelay")]
    pub r#initial_delay: Option<i32>,
    /// How often, in seconds, the probe should run. Possible values are between `1` and `240`. Defaults to `10`
    #[builder(into)]
    #[serde(rename = "intervalSeconds")]
    pub r#interval_seconds: Option<i32>,
    /// The URI to use with the `host` for http type probes. Not valid for `TCP` type probes. Defaults to `/`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The port number on which to connect. Possible values are between `1` and `65535`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// The time in seconds after the container is sent the termination signal before the process if forcibly killed.
    #[builder(into)]
    #[serde(rename = "terminationGracePeriodSeconds")]
    pub r#termination_grace_period_seconds: Option<i32>,
    /// Time in seconds after which the probe times out. Possible values are in the range `1` - `240`. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<i32>,
    /// Type of probe. Possible values are `TCP`, `HTTP`, and `HTTPS`.
    #[builder(into)]
    #[serde(rename = "transport")]
    pub r#transport: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateContainerStartupProbe {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "failure_count_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failure_count_threshold,
                )
                .await,
            );
            map.insert(
                "headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#headers,
                )
                .await,
            );
            map.insert(
                "host".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host,
                )
                .await,
            );
            map.insert(
                "initial_delay".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#initial_delay,
                )
                .await,
            );
            map.insert(
                "interval_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#interval_seconds,
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
                "termination_grace_period_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#termination_grace_period_seconds,
                )
                .await,
            );
            map.insert(
                "timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout,
                )
                .await,
            );
            map.insert(
                "transport".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transport,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateContainerStartupProbe {
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
                        let field_value = match fields_map.get("failure_count_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_count_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("initial_delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval_seconds: {
                        let field_value = match fields_map.get("interval_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("termination_grace_period_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'termination_grace_period_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
