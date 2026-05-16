#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotValueElicitationSettingSlotResolutionSetting {
    /// Specifies whether assisted slot resolution is turned on for the slot or not.
    /// Valid values are `EnhancedFallback` or `Default`.
    /// If the value is `EnhancedFallback`, assisted slot resolution is activated when Amazon Lex defaults to the `AMAZON.FallbackIntent`.
    /// If the value is `Default`, assisted slot resolution is turned off.
    #[builder(into)]
    #[serde(rename = "slotResolutionStrategy")]
    pub r#slot_resolution_strategy: String,
}
