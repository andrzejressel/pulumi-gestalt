#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderAction {
    /// Describes a header to add.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestHeaderToAdds")]
    pub r#request_header_to_adds: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionRequestHeaderToAdd>>,
    /// A list of header names for headers that need to be removed from the request prior to forwarding the request to the origin.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestHeaderToRemoves")]
    pub r#request_header_to_removes: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionRequestHeaderToRemove>>,
    /// Headers to add to the response prior to sending it back to the client.
    /// Response headers are only sent to the client, and do not have an effect on the cache serving the response.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "responseHeaderToAdds")]
    pub r#response_header_to_adds: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionResponseHeaderToAdd>>,
    /// A list of header names for headers that need to be removed from the request prior to forwarding the request to the origin.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "responseHeaderToRemoves")]
    pub r#response_header_to_removes: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionResponseHeaderToRemove>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderAction {
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
                    "request_header_to_adds",
                    &self.r#request_header_to_adds,
                ),
                to_pulumi_object_field(
                    "request_header_to_removes",
                    &self.r#request_header_to_removes,
                ),
                to_pulumi_object_field(
                    "response_header_to_adds",
                    &self.r#response_header_to_adds,
                ),
                to_pulumi_object_field(
                    "response_header_to_removes",
                    &self.r#response_header_to_removes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderAction {
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
                    r#request_header_to_adds: {
                        let field_value = match fields_map.get("request_header_to_adds") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_header_to_adds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_header_to_removes: {
                        let field_value = match fields_map.get("request_header_to_removes") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_header_to_removes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_header_to_adds: {
                        let field_value = match fields_map.get("response_header_to_adds") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_header_to_adds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_header_to_removes: {
                        let field_value = match fields_map.get("response_header_to_removes") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_header_to_removes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
