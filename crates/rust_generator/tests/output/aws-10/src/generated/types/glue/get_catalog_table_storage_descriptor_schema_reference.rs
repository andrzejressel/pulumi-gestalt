#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCatalogTableStorageDescriptorSchemaReference {
    /// Configuration block that contains schema identity fields. See `schema_id` below.
    #[builder(into)]
    #[serde(rename = "schemaIds")]
    pub r#schema_ids: Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSchemaReferenceSchemaId>,
    /// Unique ID assigned to a version of the schema.
    #[builder(into)]
    #[serde(rename = "schemaVersionId")]
    pub r#schema_version_id: String,
    /// Version number of the schema.
    #[builder(into)]
    #[serde(rename = "schemaVersionNumber")]
    pub r#schema_version_number: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCatalogTableStorageDescriptorSchemaReference {
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
                "schema_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_ids,
                )
                .await,
            );
            map.insert(
                "schema_version_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_version_id,
                )
                .await,
            );
            map.insert(
                "schema_version_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_version_number,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCatalogTableStorageDescriptorSchemaReference {
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
                    r#schema_ids: {
                        let field_value = match fields_map.get("schema_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_version_id: {
                        let field_value = match fields_map.get("schema_version_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_version_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_version_number: {
                        let field_value = match fields_map.get("schema_version_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_version_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
