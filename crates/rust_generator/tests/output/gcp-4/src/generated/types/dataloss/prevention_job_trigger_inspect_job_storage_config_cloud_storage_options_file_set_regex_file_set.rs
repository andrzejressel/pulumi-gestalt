#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSetRegexFileSet {
    /// The name of a Cloud Storage bucket.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    /// A list of regular expressions matching file paths to exclude. All files in the bucket that match at
    /// least one of these regular expressions will be excluded from the scan.
    #[builder(into)]
    #[serde(rename = "excludeRegexes")]
    pub r#exclude_regexes: Option<Vec<String>>,
    /// A list of regular expressions matching file paths to include. All files in the bucket
    /// that match at least one of these regular expressions will be included in the set of files,
    /// except for those that also match an item in excludeRegex. Leaving this field empty will
    /// match all files by default (this is equivalent to including .* in the list)
    #[builder(into)]
    #[serde(rename = "includeRegexes")]
    pub r#include_regexes: Option<Vec<String>>,
}
