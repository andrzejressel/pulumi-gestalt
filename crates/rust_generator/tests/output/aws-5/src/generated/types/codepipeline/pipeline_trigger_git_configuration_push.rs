#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineTriggerGitConfigurationPush {
    /// The field that specifies to filter on branches for the push trigger configuration. A `branches` block is documented below.
    #[builder(into)]
    #[serde(rename = "branches")]
    pub r#branches: Option<Box<super::super::types::codepipeline::PipelineTriggerGitConfigurationPushBranches>>,
    /// The field that specifies to filter on file paths for the push trigger configuration. A `file_paths` block is documented below.
    #[builder(into)]
    #[serde(rename = "filePaths")]
    pub r#file_paths: Option<Box<super::super::types::codepipeline::PipelineTriggerGitConfigurationPushFilePaths>>,
    /// The field that contains the details for the Git tags trigger configuration. A `tags` block is documented below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Box<super::super::types::codepipeline::PipelineTriggerGitConfigurationPushTags>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineTriggerGitConfigurationPush {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("branches".to_string(), self.r#branches.to_pulumi_value().await);
            map.insert("file_paths".to_string(), self.r#file_paths.to_pulumi_value().await);
            map.insert("tags".to_string(), self.r#tags.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineTriggerGitConfigurationPush {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#branches: {
                        let field_value = match fields_map.get("branches") {
                            Some(value) => value,
                            None => bail!("Missing field 'branches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codepipeline::PipelineTriggerGitConfigurationPushBranches>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#file_paths: {
                        let field_value = match fields_map.get("file_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codepipeline::PipelineTriggerGitConfigurationPushFilePaths>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codepipeline::PipelineTriggerGitConfigurationPushTags>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
