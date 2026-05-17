#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions {
    /// Max number of bytes to scan from a file. If a scanned file's size is bigger than this value
    /// then the rest of the bytes are omitted.
    #[builder(into)]
    #[serde(rename = "bytesLimitPerFile")]
    pub r#bytes_limit_per_file: Option<i32>,
    /// Max percentage of bytes to scan from a file. The rest are omitted. The number of bytes scanned is rounded down.
    /// Must be between 0 and 100, inclusively. Both 0 and 100 means no limit.
    #[builder(into)]
    #[serde(rename = "bytesLimitPerFilePercent")]
    pub r#bytes_limit_per_file_percent: Option<i32>,
    /// Set of files to scan.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fileSet")]
    pub r#file_set: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet>,
    /// List of file type groups to include in the scan. If empty, all files are scanned and available data
    /// format processors are applied. In addition, the binary content of the selected files is always scanned as well.
    /// Images are scanned only as binary if the specified region does not support image inspection and no fileTypes were specified.
    /// Each value may be one of: `BINARY_FILE`, `TEXT_FILE`, `IMAGE`, `WORD`, `PDF`, `AVRO`, `CSV`, `TSV`, `POWERPOINT`, `EXCEL`.
    #[builder(into)]
    #[serde(rename = "fileTypes")]
    pub r#file_types: Option<Vec<String>>,
    /// Limits the number of files to scan to this percentage of the input FileSet. Number of files scanned is rounded down.
    /// Must be between 0 and 100, inclusively. Both 0 and 100 means no limit.
    #[builder(into)]
    #[serde(rename = "filesLimitPercent")]
    pub r#files_limit_percent: Option<i32>,
    /// How to sample bytes if not all bytes are scanned. Meaningful only when used in conjunction with bytesLimitPerFile.
    /// If not specified, scanning would start from the top.
    /// Possible values are: `TOP`, `RANDOM_START`.
    #[builder(into)]
    #[serde(rename = "sampleMethod")]
    pub r#sample_method: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions {
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
                "bytes_limit_per_file".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bytes_limit_per_file,
                )
                .await,
            );
            map.insert(
                "bytes_limit_per_file_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bytes_limit_per_file_percent,
                )
                .await,
            );
            map.insert(
                "file_set".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_set,
                )
                .await,
            );
            map.insert(
                "file_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_types,
                )
                .await,
            );
            map.insert(
                "files_limit_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#files_limit_percent,
                )
                .await,
            );
            map.insert(
                "sample_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sample_method,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions {
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
                    r#bytes_limit_per_file: {
                        let field_value = match fields_map.get("bytes_limit_per_file") {
                            Some(value) => value,
                            None => bail!("Missing field 'bytes_limit_per_file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bytes_limit_per_file_percent: {
                        let field_value = match fields_map.get("bytes_limit_per_file_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'bytes_limit_per_file_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_set: {
                        let field_value = match fields_map.get("file_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_types: {
                        let field_value = match fields_map.get("file_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#files_limit_percent: {
                        let field_value = match fields_map.get("files_limit_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'files_limit_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_method: {
                        let field_value = match fields_map.get("sample_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
