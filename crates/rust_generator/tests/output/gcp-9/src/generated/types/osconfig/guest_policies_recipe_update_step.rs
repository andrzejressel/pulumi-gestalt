#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipeUpdateStep {
    /// Extracts an archive into the specified directory.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "archiveExtraction")]
    pub r#archive_extraction: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeUpdateStepArchiveExtraction>>,
    /// Installs a deb file via dpkg.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dpkgInstallation")]
    pub r#dpkg_installation: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeUpdateStepDpkgInstallation>>,
    /// Copies a file onto the instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fileCopy")]
    pub r#file_copy: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeUpdateStepFileCopy>>,
    /// Executes an artifact or local file.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fileExec")]
    pub r#file_exec: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeUpdateStepFileExec>>,
    /// Installs an MSI file.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "msiInstallation")]
    pub r#msi_installation: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeUpdateStepMsiInstallation>>,
    /// Installs an rpm file via the rpm utility.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rpmInstallation")]
    pub r#rpm_installation: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeUpdateStepRpmInstallation>>,
    /// Runs commands in a shell.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scriptRun")]
    pub r#script_run: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeUpdateStepScriptRun>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuestPoliciesRecipeUpdateStep {
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
                    "archiveExtraction",
                    &self.r#archive_extraction,
                ),
                to_pulumi_object_field(
                    "dpkgInstallation",
                    &self.r#dpkg_installation,
                ),
                to_pulumi_object_field(
                    "fileCopy",
                    &self.r#file_copy,
                ),
                to_pulumi_object_field(
                    "fileExec",
                    &self.r#file_exec,
                ),
                to_pulumi_object_field(
                    "msiInstallation",
                    &self.r#msi_installation,
                ),
                to_pulumi_object_field(
                    "rpmInstallation",
                    &self.r#rpm_installation,
                ),
                to_pulumi_object_field(
                    "scriptRun",
                    &self.r#script_run,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuestPoliciesRecipeUpdateStep {
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
                    r#archive_extraction: {
                        let field_value = match fields_map.get("archiveExtraction") {
                            Some(value) => value,
                            None => bail!("Missing field 'archiveExtraction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dpkg_installation: {
                        let field_value = match fields_map.get("dpkgInstallation") {
                            Some(value) => value,
                            None => bail!("Missing field 'dpkgInstallation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_copy: {
                        let field_value = match fields_map.get("fileCopy") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileCopy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_exec: {
                        let field_value = match fields_map.get("fileExec") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileExec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#msi_installation: {
                        let field_value = match fields_map.get("msiInstallation") {
                            Some(value) => value,
                            None => bail!("Missing field 'msiInstallation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rpm_installation: {
                        let field_value = match fields_map.get("rpmInstallation") {
                            Some(value) => value,
                            None => bail!("Missing field 'rpmInstallation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script_run: {
                        let field_value = match fields_map.get("scriptRun") {
                            Some(value) => value,
                            None => bail!("Missing field 'scriptRun' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
