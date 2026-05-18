#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OptionGroupOption {
    /// List of DB Security Groups for which the option is enabled.
    #[builder(into)]
    #[serde(rename = "dbSecurityGroupMemberships")]
    pub r#db_security_group_memberships: Option<Vec<String>>,
    /// Name of the option (e.g., MEMCACHED).
    #[builder(into)]
    #[serde(rename = "optionName")]
    pub r#option_name: String,
    /// The option settings to apply. See `option_settings` Block below for more details.
    #[builder(into)]
    #[serde(rename = "optionSettings")]
    pub r#option_settings: Option<Vec<super::super::types::rds::OptionGroupOptionOptionSetting>>,
    /// Port number when connecting to the option (e.g., 11211). Leaving out or removing `port` from your configuration does not remove or clear a port from the option in AWS. AWS may assign a default port. Not including `port` in your configuration means that the AWS provider will ignore a previously set value, a value set by AWS, and any port changes.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Version of the option (e.g., 13.1.0.0). Leaving out or removing `version` from your configuration does not remove or clear a version from the option in AWS. AWS may assign a default version. Not including `version` in your configuration means that the AWS provider will ignore a previously set value, a value set by AWS, and any version changes.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// List of VPC Security Groups for which the option is enabled.
    #[builder(into)]
    #[serde(rename = "vpcSecurityGroupMemberships")]
    pub r#vpc_security_group_memberships: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OptionGroupOption {
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
                    "db_security_group_memberships",
                    &self.r#db_security_group_memberships,
                ),
                to_pulumi_object_field(
                    "option_name",
                    &self.r#option_name,
                ),
                to_pulumi_object_field(
                    "option_settings",
                    &self.r#option_settings,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
                to_pulumi_object_field(
                    "vpc_security_group_memberships",
                    &self.r#vpc_security_group_memberships,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OptionGroupOption {
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
                    r#db_security_group_memberships: {
                        let field_value = match fields_map.get("db_security_group_memberships") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_security_group_memberships' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#option_name: {
                        let field_value = match fields_map.get("option_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'option_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#option_settings: {
                        let field_value = match fields_map.get("option_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'option_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#vpc_security_group_memberships: {
                        let field_value = match fields_map.get("vpc_security_group_memberships") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_security_group_memberships' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
