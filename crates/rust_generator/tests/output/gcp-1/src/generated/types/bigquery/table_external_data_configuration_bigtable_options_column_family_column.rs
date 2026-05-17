#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableExternalDataConfigurationBigtableOptionsColumnFamilyColumn {
    /// The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. 'encoding' can also be set at the column family level. However, the setting at this level takes precedence if 'encoding' is set at both levels.
    #[builder(into)]
    #[serde(rename = "encoding")]
    pub r#encoding: Option<String>,
    /// If the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as the column field name and is used as field name in queries.
    #[builder(into)]
    #[serde(rename = "fieldName")]
    pub r#field_name: Option<String>,
    /// If this is set, only the latest version of value in this column are exposed. 'onlyReadLatest' can also be set at the column family level. However, the setting at this level takes precedence if 'onlyReadLatest' is set at both levels.
    #[builder(into)]
    #[serde(rename = "onlyReadLatest")]
    pub r#only_read_latest: Option<bool>,
    /// Qualifier of the column. Columns in the parent column family that has this exact qualifier are exposed as . field. If the qualifier is valid UTF-8 string, it can be specified in the qualifierString field. Otherwise, a base-64 encoded value must be set to qualifierEncoded. The column field name is the same as the column qualifier. However, if the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as fieldName.
    #[builder(into)]
    #[serde(rename = "qualifierEncoded")]
    pub r#qualifier_encoded: Option<String>,
    /// Qualifier string.
    #[builder(into)]
    #[serde(rename = "qualifierString")]
    pub r#qualifier_string: Option<String>,
    /// The type to convert the value in cells of this column. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive): "BYTES", "STRING", "INTEGER", "FLOAT", "BOOLEAN", "JSON", Default type is "BYTES". 'type' can also be set at the column family level. However, the setting at this level takes precedence if 'type' is set at both levels.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableExternalDataConfigurationBigtableOptionsColumnFamilyColumn {
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
                "encoding".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encoding,
                )
                .await,
            );
            map.insert(
                "field_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#field_name,
                )
                .await,
            );
            map.insert(
                "only_read_latest".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#only_read_latest,
                )
                .await,
            );
            map.insert(
                "qualifier_encoded".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qualifier_encoded,
                )
                .await,
            );
            map.insert(
                "qualifier_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qualifier_string,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableExternalDataConfigurationBigtableOptionsColumnFamilyColumn {
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
                    r#encoding: {
                        let field_value = match fields_map.get("encoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'encoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field_name: {
                        let field_value = match fields_map.get("field_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#only_read_latest: {
                        let field_value = match fields_map.get("only_read_latest") {
                            Some(value) => value,
                            None => bail!("Missing field 'only_read_latest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qualifier_encoded: {
                        let field_value = match fields_map.get("qualifier_encoded") {
                            Some(value) => value,
                            None => bail!("Missing field 'qualifier_encoded' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qualifier_string: {
                        let field_value = match fields_map.get("qualifier_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'qualifier_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
