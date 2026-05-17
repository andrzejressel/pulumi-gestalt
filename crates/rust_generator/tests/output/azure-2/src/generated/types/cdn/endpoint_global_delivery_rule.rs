#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointGlobalDeliveryRule {
    /// A `cache_expiration_action` block as defined above.
    #[builder(into)]
    #[serde(rename = "cacheExpirationAction")]
    pub r#cache_expiration_action: Option<Box<super::super::types::cdn::EndpointGlobalDeliveryRuleCacheExpirationAction>>,
    /// A `cache_key_query_string_action` block as defined above.
    #[builder(into)]
    #[serde(rename = "cacheKeyQueryStringAction")]
    pub r#cache_key_query_string_action: Option<Box<super::super::types::cdn::EndpointGlobalDeliveryRuleCacheKeyQueryStringAction>>,
    /// A `modify_request_header_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "modifyRequestHeaderActions")]
    pub r#modify_request_header_actions: Option<Vec<super::super::types::cdn::EndpointGlobalDeliveryRuleModifyRequestHeaderAction>>,
    /// A `modify_response_header_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "modifyResponseHeaderActions")]
    pub r#modify_response_header_actions: Option<Vec<super::super::types::cdn::EndpointGlobalDeliveryRuleModifyResponseHeaderAction>>,
    /// A `url_redirect_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlRedirectAction")]
    pub r#url_redirect_action: Option<Box<super::super::types::cdn::EndpointGlobalDeliveryRuleUrlRedirectAction>>,
    /// A `url_rewrite_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlRewriteAction")]
    pub r#url_rewrite_action: Option<Box<super::super::types::cdn::EndpointGlobalDeliveryRuleUrlRewriteAction>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointGlobalDeliveryRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "cache_expiration_action",
                    &self.r#cache_expiration_action,
                ),
                to_pulumi_object_field(
                    "cache_key_query_string_action",
                    &self.r#cache_key_query_string_action,
                ),
                to_pulumi_object_field(
                    "modify_request_header_actions",
                    &self.r#modify_request_header_actions,
                ),
                to_pulumi_object_field(
                    "modify_response_header_actions",
                    &self.r#modify_response_header_actions,
                ),
                to_pulumi_object_field(
                    "url_redirect_action",
                    &self.r#url_redirect_action,
                ),
                to_pulumi_object_field(
                    "url_rewrite_action",
                    &self.r#url_rewrite_action,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointGlobalDeliveryRule {
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
                    r#cache_expiration_action: {
                        let field_value = match fields_map.get("cache_expiration_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_expiration_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_key_query_string_action: {
                        let field_value = match fields_map.get("cache_key_query_string_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_key_query_string_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#modify_request_header_actions: {
                        let field_value = match fields_map.get("modify_request_header_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'modify_request_header_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#modify_response_header_actions: {
                        let field_value = match fields_map.get("modify_response_header_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'modify_response_header_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_redirect_action: {
                        let field_value = match fields_map.get("url_redirect_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_redirect_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_rewrite_action: {
                        let field_value = match fields_map.get("url_rewrite_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_rewrite_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
