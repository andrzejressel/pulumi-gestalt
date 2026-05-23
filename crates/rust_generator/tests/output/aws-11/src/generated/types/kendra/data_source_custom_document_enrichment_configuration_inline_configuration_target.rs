#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTarget {
    /// The identifier of the target document attribute or metadata field. For example, 'Department' could be an identifier for the target attribute or metadata field that includes the department names associated with the documents.
    #[builder(into)]
    pub r#target_document_attribute_key: Option<String>,
    /// The target value you want to create for the target attribute. For example, 'Finance' could be the target value for the target attribute key 'Department'. See target_document_attribute_value.
    #[builder(into)]
    pub r#target_document_attribute_value: Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTargetTargetDocumentAttributeValue>>,
    /// `TRUE` to delete the existing target value for your specified target attribute key. You cannot create a target value and set this to `TRUE`. To create a target value (`TargetDocumentAttributeValue`), set this to `FALSE`.
    #[builder(into)]
    pub r#target_document_attribute_value_deletion: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTarget {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "targetDocumentAttributeKey",
                    &self.r#target_document_attribute_key,
                ),
                to_pulumi_object_field(
                    "targetDocumentAttributeValue",
                    &self.r#target_document_attribute_value,
                ),
                to_pulumi_object_field(
                    "targetDocumentAttributeValueDeletion",
                    &self.r#target_document_attribute_value_deletion,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTarget {
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
                    r#target_document_attribute_key: {
                        let field_value = match fields_map.get("targetDocumentAttributeKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetDocumentAttributeKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_document_attribute_value: {
                        let field_value = match fields_map.get("targetDocumentAttributeValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetDocumentAttributeValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_document_attribute_value_deletion: {
                        let field_value = match fields_map.get("targetDocumentAttributeValueDeletion") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetDocumentAttributeValueDeletion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
