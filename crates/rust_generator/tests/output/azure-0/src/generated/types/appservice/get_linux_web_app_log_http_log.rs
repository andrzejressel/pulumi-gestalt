#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxWebAppLogHttpLog {
    /// A `azure_blob_storage` block as defined above.
    #[builder(into)]
    #[serde(rename = "azureBlobStorages")]
    pub r#azure_blob_storages: Vec<super::super::types::appservice::GetLinuxWebAppLogHttpLogAzureBlobStorage>,
    /// A `file_system` block as defined above.
    #[builder(into)]
    #[serde(rename = "fileSystems")]
    pub r#file_systems: Vec<super::super::types::appservice::GetLinuxWebAppLogHttpLogFileSystem>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinuxWebAppLogHttpLog {
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
                "azure_blob_storages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#azure_blob_storages,
                )
                .await,
            );
            map.insert(
                "file_systems".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_systems,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinuxWebAppLogHttpLog {
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
                    r#azure_blob_storages: {
                        let field_value = match fields_map.get("azure_blob_storages") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_blob_storages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_systems: {
                        let field_value = match fields_map.get("file_systems") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_systems' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
