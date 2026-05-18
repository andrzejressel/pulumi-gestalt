#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionEnumTypeOptions {
    /// List of possible enum values.
    #[builder(into)]
    #[serde(rename = "possibleValues")]
    pub r#possible_values: Vec<String>,
    /// Make sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "validationCheckDisabled")]
    pub r#validation_check_disabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionEnumTypeOptions {
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
                    "possible_values",
                    &self.r#possible_values,
                ),
                to_pulumi_object_field(
                    "validation_check_disabled",
                    &self.r#validation_check_disabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionEnumTypeOptions {
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
                    r#possible_values: {
                        let field_value = match fields_map.get("possible_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'possible_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validation_check_disabled: {
                        let field_value = match fields_map.get("validation_check_disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'validation_check_disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
