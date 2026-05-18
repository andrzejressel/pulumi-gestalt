#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapPathMatcherPathRuleUrlRedirect {
    /// The host that will be used in the redirect response instead of the one that was
    /// supplied in the request. The value must be between 1 and 255 characters.
    #[builder(into)]
    #[serde(rename = "hostRedirect")]
    pub r#host_redirect: Option<String>,
    /// If set to true, the URL scheme in the redirected request is set to https. If set
    /// to false, the URL scheme of the redirected request will remain the same as that
    /// of the request. This must only be set for UrlMaps used in TargetHttpProxys.
    /// Setting this true for TargetHttpsProxy is not permitted. Defaults to false.
    #[builder(into)]
    #[serde(rename = "httpsRedirect")]
    pub r#https_redirect: Option<bool>,
    /// The path that will be used in the redirect response instead of the one that was
    /// supplied in the request. Only one of pathRedirect or prefixRedirect must be
    /// specified. The value must be between 1 and 1024 characters.
    #[builder(into)]
    #[serde(rename = "pathRedirect")]
    pub r#path_redirect: Option<String>,
    /// The prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,
    /// retaining the remaining portion of the URL before redirecting the request.
    #[builder(into)]
    #[serde(rename = "prefixRedirect")]
    pub r#prefix_redirect: Option<String>,
    /// The HTTP Status code to use for this RedirectAction. Supported values are:
    /// * MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.
    /// * FOUND, which corresponds to 302.
    /// * SEE_OTHER which corresponds to 303.
    /// * TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method will be retained.
    /// * PERMANENT_REDIRECT, which corresponds to 308. In this case, the request method will be retained.
    #[builder(into)]
    #[serde(rename = "redirectResponseCode")]
    pub r#redirect_response_code: Option<String>,
    /// If set to true, any accompanying query portion of the original URL is removed
    /// prior to redirecting the request. If set to false, the query portion of the
    /// original URL is retained. Defaults to false.
    #[builder(into)]
    #[serde(rename = "stripQuery")]
    pub r#strip_query: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapPathMatcherPathRuleUrlRedirect {
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
                    "host_redirect",
                    &self.r#host_redirect,
                ),
                to_pulumi_object_field(
                    "https_redirect",
                    &self.r#https_redirect,
                ),
                to_pulumi_object_field(
                    "path_redirect",
                    &self.r#path_redirect,
                ),
                to_pulumi_object_field(
                    "prefix_redirect",
                    &self.r#prefix_redirect,
                ),
                to_pulumi_object_field(
                    "redirect_response_code",
                    &self.r#redirect_response_code,
                ),
                to_pulumi_object_field(
                    "strip_query",
                    &self.r#strip_query,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapPathMatcherPathRuleUrlRedirect {
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
                    r#host_redirect: {
                        let field_value = match fields_map.get("host_redirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_redirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_redirect: {
                        let field_value = match fields_map.get("https_redirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_redirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_redirect: {
                        let field_value = match fields_map.get("path_redirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_redirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_redirect: {
                        let field_value = match fields_map.get("prefix_redirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_redirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_response_code: {
                        let field_value = match fields_map.get("redirect_response_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_response_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strip_query: {
                        let field_value = match fields_map.get("strip_query") {
                            Some(value) => value,
                            None => bail!("Missing field 'strip_query' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
