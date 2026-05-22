#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetLogicalTableMapDataTransform {
    /// A transform operation that casts a column to a different type. See cast_column_type_operation.
    #[builder(into)]
    #[serde(rename = "castColumnTypeOperation")]
    pub r#cast_column_type_operation: Option<Box<super::super::types::quicksight::DataSetLogicalTableMapDataTransformCastColumnTypeOperation>>,
    /// An operation that creates calculated columns. Columns created in one such operation form a lexical closure. See create_columns_operation.
    #[builder(into)]
    #[serde(rename = "createColumnsOperation")]
    pub r#create_columns_operation: Option<Box<super::super::types::quicksight::DataSetLogicalTableMapDataTransformCreateColumnsOperation>>,
    /// An operation that filters rows based on some condition. See filter_operation.
    #[builder(into)]
    #[serde(rename = "filterOperation")]
    pub r#filter_operation: Option<Box<super::super::types::quicksight::DataSetLogicalTableMapDataTransformFilterOperation>>,
    /// An operation that projects columns. Operations that come after a projection can only refer to projected columns. See project_operation.
    #[builder(into)]
    #[serde(rename = "projectOperation")]
    pub r#project_operation: Option<Box<super::super::types::quicksight::DataSetLogicalTableMapDataTransformProjectOperation>>,
    /// An operation that renames a column. See rename_column_operation.
    #[builder(into)]
    #[serde(rename = "renameColumnOperation")]
    pub r#rename_column_operation: Option<Box<super::super::types::quicksight::DataSetLogicalTableMapDataTransformRenameColumnOperation>>,
    /// An operation that tags a column with additional information. See tag_column_operation.
    #[builder(into)]
    #[serde(rename = "tagColumnOperation")]
    pub r#tag_column_operation: Option<Box<super::super::types::quicksight::DataSetLogicalTableMapDataTransformTagColumnOperation>>,
    /// A transform operation that removes tags associated with a column. See untag_column_operation.
    #[builder(into)]
    #[serde(rename = "untagColumnOperation")]
    pub r#untag_column_operation: Option<Box<super::super::types::quicksight::DataSetLogicalTableMapDataTransformUntagColumnOperation>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSetLogicalTableMapDataTransform {
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
                    "castColumnTypeOperation",
                    &self.r#cast_column_type_operation,
                ),
                to_pulumi_object_field(
                    "createColumnsOperation",
                    &self.r#create_columns_operation,
                ),
                to_pulumi_object_field(
                    "filterOperation",
                    &self.r#filter_operation,
                ),
                to_pulumi_object_field(
                    "projectOperation",
                    &self.r#project_operation,
                ),
                to_pulumi_object_field(
                    "renameColumnOperation",
                    &self.r#rename_column_operation,
                ),
                to_pulumi_object_field(
                    "tagColumnOperation",
                    &self.r#tag_column_operation,
                ),
                to_pulumi_object_field(
                    "untagColumnOperation",
                    &self.r#untag_column_operation,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSetLogicalTableMapDataTransform {
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
                    r#cast_column_type_operation: {
                        let field_value = match fields_map.get("castColumnTypeOperation") {
                            Some(value) => value,
                            None => bail!("Missing field 'castColumnTypeOperation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_columns_operation: {
                        let field_value = match fields_map.get("createColumnsOperation") {
                            Some(value) => value,
                            None => bail!("Missing field 'createColumnsOperation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_operation: {
                        let field_value = match fields_map.get("filterOperation") {
                            Some(value) => value,
                            None => bail!("Missing field 'filterOperation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project_operation: {
                        let field_value = match fields_map.get("projectOperation") {
                            Some(value) => value,
                            None => bail!("Missing field 'projectOperation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rename_column_operation: {
                        let field_value = match fields_map.get("renameColumnOperation") {
                            Some(value) => value,
                            None => bail!("Missing field 'renameColumnOperation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_column_operation: {
                        let field_value = match fields_map.get("tagColumnOperation") {
                            Some(value) => value,
                            None => bail!("Missing field 'tagColumnOperation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#untag_column_operation: {
                        let field_value = match fields_map.get("untagColumnOperation") {
                            Some(value) => value,
                            None => bail!("Missing field 'untagColumnOperation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
