#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSlotLogsHttpLogs {
    /// A `azure_blob_storage_http` block as defined above.
    #[builder(into)]
    #[serde(rename = "azureBlobStorage")]
    pub r#azure_blob_storage: Option<Box<super::super::types::appservice::WindowsWebAppSlotLogsHttpLogsAzureBlobStorage>>,
    /// A `file_system` block as defined above.
    #[builder(into)]
    #[serde(rename = "fileSystem")]
    pub r#file_system: Option<Box<super::super::types::appservice::WindowsWebAppSlotLogsHttpLogsFileSystem>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsWebAppSlotLogsHttpLogs {
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
                "azure_blob_storage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#azure_blob_storage,
                )
                .await,
            );
            map.insert(
                "file_system".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_system,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsWebAppSlotLogsHttpLogs {
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
                    r#azure_blob_storage: {
                        let field_value = match fields_map.get("azure_blob_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_blob_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_system: {
                        let field_value = match fields_map.get("file_system") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_system' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
