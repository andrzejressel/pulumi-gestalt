#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexIndexStatistic {
    /// A block that specifies the number of question and answer topics in the index. Detailed below.
    #[builder(into)]
    #[serde(rename = "faqStatistics")]
    pub r#faq_statistics: Option<Vec<super::super::types::kendra::IndexIndexStatisticFaqStatistic>>,
    /// A block that specifies the number of text documents indexed. Detailed below.
    #[builder(into)]
    #[serde(rename = "textDocumentStatistics")]
    pub r#text_document_statistics: Option<Vec<super::super::types::kendra::IndexIndexStatisticTextDocumentStatistic>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IndexIndexStatistic {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("faq_statistics".to_string(), self.r#faq_statistics.to_pulumi_value().await);
            map.insert("text_document_statistics".to_string(), self.r#text_document_statistics.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IndexIndexStatistic {
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
                    r#faq_statistics: {
                        let field_value = match fields_map.get("faq_statistics") {
                            Some(value) => value,
                            None => bail!("Missing field 'faq_statistics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::kendra::IndexIndexStatisticFaqStatistic>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#text_document_statistics: {
                        let field_value = match fields_map.get("text_document_statistics") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_document_statistics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::kendra::IndexIndexStatisticTextDocumentStatistic>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
