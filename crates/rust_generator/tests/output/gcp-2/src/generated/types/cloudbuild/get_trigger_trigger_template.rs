#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerTriggerTemplate {
    /// Name of the branch to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    /// This field is a regular expression.
    #[builder(into)]
    pub r#branch_name: String,
    /// Explicit commit SHA to build. Exactly one of a branch name, tag, or commit SHA must be provided.
    #[builder(into)]
    pub r#commit_sha: String,
    /// Directory, relative to the source root, in which to run the build.
    /// 
    /// This must be a relative path. If a step's dir is specified and
    /// is an absolute path, this value is ignored for that step's
    /// execution.
    #[builder(into)]
    pub r#dir: String,
    /// Only trigger a build if the revision regex does NOT match the revision regex.
    #[builder(into)]
    pub r#invert_regex: bool,
    /// ID of the project that owns the Cloud Source Repository. If
    /// omitted, the project ID requesting the build is assumed.
    #[builder(into)]
    pub r#project_id: String,
    /// Name of the Cloud Source Repository. If omitted, the name "default" is assumed.
    #[builder(into)]
    pub r#repo_name: String,
    /// Name of the tag to build. Exactly one of a branch name, tag, or commit SHA must be provided.
    /// This field is a regular expression.
    #[builder(into)]
    pub r#tag_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTriggerTriggerTemplate {
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
                    "branchName",
                    &self.r#branch_name,
                ),
                to_pulumi_object_field(
                    "commitSha",
                    &self.r#commit_sha,
                ),
                to_pulumi_object_field(
                    "dir",
                    &self.r#dir,
                ),
                to_pulumi_object_field(
                    "invertRegex",
                    &self.r#invert_regex,
                ),
                to_pulumi_object_field(
                    "projectId",
                    &self.r#project_id,
                ),
                to_pulumi_object_field(
                    "repoName",
                    &self.r#repo_name,
                ),
                to_pulumi_object_field(
                    "tagName",
                    &self.r#tag_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("branchName") {
                            Some(value) => value,
                            None => bail!("Missing field 'branchName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#commit_sha: {
                        let field_value = match fields_map.get("commitSha") {
                            Some(value) => value,
                            None => bail!("Missing field 'commitSha' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("invertRegex") {
                            Some(value) => value,
                            None => bail!("Missing field 'invertRegex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#repo_name: {
                        let field_value = match fields_map.get("repoName") {
                            Some(value) => value,
                            None => bail!("Missing field 'repoName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_name: {
                        let field_value = match fields_map.get("tagName") {
                            Some(value) => value,
                            None => bail!("Missing field 'tagName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
