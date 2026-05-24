#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineWorkloadDataflowFlexTemplateRequestLaunchParameter {
    /// Cloud Storage path to a file with a JSON-serialized ContainerSpec as content.
    #[builder(into)]
    pub r#container_spec_gcs_path: Option<String>,
    /// The runtime environment for the Flex Template job.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#FlexTemplateRuntimeEnvironment
    /// Structure is documented below.
    #[builder(into)]
    pub r#environment: Option<Box<super::super::types::dataflow::PipelineWorkloadDataflowFlexTemplateRequestLaunchParameterEnvironment>>,
    /// The job name to use for the created job. For an update job request, the job name should be the same as the existing running job.
    #[builder(into)]
    pub r#job_name: String,
    /// Launch options for this Flex Template job. This is a common set of options across languages and templates. This should not be used to pass job parameters.
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    pub r#launch_options: Option<std::collections::BTreeMap<String, String>>,
    /// 'The parameters for the Flex Template. Example: {"numWorkers":"5"}'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    pub r#parameters: Option<std::collections::BTreeMap<String, String>>,
    /// 'Use this to pass transform name mappings for streaming update jobs. Example: {"oldTransformName":"newTransformName",...}'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    pub r#transform_name_mappings: Option<std::collections::BTreeMap<String, String>>,
    /// Set this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job.
    #[builder(into)]
    pub r#update: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineWorkloadDataflowFlexTemplateRequestLaunchParameter {
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
                    "containerSpecGcsPath",
                    &self.r#container_spec_gcs_path,
                ),
                to_pulumi_object_field(
                    "environment",
                    &self.r#environment,
                ),
                to_pulumi_object_field(
                    "jobName",
                    &self.r#job_name,
                ),
                to_pulumi_object_field(
                    "launchOptions",
                    &self.r#launch_options,
                ),
                to_pulumi_object_field(
                    "parameters",
                    &self.r#parameters,
                ),
                to_pulumi_object_field(
                    "transformNameMappings",
                    &self.r#transform_name_mappings,
                ),
                to_pulumi_object_field(
                    "update",
                    &self.r#update,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineWorkloadDataflowFlexTemplateRequestLaunchParameter {
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
                    r#container_spec_gcs_path: {
                        let field_value = match fields_map.get("containerSpecGcsPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerSpecGcsPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environment: {
                        let field_value = match fields_map.get("environment") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_name: {
                        let field_value = match fields_map.get("jobName") {
                            Some(value) => value,
                            None => bail!("Missing field 'jobName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_options: {
                        let field_value = match fields_map.get("launchOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'launchOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transform_name_mappings: {
                        let field_value = match fields_map.get("transformNameMappings") {
                            Some(value) => value,
                            None => bail!("Missing field 'transformNameMappings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#update: {
                        let field_value = match fields_map.get("update") {
                            Some(value) => value,
                            None => bail!("Missing field 'update' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
