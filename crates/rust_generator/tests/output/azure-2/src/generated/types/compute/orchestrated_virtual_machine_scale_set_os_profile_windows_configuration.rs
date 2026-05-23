#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrchestratedVirtualMachineScaleSetOsProfileWindowsConfiguration {
    /// One or more `additional_unattend_content` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#additional_unattend_contents: Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationAdditionalUnattendContent>>,
    /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#admin_password: String,
    /// The username of the local administrator on each Virtual Machine Scale Set instance. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#admin_username: String,
    /// The prefix which should be used for the name of the Virtual Machines in this Scale Set. If unspecified this defaults to the value for the `name` field. If the value of the `name` field is not a valid `computer_name_prefix`, then you must specify `computer_name_prefix`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#computer_name_prefix: Option<String>,
    /// Are automatic updates enabled for this Virtual Machine? Defaults to `true`.
    #[builder(into)]
    pub r#enable_automatic_updates: Option<bool>,
    /// Should the VM be patched without requiring a reboot? Possible values are `true` or `false`. Defaults to `false`. For more information about hot patching please see the [product documentation](https://docs.microsoft.com/azure/automanage/automanage-hotpatch).
    /// 
    /// > **Note:** Hotpatching can only be enabled if the `patch_mode` is set to `AutomaticByPlatform`, the `provision_vm_agent` is set to `true`, your `source_image_reference` references a hotpatching enabled image, the VM's `sku_name` is set to a [Azure generation 2](https://docs.microsoft.com/azure/virtual-machines/generation-2#generation-2-vm-sizes) VM SKU and the `extension` contains an application health extension. An example of how to correctly configure a Virtual Machine Scale Set to provision a Windows Virtual Machine with hotpatching enabled can be found in the `./examples/orchestrated-vm-scale-set/hotpatching-enabled` directory within the GitHub Repository.
    #[builder(into)]
    pub r#hotpatching_enabled: Option<bool>,
    /// Specifies the mode of VM Guest Patching for the virtual machines that are associated to the Virtual Machine Scale Set. Possible values are `AutomaticByPlatform` or `ImageDefault`. Defaults to `ImageDefault`.
    /// 
    /// > **Note:** If the `patch_assessment_mode` is set to `AutomaticByPlatform` then the `provision_vm_agent` field must be set to `true`.
    #[builder(into)]
    pub r#patch_assessment_mode: Option<String>,
    /// Specifies the mode of in-guest patching of this Windows Virtual Machine. Possible values are `Manual`, `AutomaticByOS` and `AutomaticByPlatform`. Defaults to `AutomaticByOS`. For more information on patch modes please see the [product documentation](https://docs.microsoft.com/azure/virtual-machines/automatic-vm-guest-patching#patch-orchestration-modes).
    /// 
    /// > **Note:** If `patch_mode` is set to `AutomaticByPlatform` the `provision_vm_agent` must be set to `true` and the `extension` must contain at least one application health extension.
    #[builder(into)]
    pub r#patch_mode: Option<String>,
    /// Should the Azure VM Agent be provisioned on each Virtual Machine in the Scale Set? Defaults to `true`. Changing this value forces a new resource to be created.
    #[builder(into)]
    pub r#provision_vm_agent: Option<bool>,
    /// One or more `secret` blocks as defined below.
    #[builder(into)]
    pub r#secrets: Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationSecret>>,
    /// Specifies the time zone of the virtual machine, the possible values are defined [here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
    #[builder(into)]
    pub r#timezone: Option<String>,
    /// One or more `winrm_listener` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#winrm_listeners: Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationWinrmListener>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrchestratedVirtualMachineScaleSetOsProfileWindowsConfiguration {
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
                    "additionalUnattendContents",
                    &self.r#additional_unattend_contents,
                ),
                to_pulumi_object_field(
                    "adminPassword",
                    &self.r#admin_password,
                ),
                to_pulumi_object_field(
                    "adminUsername",
                    &self.r#admin_username,
                ),
                to_pulumi_object_field(
                    "computerNamePrefix",
                    &self.r#computer_name_prefix,
                ),
                to_pulumi_object_field(
                    "enableAutomaticUpdates",
                    &self.r#enable_automatic_updates,
                ),
                to_pulumi_object_field(
                    "hotpatchingEnabled",
                    &self.r#hotpatching_enabled,
                ),
                to_pulumi_object_field(
                    "patchAssessmentMode",
                    &self.r#patch_assessment_mode,
                ),
                to_pulumi_object_field(
                    "patchMode",
                    &self.r#patch_mode,
                ),
                to_pulumi_object_field(
                    "provisionVmAgent",
                    &self.r#provision_vm_agent,
                ),
                to_pulumi_object_field(
                    "secrets",
                    &self.r#secrets,
                ),
                to_pulumi_object_field(
                    "timezone",
                    &self.r#timezone,
                ),
                to_pulumi_object_field(
                    "winrmListeners",
                    &self.r#winrm_listeners,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrchestratedVirtualMachineScaleSetOsProfileWindowsConfiguration {
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
                    r#additional_unattend_contents: {
                        let field_value = match fields_map.get("additionalUnattendContents") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalUnattendContents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#admin_password: {
                        let field_value = match fields_map.get("adminPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'adminPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#admin_username: {
                        let field_value = match fields_map.get("adminUsername") {
                            Some(value) => value,
                            None => bail!("Missing field 'adminUsername' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#computer_name_prefix: {
                        let field_value = match fields_map.get("computerNamePrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'computerNamePrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_automatic_updates: {
                        let field_value = match fields_map.get("enableAutomaticUpdates") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableAutomaticUpdates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hotpatching_enabled: {
                        let field_value = match fields_map.get("hotpatchingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'hotpatchingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patch_assessment_mode: {
                        let field_value = match fields_map.get("patchAssessmentMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'patchAssessmentMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patch_mode: {
                        let field_value = match fields_map.get("patchMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'patchMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provision_vm_agent: {
                        let field_value = match fields_map.get("provisionVmAgent") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisionVmAgent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets: {
                        let field_value = match fields_map.get("secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timezone: {
                        let field_value = match fields_map.get("timezone") {
                            Some(value) => value,
                            None => bail!("Missing field 'timezone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#winrm_listeners: {
                        let field_value = match fields_map.get("winrmListeners") {
                            Some(value) => value,
                            None => bail!("Missing field 'winrmListeners' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
