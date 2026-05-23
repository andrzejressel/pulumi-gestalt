#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiEndpointDeployedModelPrivateEndpoint {
    /// (Output)
    /// Output only. Http(s) path to send explain requests.
    #[builder(into)]
    pub r#explain_http_uri: Option<String>,
    /// (Output)
    /// Output only. Http(s) path to send health check requests.
    #[builder(into)]
    pub r#health_http_uri: Option<String>,
    /// (Output)
    /// Output only. Http(s) path to send prediction requests.
    #[builder(into)]
    pub r#predict_http_uri: Option<String>,
    /// (Output)
    /// Output only. The name of the service attachment resource. Populated if private service connect is enabled.
    #[builder(into)]
    pub r#service_attachment: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiEndpointDeployedModelPrivateEndpoint {
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
                    "explainHttpUri",
                    &self.r#explain_http_uri,
                ),
                to_pulumi_object_field(
                    "healthHttpUri",
                    &self.r#health_http_uri,
                ),
                to_pulumi_object_field(
                    "predictHttpUri",
                    &self.r#predict_http_uri,
                ),
                to_pulumi_object_field(
                    "serviceAttachment",
                    &self.r#service_attachment,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiEndpointDeployedModelPrivateEndpoint {
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
                    r#explain_http_uri: {
                        let field_value = match fields_map.get("explainHttpUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'explainHttpUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_http_uri: {
                        let field_value = match fields_map.get("healthHttpUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthHttpUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#predict_http_uri: {
                        let field_value = match fields_map.get("predictHttpUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'predictHttpUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_attachment: {
                        let field_value = match fields_map.get("serviceAttachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAttachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
