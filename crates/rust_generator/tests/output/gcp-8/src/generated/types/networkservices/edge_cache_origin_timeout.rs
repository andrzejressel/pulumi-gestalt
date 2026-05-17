#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheOriginTimeout {
    /// The maximum duration to wait for a single origin connection to be established, including DNS lookup, TLS handshake and TCP/QUIC connection establishment.
    /// Defaults to 5 seconds. The timeout must be a value between 1s and 15s.
    /// The connectTimeout capped by the deadline set by the request's maxAttemptsTimeout.  The last connection attempt may have a smaller connectTimeout in order to adhere to the overall maxAttemptsTimeout.
    #[builder(into)]
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Option<String>,
    /// The maximum time across all connection attempts to the origin, including failover origins, before returning an error to the client. A HTTP 504 will be returned if the timeout is reached before a response is returned.
    /// Defaults to 15 seconds. The timeout must be a value between 1s and 30s.
    /// If a failoverOrigin is specified, the maxAttemptsTimeout of the first configured origin sets the deadline for all connection attempts across all failoverOrigins.
    #[builder(into)]
    #[serde(rename = "maxAttemptsTimeout")]
    pub r#max_attempts_timeout: Option<String>,
    /// The maximum duration to wait between reads of a single HTTP connection/stream.
    /// Defaults to 15 seconds.  The timeout must be a value between 1s and 30s.
    /// The readTimeout is capped by the responseTimeout.  All reads of the HTTP connection/stream must be completed by the deadline set by the responseTimeout.
    /// If the response headers have already been written to the connection, the response will be truncated and logged.
    /// 
    /// <a name="nested_aws_v4_authentication"></a>The `aws_v4_authentication` block supports:
    #[builder(into)]
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Option<String>,
    /// The maximum duration to wait for the last byte of a response to arrive when reading from the HTTP connection/stream.
    /// Defaults to 30 seconds. The timeout must be a value between 1s and 120s.
    /// The responseTimeout starts after the connection has been established.
    /// This also applies to HTTP Chunked Transfer Encoding responses, and/or when an open-ended Range request is made to the origin. Origins that take longer to write additional bytes to the response than the configured responseTimeout will result in an error being returned to the client.
    /// If the response headers have already been written to the connection, the response will be truncated and logged.
    #[builder(into)]
    #[serde(rename = "responseTimeout")]
    pub r#response_timeout: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheOriginTimeout {
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
                    "connect_timeout",
                    &self.r#connect_timeout,
                ),
                to_pulumi_object_field(
                    "max_attempts_timeout",
                    &self.r#max_attempts_timeout,
                ),
                to_pulumi_object_field(
                    "read_timeout",
                    &self.r#read_timeout,
                ),
                to_pulumi_object_field(
                    "response_timeout",
                    &self.r#response_timeout,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheOriginTimeout {
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
                    r#connect_timeout: {
                        let field_value = match fields_map.get("connect_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'connect_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_attempts_timeout: {
                        let field_value = match fields_map.get("max_attempts_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_attempts_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_timeout: {
                        let field_value = match fields_map.get("read_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_timeout: {
                        let field_value = match fields_map.get("response_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
