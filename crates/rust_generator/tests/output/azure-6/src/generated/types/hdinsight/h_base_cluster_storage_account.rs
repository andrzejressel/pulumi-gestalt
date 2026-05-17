#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HBaseClusterStorageAccount {
    /// Is this the Default Storage Account for the HDInsight Hadoop Cluster? Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** One of the `storage_account` or `storage_account_gen2` blocks must be marked as the default.
    #[builder(into)]
    #[serde(rename = "isDefault")]
    pub r#is_default: bool,
    /// The Access Key which should be used to connect to the Storage Account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountKey")]
    pub r#storage_account_key: String,
    /// The ID of the Storage Container. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** This can be obtained from the `id` of the `azure.storage.Container` resource.
    #[builder(into)]
    #[serde(rename = "storageContainerId")]
    pub r#storage_container_id: String,
    /// The ID of the Storage Account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageResourceId")]
    pub r#storage_resource_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HBaseClusterStorageAccount {
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
                "is_default".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_default,
                )
                .await,
            );
            map.insert(
                "storage_account_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_account_key,
                )
                .await,
            );
            map.insert(
                "storage_container_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_container_id,
                )
                .await,
            );
            map.insert(
                "storage_resource_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_resource_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HBaseClusterStorageAccount {
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
                    r#is_default: {
                        let field_value = match fields_map.get("is_default") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_default' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_key: {
                        let field_value = match fields_map.get("storage_account_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_container_id: {
                        let field_value = match fields_map.get("storage_container_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_container_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_resource_id: {
                        let field_value = match fields_map.get("storage_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
