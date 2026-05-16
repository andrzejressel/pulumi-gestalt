#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExperienceConfigurationContentSourceConfiguration {
    /// The identifiers of the data sources you want to use for your Amazon Kendra experience. Maximum number of 100 items.
    #[builder(into)]
    #[serde(rename = "dataSourceIds")]
    pub r#data_source_ids: Option<Vec<String>>,
    /// Whether to use documents you indexed directly using the `BatchPutDocument API`. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "directPutContent")]
    pub r#direct_put_content: Option<bool>,
    /// The identifier of the FAQs that you want to use for your Amazon Kendra experience. Maximum number of 100 items.
    #[builder(into)]
    #[serde(rename = "faqIds")]
    pub r#faq_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExperienceConfigurationContentSourceConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("data_source_ids".to_string(), self.r#data_source_ids.to_pulumi_value().await);
            map.insert("direct_put_content".to_string(), self.r#direct_put_content.to_pulumi_value().await);
            map.insert("faq_ids".to_string(), self.r#faq_ids.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExperienceConfigurationContentSourceConfiguration {
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
                    r#data_source_ids: {
                        let field_value = match fields_map.get("data_source_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_source_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#direct_put_content: {
                        let field_value = match fields_map.get("direct_put_content") {
                            Some(value) => value,
                            None => bail!("Missing field 'direct_put_content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#faq_ids: {
                        let field_value = match fields_map.get("faq_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'faq_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
