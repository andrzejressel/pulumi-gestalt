#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleStatementXssMatchStatementFieldToMatch {
    /// Inspect all query arguments.
    #[builder(into)]
    pub r#all_query_arguments: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchAllQueryArguments>>,
    /// Inspect the request body, which immediately follows the request headers.
    #[builder(into)]
    pub r#body: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchBody>>,
    /// Inspect the cookies in the web request. See Cookies below for details.
    #[builder(into)]
    pub r#cookies: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchCookies>>,
    /// Inspect the request headers. See Header Order below for details.
    #[builder(into)]
    pub r#header_orders: Option<Vec<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchHeaderOrder>>,
    /// Inspect the request headers. See Headers below for details.
    #[builder(into)]
    pub r#headers: Option<Vec<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchHeader>>,
    #[builder(into)]
    pub r#ja_3_fingerprint: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchJa3Fingerprint>>,
    /// Inspect the request body as JSON. See JSON Body for details.
    #[builder(into)]
    pub r#json_body: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchJsonBody>>,
    /// Inspect the HTTP method. The method indicates the type of operation that the request is asking the origin to perform.
    #[builder(into)]
    pub r#method: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchMethod>>,
    /// Inspect the query string. This is the part of a URL that appears after a `?` character, if any.
    #[builder(into)]
    pub r#query_string: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchQueryString>>,
    /// Inspect a single header. See Single Header below for details.
    #[builder(into)]
    pub r#single_header: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchSingleHeader>>,
    /// Inspect a single query argument. See Single Query Argument below for details.
    #[builder(into)]
    pub r#single_query_argument: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchSingleQueryArgument>>,
    /// Inspect the request URI path. This is the part of a web request that identifies a resource, for example, `/images/daily-ad.jpg`.
    #[builder(into)]
    pub r#uri_path: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchUriPath>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleStatementXssMatchStatementFieldToMatch {
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
                    "allQueryArguments",
                    &self.r#all_query_arguments,
                ),
                to_pulumi_object_field(
                    "body",
                    &self.r#body,
                ),
                to_pulumi_object_field(
                    "cookies",
                    &self.r#cookies,
                ),
                to_pulumi_object_field(
                    "headerOrders",
                    &self.r#header_orders,
                ),
                to_pulumi_object_field(
                    "headers",
                    &self.r#headers,
                ),
                to_pulumi_object_field(
                    "ja3Fingerprint",
                    &self.r#ja_3_fingerprint,
                ),
                to_pulumi_object_field(
                    "jsonBody",
                    &self.r#json_body,
                ),
                to_pulumi_object_field(
                    "method",
                    &self.r#method,
                ),
                to_pulumi_object_field(
                    "queryString",
                    &self.r#query_string,
                ),
                to_pulumi_object_field(
                    "singleHeader",
                    &self.r#single_header,
                ),
                to_pulumi_object_field(
                    "singleQueryArgument",
                    &self.r#single_query_argument,
                ),
                to_pulumi_object_field(
                    "uriPath",
                    &self.r#uri_path,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleStatementXssMatchStatementFieldToMatch {
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
                    r#all_query_arguments: {
                        let field_value = match fields_map.get("allQueryArguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'allQueryArguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#body: {
                        let field_value = match fields_map.get("body") {
                            Some(value) => value,
                            None => bail!("Missing field 'body' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookies: {
                        let field_value = match fields_map.get("cookies") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_orders: {
                        let field_value = match fields_map.get("headerOrders") {
                            Some(value) => value,
                            None => bail!("Missing field 'headerOrders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ja_3_fingerprint: {
                        let field_value = match fields_map.get("ja3Fingerprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'ja3Fingerprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#json_body: {
                        let field_value = match fields_map.get("jsonBody") {
                            Some(value) => value,
                            None => bail!("Missing field 'jsonBody' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#method: {
                        let field_value = match fields_map.get("method") {
                            Some(value) => value,
                            None => bail!("Missing field 'method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string: {
                        let field_value = match fields_map.get("queryString") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryString' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#single_header: {
                        let field_value = match fields_map.get("singleHeader") {
                            Some(value) => value,
                            None => bail!("Missing field 'singleHeader' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#single_query_argument: {
                        let field_value = match fields_map.get("singleQueryArgument") {
                            Some(value) => value,
                            None => bail!("Missing field 'singleQueryArgument' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri_path: {
                        let field_value = match fields_map.get("uriPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'uriPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
