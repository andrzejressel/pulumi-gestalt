#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BlockchainNodesEthereumDetailsAdditionalEndpoint {
    /// The assigned URL for the node's Beacon API endpoint.
    #[builder(into)]
    #[serde(rename = "beaconApiEndpoint")]
    pub r#beacon_api_endpoint: Option<String>,
    /// The assigned URL for the node's Beacon Prometheus metrics endpoint.
    #[builder(into)]
    #[serde(rename = "beaconPrometheusMetricsApiEndpoint")]
    pub r#beacon_prometheus_metrics_api_endpoint: Option<String>,
    /// The assigned URL for the node's execution client's Prometheus metrics endpoint.
    #[builder(into)]
    #[serde(rename = "executionClientPrometheusMetricsApiEndpoint")]
    pub r#execution_client_prometheus_metrics_api_endpoint: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BlockchainNodesEthereumDetailsAdditionalEndpoint {
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
                    "beacon_api_endpoint",
                    &self.r#beacon_api_endpoint,
                ),
                to_pulumi_object_field(
                    "beacon_prometheus_metrics_api_endpoint",
                    &self.r#beacon_prometheus_metrics_api_endpoint,
                ),
                to_pulumi_object_field(
                    "execution_client_prometheus_metrics_api_endpoint",
                    &self.r#execution_client_prometheus_metrics_api_endpoint,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BlockchainNodesEthereumDetailsAdditionalEndpoint {
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
                    r#beacon_api_endpoint: {
                        let field_value = match fields_map.get("beacon_api_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'beacon_api_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#beacon_prometheus_metrics_api_endpoint: {
                        let field_value = match fields_map.get("beacon_prometheus_metrics_api_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'beacon_prometheus_metrics_api_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_client_prometheus_metrics_api_endpoint: {
                        let field_value = match fields_map.get("execution_client_prometheus_metrics_api_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_client_prometheus_metrics_api_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
