#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContent {
    /// Configures which bundles to install and their corresponding install specs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bundles")]
    pub r#bundles: Option<Vec<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentBundle>>,
    /// Configures the installation of the Template Library.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "templateLibrary")]
    pub r#template_library: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentTemplateLibrary>>,
}
