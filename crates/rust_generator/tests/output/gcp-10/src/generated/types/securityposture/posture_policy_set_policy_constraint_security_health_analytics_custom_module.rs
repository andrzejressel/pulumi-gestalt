#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModule {
    /// Custom module details.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "config")]
    pub r#config: Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModuleConfig>,
    /// The display name of the Security Health Analytics custom module. This
    /// display name becomes the finding category for all findings that are
    /// returned by this custom module.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// (Output)
    /// A server generated id of custom module.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The state of enablement for the module at its level of the resource hierarchy.
    /// Possible values are: `ENABLEMENT_STATE_UNSPECIFIED`, `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "moduleEnablementState")]
    pub r#module_enablement_state: Option<String>,
}
