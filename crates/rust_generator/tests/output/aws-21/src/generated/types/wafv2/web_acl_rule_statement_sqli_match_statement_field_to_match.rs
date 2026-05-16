#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementSqliMatchStatementFieldToMatch {
    /// Inspect all query arguments.
    #[builder(into)]
    #[serde(rename = "allQueryArguments")]
    pub r#all_query_arguments: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchAllQueryArguments>>,
    /// Inspect the request body, which immediately follows the request headers. See `body` below for details.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchBody>>,
    /// Inspect the cookies in the web request. See `cookies` below for details.
    #[builder(into)]
    #[serde(rename = "cookies")]
    pub r#cookies: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchCookies>>,
    /// Inspect a string containing the list of the request's header names, ordered as they appear in the web request that AWS WAF receives for inspection. See `header_order` below for details.
    #[builder(into)]
    #[serde(rename = "headerOrders")]
    pub r#header_orders: Option<Vec<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchHeaderOrder>>,
    /// Inspect the request headers. See `headers` below for details.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchHeader>>,
    /// Inspect the JA3 fingerprint. See `ja3_fingerprint` below for details.
    #[builder(into)]
    #[serde(rename = "ja3Fingerprint")]
    pub r#ja_3_fingerprint: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchJa3Fingerprint>>,
    /// Inspect the request body as JSON. See `json_body` for details.
    #[builder(into)]
    #[serde(rename = "jsonBody")]
    pub r#json_body: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchJsonBody>>,
    /// Inspect the HTTP method. The method indicates the type of operation that the request is asking the origin to perform.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchMethod>>,
    /// Inspect the query string. This is the part of a URL that appears after a `?` character, if any.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchQueryString>>,
    /// Inspect a single header. See `single_header` below for details.
    #[builder(into)]
    #[serde(rename = "singleHeader")]
    pub r#single_header: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchSingleHeader>>,
    /// Inspect a single query argument. See `single_query_argument` below for details.
    #[builder(into)]
    #[serde(rename = "singleQueryArgument")]
    pub r#single_query_argument: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchSingleQueryArgument>>,
    /// Inspect the request URI path. This is the part of a web request that identifies a resource, for example, `/images/daily-ad.jpg`.
    #[builder(into)]
    #[serde(rename = "uriPath")]
    pub r#uri_path: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchUriPath>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementSqliMatchStatementFieldToMatch {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("all_query_arguments".to_string(), self.r#all_query_arguments.to_pulumi_value().await);
            map.insert("body".to_string(), self.r#body.to_pulumi_value().await);
            map.insert("cookies".to_string(), self.r#cookies.to_pulumi_value().await);
            map.insert("header_orders".to_string(), self.r#header_orders.to_pulumi_value().await);
            map.insert("headers".to_string(), self.r#headers.to_pulumi_value().await);
            map.insert("ja_3_fingerprint".to_string(), self.r#ja_3_fingerprint.to_pulumi_value().await);
            map.insert("json_body".to_string(), self.r#json_body.to_pulumi_value().await);
            map.insert("method".to_string(), self.r#method.to_pulumi_value().await);
            map.insert("query_string".to_string(), self.r#query_string.to_pulumi_value().await);
            map.insert("single_header".to_string(), self.r#single_header.to_pulumi_value().await);
            map.insert("single_query_argument".to_string(), self.r#single_query_argument.to_pulumi_value().await);
            map.insert("uri_path".to_string(), self.r#uri_path.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementSqliMatchStatementFieldToMatch {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#all_query_arguments: {
                        let field_value = match fields_map.get("all_query_arguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_query_arguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchAllQueryArguments>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#body: {
                        let field_value = match fields_map.get("body") {
                            Some(value) => value,
                            None => bail!("Missing field 'body' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchBody>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cookies: {
                        let field_value = match fields_map.get("cookies") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchCookies>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#header_orders: {
                        let field_value = match fields_map.get("header_orders") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_orders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchHeaderOrder>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchHeader>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ja_3_fingerprint: {
                        let field_value = match fields_map.get("ja_3_fingerprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'ja_3_fingerprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchJa3Fingerprint>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#json_body: {
                        let field_value = match fields_map.get("json_body") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_body' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchJsonBody>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#method: {
                        let field_value = match fields_map.get("method") {
                            Some(value) => value,
                            None => bail!("Missing field 'method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchMethod>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#query_string: {
                        let field_value = match fields_map.get("query_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchQueryString>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#single_header: {
                        let field_value = match fields_map.get("single_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'single_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchSingleHeader>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#single_query_argument: {
                        let field_value = match fields_map.get("single_query_argument") {
                            Some(value) => value,
                            None => bail!("Missing field 'single_query_argument' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchSingleQueryArgument>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#uri_path: {
                        let field_value = match fields_map.get("uri_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatementFieldToMatchUriPath>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
