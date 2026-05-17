#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidate {
    /// Optional arguments to pass to the source during
    /// execution.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// A remote or local file. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidateFile>>,
    /// The script interpreter to use. Possible values
    /// are: `INTERPRETER_UNSPECIFIED`, `NONE`, `SHELL`, `POWERSHELL`.
    #[builder(into)]
    #[serde(rename = "interpreter")]
    pub r#interpreter: String,
    /// Only recorded for enforce Exec. Path to an
    /// output file (that is created by this Exec) whose content will be recorded in
    /// OSPolicyResourceCompliance after a successful run. Absence or failure to
    /// read this file will result in this ExecResource being non-compliant. Output
    /// file size is limited to 100K bytes.
    #[builder(into)]
    #[serde(rename = "outputFilePath")]
    pub r#output_file_path: Option<String>,
    /// An inline script. The size of the script is limited to
    /// 1024 characters.
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidate {
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
                "args".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#args,
                )
                .await,
            );
            map.insert(
                "file".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file,
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
                "output_file_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_file_path,
                )
                .await,
            );
            map.insert(
                "script".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#script,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidate {
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
                    r#args: {
                        let field_value = match fields_map.get("args") {
                            Some(value) => value,
                            None => bail!("Missing field 'args' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file: {
                        let field_value = match fields_map.get("file") {
                            Some(value) => value,
                            None => bail!("Missing field 'file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#output_file_path: {
                        let field_value = match fields_map.get("output_file_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_file_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script: {
                        let field_value = match fields_map.get("script") {
                            Some(value) => value,
                            None => bail!("Missing field 'script' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
