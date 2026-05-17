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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "bool".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bool,
                )
                .await,
            );
            map.insert(
                "double".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#double,
                )
                .await,
            );
            map.insert(
                "enum_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enum_,
                )
                .await,
            );
            map.insert(
                "enum_list".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enum_list,
                )
                .await,
            );
            map.insert(
                "int".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#int,
                )
                .await,
            );
            map.insert(
                "int_list".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#int_list,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#string,
                )
                .await,
            );
            map.insert(
                "string_list".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#string_list,
                )
                .await,
            );
            map.insert(
                "value_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#value_type,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#bool: {
                        let field_value = match fields_map.get("bool") {
                            Some(value) => value,
                            None => bail!("Missing field 'bool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#double: {
                        let field_value = match fields_map.get("double") {
                            Some(value) => value,
                            None => bail!("Missing field 'double' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enum_: {
                        let field_value = match fields_map.get("enum_") {
                            Some(value) => value,
                            None => bail!("Missing field 'enum_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enum_list: {
                        let field_value = match fields_map.get("enum_list") {
                            Some(value) => value,
                            None => bail!("Missing field 'enum_list' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#int: {
                        let field_value = match fields_map.get("int") {
                            Some(value) => value,
                            None => bail!("Missing field 'int' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#int_list: {
                        let field_value = match fields_map.get("int_list") {
                            Some(value) => value,
                            None => bail!("Missing field 'int_list' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string: {
                        let field_value = match fields_map.get("string") {
                            Some(value) => value,
                            None => bail!("Missing field 'string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_list: {
                        let field_value = match fields_map.get("string_list") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_list' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value_type: {
                        let field_value = match fields_map.get("value_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'value_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
