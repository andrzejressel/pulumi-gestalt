#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouteSpecHttp2RouteRetryPolicy {
    #[builder(into)]
    #[serde(rename = "httpRetryEvents")]
    pub r#http_retry_events: Vec<String>,
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: i32,
    #[builder(into)]
    #[serde(rename = "perRetryTimeouts")]
    pub r#per_retry_timeouts: Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteRetryPolicyPerRetryTimeout>,
    #[builder(into)]
    #[serde(rename = "tcpRetryEvents")]
    pub r#tcp_retry_events: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouteSpecHttp2RouteRetryPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("http_retry_events".to_string(), self.r#http_retry_events.to_pulumi_value().await);
            map.insert("max_retries".to_string(), self.r#max_retries.to_pulumi_value().await);
            map.insert("per_retry_timeouts".to_string(), self.r#per_retry_timeouts.to_pulumi_value().await);
            map.insert("tcp_retry_events".to_string(), self.r#tcp_retry_events.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouteSpecHttp2RouteRetryPolicy {
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
                    r#http_retry_events: {
                        let field_value = match fields_map.get("http_retry_events") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_retry_events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_retries: {
                        let field_value = match fields_map.get("max_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#per_retry_timeouts: {
                        let field_value = match fields_map.get("per_retry_timeouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'per_retry_timeouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteRetryPolicyPerRetryTimeout> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tcp_retry_events: {
                        let field_value = match fields_map.get("tcp_retry_events") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp_retry_events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
