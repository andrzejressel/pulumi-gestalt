#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DirectoryWorkspaceCreationProperties {
    /// The identifier of your custom security group. Should relate to the same VPC, where workspaces reside in.
    #[builder(into)]
    #[serde(rename = "customSecurityGroupId")]
    pub r#custom_security_group_id: Option<String>,
    /// The default organizational unit (OU) for your WorkSpace directories. Should conform `"OU=<value>,DC=<value>,...,DC=<value>"` pattern.
    #[builder(into)]
    #[serde(rename = "defaultOu")]
    pub r#default_ou: Option<String>,
    /// Indicates whether internet access is enabled for your WorkSpaces.
    #[builder(into)]
    #[serde(rename = "enableInternetAccess")]
    pub r#enable_internet_access: Option<bool>,
    /// Indicates whether maintenance mode is enabled for your WorkSpaces. For more information, see [WorkSpace Maintenance](https://docs.aws.amazon.com/workspaces/latest/adminguide/workspace-maintenance.html)..
    #[builder(into)]
    #[serde(rename = "enableMaintenanceMode")]
    pub r#enable_maintenance_mode: Option<bool>,
    /// Indicates whether users are local administrators of their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "userEnabledAsLocalAdministrator")]
    pub r#user_enabled_as_local_administrator: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DirectoryWorkspaceCreationProperties {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("custom_security_group_id".to_string(), self.r#custom_security_group_id.to_pulumi_value().await);
            map.insert("default_ou".to_string(), self.r#default_ou.to_pulumi_value().await);
            map.insert("enable_internet_access".to_string(), self.r#enable_internet_access.to_pulumi_value().await);
            map.insert("enable_maintenance_mode".to_string(), self.r#enable_maintenance_mode.to_pulumi_value().await);
            map.insert("user_enabled_as_local_administrator".to_string(), self.r#user_enabled_as_local_administrator.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DirectoryWorkspaceCreationProperties {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#custom_security_group_id: {
                        let field_value = match fields_map.get("custom_security_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_security_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#default_ou: {
                        let field_value = match fields_map.get("default_ou") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_ou' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_internet_access: {
                        let field_value = match fields_map.get("enable_internet_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_internet_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_maintenance_mode: {
                        let field_value = match fields_map.get("enable_maintenance_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_maintenance_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_enabled_as_local_administrator: {
                        let field_value = match fields_map.get("user_enabled_as_local_administrator") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_enabled_as_local_administrator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
