#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerRuleCondition {
    /// Contains a single `values` item which is a list of host header patterns to match. The maximum size of each pattern is 128 characters. Comparison is case insensitive. Wildcard characters supported: * (matches 0 or more characters) and ? (matches exactly 1 character). Only one pattern needs to match for the condition to be satisfied.
    #[builder(into)]
    #[serde(rename = "hostHeader")]
    pub r#host_header: Option<Box<super::super::types::lb::ListenerRuleConditionHostHeader>>,
    /// HTTP headers to match. HTTP Header block fields documented below.
    #[builder(into)]
    #[serde(rename = "httpHeader")]
    pub r#http_header: Option<Box<super::super::types::lb::ListenerRuleConditionHttpHeader>>,
    /// Contains a single `values` item which is a list of HTTP request methods or verbs to match. Maximum size is 40 characters. Only allowed characters are A-Z, hyphen (-) and underscore (\_). Comparison is case sensitive. Wildcards are not supported. Only one needs to match for the condition to be satisfied. AWS recommends that GET and HEAD requests are routed in the same way because the response to a HEAD request may be cached.
    #[builder(into)]
    #[serde(rename = "httpRequestMethod")]
    pub r#http_request_method: Option<Box<super::super::types::lb::ListenerRuleConditionHttpRequestMethod>>,
    /// Contains a single `values` item which is a list of path patterns to match against the request URL. Maximum size of each pattern is 128 characters. Comparison is case sensitive. Wildcard characters supported: * (matches 0 or more characters) and ? (matches exactly 1 character). Only one pattern needs to match for the condition to be satisfied. Path pattern is compared only to the path of the URL, not to its query string. To compare against the query string, use a `query_string` condition.
    #[builder(into)]
    #[serde(rename = "pathPattern")]
    pub r#path_pattern: Option<Box<super::super::types::lb::ListenerRuleConditionPathPattern>>,
    /// Query strings to match. Query String block fields documented below.
    #[builder(into)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Option<Vec<super::super::types::lb::ListenerRuleConditionQueryString>>,
    /// Contains a single `values` item which is a list of source IP CIDR notations to match. You can use both IPv4 and IPv6 addresses. Wildcards are not supported. Condition is satisfied if the source IP address of the request matches one of the CIDR blocks. Condition is not satisfied by the addresses in the `X-Forwarded-For` header, use `http_header` condition instead.
    /// 
    /// > **NOTE::** Exactly one of `host_header`, `http_header`, `http_request_method`, `path_pattern`, `query_string` or `source_ip` must be set per condition.
    #[builder(into)]
    #[serde(rename = "sourceIp")]
    pub r#source_ip: Option<Box<super::super::types::lb::ListenerRuleConditionSourceIp>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ListenerRuleCondition {
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
                "host_header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_header,
                )
                .await,
            );
            map.insert(
                "http_header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_header,
                )
                .await,
            );
            map.insert(
                "http_request_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_request_method,
                )
                .await,
            );
            map.insert(
                "path_pattern".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path_pattern,
                )
                .await,
            );
            map.insert(
                "query_strings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_strings,
                )
                .await,
            );
            map.insert(
                "source_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_ip,
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
                        let field_value = match fields_map.get("host_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_header: {
                        let field_value = match fields_map.get("http_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_request_method: {
                        let field_value = match fields_map.get("http_request_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_request_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_pattern: {
                        let field_value = match fields_map.get("path_pattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_pattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_strings: {
                        let field_value = match fields_map.get("query_strings") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_strings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_ip: {
                        let field_value = match fields_map.get("source_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
