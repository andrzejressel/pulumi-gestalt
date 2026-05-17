#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerTriggerTemplate {
    /// Name of the branch to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    /// This field is a regular expression.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: String,
    /// Explicit commit SHA to build. Exactly one of a branch name, tag, or commit SHA must be provided.
    #[builder(into)]
    #[serde(rename = "commitSha")]
    pub r#commit_sha: String,
    /// Directory, relative to the source root, in which to run the build.
    /// 
    /// This must be a relative path. If a step's dir is specified and
    /// is an absolute path, this value is ignored for that step's
    /// execution.
    #[builder(into)]
    #[serde(rename = "dir")]
    pub r#dir: String,
    /// Only trigger a build if the revision regex does NOT match the revision regex.
    #[builder(into)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: bool,
    /// ID of the project that owns the Cloud Source Repository. If
    /// omitted, the project ID requesting the build is assumed.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: String,
    /// Name of the Cloud Source Repository. If omitted, the name "default" is assumed.
    #[builder(into)]
    #[serde(rename = "repoName")]
    pub r#repo_name: String,
    /// Name of the tag to build. Exactly one of a branch name, tag, or commit SHA must be provided.
    /// This field is a regular expression.
    #[builder(into)]
    #[serde(rename = "tagName")]
    pub r#tag_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTriggerTriggerTemplate {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "branch_name",
                    &self.r#branch_name,
                ),
                to_pulumi_object_field(
                    "commit_sha",
                    &self.r#commit_sha,
                ),
                to_pulumi_object_field(
                    "dir",
                    &self.r#dir,
                ),
                to_pulumi_object_field(
                    "invert_regex",
                    &self.r#invert_regex,
                ),
                to_pulumi_object_field(
                    "project_id",
                    &self.r#project_id,
                ),
                to_pulumi_object_field(
                    "repo_name",
                    &self.r#repo_name,
                ),
                to_pulumi_object_field(
                    "tag_name",
                    &self.r#tag_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTriggerTriggerTemplate {
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
                    r#branch_name: {
                        let field_value = match fields_map.get("branch_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'branch_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#commit_sha: {
                        let field_value = match fields_map.get("commit_sha") {
                            Some(value) => value,
                            None => bail!("Missing field 'commit_sha' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dir: {
                        let field_value = match fields_map.get("dir") {
                            Some(value) => value,
                            None => bail!("Missing field 'dir' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#invert_regex: {
                        let field_value = match fields_map.get("invert_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'invert_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#repo_name: {
                        let field_value = match fields_map.get("repo_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'repo_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_name: {
                        let field_value = match fields_map.get("tag_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
