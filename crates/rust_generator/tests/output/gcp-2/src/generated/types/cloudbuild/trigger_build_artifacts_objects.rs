#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerBuildArtifactsObjects {
    /// Cloud Storage bucket and optional object path, in the form "gs://bucket/path/to/somewhere/".
    /// Files in the workspace matching any path pattern will be uploaded to Cloud Storage with
    /// this location as a prefix.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// Path globs used to match files in the build's workspace.
    #[builder(into, default)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Option<Vec<String>>>,
    /// (Output)
    /// Output only. Stores timing information for pushing all artifact objects.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_timing"></a>The `timing` block contains:
    #[builder(into, default)]
    #[serde(rename = "timings")]
    pub r#timings: Box<Option<Vec<super::super::types::cloudbuild::TriggerBuildArtifactsObjectsTiming>>>,
}
