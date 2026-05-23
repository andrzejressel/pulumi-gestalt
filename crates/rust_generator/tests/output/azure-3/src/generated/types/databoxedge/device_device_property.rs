#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeviceDeviceProperty {
    /// The Data Box Edge/Gateway device local capacity in MB.
    #[builder(into)]
    pub r#capacity: Option<i32>,
    /// Type of compute roles configured.
    #[builder(into)]
    pub r#configured_role_types: Option<Vec<String>>,
    /// The Data Box Edge/Gateway device culture.
    #[builder(into)]
    pub r#culture: Option<String>,
    /// The device software version number of the device (e.g. 1.2.18105.6).
    #[builder(into)]
    pub r#hcs_version: Option<String>,
    /// The Data Box Edge/Gateway device model.
    #[builder(into)]
    pub r#model: Option<String>,
    /// The number of nodes in the cluster.
    #[builder(into)]
    pub r#node_count: Option<i32>,
    /// The Serial Number of Data Box Edge/Gateway device.
    #[builder(into)]
    pub r#serial_number: Option<String>,
    /// The Data Box Edge/Gateway device software version.
    #[builder(into)]
    pub r#software_version: Option<String>,
    /// The status of the Data Box Edge/Gateway device.
    #[builder(into)]
    pub r#status: Option<String>,
    /// The Data Box Edge/Gateway device timezone.
    #[builder(into)]
    pub r#time_zone: Option<String>,
    /// The type of the Data Box Edge/Gateway device.
    #[builder(into)]
    pub r#type_: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeviceDeviceProperty {
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
                    "capacity",
                    &self.r#capacity,
                ),
                to_pulumi_object_field(
                    "configuredRoleTypes",
                    &self.r#configured_role_types,
                ),
                to_pulumi_object_field(
                    "culture",
                    &self.r#culture,
                ),
                to_pulumi_object_field(
                    "hcsVersion",
                    &self.r#hcs_version,
                ),
                to_pulumi_object_field(
                    "model",
                    &self.r#model,
                ),
                to_pulumi_object_field(
                    "nodeCount",
                    &self.r#node_count,
                ),
                to_pulumi_object_field(
                    "serialNumber",
                    &self.r#serial_number,
                ),
                to_pulumi_object_field(
                    "softwareVersion",
                    &self.r#software_version,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "timeZone",
                    &self.r#time_zone,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeviceDeviceProperty {
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
                    r#capacity: {
                        let field_value = match fields_map.get("capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configured_role_types: {
                        let field_value = match fields_map.get("configuredRoleTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'configuredRoleTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#culture: {
                        let field_value = match fields_map.get("culture") {
                            Some(value) => value,
                            None => bail!("Missing field 'culture' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hcs_version: {
                        let field_value = match fields_map.get("hcsVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'hcsVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_count: {
                        let field_value = match fields_map.get("nodeCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serial_number: {
                        let field_value = match fields_map.get("serialNumber") {
                            Some(value) => value,
                            None => bail!("Missing field 'serialNumber' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#software_version: {
                        let field_value = match fields_map.get("softwareVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'softwareVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("timeZone") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeZone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
