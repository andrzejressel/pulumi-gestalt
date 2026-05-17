#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateContainerStartupProbe {
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[builder(into)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "grpc")]
    pub r#grpc: Option<Box<super::super::types::cloudrunv2::ServiceTemplateContainerStartupProbeGrpc>>,
    /// HTTPGet specifies the http request to perform. Exactly one of HTTPGet or TCPSocket must be specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "httpGet")]
    pub r#http_get: Option<Box<super::super::types::cloudrunv2::ServiceTemplateContainerStartupProbeHttpGet>>,
    /// Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[builder(into)]
    #[serde(rename = "initialDelaySeconds")]
    pub r#initial_delay_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds
    #[builder(into)]
    #[serde(rename = "periodSeconds")]
    pub r#period_seconds: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port. Exactly one of HTTPGet or TCPSocket must be specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tcpSocket")]
    pub r#tcp_socket: Option<Box<super::super::types::cloudrunv2::ServiceTemplateContainerStartupProbeTcpSocket>>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTemplateContainerStartupProbe {
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
                    "failure_threshold",
                    &self.r#failure_threshold,
                ),
                to_pulumi_object_field(
                    "grpc",
                    &self.r#grpc,
                ),
                to_pulumi_object_field(
                    "http_get",
                    &self.r#http_get,
                ),
                to_pulumi_object_field(
                    "initial_delay_seconds",
                    &self.r#initial_delay_seconds,
                ),
                to_pulumi_object_field(
                    "period_seconds",
                    &self.r#period_seconds,
                ),
                to_pulumi_object_field(
                    "tcp_socket",
                    &self.r#tcp_socket,
                ),
                to_pulumi_object_field(
                    "timeout_seconds",
                    &self.r#timeout_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTemplateContainerStartupProbe {
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
                    r#failure_threshold: {
                        let field_value = match fields_map.get("failure_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#grpc: {
                        let field_value = match fields_map.get("grpc") {
                            Some(value) => value,
                            None => bail!("Missing field 'grpc' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_get: {
                        let field_value = match fields_map.get("http_get") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_get' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_delay_seconds: {
                        let field_value = match fields_map.get("initial_delay_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_delay_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#period_seconds: {
                        let field_value = match fields_map.get("period_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'period_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_socket: {
                        let field_value = match fields_map.get("tcp_socket") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp_socket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_seconds: {
                        let field_value = match fields_map.get("timeout_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
