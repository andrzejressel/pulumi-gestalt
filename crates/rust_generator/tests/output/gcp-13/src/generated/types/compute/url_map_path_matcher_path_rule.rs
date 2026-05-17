#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapPathMatcherPathRule {
    /// customErrorResponsePolicy specifies how the Load Balancer returns error responses when BackendService or BackendBucket responds with an error.
    /// If a policy for an error code is not configured for the PathRule, a policy for the error code configured in pathMatcher.defaultCustomErrorResponsePolicy is applied. If one is not specified in pathMatcher.defaultCustomErrorResponsePolicy, the policy configured in UrlMap.defaultCustomErrorResponsePolicy takes effect.
    /// For example, consider a UrlMap with the following configuration:
    /// UrlMap.defaultCustomErrorResponsePolicy are configured with policies for 5xx and 4xx errors
    /// A PathRule for /coming_soon/ is configured for the error code 404.
    /// If the request is for www.myotherdomain.com and a 404 is encountered, the policy under UrlMap.defaultCustomErrorResponsePolicy takes effect. If a 404 response is encountered for the request www.example.com/current_events/, the pathMatcher's policy takes effect. If however, the request for www.example.com/coming_soon/ encounters a 404, the policy in PathRule.customErrorResponsePolicy takes effect. If any of the requests in this example encounter a 500 error code, the policy at UrlMap.defaultCustomErrorResponsePolicy takes effect.
    /// customErrorResponsePolicy is supported only for global external Application Load Balancers.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customErrorResponsePolicy")]
    pub r#custom_error_response_policy: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleCustomErrorResponsePolicy>>,
    /// The list of path patterns to match. Each must start with / and the only place a
    /// \* is allowed is at the end following a /. The string fed to the path matcher
    /// does not include any text after the first ? or #, and those chars are not
    /// allowed here.
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Vec<String>,
    /// In response to a matching path, the load balancer performs advanced routing
    /// actions like URL rewrites, header transformations, etc. prior to forwarding the
    /// request to the selected backend. If routeAction specifies any
    /// weightedBackendServices, service must not be set. Conversely if service is set,
    /// routeAction cannot contain any  weightedBackendServices. Only one of routeAction
    /// or urlRedirect must be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "routeAction")]
    pub r#route_action: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteAction>>,
    /// The backend service or backend bucket to use if any of the given paths match.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    /// When a path pattern is matched, the request is redirected to a URL specified
    /// by urlRedirect. If urlRedirect is specified, service or routeAction must not
    /// be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "urlRedirect")]
    pub r#url_redirect: Option<Box<super::super::types::compute::UrlMapPathMatcherPathRuleUrlRedirect>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapPathMatcherPathRule {
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
                "custom_error_response_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_error_response_policy,
                )
                .await,
            );
            map.insert(
                "paths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#paths,
                )
                .await,
            );
            map.insert(
                "route_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#route_action,
                )
                .await,
            );
            map.insert(
                "service".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service,
                )
                .await,
            );
            map.insert(
                "url_redirect".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url_redirect,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapPathMatcherPathRule {
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
                    r#custom_error_response_policy: {
                        let field_value = match fields_map.get("custom_error_response_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_error_response_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#paths: {
                        let field_value = match fields_map.get("paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_action: {
                        let field_value = match fields_map.get("route_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service: {
                        let field_value = match fields_map.get("service") {
                            Some(value) => value,
                            None => bail!("Missing field 'service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_redirect: {
                        let field_value = match fields_map.get("url_redirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_redirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
