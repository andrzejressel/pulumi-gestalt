#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreeTierVirtualInstanceThreeTierConfiguration {
    #[builder(into)]
    pub r#app_resource_group_name: String,
    /// An `application_server_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#application_server_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationApplicationServerConfiguration>,
    /// A `central_server_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#central_server_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationCentralServerConfiguration>,
    /// A `database_server_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#database_server_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationDatabaseServerConfiguration>,
    /// The high availability type for the three tier configuration. Possible values are `AvailabilitySet` and `AvailabilityZone`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#high_availability_type: Option<String>,
    /// A `resource_names` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#resource_names: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNames>>,
    /// Specifies whether a secondary IP address should be added to the network interface on all VMs of the SAP system being deployed. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#secondary_ip_enabled: Option<bool>,
    /// A `transport_create_and_mount` block as defined below. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** The file share configuration uses `skip` by default when `transport_create_and_mount` isn't set.
    /// 
    /// > **Note:** Due to [a bug in the Azure API](https://github.com/Azure/azure-rest-api-specs/issues/25209) where the Storage File Share Id is not defined correctly, it is not currently possible to support using Transport Mount.
    #[builder(into)]
    pub r#transport_create_and_mount: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationTransportCreateAndMount>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThreeTierVirtualInstanceThreeTierConfiguration {
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
                    "appResourceGroupName",
                    &self.r#app_resource_group_name,
                ),
                to_pulumi_object_field(
                    "applicationServerConfiguration",
                    &self.r#application_server_configuration,
                ),
                to_pulumi_object_field(
                    "centralServerConfiguration",
                    &self.r#central_server_configuration,
                ),
                to_pulumi_object_field(
                    "databaseServerConfiguration",
                    &self.r#database_server_configuration,
                ),
                to_pulumi_object_field(
                    "highAvailabilityType",
                    &self.r#high_availability_type,
                ),
                to_pulumi_object_field(
                    "resourceNames",
                    &self.r#resource_names,
                ),
                to_pulumi_object_field(
                    "secondaryIpEnabled",
                    &self.r#secondary_ip_enabled,
                ),
                to_pulumi_object_field(
                    "transportCreateAndMount",
                    &self.r#transport_create_and_mount,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("appResourceGroupName") {
                            Some(value) => value,
                            None => bail!("Missing field 'appResourceGroupName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_server_configuration: {
                        let field_value = match fields_map.get("applicationServerConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationServerConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#central_server_configuration: {
                        let field_value = match fields_map.get("centralServerConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'centralServerConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_server_configuration: {
                        let field_value = match fields_map.get("databaseServerConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'databaseServerConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#high_availability_type: {
                        let field_value = match fields_map.get("highAvailabilityType") {
                            Some(value) => value,
                            None => bail!("Missing field 'highAvailabilityType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_names: {
                        let field_value = match fields_map.get("resourceNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_ip_enabled: {
                        let field_value = match fields_map.get("secondaryIpEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondaryIpEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transport_create_and_mount: {
                        let field_value = match fields_map.get("transportCreateAndMount") {
                            Some(value) => value,
                            None => bail!("Missing field 'transportCreateAndMount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
