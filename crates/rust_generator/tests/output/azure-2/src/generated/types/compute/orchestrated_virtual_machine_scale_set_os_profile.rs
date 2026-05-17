#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrchestratedVirtualMachineScaleSetOsProfile {
    /// The Base64-Encoded Custom Data which should be used for this Virtual Machine Scale Set.
    /// 
    /// > **Note:** When Custom Data has been configured, it's not possible to remove it without tainting the Virtual Machine Scale Set, due to a limitation of the Azure API.
    #[builder(into)]
    #[serde(rename = "customData")]
    pub r#custom_data: Option<String>,
    /// A `linux_configuration` block as documented below.
    #[builder(into)]
    #[serde(rename = "linuxConfiguration")]
    pub r#linux_configuration: Option<Box<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileLinuxConfiguration>>,
    /// A `windows_configuration` block as documented below.
    #[builder(into)]
    #[serde(rename = "windowsConfiguration")]
    pub r#windows_configuration: Option<Box<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileWindowsConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrchestratedVirtualMachineScaleSetOsProfile {
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
                "custom_data".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_data,
                )
                .await,
            );
            map.insert(
                "linux_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linux_configuration,
                )
                .await,
            );
            map.insert(
                "windows_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#windows_configuration,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrchestratedVirtualMachineScaleSetOsProfile {
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
                    r#custom_data: {
                        let field_value = match fields_map.get("custom_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linux_configuration: {
                        let field_value = match fields_map.get("linux_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'linux_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows_configuration: {
                        let field_value = match fields_map.get("windows_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'windows_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
