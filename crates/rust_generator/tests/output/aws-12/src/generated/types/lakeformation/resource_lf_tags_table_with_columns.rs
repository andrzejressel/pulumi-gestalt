#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceLfTagsTableWithColumns {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Option<String>,
    /// Set of column names for the table.
    #[builder(into)]
    #[serde(rename = "columnNames")]
    pub r#column_names: Option<Vec<String>>,
    /// Name of the database for the table with columns resource. Unique to the Data Catalog.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// Set of column names for the table to exclude. If `excluded_column_names` is included, `wildcard` must be set to `true` to avoid the provider reporting a difference.
    #[builder(into)]
    #[serde(rename = "excludedColumnNames")]
    pub r#excluded_column_names: Option<Vec<String>>,
    /// Name of the table resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Whether to use a column wildcard. If `excluded_column_names` is included, `wildcard` must be set to `true` to avoid the provider reporting a difference.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "wildcard")]
    pub r#wildcard: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResourceLfTagsTableWithColumns {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "catalog_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#catalog_id,
                )
                .await,
            );
            map.insert(
                "column_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#column_names,
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
                "excluded_column_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_column_names,
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
                "wildcard".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#wildcard,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResourceLfTagsTableWithColumns {
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
                    r#catalog_id: {
                        let field_value = match fields_map.get("catalog_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'catalog_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#column_names: {
                        let field_value = match fields_map.get("column_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'column_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#excluded_column_names: {
                        let field_value = match fields_map.get("excluded_column_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_column_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#wildcard: {
                        let field_value = match fields_map.get("wildcard") {
                            Some(value) => value,
                            None => bail!("Missing field 'wildcard' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
