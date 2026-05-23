#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerRuleCondition {
    /// Contains a single `values` item which is a list of host header patterns to match. The maximum size of each pattern is 128 characters. Comparison is case insensitive. Wildcard characters supported: * (matches 0 or more characters) and ? (matches exactly 1 character). Only one pattern needs to match for the condition to be satisfied.
    #[builder(into)]
    pub r#host_header: Option<Box<super::super::types::alb::ListenerRuleConditionHostHeader>>,
    /// HTTP headers to match. HTTP Header block fields documented below.
    #[builder(into)]
    pub r#http_header: Option<Box<super::super::types::alb::ListenerRuleConditionHttpHeader>>,
    /// Contains a single `values` item which is a list of HTTP request methods or verbs to match. Maximum size is 40 characters. Only allowed characters are A-Z, hyphen (-) and underscore (\_). Comparison is case sensitive. Wildcards are not supported. Only one needs to match for the condition to be satisfied. AWS recommends that GET and HEAD requests are routed in the same way because the response to a HEAD request may be cached.
    #[builder(into)]
    pub r#http_request_method: Option<Box<super::super::types::alb::ListenerRuleConditionHttpRequestMethod>>,
    /// Contains a single `values` item which is a list of path patterns to match against the request URL. Maximum size of each pattern is 128 characters. Comparison is case sensitive. Wildcard characters supported: * (matches 0 or more characters) and ? (matches exactly 1 character). Only one pattern needs to match for the condition to be satisfied. Path pattern is compared only to the path of the URL, not to its query string. To compare against the query string, use a `query_string` condition.
    #[builder(into)]
    pub r#path_pattern: Option<Box<super::super::types::alb::ListenerRuleConditionPathPattern>>,
    /// Query strings to match. Query String block fields documented below.
    #[builder(into)]
    pub r#query_strings: Option<Vec<super::super::types::alb::ListenerRuleConditionQueryString>>,
    /// Contains a single `values` item which is a list of source IP CIDR notations to match. You can use both IPv4 and IPv6 addresses. Wildcards are not supported. Condition is satisfied if the source IP address of the request matches one of the CIDR blocks. Condition is not satisfied by the addresses in the `X-Forwarded-For` header, use `http_header` condition instead.
    /// 
    /// > **NOTE::** Exactly one of `host_header`, `http_header`, `http_request_method`, `path_pattern`, `query_string` or `source_ip` must be set per condition.
    #[builder(into)]
    pub r#source_ip: Option<Box<super::super::types::alb::ListenerRuleConditionSourceIp>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ListenerRuleCondition {
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
                    "hostHeader",
                    &self.r#host_header,
                ),
                to_pulumi_object_field(
                    "httpHeader",
                    &self.r#http_header,
                ),
                to_pulumi_object_field(
                    "httpRequestMethod",
                    &self.r#http_request_method,
                ),
                to_pulumi_object_field(
                    "pathPattern",
                    &self.r#path_pattern,
                ),
                to_pulumi_object_field(
                    "queryStrings",
                    &self.r#query_strings,
                ),
                to_pulumi_object_field(
                    "sourceIp",
                    &self.r#source_ip,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ListenerRuleCondition {
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
                    r#host_header: {
                        let field_value = match fields_map.get("hostHeader") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostHeader' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_header: {
                        let field_value = match fields_map.get("httpHeader") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpHeader' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_request_method: {
                        let field_value = match fields_map.get("httpRequestMethod") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpRequestMethod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_pattern: {
                        let field_value = match fields_map.get("pathPattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'pathPattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_strings: {
                        let field_value = match fields_map.get("queryStrings") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryStrings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_ip: {
                        let field_value = match fields_map.get("sourceIp") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceIp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
