#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuardrailSensitiveInformationPolicyConfig {
    /// List of entities. See PII Entities Config for more information.
    #[builder(into)]
    #[serde(rename = "piiEntitiesConfigs")]
    pub r#pii_entities_configs: Option<Vec<super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfigPiiEntitiesConfig>>,
    /// List of regex. See Regexes Config for more information.
    #[builder(into)]
    #[serde(rename = "regexesConfigs")]
    pub r#regexes_configs: Option<Vec<super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfigRegexesConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuardrailSensitiveInformationPolicyConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("pii_entities_configs".to_string(), self.r#pii_entities_configs.to_pulumi_value().await);
            map.insert("regexes_configs".to_string(), self.r#regexes_configs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuardrailSensitiveInformationPolicyConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#pii_entities_configs: {
                        let field_value = match fields_map.get("pii_entities_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'pii_entities_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfigPiiEntitiesConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#regexes_configs: {
                        let field_value = match fields_map.get("regexes_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'regexes_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfigRegexesConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
