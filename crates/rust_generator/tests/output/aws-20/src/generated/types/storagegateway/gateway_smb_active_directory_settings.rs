#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GatewaySmbActiveDirectorySettings {
    #[builder(into)]
    pub r#active_directory_status: Option<String>,
    /// List of IPv4 addresses, NetBIOS names, or host names of your domain server.
    /// If you need to specify the port number include it after the colon (“:”). For example, `mydc.mydomain.com:389`.
    #[builder(into)]
    pub r#domain_controllers: Option<Vec<String>>,
    /// The name of the domain that you want the gateway to join.
    #[builder(into)]
    pub r#domain_name: String,
    /// The organizational unit (OU) is a container in an Active Directory that can hold users, groups,
    /// computers, and other OUs and this parameter specifies the OU that the gateway will join within the AD domain.
    #[builder(into)]
    pub r#organizational_unit: Option<String>,
    /// The password of the user who has permission to add the gateway to the Active Directory domain.
    #[builder(into)]
    pub r#password: String,
    /// Specifies the time in seconds, in which the JoinDomain operation must complete. The default is `20` seconds.
    #[builder(into)]
    pub r#timeout_in_seconds: Option<i32>,
    /// The user name of user who has permission to add the gateway to the Active Directory domain.
    #[builder(into)]
    pub r#username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GatewaySmbActiveDirectorySettings {
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
                    "activeDirectoryStatus",
                    &self.r#active_directory_status,
                ),
                to_pulumi_object_field(
                    "domainControllers",
                    &self.r#domain_controllers,
                ),
                to_pulumi_object_field(
                    "domainName",
                    &self.r#domain_name,
                ),
                to_pulumi_object_field(
                    "organizationalUnit",
                    &self.r#organizational_unit,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
                to_pulumi_object_field(
                    "timeoutInSeconds",
                    &self.r#timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "username",
                    &self.r#username,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GatewaySmbActiveDirectorySettings {
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
                    r#active_directory_status: {
                        let field_value = match fields_map.get("activeDirectoryStatus") {
                            Some(value) => value,
                            None => bail!("Missing field 'activeDirectoryStatus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_controllers: {
                        let field_value = match fields_map.get("domainControllers") {
                            Some(value) => value,
                            None => bail!("Missing field 'domainControllers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_name: {
                        let field_value = match fields_map.get("domainName") {
                            Some(value) => value,
                            None => bail!("Missing field 'domainName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit: {
                        let field_value = match fields_map.get("organizationalUnit") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizationalUnit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
