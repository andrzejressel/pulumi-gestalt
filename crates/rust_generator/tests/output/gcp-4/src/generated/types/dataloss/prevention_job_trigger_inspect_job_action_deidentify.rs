#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobActionDeidentify {
    /// User settable Cloud Storage bucket and folders to store de-identified files.
    /// This field must be set for cloud storage deidentification.
    /// The output Cloud Storage bucket must be different from the input bucket.
    /// De-identified files will overwrite files in the output path.
    /// Form of: gs://bucket/folder/ or gs://bucket
    #[builder(into)]
    #[serde(rename = "cloudStorageOutput")]
    pub r#cloud_storage_output: String,
    /// List of user-specified file type groups to transform. If specified, only the files with these filetypes will be transformed.
    /// If empty, all supported files will be transformed. Supported types may be automatically added over time.
    /// If a file type is set in this field that isn't supported by the Deidentify action then the job will fail and will not be successfully created/started.
    /// Each value may be one of: `IMAGE`, `TEXT_FILE`, `CSV`, `TSV`.
    #[builder(into)]
    #[serde(rename = "fileTypesToTransforms")]
    pub r#file_types_to_transforms: Option<Vec<String>>,
    /// User specified deidentify templates and configs for structured, unstructured, and image files.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "transformationConfig")]
    pub r#transformation_config: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionDeidentifyTransformationConfig>>,
    /// Config for storing transformation details.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "transformationDetailsStorageConfig")]
    pub r#transformation_details_storage_config: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionDeidentifyTransformationDetailsStorageConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobActionDeidentify {
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
                "cloud_storage_output".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_storage_output,
                )
                .await,
            );
            map.insert(
                "file_types_to_transforms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_types_to_transforms,
                )
                .await,
            );
            map.insert(
                "transformation_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transformation_config,
                )
                .await,
            );
            map.insert(
                "transformation_details_storage_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transformation_details_storage_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobActionDeidentify {
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
                    r#cloud_storage_output: {
                        let field_value = match fields_map.get("cloud_storage_output") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_storage_output' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_types_to_transforms: {
                        let field_value = match fields_map.get("file_types_to_transforms") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_types_to_transforms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transformation_config: {
                        let field_value = match fields_map.get("transformation_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'transformation_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transformation_details_storage_config: {
                        let field_value = match fields_map.get("transformation_details_storage_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'transformation_details_storage_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
