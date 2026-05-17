#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionUrlMapPathMatcherPathRule {
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
    pub r#route_action: Option<Box<super::super::types::compute::RegionUrlMapPathMatcherPathRuleRouteAction>>,
    /// The region backend service resource to which traffic is
    /// directed if this rule is matched. If routeAction is additionally specified,
    /// advanced routing actions like URL Rewrites, etc. take effect prior to sending
    /// the request to the backend. However, if service is specified, routeAction cannot
    /// contain any weightedBackendService s. Conversely, if routeAction specifies any
    /// weightedBackendServices, service must not be specified. Only one of urlRedirect,
    /// service or routeAction.weightedBackendService must be set.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    /// When a path pattern is matched, the request is redirected to a URL specified
    /// by urlRedirect. If urlRedirect is specified, service or routeAction must not
    /// be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "urlRedirect")]
    pub r#url_redirect: Option<Box<super::super::types::compute::RegionUrlMapPathMatcherPathRuleUrlRedirect>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionUrlMapPathMatcherPathRule {
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
                    "paths",
                    &self.r#paths,
                ),
                to_pulumi_object_field(
                    "route_action",
                    &self.r#route_action,
                ),
                to_pulumi_object_field(
                    "service",
                    &self.r#service,
                ),
                to_pulumi_object_field(
                    "url_redirect",
                    &self.r#url_redirect,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionUrlMapPathMatcherPathRule {
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
