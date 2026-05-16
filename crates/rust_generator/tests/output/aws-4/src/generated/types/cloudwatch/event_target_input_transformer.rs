#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventTargetInputTransformer {
    /// Key value pairs specified in the form of JSONPath (for example, time = $.time)
    /// * You can have as many as 100 key-value pairs.
    /// * You must use JSON dot notation, not bracket notation.
    /// * The keys can't start with "AWS".
    #[builder(into)]
    #[serde(rename = "inputPaths")]
    pub r#input_paths: Option<std::collections::HashMap<String, String>>,
    /// Template to customize data sent to the target. Must be valid JSON. To send a string value, the string value must include double quotes.
    #[builder(into)]
    #[serde(rename = "inputTemplate")]
    pub r#input_template: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventTargetInputTransformer {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("input_paths".to_string(), self.r#input_paths.to_pulumi_value().await);
            map.insert("input_template".to_string(), self.r#input_template.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventTargetInputTransformer {
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
                    r#input_paths: {
                        let field_value = match fields_map.get("input_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_template: {
                        let field_value = match fields_map.get("input_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
