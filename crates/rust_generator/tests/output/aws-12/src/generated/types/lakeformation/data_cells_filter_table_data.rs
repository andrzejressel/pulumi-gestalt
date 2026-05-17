#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataCellsFilterTableData {
    /// A list of column names and/or nested column attributes.
    #[builder(into)]
    #[serde(rename = "columnNames")]
    pub r#column_names: Option<Vec<String>>,
    /// A wildcard with exclusions. See Column Wildcard below for details.
    #[builder(into)]
    #[serde(rename = "columnWildcard")]
    pub r#column_wildcard: Option<Box<super::super::types::lakeformation::DataCellsFilterTableDataColumnWildcard>>,
    /// The name of the database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// The name of the data cells filter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A PartiQL predicate. See Row Filter below for details.
    #[builder(into)]
    #[serde(rename = "rowFilter")]
    pub r#row_filter: Option<Box<super::super::types::lakeformation::DataCellsFilterTableDataRowFilter>>,
    /// The ID of the Data Catalog.
    #[builder(into)]
    #[serde(rename = "tableCatalogId")]
    pub r#table_catalog_id: String,
    /// The name of the table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
    /// ID of the data cells filter version.
    #[builder(into)]
    #[serde(rename = "versionId")]
    pub r#version_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataCellsFilterTableData {
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
                "column_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#column_names,
                )
                .await,
            );
            map.insert(
                "column_wildcard".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#column_wildcard,
                )
                .await,
            );
            map.insert(
                "database_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_name,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "row_filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#row_filter,
                )
                .await,
            );
            map.insert(
                "table_catalog_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table_catalog_id,
                )
                .await,
            );
            map.insert(
                "table_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table_name,
                )
                .await,
            );
            map.insert(
                "version_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataCellsFilterTableData {
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
                    r#column_names: {
                        let field_value = match fields_map.get("column_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'column_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#column_wildcard: {
                        let field_value = match fields_map.get("column_wildcard") {
                            Some(value) => value,
                            None => bail!("Missing field 'column_wildcard' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_name: {
                        let field_value = match fields_map.get("database_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#row_filter: {
                        let field_value = match fields_map.get("row_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'row_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_catalog_id: {
                        let field_value = match fields_map.get("table_catalog_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_catalog_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_name: {
                        let field_value = match fields_map.get("table_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_id: {
                        let field_value = match fields_map.get("version_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
