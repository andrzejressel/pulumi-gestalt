#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupContainerReadinessProbe {
    /// Commands to be run to validate container readiness. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "execs")]
    pub r#execs: Option<Vec<String>>,
    /// How many times to try the probe before restarting the container (liveness probe) or marking the container as unhealthy (readiness probe). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Option<i32>,
    /// The definition of the http_get for this container as documented in the `http_get` block below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "httpGets")]
    pub r#http_gets: Option<Vec<super::super::types::containerservice::GroupContainerReadinessProbeHttpGet>>,
    /// Number of seconds after the container has started before liveness or readiness probes are initiated. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "initialDelaySeconds")]
    pub r#initial_delay_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "periodSeconds")]
    pub r#period_seconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "successThreshold")]
    pub r#success_threshold: Option<i32>,
    /// Number of seconds after which the probe times out. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupContainerReadinessProbe {
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
                    "execs",
                    &self.r#execs,
                ),
                to_pulumi_object_field(
                    "failure_threshold",
                    &self.r#failure_threshold,
                ),
                to_pulumi_object_field(
                    "http_gets",
                    &self.r#http_gets,
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
                    "success_threshold",
                    &self.r#success_threshold,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupContainerReadinessProbe {
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
                    r#execs: {
                        let field_value = match fields_map.get("execs") {
                            Some(value) => value,
                            None => bail!("Missing field 'execs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#http_gets: {
                        let field_value = match fields_map.get("http_gets") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_gets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#success_threshold: {
                        let field_value = match fields_map.get("success_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
