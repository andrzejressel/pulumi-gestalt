#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HttpRouteRuleAction {
    /// The specification for allowing client side cross-origin requests.
    /// Structure is documented below.
    #[builder(into)]
    pub r#cors_policy: Option<Box<super::super::types::networkservices::HttpRouteRuleActionCorsPolicy>>,
    /// The destination to which traffic should be forwarded.
    /// Structure is documented below.
    #[builder(into)]
    pub r#destinations: Option<Vec<super::super::types::networkservices::HttpRouteRuleActionDestination>>,
    /// The specification for fault injection introduced into traffic to test the resiliency of clients to backend service failure.
    /// Structure is documented below.
    #[builder(into)]
    pub r#fault_injection_policy: Option<Box<super::super::types::networkservices::HttpRouteRuleActionFaultInjectionPolicy>>,
    /// If set, the request is directed as configured by this field.
    /// Structure is documented below.
    #[builder(into)]
    pub r#redirect: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRedirect>>,
    /// The specification for modifying the headers of a matching request prior to delivery of the request to the destination.
    /// Structure is documented below.
    #[builder(into)]
    pub r#request_header_modifier: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRequestHeaderModifier>>,
    /// Specifies the policy on how requests intended for the routes destination are shadowed to a separate mirrored destination.
    /// Structure is documented below.
    #[builder(into)]
    pub r#request_mirror_policy: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRequestMirrorPolicy>>,
    /// The specification for modifying the headers of a response prior to sending the response back to the client.
    /// Structure is documented below.
    #[builder(into)]
    pub r#response_header_modifier: Option<Box<super::super::types::networkservices::HttpRouteRuleActionResponseHeaderModifier>>,
    /// Specifies the retry policy associated with this route.
    /// Structure is documented below.
    #[builder(into)]
    pub r#retry_policy: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRetryPolicy>>,
    /// Specifies the timeout for selected route.
    #[builder(into)]
    pub r#timeout: Option<String>,
    /// The specification for rewrite URL before forwarding requests to the destination.
    /// Structure is documented below.
    #[builder(into)]
    pub r#url_rewrite: Option<Box<super::super::types::networkservices::HttpRouteRuleActionUrlRewrite>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HttpRouteRuleAction {
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
                    "corsPolicy",
                    &self.r#cors_policy,
                ),
                to_pulumi_object_field(
                    "destinations",
                    &self.r#destinations,
                ),
                to_pulumi_object_field(
                    "faultInjectionPolicy",
                    &self.r#fault_injection_policy,
                ),
                to_pulumi_object_field(
                    "redirect",
                    &self.r#redirect,
                ),
                to_pulumi_object_field(
                    "requestHeaderModifier",
                    &self.r#request_header_modifier,
                ),
                to_pulumi_object_field(
                    "requestMirrorPolicy",
                    &self.r#request_mirror_policy,
                ),
                to_pulumi_object_field(
                    "responseHeaderModifier",
                    &self.r#response_header_modifier,
                ),
                to_pulumi_object_field(
                    "retryPolicy",
                    &self.r#retry_policy,
                ),
                to_pulumi_object_field(
                    "timeout",
                    &self.r#timeout,
                ),
                to_pulumi_object_field(
                    "urlRewrite",
                    &self.r#url_rewrite,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HttpRouteRuleAction {
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
                        let field_value = match fields_map.get("corsPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'corsPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destinations: {
                        let field_value = match fields_map.get("destinations") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fault_injection_policy: {
                        let field_value = match fields_map.get("faultInjectionPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'faultInjectionPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect: {
                        let field_value = match fields_map.get("redirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_header_modifier: {
                        let field_value = match fields_map.get("requestHeaderModifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestHeaderModifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_mirror_policy: {
                        let field_value = match fields_map.get("requestMirrorPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestMirrorPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_header_modifier: {
                        let field_value = match fields_map.get("responseHeaderModifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'responseHeaderModifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_policy: {
                        let field_value = match fields_map.get("retryPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'retryPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("urlRewrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlRewrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
