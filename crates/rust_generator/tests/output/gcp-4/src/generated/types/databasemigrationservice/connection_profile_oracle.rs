#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileOracle {
    /// Required. Database service for the Oracle connection.
    #[builder(into)]
    #[serde(rename = "databaseService")]
    pub r#database_service: String,
    /// SSL configuration for the destination to connect to the source database.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "forwardSshConnectivity")]
    pub r#forward_ssh_connectivity: Option<Box<super::super::types::databasemigrationservice::ConnectionProfileOracleForwardSshConnectivity>>,
    /// Required. The IP or hostname of the source Oracle database.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: String,
    /// Required. Input only. The password for the user that Database Migration Service will be using to connect to the database.
    /// This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// (Output)
    /// Output only. Indicates If this connection profile password is stored.
    #[builder(into)]
    #[serde(rename = "passwordSet")]
    pub r#password_set: Option<bool>,
    /// Required. The network port of the source Oracle database.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// Configuration for using a private network to communicate with the source database
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "privateConnectivity")]
    pub r#private_connectivity: Option<Box<super::super::types::databasemigrationservice::ConnectionProfileOraclePrivateConnectivity>>,
    /// SSL configuration for the destination to connect to the source database.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ssl")]
    pub r#ssl: Option<Box<super::super::types::databasemigrationservice::ConnectionProfileOracleSsl>>,
    /// This object has no nested fields.
    /// Static IP address connectivity configured on service project.
    #[builder(into)]
    #[serde(rename = "staticServiceIpConnectivity")]
    pub r#static_service_ip_connectivity: Option<Box<super::super::types::databasemigrationservice::ConnectionProfileOracleStaticServiceIpConnectivity>>,
    /// Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionProfileOracle {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "database_service".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_service,
                )
                .await,
            );
            map.insert(
                "forward_ssh_connectivity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forward_ssh_connectivity,
                )
                .await,
            );
            map.insert(
                "host".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host,
                )
                .await,
            );
            map.insert(
                "password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#password,
                )
                .await,
            );
            map.insert(
                "password_set".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#password_set,
                )
                .await,
            );
            map.insert(
                "port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port,
                )
                .await,
            );
            map.insert(
                "private_connectivity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_connectivity,
                )
                .await,
            );
            map.insert(
                "ssl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl,
                )
                .await,
            );
            map.insert(
                "static_service_ip_connectivity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#static_service_ip_connectivity,
                )
                .await,
            );
            map.insert(
                "username".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#username,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionProfileOracle {
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
                    r#database_service: {
                        let field_value = match fields_map.get("database_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forward_ssh_connectivity: {
                        let field_value = match fields_map.get("forward_ssh_connectivity") {
                            Some(value) => value,
                            None => bail!("Missing field 'forward_ssh_connectivity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host: {
                        let field_value = match fields_map.get("host") {
                            Some(value) => value,
                            None => bail!("Missing field 'host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#password_set: {
                        let field_value = match fields_map.get("password_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'password_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#private_connectivity: {
                        let field_value = match fields_map.get("private_connectivity") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_connectivity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl: {
                        let field_value = match fields_map.get("ssl") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#static_service_ip_connectivity: {
                        let field_value = match fields_map.get("static_service_ip_connectivity") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_service_ip_connectivity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
