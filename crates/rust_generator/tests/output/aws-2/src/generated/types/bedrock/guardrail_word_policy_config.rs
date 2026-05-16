#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuardrailWordPolicyConfig {
    /// A config for the list of managed words. See Managed Word Lists Config for more information.
    #[builder(into)]
    #[serde(rename = "managedWordListsConfigs")]
    pub r#managed_word_lists_configs: Option<Vec<super::super::types::bedrock::GuardrailWordPolicyConfigManagedWordListsConfig>>,
    /// List of custom word configs. See Words Config for more information.
    #[builder(into)]
    #[serde(rename = "wordsConfigs")]
    pub r#words_configs: Option<Vec<super::super::types::bedrock::GuardrailWordPolicyConfigWordsConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuardrailWordPolicyConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("managed_word_lists_configs".to_string(), self.r#managed_word_lists_configs.to_pulumi_value().await);
            map.insert("words_configs".to_string(), self.r#words_configs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuardrailWordPolicyConfig {
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
                    r#managed_word_lists_configs: {
                        let field_value = match fields_map.get("managed_word_lists_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_word_lists_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::bedrock::GuardrailWordPolicyConfigManagedWordListsConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#words_configs: {
                        let field_value = match fields_map.get("words_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'words_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::bedrock::GuardrailWordPolicyConfigWordsConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
