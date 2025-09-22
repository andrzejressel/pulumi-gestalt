#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateConfigEditList {
    /// List of values identifying files that should be used in this atom.
    #[builder(into)]
    #[serde(rename = "inputs")]
    pub r#inputs: Option<Vec<String>>,
    /// A unique key for this atom.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Start time in seconds for the atom, relative to the input file timeline.  The default is `0s`.
    #[builder(into)]
    #[serde(rename = "startTimeOffset")]
    pub r#start_time_offset: Option<String>,
}
