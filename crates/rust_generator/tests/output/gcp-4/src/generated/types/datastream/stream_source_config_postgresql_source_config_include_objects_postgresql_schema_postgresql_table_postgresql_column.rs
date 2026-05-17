#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigPostgresqlSourceConfigIncludeObjectsPostgresqlSchemaPostgresqlTablePostgresqlColumn {
    /// Column name.
    #[builder(into)]
    #[serde(rename = "column")]
    pub r#column: Option<String>,
    /// The PostgreSQL data type. Full data types list can be found here:
    /// https://www.postgresql.org/docs/current/datatype.html
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
    /// (Output)
    /// Column precision.
    #[builder(into)]
    #[serde(rename = "precision")]
    pub r#precision: Option<i32>,
    /// Whether or not the column represents a primary key.
    #[builder(into)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Option<bool>,
    /// (Output)
    /// Column scale.
    #[builder(into)]
    #[serde(rename = "scale")]
    pub r#scale: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamSourceConfigPostgresqlSourceConfigIncludeObjectsPostgresqlSchemaPostgresqlTablePostgresqlColumn {
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
                "column".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#column,
                )
                .await,
            );
            map.insert(
                "data_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_type,
                )
                .await,
            );
            map.insert(
                "length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#length,
                )
                .await,
            );
            map.insert(
                "nullable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nullable,
                )
                .await,
            );
            map.insert(
                "ordinal_position".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ordinal_position,
                )
                .await,
            );
            map.insert(
                "precision".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#precision,
                )
                .await,
            );
            map.insert(
                "primary_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primary_key,
                )
                .await,
            );
            map.insert(
                "scale".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scale,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamSourceConfigPostgresqlSourceConfigIncludeObjectsPostgresqlSchemaPostgresqlTablePostgresqlColumn {
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
                    r#precision: {
                        let field_value = match fields_map.get("precision") {
                            Some(value) => value,
                            None => bail!("Missing field 'precision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#scale: {
                        let field_value = match fields_map.get("scale") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
