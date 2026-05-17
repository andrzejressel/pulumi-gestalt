#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineWorkloadDataflowFlexTemplateRequestLaunchParameter {
    /// Cloud Storage path to a file with a JSON-serialized ContainerSpec as content.
    #[builder(into)]
    #[serde(rename = "containerSpecGcsPath")]
    pub r#container_spec_gcs_path: Option<String>,
    /// The runtime environment for the Flex Template job.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#FlexTemplateRuntimeEnvironment
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<Box<super::super::types::dataflow::PipelineWorkloadDataflowFlexTemplateRequestLaunchParameterEnvironment>>,
    /// The job name to use for the created job. For an update job request, the job name should be the same as the existing running job.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: String,
    /// Launch options for this Flex Template job. This is a common set of options across languages and templates. This should not be used to pass job parameters.
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    #[serde(rename = "launchOptions")]
    pub r#launch_options: Option<std::collections::HashMap<String, String>>,
    /// 'The parameters for the Flex Template. Example: {"numWorkers":"5"}'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// 'Use this to pass transform name mappings for streaming update jobs. Example: {"oldTransformName":"newTransformName",...}'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    #[serde(rename = "transformNameMappings")]
    pub r#transform_name_mappings: Option<std::collections::HashMap<String, String>>,
    /// Set this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job.
    #[builder(into)]
    #[serde(rename = "update")]
    pub r#update: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineWorkloadDataflowFlexTemplateRequestLaunchParameter {
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
                "container_spec_gcs_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_spec_gcs_path,
                )
                .await,
            );
            map.insert(
                "environment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#environment,
                )
                .await,
            );
            map.insert(
                "job_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#job_name,
                )
                .await,
            );
            map.insert(
                "launch_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#launch_options,
                )
                .await,
            );
            map.insert(
                "parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameters,
                )
                .await,
            );
            map.insert(
                "transform_name_mappings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transform_name_mappings,
                )
                .await,
            );
            map.insert(
                "update".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#update,
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
                        let field_value = match fields_map.get("container_spec_gcs_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_spec_gcs_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("job_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_options: {
                        let field_value = match fields_map.get("launch_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("transform_name_mappings") {
                            Some(value) => value,
                            None => bail!("Missing field 'transform_name_mappings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
