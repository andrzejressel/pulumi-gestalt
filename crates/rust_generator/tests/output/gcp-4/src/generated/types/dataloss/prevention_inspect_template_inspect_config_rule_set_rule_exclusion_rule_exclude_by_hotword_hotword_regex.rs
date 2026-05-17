#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeByHotwordHotwordRegex {
    /// The index of the submatch to extract as findings. When not specified,
    /// the entire match is returned. No more than 3 may be included.
    #[builder(into)]
    #[serde(rename = "groupIndexes")]
    pub r#group_indexes: Option<Vec<i32>>,
    /// Pattern defining the regular expression. Its syntax
    /// (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub.
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeByHotwordHotwordRegex {
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
                "group_indexes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#group_indexes,
                )
                .await,
            );
            map.insert(
                "pattern".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pattern,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeByHotwordHotwordRegex {
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
                    r#group_indexes: {
                        let field_value = match fields_map.get("group_indexes") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_indexes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pattern: {
                        let field_value = match fields_map.get("pattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'pattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
