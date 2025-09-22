#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RunBookDraft {
    /// A `publish_content_link` block as defined above.
    #[builder(into)]
    #[serde(rename = "contentLink")]
    pub r#content_link: Box<Option<super::super::types::automation::RunBookDraftContentLink>>,
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: Option<String>,
    /// Whether the draft in edit mode.
    #[builder(into)]
    #[serde(rename = "editModeEnabled")]
    pub r#edit_mode_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "lastModifiedTime")]
    pub r#last_modified_time: Option<String>,
    /// Specifies the output types of the runbook.
    #[builder(into)]
    #[serde(rename = "outputTypes")]
    pub r#output_types: Option<Vec<String>>,
    /// A list of `parameters` block as defined below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::automation::RunBookDraftParameter>>,
}
