#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamBackfillAllMysqlExcludedObjectsMysqlDatabaseMysqlTableMysqlColumn {
    /// Column collation.
    #[builder(into)]
    #[serde(rename = "collation")]
    pub r#collation: Option<String>,
    /// Column name.
    #[builder(into)]
    #[serde(rename = "column")]
    pub r#column: Option<String>,
    /// The MySQL data type. Full data types list can be found here:
    /// https://dev.mysql.com/doc/refman/8.0/en/data-types.html
    #[builder(into)]
    #[serde(rename = "dataType")]
    pub r#data_type: Option<String>,
    /// (Output)
    /// Column length.
    #[builder(into)]
    #[serde(rename = "length")]
    pub r#length: Option<i32>,
    /// Whether or not the column can accept a null value.
    #[builder(into)]
    #[serde(rename = "nullable")]
    pub r#nullable: Option<bool>,
    /// The ordinal position of the column in the table.
    #[builder(into)]
    #[serde(rename = "ordinalPosition")]
    pub r#ordinal_position: Option<i32>,
    /// Whether or not the column represents a primary key.
    #[builder(into)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamBackfillAllMysqlExcludedObjectsMysqlDatabaseMysqlTableMysqlColumn {
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
                    "collation",
                    &self.r#collation,
                ),
                to_pulumi_object_field(
                    "column",
                    &self.r#column,
                ),
                to_pulumi_object_field(
                    "data_type",
                    &self.r#data_type,
                ),
                to_pulumi_object_field(
                    "length",
                    &self.r#length,
                ),
                to_pulumi_object_field(
                    "nullable",
                    &self.r#nullable,
                ),
                to_pulumi_object_field(
                    "ordinal_position",
                    &self.r#ordinal_position,
                ),
                to_pulumi_object_field(
                    "primary_key",
                    &self.r#primary_key,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamBackfillAllMysqlExcludedObjectsMysqlDatabaseMysqlTableMysqlColumn {
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
                    r#collation: {
                        let field_value = match fields_map.get("collation") {
                            Some(value) => value,
                            None => bail!("Missing field 'collation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#column: {
                        let field_value = match fields_map.get("column") {
                            Some(value) => value,
                            None => bail!("Missing field 'column' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_type: {
                        let field_value = match fields_map.get("data_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#length: {
                        let field_value = match fields_map.get("length") {
                            Some(value) => value,
                            None => bail!("Missing field 'length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nullable: {
                        let field_value = match fields_map.get("nullable") {
                            Some(value) => value,
                            None => bail!("Missing field 'nullable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ordinal_position: {
                        let field_value = match fields_map.get("ordinal_position") {
                            Some(value) => value,
                            None => bail!("Missing field 'ordinal_position' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_key: {
                        let field_value = match fields_map.get("primary_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
