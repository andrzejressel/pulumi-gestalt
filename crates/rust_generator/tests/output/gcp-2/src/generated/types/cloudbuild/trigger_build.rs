#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerBuild {
    /// Artifacts produced by the build that should be uploaded upon successful completion of all build steps.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "artifacts")]
    pub r#artifacts: Box<Option<super::super::types::cloudbuild::TriggerBuildArtifacts>>,
    /// Secrets and secret environment variables.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "availableSecrets")]
    pub r#available_secrets: Box<Option<super::super::types::cloudbuild::TriggerBuildAvailableSecrets>>,
    /// A list of images to be pushed upon the successful completion of all build steps.
    /// The images are pushed using the builder service account's credentials.
    /// The digests of the pushed images will be stored in the Build resource's results field.
    /// If any of the images fail to be pushed, the build status is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "images")]
    pub r#images: Option<Vec<String>>,
    /// Google Cloud Storage bucket where logs should be written.
    /// Logs file names will be of the format ${logsBucket}/log-${build_id}.txt.
    #[builder(into)]
    #[serde(rename = "logsBucket")]
    pub r#logs_bucket: Option<String>,
    /// Special options for this build.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: Box<Option<super::super::types::cloudbuild::TriggerBuildOptions>>,
    /// TTL in queue for this build. If provided and the build is enqueued longer than this value,
    /// the build will expire and the build status will be EXPIRED.
    /// The TTL starts ticking from createTime.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "queueTtl")]
    pub r#queue_ttl: Option<String>,
    /// Secrets to decrypt using Cloud Key Management Service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Option<Vec<super::super::types::cloudbuild::TriggerBuildSecret>>,
    /// The location of the source files to build.
    /// One of `storageSource` or `repoSource` must be provided.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<super::super::types::cloudbuild::TriggerBuildSource>>,
    /// The operations to be performed on the workspace.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "steps")]
    pub r#steps: Vec<super::super::types::cloudbuild::TriggerBuildStep>,
    /// Substitutions data for Build resource.
    #[builder(into)]
    #[serde(rename = "substitutions")]
    pub r#substitutions: Option<std::collections::HashMap<String, String>>,
    /// Tags for annotation of a Build. These are not docker tags.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<String>>,
    /// Amount of time that this build should be allowed to run, to second granularity.
    /// If this amount of time elapses, work on the build will cease and the build status will be TIMEOUT.
    /// This timeout must be equal to or greater than the sum of the timeouts for build steps within the build.
    /// The expected format is the number of seconds followed by s.
    /// Default time is ten minutes (600s).
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}
