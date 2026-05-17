#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriptionCostManagementExportExportDataStorageLocation {
    /// The Resource Manager ID of the container where exports will be uploaded. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "containerId")]
    pub r#container_id: String,
    /// The path of the directory where exports will be uploaded. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** The Resource Manager ID of a Storage Container is exposed via the `resource_manager_id` attribute of the `azure.storage.Container` resource.
    #[builder(into)]
    #[serde(rename = "rootFolderPath")]
    pub r#root_folder_path: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriptionCostManagementExportExportDataStorageLocation {
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
                "container_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_id,
                )
                .await,
            );
            map.insert(
                "root_folder_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_folder_path,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriptionCostManagementExportExportDataStorageLocation {
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
                    r#container_id: {
                        let field_value = match fields_map.get("container_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_folder_path: {
                        let field_value = match fields_map.get("root_folder_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_folder_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
