#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollectionIncludeRegexesPatternCloudStorageRegex {
    /// Regex to test the bucket name against. If empty, all buckets match. Example: "marketing2021" or "(marketing)\d{4}" will both match the bucket gs://marketing2021
    #[builder(into)]
    #[serde(rename = "bucketNameRegex")]
    pub r#bucket_name_regex: Option<String>,
    /// For organizations, if unset, will match all projects.
    #[builder(into)]
    #[serde(rename = "projectIdRegex")]
    pub r#project_id_regex: Option<String>,
}
