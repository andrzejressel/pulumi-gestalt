#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineStageAction {
    /// A category defines what kind of action can be taken in the stage, and constrains the provider type for the action. Possible values are `Approval`, `Build`, `Deploy`, `Invoke`, `Source` and `Test`.
    #[builder(into)]
    pub r#category: String,
    /// A map of the action declaration's configuration. Configurations options for action types and providers can be found in the [Pipeline Structure Reference](http://docs.aws.amazon.com/codepipeline/latest/userguide/reference-pipeline-structure.html#action-requirements) and [Action Structure Reference](https://docs.aws.amazon.com/codepipeline/latest/userguide/action-reference.html) documentation. Note: The `DetectChanges` parameter (optional, default value is true) in the `configuration` section causes CodePipeline to automatically start your pipeline upon new commits. Please refer to AWS Documentation for more details: https://docs.aws.amazon.com/codepipeline/latest/userguide/action-reference-CodestarConnectionSource.html#action-reference-CodestarConnectionSource-config.
    #[builder(into)]
    pub r#configuration: Option<std::collections::BTreeMap<String, String>>,
    /// A list of artifact names to be worked on.
    #[builder(into)]
    pub r#input_artifacts: Option<Vec<String>>,
    /// The action declaration's name.
    #[builder(into)]
    pub r#name: String,
    /// The namespace all output variables will be accessed from.
    #[builder(into)]
    pub r#namespace: Option<String>,
    /// A list of artifact names to output. Output artifact names must be unique within a pipeline.
    #[builder(into)]
    pub r#output_artifacts: Option<Vec<String>>,
    /// The creator of the action being called. Possible values are `AWS`, `Custom` and `ThirdParty`.
    #[builder(into)]
    pub r#owner: String,
    /// The provider of the service being called by the action. Valid providers are determined by the action category. Provider names are listed in the [Action Structure Reference](https://docs.aws.amazon.com/codepipeline/latest/userguide/action-reference.html) documentation.
    #[builder(into)]
    pub r#provider: String,
    /// The region in which to run the action.
    #[builder(into)]
    pub r#region: Option<String>,
    /// The ARN of the IAM service role that will perform the declared action. This is assumed through the roleArn for the pipeline.
    #[builder(into)]
    pub r#role_arn: Option<String>,
    /// The order in which actions are run.
    #[builder(into)]
    pub r#run_order: Option<i32>,
    #[builder(into)]
    pub r#timeout_in_minutes: Option<i32>,
    /// A string that identifies the action type.
    #[builder(into)]
    pub r#version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineStageAction {
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
                    "category",
                    &self.r#category,
                ),
                to_pulumi_object_field(
                    "configuration",
                    &self.r#configuration,
                ),
                to_pulumi_object_field(
                    "inputArtifacts",
                    &self.r#input_artifacts,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "namespace",
                    &self.r#namespace,
                ),
                to_pulumi_object_field(
                    "outputArtifacts",
                    &self.r#output_artifacts,
                ),
                to_pulumi_object_field(
                    "owner",
                    &self.r#owner,
                ),
                to_pulumi_object_field(
                    "provider",
                    &self.r#provider,
                ),
                to_pulumi_object_field(
                    "region",
                    &self.r#region,
                ),
                to_pulumi_object_field(
                    "roleArn",
                    &self.r#role_arn,
                ),
                to_pulumi_object_field(
                    "runOrder",
                    &self.r#run_order,
                ),
                to_pulumi_object_field(
                    "timeoutInMinutes",
                    &self.r#timeout_in_minutes,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineStageAction {
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
                    r#category: {
                        let field_value = match fields_map.get("category") {
                            Some(value) => value,
                            None => bail!("Missing field 'category' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configuration: {
                        let field_value = match fields_map.get("configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_artifacts: {
                        let field_value = match fields_map.get("inputArtifacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputArtifacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#namespace: {
                        let field_value = match fields_map.get("namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_artifacts: {
                        let field_value = match fields_map.get("outputArtifacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputArtifacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#owner: {
                        let field_value = match fields_map.get("owner") {
                            Some(value) => value,
                            None => bail!("Missing field 'owner' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provider: {
                        let field_value = match fields_map.get("provider") {
                            Some(value) => value,
                            None => bail!("Missing field 'provider' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("roleArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'roleArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run_order: {
                        let field_value = match fields_map.get("runOrder") {
                            Some(value) => value,
                            None => bail!("Missing field 'runOrder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_minutes: {
                        let field_value = match fields_map.get("timeoutInMinutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeoutInMinutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
