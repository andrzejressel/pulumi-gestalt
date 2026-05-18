#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerRuleCondition {
    /// Contains a single attribute `values`, which contains a set of host names.
    #[builder(into)]
    #[serde(rename = "hostHeader")]
    pub r#host_header: Option<Box<super::super::types::lb::GetListenerRuleConditionHostHeader>>,
    /// HTTP header and values to match.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "httpHeader")]
    pub r#http_header: Option<Box<super::super::types::lb::GetListenerRuleConditionHttpHeader>>,
    /// Contains a single attribute `values`, which contains a set of HTTP request methods.
    #[builder(into)]
    #[serde(rename = "httpRequestMethod")]
    pub r#http_request_method: Option<Box<super::super::types::lb::GetListenerRuleConditionHttpRequestMethod>>,
    /// Contains a single attribute `values`, which contains a set of path patterns to compare against the request URL.
    #[builder(into)]
    #[serde(rename = "pathPattern")]
    pub r#path_pattern: Option<Box<super::super::types::lb::GetListenerRuleConditionPathPattern>>,
    /// Query string parameters to match.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<Box<super::super::types::lb::GetListenerRuleConditionQueryString>>,
    /// Contains a single attribute `values`, which contains a set of source IPs in CIDR notation.
    #[builder(into)]
    #[serde(rename = "sourceIp")]
    pub r#source_ip: Option<Box<super::super::types::lb::GetListenerRuleConditionSourceIp>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetListenerRuleCondition {
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
                    "host_header",
                    &self.r#host_header,
                ),
                to_pulumi_object_field(
                    "http_header",
                    &self.r#http_header,
                ),
                to_pulumi_object_field(
                    "http_request_method",
                    &self.r#http_request_method,
                ),
                to_pulumi_object_field(
                    "path_pattern",
                    &self.r#path_pattern,
                ),
                to_pulumi_object_field(
                    "query_string",
                    &self.r#query_string,
                ),
                to_pulumi_object_field(
                    "source_ip",
                    &self.r#source_ip,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetListenerRuleCondition {
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
                    r#query_string: {
                        let field_value = match fields_map.get("query_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
