#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter {
    /// The bool `value` for a Boolean-typed Security Hub Control Parameter.
    #[builder(into)]
    #[serde(rename = "bool")]
    pub r#bool: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBool>>,
    /// The float `value` for a Double-typed Security Hub Control Parameter.
    #[builder(into)]
    #[serde(rename = "double")]
    pub r#double: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterDouble>>,
    /// The string `value` for a Enum-typed Security Hub Control Parameter.
    #[builder(into)]
    #[serde(rename = "enum")]
    pub r#enum_: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnum>>,
    /// The string list `value` for a EnumList-typed Security Hub Control Parameter.
    #[builder(into)]
    #[serde(rename = "enumList")]
    pub r#enum_list: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumList>>,
    /// The int `value` for a Int-typed Security Hub Control Parameter.
    #[builder(into)]
    #[serde(rename = "int")]
    pub r#int: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterInt>>,
    /// The int list `value` for a IntList-typed Security Hub Control Parameter.
    #[builder(into)]
    #[serde(rename = "intList")]
    pub r#int_list: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntList>>,
    /// The name of the control parameter. For more information see the [Security Hub controls reference] documentation.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The string `value` for a String-typed Security Hub Control Parameter.
    #[builder(into)]
    #[serde(rename = "string")]
    pub r#string: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterString>>,
    /// The string list `value` for a StringList-typed Security Hub Control Parameter.
    #[builder(into)]
    #[serde(rename = "stringList")]
    pub r#string_list: Option<Box<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringList>>,
    /// Identifies whether a control parameter uses a custom user-defined value or subscribes to the default Security Hub behavior. Valid values: `DEFAULT`, `CUSTOM`.
    #[builder(into)]
    #[serde(rename = "valueType")]
    pub r#value_type: String,
}
