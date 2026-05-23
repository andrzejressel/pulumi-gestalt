#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrchestratedVirtualMachineScaleSetOsProfileLinuxConfiguration {
    /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#admin_password: Option<String>,
    /// A `admin_ssh_key` block as documented below.
    #[builder(into)]
    pub r#admin_ssh_keys: Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileLinuxConfigurationAdminSshKey>>,
    /// The username of the local administrator on each Virtual Machine Scale Set instance. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#admin_username: String,
    /// The prefix which should be used for the name of the Virtual Machines in this Scale Set. If unspecified this defaults to the value for the name field. If the value of the name field is not a valid `computer_name_prefix`, then you must specify `computer_name_prefix`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#computer_name_prefix: Option<String>,
    /// When an `admin_password` is specified `disable_password_authentication` must be set to `false`. Defaults to `true`.
    /// 
    /// > **Note:** Either `admin_password` or `admin_ssh_key` must be specified.
    #[builder(into)]
    pub r#disable_password_authentication: Option<bool>,
    /// Specifies the mode of VM Guest Patching for the virtual machines that are associated to the Virtual Machine Scale Set. Possible values are `AutomaticByPlatform` or `ImageDefault`. Defaults to `ImageDefault`.
    /// 
    /// > **Note:** If the `patch_assessment_mode` is set to `AutomaticByPlatform` then the `provision_vm_agent` field must be set to `true`.
    #[builder(into)]
    pub r#patch_assessment_mode: Option<String>,
    /// Specifies the mode of in-guest patching of this Windows Virtual Machine. Possible values are `ImageDefault` or `AutomaticByPlatform`. Defaults to `ImageDefault`. For more information on patch modes please see the [product documentation](https://docs.microsoft.com/azure/virtual-machines/automatic-vm-guest-patching#patch-orchestration-modes).
    /// 
    /// > **Note:** If `patch_mode` is set to `AutomaticByPlatform` the `provision_vm_agent` must be set to `true` and the `extension` must contain at least one application health extension.  An example of how to correctly configure a Virtual Machine Scale Set to provision a Linux Virtual Machine with Automatic VM Guest Patching enabled can be found in the `./examples/orchestrated-vm-scale-set/automatic-vm-guest-patching` directory within the GitHub Repository.
    #[builder(into)]
    pub r#patch_mode: Option<String>,
    /// Should the Azure VM Agent be provisioned on each Virtual Machine in the Scale Set? Defaults to `true`. Changing this value forces a new resource to be created.
    #[builder(into)]
    pub r#provision_vm_agent: Option<bool>,
    /// One or more `secret` blocks as defined below.
    #[builder(into)]
    pub r#secrets: Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileLinuxConfigurationSecret>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrchestratedVirtualMachineScaleSetOsProfileLinuxConfiguration {
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
                    "adminPassword",
                    &self.r#admin_password,
                ),
                to_pulumi_object_field(
                    "adminSshKeys",
                    &self.r#admin_ssh_keys,
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
                    "disablePasswordAuthentication",
                    &self.r#disable_password_authentication,
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
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrchestratedVirtualMachineScaleSetOsProfileLinuxConfiguration {
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
                    r#admin_password: {
                        let field_value = match fields_map.get("adminPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'adminPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#admin_ssh_keys: {
                        let field_value = match fields_map.get("adminSshKeys") {
                            Some(value) => value,
                            None => bail!("Missing field 'adminSshKeys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#disable_password_authentication: {
                        let field_value = match fields_map.get("disablePasswordAuthentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'disablePasswordAuthentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
