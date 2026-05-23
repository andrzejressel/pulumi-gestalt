#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateJobTemplateData {
    /// The configuration settings that are used to override defaults configuration.
    #[builder(into)]
    pub r#configuration_overrides: Option<Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverrides>>,
    /// The execution role ARN of the job run.
    #[builder(into)]
    pub r#execution_role_arn: String,
    /// Specify the driver that the job runs on. Exactly one of the two available job drivers is required, either sparkSqlJobDriver or sparkSubmitJobDriver.
    #[builder(into)]
    pub r#job_driver: Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataJobDriver>,
    /// The tags assigned to jobs started using the job template.
    #[builder(into)]
    pub r#job_tags: Option<std::collections::HashMap<String, String>>,
    /// The release version of Amazon EMR.
    #[builder(into)]
    pub r#release_label: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateJobTemplateData {
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
                    "configurationOverrides",
                    &self.r#configuration_overrides,
                ),
                to_pulumi_object_field(
                    "executionRoleArn",
                    &self.r#execution_role_arn,
                ),
                to_pulumi_object_field(
                    "jobDriver",
                    &self.r#job_driver,
                ),
                to_pulumi_object_field(
                    "jobTags",
                    &self.r#job_tags,
                ),
                to_pulumi_object_field(
                    "releaseLabel",
                    &self.r#release_label,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateJobTemplateData {
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
                    r#configuration_overrides: {
                        let field_value = match fields_map.get("configurationOverrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'configurationOverrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_role_arn: {
                        let field_value = match fields_map.get("executionRoleArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'executionRoleArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_driver: {
                        let field_value = match fields_map.get("jobDriver") {
                            Some(value) => value,
                            None => bail!("Missing field 'jobDriver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_tags: {
                        let field_value = match fields_map.get("jobTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'jobTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#release_label: {
                        let field_value = match fields_map.get("releaseLabel") {
                            Some(value) => value,
                            None => bail!("Missing field 'releaseLabel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
