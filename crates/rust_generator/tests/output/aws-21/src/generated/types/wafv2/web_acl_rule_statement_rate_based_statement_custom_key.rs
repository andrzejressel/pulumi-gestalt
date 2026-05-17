#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementRateBasedStatementCustomKey {
    /// Use the value of a cookie in the request as an aggregate key. See RateLimit `cookie` below for details.
    #[builder(into)]
    #[serde(rename = "cookie")]
    pub r#cookie: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKeyCookie>>,
    /// Use the first IP address in an HTTP header as an aggregate key. See `forwarded_ip` below for details.
    #[builder(into)]
    #[serde(rename = "forwardedIp")]
    pub r#forwarded_ip: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKeyForwardedIp>>,
    /// Use the value of a header in the request as an aggregate key. See RateLimit `header` below for details.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKeyHeader>>,
    /// Use the request's HTTP method as an aggregate key. See RateLimit `http_method` below for details.
    #[builder(into)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKeyHttpMethod>>,
    /// Use the request's originating IP address as an aggregate key. See `RateLimit ip` below for details.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKeyIp>>,
    /// Use the specified label namespace as an aggregate key. See RateLimit `label_namespace` below for details.
    #[builder(into)]
    #[serde(rename = "labelNamespace")]
    pub r#label_namespace: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKeyLabelNamespace>>,
    /// Use the specified query argument as an aggregate key. See RateLimit `query_argument` below for details.
    #[builder(into)]
    #[serde(rename = "queryArgument")]
    pub r#query_argument: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKeyQueryArgument>>,
    /// Use the request's query string as an aggregate key. See RateLimit `query_string` below for details.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKeyQueryString>>,
    /// Use the request's URI path as an aggregate key. See RateLimit `uri_path` below for details.
    #[builder(into)]
    #[serde(rename = "uriPath")]
    pub r#uri_path: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKeyUriPath>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementRateBasedStatementCustomKey {
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
                "cookie".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cookie,
                )
                .await,
            );
            map.insert(
                "forwarded_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forwarded_ip,
                )
                .await,
            );
            map.insert(
                "header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#header,
                )
                .await,
            );
            map.insert(
                "http_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_method,
                )
                .await,
            );
            map.insert(
                "ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip,
                )
                .await,
            );
            map.insert(
                "label_namespace".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#label_namespace,
                )
                .await,
            );
            map.insert(
                "query_argument".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_argument,
                )
                .await,
            );
            map.insert(
                "query_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_string,
                )
                .await,
            );
            map.insert(
                "uri_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#uri_path,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementRateBasedStatementCustomKey {
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
