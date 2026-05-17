#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionUrlMapPathMatcherRouteRuleRouteActionRetryPolicy {
    /// Specifies the allowed number retries. This number must be > 0. If not specified, defaults to 1.
    #[builder(into)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: i32,
    /// Specifies a non-zero timeout per retry attempt.
    /// If not specified, will use the timeout set in HttpRouteAction. If timeout in HttpRouteAction is not set,
    /// will use the largest timeout among all backend services associated with the route.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "perTryTimeout")]
    pub r#per_try_timeout: Option<Box<super::super::types::compute::RegionUrlMapPathMatcherRouteRuleRouteActionRetryPolicyPerTryTimeout>>,
    /// Specifies one or more conditions when this retry policy applies.
    /// Valid values are listed below. Only the following codes are supported when the URL map is bound to target gRPC proxy that has validateForProxyless field set to true: cancelled, deadline-exceeded, internal, resource-exhausted, unavailable.
    /// - 5xx : retry is attempted if the instance or endpoint responds with any 5xx response code, or if the instance or endpoint does not respond at all. For example, disconnects, reset, read timeout, connection failure, and refused streams.
    /// - gateway-error : Similar to 5xx, but only applies to response codes 502, 503 or 504.
    /// - connect-failure : a retry is attempted on failures connecting to the instance or endpoint. For example, connection timeouts.
    /// - retriable-4xx : a retry is attempted if the instance or endpoint responds with a 4xx response code. The only error that you can retry is error code 409.
    /// - refused-stream : a retry is attempted if the instance or endpoint resets the stream with a REFUSED_STREAM error code. This reset type indicates that it is safe to retry.
    /// - cancelled : a retry is attempted if the gRPC status code in the response header is set to cancelled.
    /// - deadline-exceeded : a retry is attempted if the gRPC status code in the response header is set to deadline-exceeded.
    /// - internal :  a retry is attempted if the gRPC status code in the response header is set to internal.
    /// - resource-exhausted : a retry is attempted if the gRPC status code in the response header is set to resource-exhausted.
    /// - unavailable : a retry is attempted if the gRPC status code in the response header is set to unavailable.
    #[builder(into)]
    #[serde(rename = "retryConditions")]
    pub r#retry_conditions: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionUrlMapPathMatcherRouteRuleRouteActionRetryPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "num_retries",
                    &self.r#num_retries,
                ),
                to_pulumi_object_field(
                    "per_try_timeout",
                    &self.r#per_try_timeout,
                ),
                to_pulumi_object_field(
                    "retry_conditions",
                    &self.r#retry_conditions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionUrlMapPathMatcherRouteRuleRouteActionRetryPolicy {
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
