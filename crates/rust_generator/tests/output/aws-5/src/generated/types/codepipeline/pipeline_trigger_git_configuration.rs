#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineTriggerGitConfiguration {
    /// The field where the repository event that will start the pipeline is specified as pull requests. A `pull_request` block is documented below.
    #[builder(into)]
    #[serde(rename = "pullRequests")]
    pub r#pull_requests: Option<Vec<super::super::types::codepipeline::PipelineTriggerGitConfigurationPullRequest>>,
    /// The field where the repository event that will start the pipeline, such as pushing Git tags, is specified with details. A `push` block is documented below.
    #[builder(into)]
    #[serde(rename = "pushes")]
    pub r#pushes: Option<Vec<super::super::types::codepipeline::PipelineTriggerGitConfigurationPush>>,
    /// The name of the pipeline source action where the trigger configuration, such as Git tags, is specified. The trigger configuration will start the pipeline upon the specified change only.
    #[builder(into)]
    #[serde(rename = "sourceActionName")]
    pub r#source_action_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineTriggerGitConfiguration {
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
                    "pull_requests",
                    &self.r#pull_requests,
                ),
                to_pulumi_object_field(
                    "pushes",
                    &self.r#pushes,
                ),
                to_pulumi_object_field(
                    "source_action_name",
                    &self.r#source_action_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineTriggerGitConfiguration {
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
                    r#pull_requests: {
                        let field_value = match fields_map.get("pull_requests") {
                            Some(value) => value,
                            None => bail!("Missing field 'pull_requests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pushes: {
                        let field_value = match fields_map.get("pushes") {
                            Some(value) => value,
                            None => bail!("Missing field 'pushes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_action_name: {
                        let field_value = match fields_map.get("source_action_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_action_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
