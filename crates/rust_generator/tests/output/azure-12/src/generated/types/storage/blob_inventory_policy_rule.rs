#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BlobInventoryPolicyRule {
    /// A `filter` block as defined above.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<Box<super::super::types::storage::BlobInventoryPolicyRuleFilter>>,
    /// The format of the inventory files. Possible values are `Csv` and `Parquet`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: String,
    /// The name which should be used for this Blob Inventory Policy Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The inventory schedule applied by this rule. Possible values are `Daily` and `Weekly`.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: String,
    /// A list of fields to be included in the inventory. See the [Azure API reference](https://docs.microsoft.com/rest/api/storagerp/blob-inventory-policies/create-or-update#blobinventorypolicydefinition) for all the supported fields.
    #[builder(into)]
    #[serde(rename = "schemaFields")]
    pub r#schema_fields: Vec<String>,
    /// The scope of the inventory for this rule. Possible values are `Blob` and `Container`.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: String,
    /// The storage container name to store the blob inventory files for this rule.
    #[builder(into)]
    #[serde(rename = "storageContainerName")]
    pub r#storage_container_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BlobInventoryPolicyRule {
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
                "filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter,
                )
                .await,
            );
            map.insert(
                "format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#format,
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
                "schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schedule,
                )
                .await,
            );
            map.insert(
                "schema_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_fields,
                )
                .await,
            );
            map.insert(
                "scope".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scope,
                )
                .await,
            );
            map.insert(
                "storage_container_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_container_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BlobInventoryPolicyRule {
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
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#format: {
                        let field_value = match fields_map.get("format") {
                            Some(value) => value,
                            None => bail!("Missing field 'format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#schedule: {
                        let field_value = match fields_map.get("schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_fields: {
                        let field_value = match fields_map.get("schema_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scope: {
                        let field_value = match fields_map.get("scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_container_name: {
                        let field_value = match fields_map.get("storage_container_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_container_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
