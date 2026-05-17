#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformation {
    /// Only apply the transformation if the condition evaluates to true for the given RecordCondition. The conditions are allowed to reference fields that are not used in the actual transformation.
    /// Example Use Cases:
    /// - Apply a different bucket transformation to an age column if the zip code column for the same record is within a specific range.
    /// - Redact a field if the date of birth field is greater than 85.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationCondition>>,
    /// Input field(s) to apply the transformation to. When you have columns that reference their position within a list, omit the index from the FieldId.
    /// FieldId name matching ignores the index. For example, instead of "contact.nums[0].type", use "contact.nums.type".
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fields")]
    pub r#fields: Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationField>,
    /// Treat the contents of the field as free text, and selectively transform content that matches an InfoType.
    /// Only one of `primitive_transformation` or `info_type_transformations` must be specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "infoTypeTransformations")]
    pub r#info_type_transformations: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformations>>,
    /// Apply the transformation to the entire field.
    /// The `primitive_transformation` block must only contain one argument, corresponding to the type of transformation.
    /// Only one of `primitive_transformation` or `info_type_transformations` must be specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "primitiveTransformation")]
    pub r#primitive_transformation: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformation>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "condition",
                    &self.r#condition,
                ),
                to_pulumi_object_field(
                    "fields",
                    &self.r#fields,
                ),
                to_pulumi_object_field(
                    "info_type_transformations",
                    &self.r#info_type_transformations,
                ),
                to_pulumi_object_field(
                    "primitive_transformation",
                    &self.r#primitive_transformation,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformation {
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
                    r#condition: {
                        let field_value = match fields_map.get("condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fields: {
                        let field_value = match fields_map.get("fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#info_type_transformations: {
                        let field_value = match fields_map.get("info_type_transformations") {
                            Some(value) => value,
                            None => bail!("Missing field 'info_type_transformations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primitive_transformation: {
                        let field_value = match fields_map.get("primitive_transformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'primitive_transformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
