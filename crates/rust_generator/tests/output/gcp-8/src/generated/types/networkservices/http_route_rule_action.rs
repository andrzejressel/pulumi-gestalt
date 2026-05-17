#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HttpRouteRuleAction {
    /// The specification for allowing client side cross-origin requests.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "corsPolicy")]
    pub r#cors_policy: Option<Box<super::super::types::networkservices::HttpRouteRuleActionCorsPolicy>>,
    /// The destination to which traffic should be forwarded.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<super::super::types::networkservices::HttpRouteRuleActionDestination>>,
    /// The specification for fault injection introduced into traffic to test the resiliency of clients to backend service failure.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "faultInjectionPolicy")]
    pub r#fault_injection_policy: Option<Box<super::super::types::networkservices::HttpRouteRuleActionFaultInjectionPolicy>>,
    /// If set, the request is directed as configured by this field.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "redirect")]
    pub r#redirect: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRedirect>>,
    /// The specification for modifying the headers of a matching request prior to delivery of the request to the destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestHeaderModifier")]
    pub r#request_header_modifier: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRequestHeaderModifier>>,
    /// Specifies the policy on how requests intended for the routes destination are shadowed to a separate mirrored destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestMirrorPolicy")]
    pub r#request_mirror_policy: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRequestMirrorPolicy>>,
    /// The specification for modifying the headers of a response prior to sending the response back to the client.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "responseHeaderModifier")]
    pub r#response_header_modifier: Option<Box<super::super::types::networkservices::HttpRouteRuleActionResponseHeaderModifier>>,
    /// Specifies the retry policy associated with this route.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRetryPolicy>>,
    /// Specifies the timeout for selected route.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
    /// The specification for rewrite URL before forwarding requests to the destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "urlRewrite")]
    pub r#url_rewrite: Option<Box<super::super::types::networkservices::HttpRouteRuleActionUrlRewrite>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HttpRouteRuleAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

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
                "destinations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destinations,
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
                "redirect".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirect,
                )
                .await,
            );
            map.insert(
                "request_header_modifier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_header_modifier,
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
                "response_header_modifier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#response_header_modifier,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("cors_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'cors_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("fault_injection_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'fault_injection_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("request_header_modifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_header_modifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#response_header_modifier: {
                        let field_value = match fields_map.get("response_header_modifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_header_modifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
