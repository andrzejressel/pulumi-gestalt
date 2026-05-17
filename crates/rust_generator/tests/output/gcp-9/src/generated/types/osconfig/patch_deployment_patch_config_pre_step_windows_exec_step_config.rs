#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentPatchConfigPreStepWindowsExecStepConfig {
    /// Defaults to [0]. A list of possible return values that the execution can return to indicate a success.
    #[builder(into)]
    #[serde(rename = "allowedSuccessCodes")]
    pub r#allowed_success_codes: Option<Vec<i32>>,
    /// A Cloud Storage object containing the executable.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gcsObject")]
    pub r#gcs_object: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPreStepWindowsExecStepConfigGcsObject>>,
    /// The script interpreter to use to run the script. If no interpreter is specified the script will
    /// be executed directly, which will likely only succeed for scripts with shebang lines.
    /// Possible values are: `SHELL`, `POWERSHELL`.
    #[builder(into)]
    #[serde(rename = "interpreter")]
    pub r#interpreter: Option<String>,
    /// An absolute path to the executable on the VM.
    #[builder(into)]
    #[serde(rename = "localPath")]
    pub r#local_path: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PatchDeploymentPatchConfigPreStepWindowsExecStepConfig {
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
                "allowed_success_codes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_success_codes,
                )
                .await,
            );
            map.insert(
                "gcs_object".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcs_object,
                )
                .await,
            );
            map.insert(
                "interpreter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#interpreter,
                )
                .await,
            );
            map.insert(
                "local_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_path,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PatchDeploymentPatchConfigPreStepWindowsExecStepConfig {
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
                    r#allowed_success_codes: {
                        let field_value = match fields_map.get("allowed_success_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_success_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs_object: {
                        let field_value = match fields_map.get("gcs_object") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcs_object' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interpreter: {
                        let field_value = match fields_map.get("interpreter") {
                            Some(value) => value,
                            None => bail!("Missing field 'interpreter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_path: {
                        let field_value = match fields_map.get("local_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
