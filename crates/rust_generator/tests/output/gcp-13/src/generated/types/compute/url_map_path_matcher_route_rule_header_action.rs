#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapPathMatcherRouteRuleHeaderAction {
    /// Headers to add to a matching request prior to forwarding the request to the backendService.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestHeadersToAdds")]
    pub r#request_headers_to_adds: Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleHeaderActionRequestHeadersToAdd>>,
    /// A list of header names for headers that need to be removed from the request prior to
    /// forwarding the request to the backendService.
    #[builder(into)]
    #[serde(rename = "requestHeadersToRemoves")]
    pub r#request_headers_to_removes: Option<Vec<String>>,
    /// Headers to add the response prior to sending the response back to the client.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "responseHeadersToAdds")]
    pub r#response_headers_to_adds: Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleHeaderActionResponseHeadersToAdd>>,
    /// A list of header names for headers that need to be removed from the response prior to sending the
    /// response back to the client.
    #[builder(into)]
    #[serde(rename = "responseHeadersToRemoves")]
    pub r#response_headers_to_removes: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapPathMatcherRouteRuleHeaderAction {
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
                    "request_headers_to_adds",
                    &self.r#request_headers_to_adds,
                ),
                to_pulumi_object_field(
                    "request_headers_to_removes",
                    &self.r#request_headers_to_removes,
                ),
                to_pulumi_object_field(
                    "response_headers_to_adds",
                    &self.r#response_headers_to_adds,
                ),
                to_pulumi_object_field(
                    "response_headers_to_removes",
                    &self.r#response_headers_to_removes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapPathMatcherRouteRuleHeaderAction {
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
                    r#request_headers_to_adds: {
                        let field_value = match fields_map.get("request_headers_to_adds") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_headers_to_adds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_headers_to_removes: {
                        let field_value = match fields_map.get("request_headers_to_removes") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_headers_to_removes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_headers_to_adds: {
                        let field_value = match fields_map.get("response_headers_to_adds") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_headers_to_adds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_headers_to_removes: {
                        let field_value = match fields_map.get("response_headers_to_removes") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_headers_to_removes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
