#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EntryGcsFilesetSpec {
    /// Patterns to identify a set of files in Google Cloud Storage.
    /// See [Cloud Storage documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames)
    /// for more information. Note that bucket wildcards are currently not supported. Examples of valid filePatterns:
    /// * gs://bucket_name/dir/*: matches all files within bucket_name/dir directory.
    /// * gs://bucket_name/dir/**: matches all files in bucket_name/dir spanning all subdirectories.
    /// * gs://bucket_name/file*: matches files prefixed by file in bucket_name
    /// * gs://bucket_name/??.txt: matches files with two characters followed by .txt in bucket_name
    /// * gs://bucket_name/[aeiou].txt: matches files that contain a single vowel character followed by .txt in bucket_name
    /// * gs://bucket_name/[a-m].txt: matches files that contain a, b, ... or m followed by .txt in bucket_name
    /// * gs://bucket_name/a/*/b: matches all files in bucket_name that match a/*/b pattern, such as a/c/b, a/d/b
    /// * gs://another_bucket/a.txt: matches gs://another_bucket/a.txt
    #[builder(into)]
    #[serde(rename = "filePatterns")]
    pub r#file_patterns: Box<Vec<String>>,
    /// (Output)
    /// Sample files contained in this fileset, not all files contained in this fileset are represented here.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_sample_gcs_file_specs"></a>The `sample_gcs_file_specs` block contains:
    #[builder(into, default)]
    #[serde(rename = "sampleGcsFileSpecs")]
    pub r#sample_gcs_file_specs: Box<Option<Vec<super::super::types::datacatalog::EntryGcsFilesetSpecSampleGcsFileSpec>>>,
}
