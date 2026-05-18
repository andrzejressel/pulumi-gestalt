#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolUserAccount {
    /// The elevation level of the user account. "NonAdmin" - The auto user is a standard user without elevated access. "Admin" - The auto user is a user with elevated access and operates with full Administrator permissions. The default value is nonAdmin.
    #[builder(into)]
    #[serde(rename = "elevationLevel")]
    pub r#elevation_level: String,
    /// The `linux_user_configuration` block defined below is a linux-specific user configuration for the user account. This property is ignored if specified on a Windows pool. If not specified, the user is created with the default options.
    #[builder(into)]
    #[serde(rename = "linuxUserConfigurations")]
    pub r#linux_user_configurations: Vec<super::super::types::batch::GetPoolUserAccountLinuxUserConfiguration>,
    /// The name of the user account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The password for the user account.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// The `windows_user_configuration` block defined below is a windows-specific user configuration for the user account. This property can only be specified if the user is on a Windows pool. If not specified and on a Windows pool, the user is created with the default options.
    #[builder(into)]
    #[serde(rename = "windowsUserConfigurations")]
    pub r#windows_user_configurations: Vec<super::super::types::batch::GetPoolUserAccountWindowsUserConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPoolUserAccount {
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
                    "elevation_level",
                    &self.r#elevation_level,
                ),
                to_pulumi_object_field(
                    "linux_user_configurations",
                    &self.r#linux_user_configurations,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
                to_pulumi_object_field(
                    "windows_user_configurations",
                    &self.r#windows_user_configurations,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPoolUserAccount {
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
                    r#elevation_level: {
                        let field_value = match fields_map.get("elevation_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'elevation_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linux_user_configurations: {
                        let field_value = match fields_map.get("linux_user_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'linux_user_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows_user_configurations: {
                        let field_value = match fields_map.get("windows_user_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'windows_user_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
