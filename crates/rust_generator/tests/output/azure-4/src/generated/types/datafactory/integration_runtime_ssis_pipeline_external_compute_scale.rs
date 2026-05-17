#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntegrationRuntimeSsisPipelineExternalComputeScale {
    /// Specifies the number of the external nodes, which should be greater than `0` and less than `11`.
    #[builder(into)]
    #[serde(rename = "numberOfExternalNodes")]
    pub r#number_of_external_nodes: Option<i32>,
    /// Specifies the number of the pipeline nodes, which should be greater than `0` and less than `11`.
    #[builder(into)]
    #[serde(rename = "numberOfPipelineNodes")]
    pub r#number_of_pipeline_nodes: Option<i32>,
    /// Specifies the time to live (in minutes) setting of integration runtime which will execute copy activity. Possible values are at least `5`.
    #[builder(into)]
    #[serde(rename = "timeToLive")]
    pub r#time_to_live: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IntegrationRuntimeSsisPipelineExternalComputeScale {
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
                    "number_of_external_nodes",
                    &self.r#number_of_external_nodes,
                ),
                to_pulumi_object_field(
                    "number_of_pipeline_nodes",
                    &self.r#number_of_pipeline_nodes,
                ),
                to_pulumi_object_field(
                    "time_to_live",
                    &self.r#time_to_live,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IntegrationRuntimeSsisPipelineExternalComputeScale {
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
                    r#number_of_external_nodes: {
                        let field_value = match fields_map.get("number_of_external_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_external_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_pipeline_nodes: {
                        let field_value = match fields_map.get("number_of_pipeline_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_pipeline_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_to_live: {
                        let field_value = match fields_map.get("time_to_live") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_to_live' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
