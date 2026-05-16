#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GatewaySmbActiveDirectorySettings {
    #[builder(into)]
    #[serde(rename = "activeDirectoryStatus")]
    pub r#active_directory_status: Option<String>,
    /// List of IPv4 addresses, NetBIOS names, or host names of your domain server.
    /// If you need to specify the port number include it after the colon (“:”). For example, `mydc.mydomain.com:389`.
    #[builder(into)]
    #[serde(rename = "domainControllers")]
    pub r#domain_controllers: Option<Vec<String>>,
    /// The name of the domain that you want the gateway to join.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// The organizational unit (OU) is a container in an Active Directory that can hold users, groups,
    /// computers, and other OUs and this parameter specifies the OU that the gateway will join within the AD domain.
    #[builder(into)]
    #[serde(rename = "organizationalUnit")]
    pub r#organizational_unit: Option<String>,
    /// The password of the user who has permission to add the gateway to the Active Directory domain.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// Specifies the time in seconds, in which the JoinDomain operation must complete. The default is `20` seconds.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Option<i32>,
    /// The user name of user who has permission to add the gateway to the Active Directory domain.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GatewaySmbActiveDirectorySettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("active_directory_status".to_string(), self.r#active_directory_status.to_pulumi_value().await);
            map.insert("domain_controllers".to_string(), self.r#domain_controllers.to_pulumi_value().await);
            map.insert("domain_name".to_string(), self.r#domain_name.to_pulumi_value().await);
            map.insert("organizational_unit".to_string(), self.r#organizational_unit.to_pulumi_value().await);
            map.insert("password".to_string(), self.r#password.to_pulumi_value().await);
            map.insert("timeout_in_seconds".to_string(), self.r#timeout_in_seconds.to_pulumi_value().await);
            map.insert("username".to_string(), self.r#username.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GatewaySmbActiveDirectorySettings {
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
                    r#active_directory_status: {
                        let field_value = match fields_map.get("active_directory_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directory_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#domain_controllers: {
                        let field_value = match fields_map.get("domain_controllers") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_controllers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#domain_name: {
                        let field_value = match fields_map.get("domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit: {
                        let field_value = match fields_map.get("organizational_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizational_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
