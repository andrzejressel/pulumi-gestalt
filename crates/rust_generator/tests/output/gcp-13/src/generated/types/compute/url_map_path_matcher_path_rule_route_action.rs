#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapPathMatcherPathRuleRouteAction {
    /// The specification for allowing client side cross-origin requests. Please see W3C
    /// Recommendation for Cross Origin Resource Sharing
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "corsPolicy")]
    pub r#cors_policy: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionCorsPolicy>>,
    /// The specification for fault injection introduced into traffic to test the
    /// resiliency of clients to backend service failure. As part of fault injection,
    /// when clients send requests to a backend service, delays can be introduced by
    /// Loadbalancer on a percentage of requests before sending those request to the
    /// backend service. Similarly requests from clients can be aborted by the
    /// Loadbalancer for a percentage of requests. timeout and retry_policy will be
    /// ignored by clients that are configured with a fault_injection_policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "faultInjectionPolicy")]
    pub r#fault_injection_policy: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionFaultInjectionPolicy>>,
    /// Specifies the policy on how requests intended for the route's backends are
    /// shadowed to a separate mirrored backend service. Loadbalancer does not wait for
    /// responses from the shadow service. Prior to sending traffic to the shadow
    /// service, the host / authority header is suffixed with -shadow.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestMirrorPolicy")]
    pub r#request_mirror_policy: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionRequestMirrorPolicy>>,
    /// Specifies the retry policy associated with this route.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionRetryPolicy>>,
    /// Specifies the timeout for the selected route. Timeout is computed from the time
    /// the request is has been fully processed (i.e. end-of-stream) up until the
    /// response has been completely processed. Timeout includes all retries. If not
    /// specified, the default value is 15 seconds.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionTimeout>>,
    /// The spec to modify the URL of the request, prior to forwarding the request to
    /// the matched service
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "urlRewrite")]
    pub r#url_rewrite: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionUrlRewrite>>,
    /// A list of weighted backend services to send traffic to when a route match
    /// occurs. The weights determine the fraction of traffic that flows to their
    /// corresponding backend service. If all traffic needs to go to a single backend
    /// service, there must be one  weightedBackendService with weight set to a non 0
    /// number. Once a backendService is identified and before forwarding the request to
    /// the backend service, advanced routing actions like Url rewrites and header
    /// transformations are applied depending on additional settings specified in this
    /// HttpRouteAction.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "weightedBackendServices")]
    pub r#weighted_backend_services: Option<Vec<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionWeightedBackendService>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapPathMatcherPathRuleRouteAction {
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
                "cors_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cors_policy,
                )
                .await,
            );
            map.insert(
                "fault_injection_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fault_injection_policy,
                )
                .await,
            );
            map.insert(
                "request_mirror_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_mirror_policy,
                )
                .await,
            );
            map.insert(
                "retry_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retry_policy,
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
                "url_rewrite".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url_rewrite,
                )
                .await,
            );
            map.insert(
                "weighted_backend_services".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weighted_backend_services,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapPathMatcherPathRuleRouteAction {
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
                    r#cors_policy: {
                        let field_value = match fields_map.get("cors_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'cors_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fault_injection_policy: {
                        let field_value = match fields_map.get("fault_injection_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'fault_injection_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_mirror_policy: {
                        let field_value = match fields_map.get("request_mirror_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_mirror_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_policy: {
                        let field_value = match fields_map.get("retry_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#url_rewrite: {
                        let field_value = match fields_map.get("url_rewrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_rewrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weighted_backend_services: {
                        let field_value = match fields_map.get("weighted_backend_services") {
                            Some(value) => value,
                            None => bail!("Missing field 'weighted_backend_services' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
