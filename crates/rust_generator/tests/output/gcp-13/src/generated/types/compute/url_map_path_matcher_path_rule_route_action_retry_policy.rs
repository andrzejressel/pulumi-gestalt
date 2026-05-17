#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapPathMatcherPathRuleRouteActionRetryPolicy {
    /// Specifies the allowed number retries. This number must be > 0. If not specified, defaults to 1.
    #[builder(into)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Option<i32>,
    /// Specifies a non-zero timeout per retry attempt.
    /// If not specified, will use the timeout set in HttpRouteAction. If timeout in HttpRouteAction is not set,
    /// will use the largest timeout among all backend services associated with the route.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "perTryTimeout")]
    pub r#per_try_timeout: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionRetryPolicyPerTryTimeout>>,
    /// Specfies one or more conditions when this retry rule applies. Valid values are:
    /// * 5xx: Loadbalancer will attempt a retry if the backend service responds with any 5xx response code,
    /// or if the backend service does not respond at all, example: disconnects, reset, read timeout,
    /// * connection failure, and refused streams.
    /// * gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504.
    /// * connect-failure: Loadbalancer will retry on failures connecting to backend services,
    /// for example due to connection timeouts.
    /// * retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.
    /// Currently the only retriable error supported is 409.
    /// * refused-stream:Loadbalancer will retry if the backend service resets the stream with a REFUSED_STREAM error code.
    /// This reset type indicates that it is safe to retry.
    /// * cancelled: Loadbalancer will retry if the gRPC status code in the response header is set to cancelled
    /// * deadline-exceeded: Loadbalancer will retry if the gRPC status code in the response header is set to deadline-exceeded
    /// * resource-exhausted: Loadbalancer will retry if the gRPC status code in the response header is set to resource-exhausted
    /// * unavailable: Loadbalancer will retry if the gRPC status code in the response header is set to unavailable
    #[builder(into)]
    #[serde(rename = "retryConditions")]
    pub r#retry_conditions: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapPathMatcherPathRuleRouteActionRetryPolicy {
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
                "num_retries".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#num_retries,
                )
                .await,
            );
            map.insert(
                "per_try_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#per_try_timeout,
                )
                .await,
            );
            map.insert(
                "retry_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retry_conditions,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapPathMatcherPathRuleRouteActionRetryPolicy {
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
                    r#num_retries: {
                        let field_value = match fields_map.get("num_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#per_try_timeout: {
                        let field_value = match fields_map.get("per_try_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'per_try_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_conditions: {
                        let field_value = match fields_map.get("retry_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
