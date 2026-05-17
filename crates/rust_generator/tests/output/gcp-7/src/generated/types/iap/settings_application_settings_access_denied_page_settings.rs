#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SettingsApplicationSettingsAccessDeniedPageSettings {
    /// The URI to be redirected to when access is denied.
    #[builder(into)]
    #[serde(rename = "accessDeniedPageUri")]
    pub r#access_denied_page_uri: Option<String>,
    /// Whether to generate a troubleshooting URL on access denied events to this application.
    #[builder(into)]
    #[serde(rename = "generateTroubleshootingUri")]
    pub r#generate_troubleshooting_uri: Option<bool>,
    /// Whether to generate remediation token on access denied events to this application.
    #[builder(into)]
    #[serde(rename = "remediationTokenGenerationEnabled")]
    pub r#remediation_token_generation_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SettingsApplicationSettingsAccessDeniedPageSettings {
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
                "access_denied_page_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_denied_page_uri,
                )
                .await,
            );
            map.insert(
                "generate_troubleshooting_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#generate_troubleshooting_uri,
                )
                .await,
            );
            map.insert(
                "remediation_token_generation_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#remediation_token_generation_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SettingsApplicationSettingsAccessDeniedPageSettings {
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
                    r#access_denied_page_uri: {
                        let field_value = match fields_map.get("access_denied_page_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_denied_page_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#generate_troubleshooting_uri: {
                        let field_value = match fields_map.get("generate_troubleshooting_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'generate_troubleshooting_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remediation_token_generation_enabled: {
                        let field_value = match fields_map.get("remediation_token_generation_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'remediation_token_generation_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
