#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GlossaryTermTermRelations {
    /// String array that calssifies the term relations.
    #[builder(into)]
    #[serde(rename = "classifies")]
    pub r#classifies: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "isAs")]
    pub r#is_as: Option<Vec<String>>,
}
