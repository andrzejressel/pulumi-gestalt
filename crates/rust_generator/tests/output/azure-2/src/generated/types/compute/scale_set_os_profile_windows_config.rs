#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetOsProfileWindowsConfig {
    /// An `additional_unattend_config` block as documented below.
    #[builder(into)]
    #[serde(rename = "additionalUnattendConfigs")]
    pub r#additional_unattend_configs: Option<Vec<super::super::types::compute::ScaleSetOsProfileWindowsConfigAdditionalUnattendConfig>>,
    /// Indicates whether virtual machines in the scale set are enabled for automatic updates.
    #[builder(into)]
    #[serde(rename = "enableAutomaticUpgrades")]
    pub r#enable_automatic_upgrades: Option<bool>,
    /// Indicates whether virtual machine agent should be provisioned on the virtual machines in the scale set.
    #[builder(into)]
    #[serde(rename = "provisionVmAgent")]
    pub r#provision_vm_agent: Option<bool>,
    /// A collection of `winrm` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "winrms")]
    pub r#winrms: Option<Vec<super::super::types::compute::ScaleSetOsProfileWindowsConfigWinrm>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScaleSetOsProfileWindowsConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "additional_unattend_configs",
                    &self.r#additional_unattend_configs,
                ),
                to_pulumi_object_field(
                    "enable_automatic_upgrades",
                    &self.r#enable_automatic_upgrades,
                ),
                to_pulumi_object_field(
                    "provision_vm_agent",
                    &self.r#provision_vm_agent,
                ),
                to_pulumi_object_field(
                    "winrms",
                    &self.r#winrms,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScaleSetOsProfileWindowsConfig {
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
                    r#additional_unattend_configs: {
                        let field_value = match fields_map.get("additional_unattend_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_unattend_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_automatic_upgrades: {
                        let field_value = match fields_map.get("enable_automatic_upgrades") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_automatic_upgrades' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provision_vm_agent: {
                        let field_value = match fields_map.get("provision_vm_agent") {
                            Some(value) => value,
                            None => bail!("Missing field 'provision_vm_agent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#winrms: {
                        let field_value = match fields_map.get("winrms") {
                            Some(value) => value,
                            None => bail!("Missing field 'winrms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
