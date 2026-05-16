#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PosturePolicySetPolicyConstraintSecurityHealthAnalyticsModule {
    /// The state of enablement for the module at its level of the resource hierarchy.
    /// Possible values are: `ENABLEMENT_STATE_UNSPECIFIED`, `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "moduleEnablementState")]
    pub r#module_enablement_state: Option<String>,
    /// The name of the module eg: BIGQUERY_TABLE_CMEK_DISABLED.
    #[builder(into)]
    #[serde(rename = "moduleName")]
    pub r#module_name: String,
}
