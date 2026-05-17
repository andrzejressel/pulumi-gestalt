#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateContainerLivenessProbe {
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[builder(into)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "grpc")]
    pub r#grpc: Option<Box<super::super::types::cloudrunv2::ServiceTemplateContainerLivenessProbeGrpc>>,
    /// HTTPGet specifies the http request to perform.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "httpGet")]
    pub r#http_get: Option<Box<super::super::types::cloudrunv2::ServiceTemplateContainerLivenessProbeHttpGet>>,
    /// Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[builder(into)]
    #[serde(rename = "initialDelaySeconds")]
    pub r#initial_delay_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds
    #[builder(into)]
    #[serde(rename = "periodSeconds")]
    pub r#period_seconds: Option<i32>,
    /// TCPSocketAction describes an action based on opening a socket
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tcpSocket")]
    pub r#tcp_socket: Option<Box<super::super::types::cloudrunv2::ServiceTemplateContainerLivenessProbeTcpSocket>>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTemplateContainerLivenessProbe {
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
                "failure_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failure_threshold,
                )
                .await,
            );
            map.insert(
                "grpc".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#grpc,
                )
                .await,
            );
            map.insert(
                "http_get".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_get,
                )
                .await,
            );
            map.insert(
                "initial_delay_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#initial_delay_seconds,
                )
                .await,
            );
            map.insert(
                "period_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#period_seconds,
                )
                .await,
            );
            map.insert(
                "tcp_socket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tcp_socket,
                )
                .await,
            );
            map.insert(
                "timeout_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_seconds,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTemplateContainerLivenessProbe {
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
