#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerRecipeInstanceConfigurationBlockDeviceMapping {
    /// Name of the device. For example, `/dev/sda` or `/dev/xvdb`.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Option<String>,
    /// Configuration block with Elastic Block Storage (EBS) block device mapping settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "ebs")]
    pub r#ebs: Option<Box<super::super::types::imagebuilder::ContainerRecipeInstanceConfigurationBlockDeviceMappingEbs>>,
    /// Set to `true` to remove a mapping from the parent image.
    #[builder(into)]
    #[serde(rename = "noDevice")]
    pub r#no_device: Option<bool>,
    /// Virtual device name. For example, `ephemeral0`. Instance store volumes are numbered starting from 0.
    #[builder(into)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContainerRecipeInstanceConfigurationBlockDeviceMapping {
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
                "device_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_name,
                )
                .await,
            );
            map.insert(
                "ebs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ebs,
                )
                .await,
            );
            map.insert(
                "no_device".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#no_device,
                )
                .await,
            );
            map.insert(
                "virtual_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContainerRecipeInstanceConfigurationBlockDeviceMapping {
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
                    r#device_name: {
                        let field_value = match fields_map.get("device_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebs: {
                        let field_value = match fields_map.get("ebs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_device: {
                        let field_value = match fields_map.get("no_device") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_device' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_name: {
                        let field_value = match fields_map.get("virtual_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
