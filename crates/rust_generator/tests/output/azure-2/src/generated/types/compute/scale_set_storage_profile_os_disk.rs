#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetStorageProfileOsDisk {
    /// Specifies the caching requirements. Possible values include: `None` (default), `ReadOnly`, `ReadWrite`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: Option<String>,
    /// Specifies how the virtual machine should be created. The only possible option is `FromImage`.
    #[builder(into)]
    #[serde(rename = "createOption")]
    pub r#create_option: String,
    /// Specifies the blob URI for user image. A virtual machine scale set creates an os disk in the same container as the user image.
    /// Updating the osDisk image causes the existing disk to be deleted and a new one created with the new image. If the VM scale set is in Manual upgrade mode then the virtual machines are not updated until they have manualUpgrade applied to them.
    /// When setting this field `os_type` needs to be specified. Cannot be used when `vhd_containers`, `managed_disk_type` or `storage_profile_image_reference` are specified.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Option<String>,
    /// Specifies the type of managed disk to create. Value you must be either `Standard_LRS`, `StandardSSD_LRS` or `Premium_LRS`. Cannot be used when `vhd_containers` or `image` is specified.
    #[builder(into)]
    #[serde(rename = "managedDiskType")]
    pub r#managed_disk_type: Option<String>,
    /// Specifies the disk name. Must be specified when using unmanaged disk ('managed_disk_type' property not set).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Specifies the operating system Type, valid values are windows, Linux.
    #[builder(into)]
    #[serde(rename = "osType")]
    pub r#os_type: Option<String>,
    /// Specifies the VHD URI. Cannot be used when `image` or `managed_disk_type` is specified.
    #[builder(into)]
    #[serde(rename = "vhdContainers")]
    pub r#vhd_containers: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScaleSetStorageProfileOsDisk {
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
                "caching".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#caching,
                )
                .await,
            );
            map.insert(
                "create_option".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#create_option,
                )
                .await,
            );
            map.insert(
                "image".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image,
                )
                .await,
            );
            map.insert(
                "managed_disk_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_disk_type,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "os_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_type,
                )
                .await,
            );
            map.insert(
                "vhd_containers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vhd_containers,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScaleSetStorageProfileOsDisk {
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
                    r#caching: {
                        let field_value = match fields_map.get("caching") {
                            Some(value) => value,
                            None => bail!("Missing field 'caching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_option: {
                        let field_value = match fields_map.get("create_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image: {
                        let field_value = match fields_map.get("image") {
                            Some(value) => value,
                            None => bail!("Missing field 'image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_disk_type: {
                        let field_value = match fields_map.get("managed_disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_type: {
                        let field_value = match fields_map.get("os_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vhd_containers: {
                        let field_value = match fields_map.get("vhd_containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'vhd_containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
