#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProjectArtifacts {
    /// Artifact identifier. Must be the same specified inside the AWS CodeBuild build specification.
    #[builder(into)]
    #[serde(rename = "artifactIdentifier")]
    pub r#artifact_identifier: Option<String>,
    /// Specifies the bucket owner's access for objects that another account uploads to their Amazon S3 bucket. By default, only the account that uploads the objects to the bucket has access to these objects. This property allows you to give the bucket owner access to these objects. Valid values are `NONE`, `READ_ONLY`, and `FULL`. your CodeBuild service role must have the `s3:PutBucketAcl` permission. This permission allows CodeBuild to modify the access control list for the bucket.
    #[builder(into)]
    #[serde(rename = "bucketOwnerAccess")]
    pub r#bucket_owner_access: Option<String>,
    /// Whether to disable encrypting output artifacts. If `type` is set to `NO_ARTIFACTS`, this value is ignored. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "encryptionDisabled")]
    pub r#encryption_disabled: Option<bool>,
    /// Information about the build output artifact location. If `type` is set to `CODEPIPELINE` or `NO_ARTIFACTS`, this value is ignored. If `type` is set to `S3`, this is the name of the output bucket.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Name of the project. If `type` is set to `S3`, this is the name of the output artifact object
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Namespace to use in storing build artifacts. If `type` is set to `S3`, then valid values are `BUILD_ID`, `NONE`.
    #[builder(into)]
    #[serde(rename = "namespaceType")]
    pub r#namespace_type: Option<String>,
    /// Whether a name specified in the build specification overrides the artifact name.
    #[builder(into)]
    #[serde(rename = "overrideArtifactName")]
    pub r#override_artifact_name: Option<bool>,
    /// Type of build output artifact to create. If `type` is set to `S3`, valid values are `NONE`, `ZIP`
    #[builder(into)]
    #[serde(rename = "packaging")]
    pub r#packaging: Option<String>,
    /// If `type` is set to `S3`, this is the path to the output artifact.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Build output artifact's type. Valid values: `CODEPIPELINE`, `NO_ARTIFACTS`, `S3`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
