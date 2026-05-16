#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationInlineConfiguration {
    /// Configuration of the condition used for the target document attribute or metadata field when ingesting documents into Amazon Kendra. See condition.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationCondition>>,
    /// `TRUE` to delete content if the condition used for the target attribute is met.
    #[builder(into)]
    #[serde(rename = "documentContentDeletion")]
    pub r#document_content_deletion: Option<bool>,
    /// Configuration of the target document attribute or metadata field when ingesting documents into Amazon Kendra. You can also include a value. Detailed below.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTarget>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceCustomDocumentEnrichmentConfigurationInlineConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("condition".to_string(), self.r#condition.to_pulumi_value().await);
            map.insert("document_content_deletion".to_string(), self.r#document_content_deletion.to_pulumi_value().await);
            map.insert("target".to_string(), self.r#target.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceCustomDocumentEnrichmentConfigurationInlineConfiguration {
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
                    r#condition: {
                        let field_value = match fields_map.get("condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationCondition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#document_content_deletion: {
                        let field_value = match fields_map.get("document_content_deletion") {
                            Some(value) => value,
                            None => bail!("Missing field 'document_content_deletion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTarget>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
