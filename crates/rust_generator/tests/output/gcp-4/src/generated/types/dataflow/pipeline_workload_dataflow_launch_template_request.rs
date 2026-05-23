#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineWorkloadDataflowLaunchTemplateRequest {
    /// A Cloud Storage path to the template from which to create the job. Must be a valid Cloud Storage URL, beginning with 'gs://'.
    #[builder(into)]
    pub r#gcs_path: Option<String>,
    /// The parameters of the template to launch. This should be part of the body of the POST request.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchtemplateparameters
    /// Structure is documented below.
    #[builder(into)]
    pub r#launch_parameters: Option<Box<super::super::types::dataflow::PipelineWorkloadDataflowLaunchTemplateRequestLaunchParameters>>,
    /// The regional endpoint to which to direct the request.
    #[builder(into)]
    pub r#location: Option<String>,
    /// The ID of the Cloud Platform project that the job belongs to.
    #[builder(into)]
    pub r#project_id: String,
    /// (Optional)
    #[builder(into)]
    pub r#validate_only: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineWorkloadDataflowLaunchTemplateRequest {
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
                    "gcsPath",
                    &self.r#gcs_path,
                ),
                to_pulumi_object_field(
                    "launchParameters",
                    &self.r#launch_parameters,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "projectId",
                    &self.r#project_id,
                ),
                to_pulumi_object_field(
                    "validateOnly",
                    &self.r#validate_only,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineWorkloadDataflowLaunchTemplateRequest {
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
                    r#gcs_path: {
                        let field_value = match fields_map.get("gcsPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcsPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_parameters: {
                        let field_value = match fields_map.get("launchParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'launchParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("projectId") {
                            Some(value) => value,
                            None => bail!("Missing field 'projectId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validate_only: {
                        let field_value = match fields_map.get("validateOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'validateOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
