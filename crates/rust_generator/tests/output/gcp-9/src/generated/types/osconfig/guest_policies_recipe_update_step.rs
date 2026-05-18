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
                    "archive_extraction",
                    &self.r#archive_extraction,
                ),
                to_pulumi_object_field(
                    "dpkg_installation",
                    &self.r#dpkg_installation,
                ),
                to_pulumi_object_field(
                    "file_copy",
                    &self.r#file_copy,
                ),
                to_pulumi_object_field(
                    "file_exec",
                    &self.r#file_exec,
                ),
                to_pulumi_object_field(
                    "msi_installation",
                    &self.r#msi_installation,
                ),
                to_pulumi_object_field(
                    "rpm_installation",
                    &self.r#rpm_installation,
                ),
                to_pulumi_object_field(
                    "script_run",
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
                        let field_value = match fields_map.get("archive_extraction") {
                            Some(value) => value,
                            None => bail!("Missing field 'archive_extraction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dpkg_installation: {
                        let field_value = match fields_map.get("dpkg_installation") {
                            Some(value) => value,
                            None => bail!("Missing field 'dpkg_installation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_copy: {
                        let field_value = match fields_map.get("file_copy") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_copy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_exec: {
                        let field_value = match fields_map.get("file_exec") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_exec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#msi_installation: {
                        let field_value = match fields_map.get("msi_installation") {
                            Some(value) => value,
                            None => bail!("Missing field 'msi_installation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rpm_installation: {
                        let field_value = match fields_map.get("rpm_installation") {
                            Some(value) => value,
                            None => bail!("Missing field 'rpm_installation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script_run: {
                        let field_value = match fields_map.get("script_run") {
                            Some(value) => value,
                            None => bail!("Missing field 'script_run' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
