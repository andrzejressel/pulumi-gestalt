#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentPatchConfig {
    /// Apt update settings. Use this setting to override the default apt patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "apt")]
    pub r#apt: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigApt>>,
    /// goo update settings. Use this setting to override the default goo patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "goo")]
    pub r#goo: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigGoo>>,
    /// Allows the patch job to run on Managed instance groups (MIGs).
    #[builder(into)]
    #[serde(rename = "migInstancesAllowed")]
    pub r#mig_instances_allowed: Option<bool>,
    /// The ExecStep to run after the patch update.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postStep")]
    pub r#post_step: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPostStep>>,
    /// The ExecStep to run before the patch update.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "preStep")]
    pub r#pre_step: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPreStep>>,
    /// Post-patch reboot settings.
    /// Possible values are: `DEFAULT`, `ALWAYS`, `NEVER`.
    #[builder(into)]
    #[serde(rename = "rebootConfig")]
    pub r#reboot_config: Option<String>,
    /// Windows update settings. Use this setting to override the default Windows patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "windowsUpdate")]
    pub r#windows_update: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigWindowsUpdate>>,
    /// Yum update settings. Use this setting to override the default yum patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "yum")]
    pub r#yum: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigYum>>,
    /// zypper update settings. Use this setting to override the default zypper patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "zypper")]
    pub r#zypper: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigZypper>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PatchDeploymentPatchConfig {
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
                "apt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apt,
                )
                .await,
            );
            map.insert(
                "goo".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#goo,
                )
                .await,
            );
            map.insert(
                "mig_instances_allowed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mig_instances_allowed,
                )
                .await,
            );
            map.insert(
                "post_step".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#post_step,
                )
                .await,
            );
            map.insert(
                "pre_step".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pre_step,
                )
                .await,
            );
            map.insert(
                "reboot_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reboot_config,
                )
                .await,
            );
            map.insert(
                "windows_update".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#windows_update,
                )
                .await,
            );
            map.insert(
                "yum".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#yum,
                )
                .await,
            );
            map.insert(
                "zypper".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zypper,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PatchDeploymentPatchConfig {
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
                    r#apt: {
                        let field_value = match fields_map.get("apt") {
                            Some(value) => value,
                            None => bail!("Missing field 'apt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#goo: {
                        let field_value = match fields_map.get("goo") {
                            Some(value) => value,
                            None => bail!("Missing field 'goo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mig_instances_allowed: {
                        let field_value = match fields_map.get("mig_instances_allowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'mig_instances_allowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_step: {
                        let field_value = match fields_map.get("post_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_step: {
                        let field_value = match fields_map.get("pre_step") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_step' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reboot_config: {
                        let field_value = match fields_map.get("reboot_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'reboot_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows_update: {
                        let field_value = match fields_map.get("windows_update") {
                            Some(value) => value,
                            None => bail!("Missing field 'windows_update' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#yum: {
                        let field_value = match fields_map.get("yum") {
                            Some(value) => value,
                            None => bail!("Missing field 'yum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zypper: {
                        let field_value = match fields_map.get("zypper") {
                            Some(value) => value,
                            None => bail!("Missing field 'zypper' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
