#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleStatementRateBasedStatementCustomKey {
    /// (Optional) Use the value of a cookie in the request as an aggregate key. See RateLimit `cookie` below for details.
    #[builder(into)]
    #[serde(rename = "cookie")]
    pub r#cookie: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyCookie>>,
    /// (Optional) Use the first IP address in an HTTP header as an aggregate key. See `forwarded_ip` below for details.
    #[builder(into)]
    #[serde(rename = "forwardedIp")]
    pub r#forwarded_ip: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyForwardedIp>>,
    /// (Optional) Use the value of a header in the request as an aggregate key. See RateLimit `header` below for details.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyHeader>>,
    /// (Optional) Use the request's HTTP method as an aggregate key. See RateLimit `http_method` below for details.
    #[builder(into)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyHttpMethod>>,
    /// (Optional) Use the request's originating IP address as an aggregate key. See `RateLimit ip` below for details.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyIp>>,
    /// (Optional) Use the specified label namespace as an aggregate key. See RateLimit `label_namespace` below for details.
    #[builder(into)]
    #[serde(rename = "labelNamespace")]
    pub r#label_namespace: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyLabelNamespace>>,
    /// (Optional) Use the specified query argument as an aggregate key. See RateLimit `query_argument` below for details.
    #[builder(into)]
    #[serde(rename = "queryArgument")]
    pub r#query_argument: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyQueryArgument>>,
    /// (Optional) Use the request's query string as an aggregate key. See RateLimit `query_string` below for details.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyQueryString>>,
    /// (Optional) Use the request's URI path as an aggregate key. See RateLimit `uri_path` below for details.
    #[builder(into)]
    #[serde(rename = "uriPath")]
    pub r#uri_path: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyUriPath>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleStatementRateBasedStatementCustomKey {
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
                    "cookie",
                    &self.r#cookie,
                ),
                to_pulumi_object_field(
                    "forwarded_ip",
                    &self.r#forwarded_ip,
                ),
                to_pulumi_object_field(
                    "header",
                    &self.r#header,
                ),
                to_pulumi_object_field(
                    "http_method",
                    &self.r#http_method,
                ),
                to_pulumi_object_field(
                    "ip",
                    &self.r#ip,
                ),
                to_pulumi_object_field(
                    "label_namespace",
                    &self.r#label_namespace,
                ),
                to_pulumi_object_field(
                    "query_argument",
                    &self.r#query_argument,
                ),
                to_pulumi_object_field(
                    "query_string",
                    &self.r#query_string,
                ),
                to_pulumi_object_field(
                    "uri_path",
                    &self.r#uri_path,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleStatementRateBasedStatementCustomKey {
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
                    r#cookie: {
                        let field_value = match fields_map.get("cookie") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookie' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forwarded_ip: {
                        let field_value = match fields_map.get("forwarded_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarded_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header: {
                        let field_value = match fields_map.get("header") {
                            Some(value) => value,
                            None => bail!("Missing field 'header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_method: {
                        let field_value = match fields_map.get("http_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip: {
                        let field_value = match fields_map.get("ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label_namespace: {
                        let field_value = match fields_map.get("label_namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'label_namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_argument: {
                        let field_value = match fields_map.get("query_argument") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_argument' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string: {
                        let field_value = match fields_map.get("query_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri_path: {
                        let field_value = match fields_map.get("uri_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
