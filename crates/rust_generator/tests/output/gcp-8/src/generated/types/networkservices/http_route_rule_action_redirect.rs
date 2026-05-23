#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HttpRouteRuleActionRedirect {
    /// The host that will be used in the redirect response instead of the one that was supplied in the request.
    #[builder(into)]
    pub r#host_redirect: Option<String>,
    /// If set to true, the URL scheme in the redirected request is set to https.
    #[builder(into)]
    pub r#https_redirect: Option<bool>,
    /// The path that will be used in the redirect response instead of the one that was supplied in the request. pathRedirect can not be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect.
    #[builder(into)]
    pub r#path_redirect: Option<String>,
    /// The port that will be used in the redirected request instead of the one that was supplied in the request.
    #[builder(into)]
    pub r#port_redirect: Option<i32>,
    /// Indicates that during redirection, the matched prefix (or path) should be swapped with this value.
    #[builder(into)]
    pub r#prefix_rewrite: Option<String>,
    /// The HTTP Status code to use for the redirect.
    #[builder(into)]
    pub r#response_code: Option<String>,
    /// If set to true, any accompanying query portion of the original URL is removed prior to redirecting the request.
    #[builder(into)]
    pub r#strip_query: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HttpRouteRuleActionRedirect {
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
                    "hostRedirect",
                    &self.r#host_redirect,
                ),
                to_pulumi_object_field(
                    "httpsRedirect",
                    &self.r#https_redirect,
                ),
                to_pulumi_object_field(
                    "pathRedirect",
                    &self.r#path_redirect,
                ),
                to_pulumi_object_field(
                    "portRedirect",
                    &self.r#port_redirect,
                ),
                to_pulumi_object_field(
                    "prefixRewrite",
                    &self.r#prefix_rewrite,
                ),
                to_pulumi_object_field(
                    "responseCode",
                    &self.r#response_code,
                ),
                to_pulumi_object_field(
                    "stripQuery",
                    &self.r#strip_query,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HttpRouteRuleActionRedirect {
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
                        let field_value = match fields_map.get("hostRedirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostRedirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_redirect: {
                        let field_value = match fields_map.get("httpsRedirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpsRedirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_redirect: {
                        let field_value = match fields_map.get("pathRedirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'pathRedirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_redirect: {
                        let field_value = match fields_map.get("portRedirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'portRedirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_rewrite: {
                        let field_value = match fields_map.get("prefixRewrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefixRewrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_code: {
                        let field_value = match fields_map.get("responseCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'responseCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strip_query: {
                        let field_value = match fields_map.get("stripQuery") {
                            Some(value) => value,
                            None => bail!("Missing field 'stripQuery' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
