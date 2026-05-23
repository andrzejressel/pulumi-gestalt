#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DirectoryWorkspaceCreationProperties {
    /// The identifier of your custom security group. Should relate to the same VPC, where workspaces reside in.
    #[builder(into)]
    pub r#custom_security_group_id: Option<String>,
    /// The default organizational unit (OU) for your WorkSpace directories. Should conform `"OU=<value>,DC=<value>,...,DC=<value>"` pattern.
    #[builder(into)]
    pub r#default_ou: Option<String>,
    /// Indicates whether internet access is enabled for your WorkSpaces.
    #[builder(into)]
    pub r#enable_internet_access: Option<bool>,
    /// Indicates whether maintenance mode is enabled for your WorkSpaces. For more information, see [WorkSpace Maintenance](https://docs.aws.amazon.com/workspaces/latest/adminguide/workspace-maintenance.html)..
    #[builder(into)]
    pub r#enable_maintenance_mode: Option<bool>,
    /// Indicates whether users are local administrators of their WorkSpaces.
    #[builder(into)]
    pub r#user_enabled_as_local_administrator: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DirectoryWorkspaceCreationProperties {
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
                    "customSecurityGroupId",
                    &self.r#custom_security_group_id,
                ),
                to_pulumi_object_field(
                    "defaultOu",
                    &self.r#default_ou,
                ),
                to_pulumi_object_field(
                    "enableInternetAccess",
                    &self.r#enable_internet_access,
                ),
                to_pulumi_object_field(
                    "enableMaintenanceMode",
                    &self.r#enable_maintenance_mode,
                ),
                to_pulumi_object_field(
                    "userEnabledAsLocalAdministrator",
                    &self.r#user_enabled_as_local_administrator,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DirectoryWorkspaceCreationProperties {
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
                    r#custom_security_group_id: {
                        let field_value = match fields_map.get("customSecurityGroupId") {
                            Some(value) => value,
                            None => bail!("Missing field 'customSecurityGroupId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_ou: {
                        let field_value = match fields_map.get("defaultOu") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultOu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_internet_access: {
                        let field_value = match fields_map.get("enableInternetAccess") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableInternetAccess' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_maintenance_mode: {
                        let field_value = match fields_map.get("enableMaintenanceMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableMaintenanceMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_enabled_as_local_administrator: {
                        let field_value = match fields_map.get("userEnabledAsLocalAdministrator") {
                            Some(value) => value,
                            None => bail!("Missing field 'userEnabledAsLocalAdministrator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
