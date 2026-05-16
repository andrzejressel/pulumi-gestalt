#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SlotTypeEnumerationValue {
    /// Additional values related to the slot type value. Each item must be less than or equal to 140 characters in length.
    #[builder(into)]
    #[serde(rename = "synonyms")]
    pub r#synonyms: Option<Vec<String>>,
    /// The value of the slot type. Must be less than or equal to 140 characters in length.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
