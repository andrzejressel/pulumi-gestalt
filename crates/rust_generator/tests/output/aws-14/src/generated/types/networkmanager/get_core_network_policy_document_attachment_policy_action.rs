#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCoreNetworkPolicyDocumentAttachmentPolicyAction {
    /// The name of the network function group to attach to the attachment policy.
    #[builder(into)]
    #[serde(rename = "addToNetworkFunctionGroup")]
    pub r#add_to_network_function_group: Option<String>,
    /// Defines how a segment is mapped. Values can be `constant` or `tag`. `constant` statically defines the segment to associate the attachment to. `tag` uses the value of a tag to dynamically try to map to a segment.reference_policies_elements_condition_operators.html) to evaluate.
    #[builder(into)]
    #[serde(rename = "associationMethod")]
    pub r#association_method: Option<String>,
    /// Determines if this mapping should override the segment value for `require_attachment_acceptance`. You can only set this to `true`, indicating that this setting applies only to segments that have `require_attachment_acceptance` set to `false`. If the segment already has the default `require_attachment_acceptance`, you can set this to inherit segment’s acceptance value.
    #[builder(into)]
    #[serde(rename = "requireAcceptance")]
    pub r#require_acceptance: Option<bool>,
    /// Name of the `segment` to share as defined in the `segments` section. This is used only when the `association_method` is `constant`.
    #[builder(into)]
    #[serde(rename = "segment")]
    pub r#segment: Option<String>,
    /// Maps the attachment to the value of a known key. This is used with the `association_method` is `tag`. For example a `tag` of `stage = “test”`, will map to a segment named `test`. The value must exactly match the name of a segment. This allows you to have many segments, but use only a single rule without having to define multiple nearly identical conditions. This prevents creating many similar conditions that all use the same keys to map to segments.
    #[builder(into)]
    #[serde(rename = "tagValueOfKey")]
    pub r#tag_value_of_key: Option<String>,
}
