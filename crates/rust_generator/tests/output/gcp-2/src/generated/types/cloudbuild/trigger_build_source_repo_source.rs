#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerBuildSourceRepoSource {
    /// Regex matching branches to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    /// The syntax of the regular expressions accepted is the syntax accepted by RE2 and
    /// described at https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Option<String>,
    /// Explicit commit SHA to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    #[builder(into)]
    #[serde(rename = "commitSha")]
    pub r#commit_sha: Option<String>,
    /// Directory, relative to the source root, in which to run the build.
    /// This must be a relative path. If a step's dir is specified and is an absolute path,
    /// this value is ignored for that step's execution.
    #[builder(into)]
    #[serde(rename = "dir")]
    pub r#dir: Option<String>,
    /// Only trigger a build if the revision regex does NOT match the revision regex.
    #[builder(into)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Option<bool>,
    /// ID of the project that owns the Cloud Source Repository.
    /// If omitted, the project ID requesting the build is assumed.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Option<String>,
    /// Name of the Cloud Source Repository.
    #[builder(into)]
    #[serde(rename = "repoName")]
    pub r#repo_name: String,
    /// Substitutions to use in a triggered build. Should only be used with triggers.run
    #[builder(into)]
    #[serde(rename = "substitutions")]
    pub r#substitutions: Option<std::collections::HashMap<String, String>>,
    /// Regex matching tags to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    /// The syntax of the regular expressions accepted is the syntax accepted by RE2 and
    /// described at https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "tagName")]
    pub r#tag_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TriggerBuildSourceRepoSource {
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
                "branch_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#branch_name,
                )
                .await,
            );
            map.insert(
                "commit_sha".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#commit_sha,
                )
                .await,
            );
            map.insert(
                "dir".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dir,
                )
                .await,
            );
            map.insert(
                "invert_regex".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#invert_regex,
                )
                .await,
            );
            map.insert(
                "project_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#project_id,
                )
                .await,
            );
            map.insert(
                "repo_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repo_name,
                )
                .await,
            );
            map.insert(
                "substitutions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#substitutions,
                )
                .await,
            );
            map.insert(
                "tag_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TriggerBuildSourceRepoSource {
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
                    r#substitutions: {
                        let field_value = match fields_map.get("substitutions") {
                            Some(value) => value,
                            None => bail!("Missing field 'substitutions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
