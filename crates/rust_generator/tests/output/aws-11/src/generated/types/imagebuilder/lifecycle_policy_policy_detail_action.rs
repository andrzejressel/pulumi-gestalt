#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailAction {
    /// Specifies the resources that the lifecycle policy applies to. Detailed below.
    #[builder(into)]
    #[serde(rename = "includeResources")]
    pub r#include_resources: Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailActionIncludeResources>>,
    /// Specifies the lifecycle action to take. Valid values: `DELETE`, `DEPRECATE` or `DISABLE`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
