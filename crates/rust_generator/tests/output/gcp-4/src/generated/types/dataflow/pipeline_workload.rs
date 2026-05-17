#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineWorkload {
    /// Template information and additional parameters needed to launch a Dataflow job using the flex launch API.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchflextemplaterequest
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dataflowFlexTemplateRequest")]
    pub r#dataflow_flex_template_request: Option<Box<super::super::types::dataflow::PipelineWorkloadDataflowFlexTemplateRequest>>,
    /// Template information and additional parameters needed to launch a Dataflow job using the standard launch API.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchtemplaterequest
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dataflowLaunchTemplateRequest")]
    pub r#dataflow_launch_template_request: Option<Box<super::super::types::dataflow::PipelineWorkloadDataflowLaunchTemplateRequest>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineWorkload {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "dataflow_flex_template_request".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dataflow_flex_template_request,
                )
                .await,
            );
            map.insert(
                "dataflow_launch_template_request".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dataflow_launch_template_request,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineWorkload {
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
                    r#dataflow_flex_template_request: {
                        let field_value = match fields_map.get("dataflow_flex_template_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataflow_flex_template_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dataflow_launch_template_request: {
                        let field_value = match fields_map.get("dataflow_launch_template_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataflow_launch_template_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
