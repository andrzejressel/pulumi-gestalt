#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration {
    /// A list of security controls that are disabled in the configuration policy Security Hub enables all other controls (including newly released controls) other than the listed controls. Conflicts with `enabled_control_identifiers`.
    #[builder(into)]
    #[serde(rename = "disabledControlIdentifiers")]
    pub r#disabled_control_identifiers: Option<Vec<String>>,
    /// A list of security controls that are enabled in the configuration policy. Security Hub disables all other controls (including newly released controls) other than the listed controls. Conflicts with `disabled_control_identifiers`.
    #[builder(into)]
    #[serde(rename = "enabledControlIdentifiers")]
    pub r#enabled_control_identifiers: Option<Vec<String>>,
    /// A list of control parameter customizations that are included in a configuration policy. Include multiple blocks to define multiple control custom parameters. See below.
    #[builder(into)]
    #[serde(rename = "securityControlCustomParameters")]
    pub r#security_control_custom_parameters: Option<Vec<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "disabled_control_identifiers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disabled_control_identifiers,
                )
                .await,
            );
            map.insert(
                "enabled_control_identifiers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled_control_identifiers,
                )
                .await,
            );
            map.insert(
                "security_control_custom_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_control_custom_parameters,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration {
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
                    r#disabled_control_identifiers: {
                        let field_value = match fields_map.get("disabled_control_identifiers") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled_control_identifiers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled_control_identifiers: {
                        let field_value = match fields_map.get("enabled_control_identifiers") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled_control_identifiers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_control_custom_parameters: {
                        let field_value = match fields_map.get("security_control_custom_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_control_custom_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
