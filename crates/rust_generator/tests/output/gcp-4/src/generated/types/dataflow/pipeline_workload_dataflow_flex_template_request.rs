#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineWorkloadDataflowFlexTemplateRequest {
    /// Parameter to launch a job from a Flex Template.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchflextemplateparameter
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "launchParameter")]
    pub r#launch_parameter: Box<super::super::types::dataflow::PipelineWorkloadDataflowFlexTemplateRequestLaunchParameter>,
    /// The regional endpoint to which to direct the request. For example, us-central1, us-west1.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The ID of the Cloud Platform project that the job belongs to.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: String,
    /// If true, the request is validated but not actually executed. Defaults to false.
    #[builder(into)]
    #[serde(rename = "validateOnly")]
    pub r#validate_only: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineWorkloadDataflowFlexTemplateRequest {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "launch_parameter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#launch_parameter,
                )
                .await,
            );
            map.insert(
                "location".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#location,
                )
                .await,
            );
            map.insert(
                "project_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#project_id,
                )
                .await,
            );
            map.insert(
                "validate_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#validate_only,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineWorkloadDataflowFlexTemplateRequest {
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
                    r#launch_parameter: {
                        let field_value = match fields_map.get("launch_parameter") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_parameter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project_id: {
                        let field_value = match fields_map.get("project_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'project_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validate_only: {
                        let field_value = match fields_map.get("validate_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'validate_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
