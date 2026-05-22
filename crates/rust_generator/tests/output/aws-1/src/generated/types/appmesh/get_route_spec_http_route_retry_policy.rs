#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouteSpecHttpRouteRetryPolicy {
    #[builder(into)]
    #[serde(rename = "httpRetryEvents")]
    pub r#http_retry_events: Vec<String>,
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: i32,
    #[builder(into)]
    #[serde(rename = "perRetryTimeouts")]
    pub r#per_retry_timeouts: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteRetryPolicyPerRetryTimeout>,
    #[builder(into)]
    #[serde(rename = "tcpRetryEvents")]
    pub r#tcp_retry_events: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouteSpecHttpRouteRetryPolicy {
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
                    "httpRetryEvents",
                    &self.r#http_retry_events,
                ),
                to_pulumi_object_field(
                    "maxRetries",
                    &self.r#max_retries,
                ),
                to_pulumi_object_field(
                    "perRetryTimeouts",
                    &self.r#per_retry_timeouts,
                ),
                to_pulumi_object_field(
                    "tcpRetryEvents",
                    &self.r#tcp_retry_events,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouteSpecHttpRouteRetryPolicy {
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
                    r#http_retry_events: {
                        let field_value = match fields_map.get("httpRetryEvents") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpRetryEvents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_retries: {
                        let field_value = match fields_map.get("maxRetries") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxRetries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#per_retry_timeouts: {
                        let field_value = match fields_map.get("perRetryTimeouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'perRetryTimeouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_retry_events: {
                        let field_value = match fields_map.get("tcpRetryEvents") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcpRetryEvents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
