#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentPatchConfigPreStep {
    /// The ExecStepConfig for all Linux VMs targeted by the PatchJob.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "linuxExecStepConfig")]
    pub r#linux_exec_step_config: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPreStepLinuxExecStepConfig>>,
    /// The ExecStepConfig for all Windows VMs targeted by the PatchJob.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "windowsExecStepConfig")]
    pub r#windows_exec_step_config: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPreStepWindowsExecStepConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PatchDeploymentPatchConfigPreStep {
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
                "linux_exec_step_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linux_exec_step_config,
                )
                .await,
            );
            map.insert(
                "windows_exec_step_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#windows_exec_step_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PatchDeploymentPatchConfigPreStep {
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
                    r#linux_exec_step_config: {
                        let field_value = match fields_map.get("linux_exec_step_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'linux_exec_step_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows_exec_step_config: {
                        let field_value = match fields_map.get("windows_exec_step_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'windows_exec_step_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
