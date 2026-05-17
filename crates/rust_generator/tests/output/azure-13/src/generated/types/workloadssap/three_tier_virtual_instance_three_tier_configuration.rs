#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreeTierVirtualInstanceThreeTierConfiguration {
    #[builder(into)]
    #[serde(rename = "appResourceGroupName")]
    pub r#app_resource_group_name: String,
    /// An `application_server_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "applicationServerConfiguration")]
    pub r#application_server_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationApplicationServerConfiguration>,
    /// A `central_server_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "centralServerConfiguration")]
    pub r#central_server_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationCentralServerConfiguration>,
    /// A `database_server_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "databaseServerConfiguration")]
    pub r#database_server_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationDatabaseServerConfiguration>,
    /// The high availability type for the three tier configuration. Possible values are `AvailabilitySet` and `AvailabilityZone`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "highAvailabilityType")]
    pub r#high_availability_type: Option<String>,
    /// A `resource_names` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "resourceNames")]
    pub r#resource_names: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNames>>,
    /// Specifies whether a secondary IP address should be added to the network interface on all VMs of the SAP system being deployed. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "secondaryIpEnabled")]
    pub r#secondary_ip_enabled: Option<bool>,
    /// A `transport_create_and_mount` block as defined below. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** The file share configuration uses `skip` by default when `transport_create_and_mount` isn't set.
    /// 
    /// > **Note:** Due to [a bug in the Azure API](https://github.com/Azure/azure-rest-api-specs/issues/25209) where the Storage File Share Id is not defined correctly, it is not currently possible to support using Transport Mount.
    #[builder(into)]
    #[serde(rename = "transportCreateAndMount")]
    pub r#transport_create_and_mount: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationTransportCreateAndMount>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThreeTierVirtualInstanceThreeTierConfiguration {
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
                    "app_resource_group_name",
                    &self.r#app_resource_group_name,
                ),
                to_pulumi_object_field(
                    "application_server_configuration",
                    &self.r#application_server_configuration,
                ),
                to_pulumi_object_field(
                    "central_server_configuration",
                    &self.r#central_server_configuration,
                ),
                to_pulumi_object_field(
                    "database_server_configuration",
                    &self.r#database_server_configuration,
                ),
                to_pulumi_object_field(
                    "high_availability_type",
                    &self.r#high_availability_type,
                ),
                to_pulumi_object_field(
                    "resource_names",
                    &self.r#resource_names,
                ),
                to_pulumi_object_field(
                    "secondary_ip_enabled",
                    &self.r#secondary_ip_enabled,
                ),
                to_pulumi_object_field(
                    "transport_create_and_mount",
                    &self.r#transport_create_and_mount,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThreeTierVirtualInstanceThreeTierConfiguration {
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
                    r#app_resource_group_name: {
                        let field_value = match fields_map.get("app_resource_group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_resource_group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_server_configuration: {
                        let field_value = match fields_map.get("application_server_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_server_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#central_server_configuration: {
                        let field_value = match fields_map.get("central_server_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'central_server_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_server_configuration: {
                        let field_value = match fields_map.get("database_server_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_server_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#high_availability_type: {
                        let field_value = match fields_map.get("high_availability_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'high_availability_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_names: {
                        let field_value = match fields_map.get("resource_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_ip_enabled: {
                        let field_value = match fields_map.get("secondary_ip_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_ip_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transport_create_and_mount: {
                        let field_value = match fields_map.get("transport_create_and_mount") {
                            Some(value) => value,
                            None => bail!("Missing field 'transport_create_and_mount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
