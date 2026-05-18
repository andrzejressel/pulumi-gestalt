#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeTargetParametersBatchJobParameters {
    /// The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. This parameter is used only if the target is an AWS Batch job. Detailed below.
    #[builder(into)]
    #[serde(rename = "arrayProperties")]
    pub r#array_properties: Option<Box<super::super::types::pipes::PipeTargetParametersBatchJobParametersArrayProperties>>,
    /// The overrides that are sent to a container. Detailed below.
    #[builder(into)]
    #[serde(rename = "containerOverrides")]
    pub r#container_overrides: Option<Box<super::super::types::pipes::PipeTargetParametersBatchJobParametersContainerOverrides>>,
    /// A list of dependencies for the job. A job can depend upon a maximum of 20 jobs. You can specify a SEQUENTIAL type dependency without specifying a job ID for array jobs so that each child array job completes sequentially, starting at index 0. You can also specify an N_TO_N type dependency with a job ID for array jobs. In that case, each index child of this job must wait for the corresponding index child of each dependency to complete before it can begin. Detailed below.
    #[builder(into)]
    #[serde(rename = "dependsOns")]
    pub r#depends_ons: Option<Vec<super::super::types::pipes::PipeTargetParametersBatchJobParametersDependsOn>>,
    /// The job definition used by this job. This value can be one of name, name:revision, or the Amazon Resource Name (ARN) for the job definition. If name is specified without a revision then the latest active revision is used.
    #[builder(into)]
    #[serde(rename = "jobDefinition")]
    pub r#job_definition: String,
    /// The name of the job. It can be up to 128 letters long.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: String,
    /// Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and value pair mapping. Parameters included here override any corresponding parameter defaults from the job definition. Detailed below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// The retry strategy to use for failed jobs. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition. Detailed below.
    #[builder(into)]
    #[serde(rename = "retryStrategy")]
    pub r#retry_strategy: Option<Box<super::super::types::pipes::PipeTargetParametersBatchJobParametersRetryStrategy>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeTargetParametersBatchJobParameters {
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
                    "array_properties",
                    &self.r#array_properties,
                ),
                to_pulumi_object_field(
                    "container_overrides",
                    &self.r#container_overrides,
                ),
                to_pulumi_object_field(
                    "depends_ons",
                    &self.r#depends_ons,
                ),
                to_pulumi_object_field(
                    "job_definition",
                    &self.r#job_definition,
                ),
                to_pulumi_object_field(
                    "job_name",
                    &self.r#job_name,
                ),
                to_pulumi_object_field(
                    "parameters",
                    &self.r#parameters,
                ),
                to_pulumi_object_field(
                    "retry_strategy",
                    &self.r#retry_strategy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeTargetParametersBatchJobParameters {
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
                    r#array_properties: {
                        let field_value = match fields_map.get("array_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'array_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_overrides: {
                        let field_value = match fields_map.get("container_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#depends_ons: {
                        let field_value = match fields_map.get("depends_ons") {
                            Some(value) => value,
                            None => bail!("Missing field 'depends_ons' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_definition: {
                        let field_value = match fields_map.get("job_definition") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_definition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_strategy: {
                        let field_value = match fields_map.get("retry_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
