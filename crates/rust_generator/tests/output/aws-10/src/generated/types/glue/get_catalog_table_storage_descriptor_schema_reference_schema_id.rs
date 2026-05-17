#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCatalogTableStorageDescriptorSchemaReferenceSchemaId {
    /// Name of the schema registry that contains the schema.
    #[builder(into)]
    #[serde(rename = "registryName")]
    pub r#registry_name: String,
    /// ARN of the schema.
    #[builder(into)]
    #[serde(rename = "schemaArn")]
    pub r#schema_arn: String,
    /// Name of the schema.
    #[builder(into)]
    #[serde(rename = "schemaName")]
    pub r#schema_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCatalogTableStorageDescriptorSchemaReferenceSchemaId {
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
                "registry_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#registry_name,
                )
                .await,
            );
            map.insert(
                "schema_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_arn,
                )
                .await,
            );
            map.insert(
                "schema_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCatalogTableStorageDescriptorSchemaReferenceSchemaId {
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
                    r#registry_name: {
                        let field_value = match fields_map.get("registry_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'registry_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_arn: {
                        let field_value = match fields_map.get("schema_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_name: {
                        let field_value = match fields_map.get("schema_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
