#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDevicesDevice {
    /// When the device was created.
    #[builder(into)]
    #[serde(rename = "created")]
    pub r#created: Option<String>,
    /// Whether the device has been deleted.
    #[builder(into)]
    #[serde(rename = "deleted")]
    pub r#deleted: Option<bool>,
    /// The type of the device.
    #[builder(into)]
    #[serde(rename = "deviceType")]
    pub r#device_type: Option<String>,
    /// Device ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// IPv4 or IPv6 address.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Option<String>,
    /// The device's public key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// When the device was last seen.
    #[builder(into)]
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Option<String>,
    /// The device's MAC address.
    #[builder(into)]
    #[serde(rename = "macAddress")]
    pub r#mac_address: Option<String>,
    /// The device manufacturer's name.
    #[builder(into)]
    #[serde(rename = "manufacturer")]
    pub r#manufacturer: Option<String>,
    /// The device model name.
    #[builder(into)]
    #[serde(rename = "model")]
    pub r#model: Option<String>,
    /// The device name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The Linux distribution name.
    #[builder(into)]
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Option<String>,
    /// The Linux distribution revision.
    #[builder(into)]
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Option<String>,
    /// The operating system version.
    #[builder(into)]
    #[serde(rename = "osVersion")]
    pub r#os_version: Option<String>,
    /// Extra version value following the operating system version.
    #[builder(into)]
    #[serde(rename = "osVersionExtra")]
    pub r#os_version_extra: Option<String>,
    /// When the device was revoked.
    #[builder(into)]
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Option<String>,
    /// The device's serial number.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<String>,
    /// When the device was updated.
    #[builder(into)]
    #[serde(rename = "updated")]
    pub r#updated: Option<String>,
    /// User's email.
    #[builder(into)]
    #[serde(rename = "userEmail")]
    pub r#user_email: Option<String>,
    /// User's ID.
    #[builder(into)]
    #[serde(rename = "userId")]
    pub r#user_id: Option<String>,
    /// User's Name.
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Option<String>,
    /// The WARP client version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDevicesDevice {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "created",
                    &self.r#created,
                ),
                to_pulumi_object_field(
                    "deleted",
                    &self.r#deleted,
                ),
                to_pulumi_object_field(
                    "device_type",
                    &self.r#device_type,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "ip",
                    &self.r#ip,
                ),
                to_pulumi_object_field(
                    "key",
                    &self.r#key,
                ),
                to_pulumi_object_field(
                    "last_seen",
                    &self.r#last_seen,
                ),
                to_pulumi_object_field(
                    "mac_address",
                    &self.r#mac_address,
                ),
                to_pulumi_object_field(
                    "manufacturer",
                    &self.r#manufacturer,
                ),
                to_pulumi_object_field(
                    "model",
                    &self.r#model,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "os_distro_name",
                    &self.r#os_distro_name,
                ),
                to_pulumi_object_field(
                    "os_distro_revision",
                    &self.r#os_distro_revision,
                ),
                to_pulumi_object_field(
                    "os_version",
                    &self.r#os_version,
                ),
                to_pulumi_object_field(
                    "os_version_extra",
                    &self.r#os_version_extra,
                ),
                to_pulumi_object_field(
                    "revoked_at",
                    &self.r#revoked_at,
                ),
                to_pulumi_object_field(
                    "serial_number",
                    &self.r#serial_number,
                ),
                to_pulumi_object_field(
                    "updated",
                    &self.r#updated,
                ),
                to_pulumi_object_field(
                    "user_email",
                    &self.r#user_email,
                ),
                to_pulumi_object_field(
                    "user_id",
                    &self.r#user_id,
                ),
                to_pulumi_object_field(
                    "user_name",
                    &self.r#user_name,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDevicesDevice {
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
                    r#created: {
                        let field_value = match fields_map.get("created") {
                            Some(value) => value,
                            None => bail!("Missing field 'created' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deleted: {
                        let field_value = match fields_map.get("deleted") {
                            Some(value) => value,
                            None => bail!("Missing field 'deleted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_type: {
                        let field_value = match fields_map.get("device_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip: {
                        let field_value = match fields_map.get("ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key: {
                        let field_value = match fields_map.get("key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_seen: {
                        let field_value = match fields_map.get("last_seen") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_seen' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mac_address: {
                        let field_value = match fields_map.get("mac_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'mac_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manufacturer: {
                        let field_value = match fields_map.get("manufacturer") {
                            Some(value) => value,
                            None => bail!("Missing field 'manufacturer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model: {
                        let field_value = match fields_map.get("model") {
                            Some(value) => value,
                            None => bail!("Missing field 'model' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#os_distro_name: {
                        let field_value = match fields_map.get("os_distro_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_distro_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_distro_revision: {
                        let field_value = match fields_map.get("os_distro_revision") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_distro_revision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_version: {
                        let field_value = match fields_map.get("os_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_version_extra: {
                        let field_value = match fields_map.get("os_version_extra") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_version_extra' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revoked_at: {
                        let field_value = match fields_map.get("revoked_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'revoked_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serial_number: {
                        let field_value = match fields_map.get("serial_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'serial_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#updated: {
                        let field_value = match fields_map.get("updated") {
                            Some(value) => value,
                            None => bail!("Missing field 'updated' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_email: {
                        let field_value = match fields_map.get("user_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_id: {
                        let field_value = match fields_map.get("user_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_name: {
                        let field_value = match fields_map.get("user_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
