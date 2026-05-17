#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatasetBinaryAzureBlobStorageLocation {
    /// The container on the Azure Blob Storage Account hosting the file.
    #[builder(into)]
    #[serde(rename = "container")]
    pub r#container: String,
    /// Is the `container` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "dynamicContainerEnabled")]
    pub r#dynamic_container_enabled: Option<bool>,
    /// Is the `filename` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "dynamicFilenameEnabled")]
    pub r#dynamic_filename_enabled: Option<bool>,
    /// Is the `path` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "dynamicPathEnabled")]
    pub r#dynamic_path_enabled: Option<bool>,
    /// The filename of the file in the blob container.
    #[builder(into)]
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// The folder path to the file in the blob container.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatasetBinaryAzureBlobStorageLocation {
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
                "container".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container,
                )
                .await,
            );
            map.insert(
                "dynamic_container_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dynamic_container_enabled,
                )
                .await,
            );
            map.insert(
                "dynamic_filename_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dynamic_filename_enabled,
                )
                .await,
            );
            map.insert(
                "dynamic_path_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dynamic_path_enabled,
                )
                .await,
            );
            map.insert(
                "filename".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filename,
                )
                .await,
            );
            map.insert(
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatasetBinaryAzureBlobStorageLocation {
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
                    r#container: {
                        let field_value = match fields_map.get("container") {
                            Some(value) => value,
                            None => bail!("Missing field 'container' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_container_enabled: {
                        let field_value = match fields_map.get("dynamic_container_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_container_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_filename_enabled: {
                        let field_value = match fields_map.get("dynamic_filename_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_filename_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_path_enabled: {
                        let field_value = match fields_map.get("dynamic_path_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_path_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filename: {
                        let field_value = match fields_map.get("filename") {
                            Some(value) => value,
                            None => bail!("Missing field 'filename' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
