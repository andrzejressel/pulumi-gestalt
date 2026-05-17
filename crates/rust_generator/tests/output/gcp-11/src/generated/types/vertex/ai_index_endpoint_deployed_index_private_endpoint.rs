#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiIndexEndpointDeployedIndexPrivateEndpoint {
    /// (Output)
    /// The ip address used to send match gRPC requests.
    #[builder(into)]
    #[serde(rename = "matchGrpcAddress")]
    pub r#match_grpc_address: Option<String>,
    /// (Output)
    /// PscAutomatedEndpoints is populated if private service connect is enabled if PscAutomatedConfig is set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pscAutomatedEndpoints")]
    pub r#psc_automated_endpoints: Option<Vec<super::super::types::vertex::AiIndexEndpointDeployedIndexPrivateEndpointPscAutomatedEndpoint>>,
    /// (Output)
    /// The name of the service attachment resource. Populated if private service connect is enabled.
    #[builder(into)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiIndexEndpointDeployedIndexPrivateEndpoint {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "match_grpc_address",
                    &self.r#match_grpc_address,
                ),
                to_pulumi_object_field(
                    "psc_automated_endpoints",
                    &self.r#psc_automated_endpoints,
                ),
                to_pulumi_object_field(
                    "service_attachment",
                    &self.r#service_attachment,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiIndexEndpointDeployedIndexPrivateEndpoint {
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
                    r#match_grpc_address: {
                        let field_value = match fields_map.get("match_grpc_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_grpc_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_automated_endpoints: {
                        let field_value = match fields_map.get("psc_automated_endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'psc_automated_endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_attachment: {
                        let field_value = match fields_map.get("service_attachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_attachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
