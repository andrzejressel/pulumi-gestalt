#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfig {
    /// Replace each input value with a given value.
    /// The `new_value` block must only contain one argument. For example when replacing the contents of a string-type field, only `string_value` should be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "newValue")]
    pub r#new_value: Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfigNewValue>,
}
