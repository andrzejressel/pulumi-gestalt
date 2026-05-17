#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectSecondarySource {
    /// Configuration block that contains information that defines how the build project reports the build status to the source provider. This option is only used when the source provider is GitHub, GitHub Enterprise, GitLab, GitLab Self Managed, or Bitbucket. `build_status_config` blocks are documented below.
    #[builder(into)]
    #[serde(rename = "buildStatusConfig")]
    pub r#build_status_config: Option<Box<super::super::types::codebuild::ProjectSecondarySourceBuildStatusConfig>>,
    /// The build spec declaration to use for this build project's related builds. This must be set when `type` is `NO_SOURCE`. It can either be a path to a file residing in the repository to be built or a local file path leveraging the `file()` built-in.
    #[builder(into)]
    #[serde(rename = "buildspec")]
    pub r#buildspec: Option<String>,
    /// Truncate git history to this many commits. Use `0` for a `Full` checkout which you need to run commands like `git branch --show-current`. See [AWS CodePipeline User Guide: Tutorial: Use full clone with a GitHub pipeline source](https://docs.aws.amazon.com/codepipeline/latest/userguide/tutorials-github-gitclone.html) for details.
    #[builder(into)]
    #[serde(rename = "gitCloneDepth")]
    pub r#git_clone_depth: Option<i32>,
    /// Configuration block. Detailed below.
    #[builder(into)]
    #[serde(rename = "gitSubmodulesConfig")]
    pub r#git_submodules_config: Option<Box<super::super::types::codebuild::ProjectSecondarySourceGitSubmodulesConfig>>,
    /// Ignore SSL warnings when connecting to source control.
    #[builder(into)]
    #[serde(rename = "insecureSsl")]
    pub r#insecure_ssl: Option<bool>,
    /// Location of the source code from git or s3.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Whether to report the status of a build's start and finish to your source provider. This option is valid only when your source provider is GitHub, GitHub Enterprise, GitLab, GitLab Self Managed, or Bitbucket.
    #[builder(into)]
    #[serde(rename = "reportBuildStatus")]
    pub r#report_build_status: Option<bool>,
    /// An identifier for this project source. The identifier can only contain alphanumeric characters and underscores, and must be less than 128 characters in length.
    #[builder(into)]
    #[serde(rename = "sourceIdentifier")]
    pub r#source_identifier: String,
    /// Type of repository that contains the source code to be built. Valid values: `BITBUCKET`, `CODECOMMIT`, `CODEPIPELINE`, `GITHUB`, `GITHUB_ENTERPRISE`, `GITLAB`, `GITLAB_SELF_MANAGED`, `NO_SOURCE`, `S3`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProjectSecondarySource {
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
                    "build_status_config",
                    &self.r#build_status_config,
                ),
                to_pulumi_object_field(
                    "buildspec",
                    &self.r#buildspec,
                ),
                to_pulumi_object_field(
                    "git_clone_depth",
                    &self.r#git_clone_depth,
                ),
                to_pulumi_object_field(
                    "git_submodules_config",
                    &self.r#git_submodules_config,
                ),
                to_pulumi_object_field(
                    "insecure_ssl",
                    &self.r#insecure_ssl,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "report_build_status",
                    &self.r#report_build_status,
                ),
                to_pulumi_object_field(
                    "source_identifier",
                    &self.r#source_identifier,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProjectSecondarySource {
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
                    r#build_status_config: {
                        let field_value = match fields_map.get("build_status_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'build_status_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buildspec: {
                        let field_value = match fields_map.get("buildspec") {
                            Some(value) => value,
                            None => bail!("Missing field 'buildspec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git_clone_depth: {
                        let field_value = match fields_map.get("git_clone_depth") {
                            Some(value) => value,
                            None => bail!("Missing field 'git_clone_depth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git_submodules_config: {
                        let field_value = match fields_map.get("git_submodules_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'git_submodules_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#insecure_ssl: {
                        let field_value = match fields_map.get("insecure_ssl") {
                            Some(value) => value,
                            None => bail!("Missing field 'insecure_ssl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#report_build_status: {
                        let field_value = match fields_map.get("report_build_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'report_build_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_identifier: {
                        let field_value = match fields_map.get("source_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
