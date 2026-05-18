#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackendServiceCircuitBreaker {
    /// The timeout for new network connections to hosts.
    #[builder(into)]
    #[serde(rename = "connectTimeouts")]
    pub r#connect_timeouts: Vec<super::super::types::compute::GetBackendServiceCircuitBreakerConnectTimeout>,
    /// The maximum number of connections to the backend cluster.
    /// Defaults to 1024.
    #[builder(into)]
    #[serde(rename = "maxConnections")]
    pub r#max_connections: i32,
    /// The maximum number of pending requests to the backend cluster.
    /// Defaults to 1024.
    #[builder(into)]
    #[serde(rename = "maxPendingRequests")]
    pub r#max_pending_requests: i32,
    /// The maximum number of parallel requests to the backend cluster.
    /// Defaults to 1024.
    #[builder(into)]
    #[serde(rename = "maxRequests")]
    pub r#max_requests: i32,
    /// Maximum requests for a single backend connection. This parameter
    /// is respected by both the HTTP/1.1 and HTTP/2 implementations. If
    /// not specified, there is no limit. Setting this parameter to 1
    /// will effectively disable keep alive.
    #[builder(into)]
    #[serde(rename = "maxRequestsPerConnection")]
    pub r#max_requests_per_connection: i32,
    /// The maximum number of parallel retries to the backend cluster.
    /// Defaults to 3.
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBackendServiceCircuitBreaker {
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
                    "connect_timeouts",
                    &self.r#connect_timeouts,
                ),
                to_pulumi_object_field(
                    "max_connections",
                    &self.r#max_connections,
                ),
                to_pulumi_object_field(
                    "max_pending_requests",
                    &self.r#max_pending_requests,
                ),
                to_pulumi_object_field(
                    "max_requests",
                    &self.r#max_requests,
                ),
                to_pulumi_object_field(
                    "max_requests_per_connection",
                    &self.r#max_requests_per_connection,
                ),
                to_pulumi_object_field(
                    "max_retries",
                    &self.r#max_retries,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBackendServiceCircuitBreaker {
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
                    r#connect_timeouts: {
                        let field_value = match fields_map.get("connect_timeouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'connect_timeouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_connections: {
                        let field_value = match fields_map.get("max_connections") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_connections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_pending_requests: {
                        let field_value = match fields_map.get("max_pending_requests") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_pending_requests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_requests: {
                        let field_value = match fields_map.get("max_requests") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_requests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_requests_per_connection: {
                        let field_value = match fields_map.get("max_requests_per_connection") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_requests_per_connection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_retries: {
                        let field_value = match fields_map.get("max_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
