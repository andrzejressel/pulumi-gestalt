#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetExperienceConfigurationContentSourceConfiguration {
    /// Identifiers of the data sources you want to use for your Amazon Kendra Experience.
    #[builder(into)]
    #[serde(rename = "dataSourceIds")]
    pub r#data_source_ids: Vec<String>,
    /// Whether to use documents you indexed directly using the `BatchPutDocument API`.
    #[builder(into)]
    #[serde(rename = "directPutContent")]
    pub r#direct_put_content: bool,
    /// Identifier of the FAQs that you want to use for your Amazon Kendra Experience.
    #[builder(into)]
    #[serde(rename = "faqIds")]
    pub r#faq_ids: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetExperienceConfigurationContentSourceConfiguration {
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
                "data_source_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_source_ids,
                )
                .await,
            );
            map.insert(
                "direct_put_content".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#direct_put_content,
                )
                .await,
            );
            map.insert(
                "faq_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#faq_ids,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetExperienceConfigurationContentSourceConfiguration {
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
                    r#data_source_ids: {
                        let field_value = match fields_map.get("data_source_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_source_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#direct_put_content: {
                        let field_value = match fields_map.get("direct_put_content") {
                            Some(value) => value,
                            None => bail!("Missing field 'direct_put_content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#faq_ids: {
                        let field_value = match fields_map.get("faq_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'faq_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
