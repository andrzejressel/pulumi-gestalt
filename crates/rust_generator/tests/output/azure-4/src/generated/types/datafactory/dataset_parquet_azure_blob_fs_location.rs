#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatasetParquetAzureBlobFsLocation {
    /// Is the `file_system` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into)]
    pub r#dynamic_file_system_enabled: Option<bool>,
    /// Is the `filename` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into)]
    pub r#dynamic_filename_enabled: Option<bool>,
    /// Is the `path` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into)]
    pub r#dynamic_path_enabled: Option<bool>,
    /// The container on the Azure Data Lake Storage Account hosting the file.
    #[builder(into)]
    pub r#file_system: Option<String>,
    /// The filename of the file on the Azure Data Lake Storage Account.
    #[builder(into)]
    pub r#filename: Option<String>,
    /// The folder path to the file on the Azure Data Lake Storage Account.
    #[builder(into)]
    pub r#path: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatasetParquetAzureBlobFsLocation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "dynamicFileSystemEnabled",
                    &self.r#dynamic_file_system_enabled,
                ),
                to_pulumi_object_field(
                    "dynamicFilenameEnabled",
                    &self.r#dynamic_filename_enabled,
                ),
                to_pulumi_object_field(
                    "dynamicPathEnabled",
                    &self.r#dynamic_path_enabled,
                ),
                to_pulumi_object_field(
                    "fileSystem",
                    &self.r#file_system,
                ),
                to_pulumi_object_field(
                    "filename",
                    &self.r#filename,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatasetParquetAzureBlobFsLocation {
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
                    r#dynamic_file_system_enabled: {
                        let field_value = match fields_map.get("dynamicFileSystemEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamicFileSystemEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_filename_enabled: {
                        let field_value = match fields_map.get("dynamicFilenameEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamicFilenameEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_path_enabled: {
                        let field_value = match fields_map.get("dynamicPathEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamicPathEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_system: {
                        let field_value = match fields_map.get("fileSystem") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileSystem' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
