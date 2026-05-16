#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectCustomModuleCustomConfigCustomOutputProperty {
    /// Name of the property for the custom output.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The CEL expression for the custom output. A resource property can be specified
    /// to return the value of the property or a text string enclosed in quotation marks.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "valueExpression")]
    pub r#value_expression: Option<Box<super::super::types::securitycenter::ProjectCustomModuleCustomConfigCustomOutputPropertyValueExpression>>,
}
