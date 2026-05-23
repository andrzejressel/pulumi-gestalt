#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEndpointRedisSetting {
    #[builder(into)]
    pub r#auth_password: String,
    #[builder(into)]
    pub r#auth_type: String,
    #[builder(into)]
    pub r#auth_user_name: String,
    #[builder(into)]
    pub r#port: i32,
    #[builder(into)]
    pub r#server_name: String,
    #[builder(into)]
    pub r#ssl_ca_certificate_arn: String,
    #[builder(into)]
    pub r#ssl_security_protocol: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEndpointRedisSetting {
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
                    "authPassword",
                    &self.r#auth_password,
                ),
                to_pulumi_object_field(
                    "authType",
                    &self.r#auth_type,
                ),
                to_pulumi_object_field(
                    "authUserName",
                    &self.r#auth_user_name,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "serverName",
                    &self.r#server_name,
                ),
                to_pulumi_object_field(
                    "sslCaCertificateArn",
                    &self.r#ssl_ca_certificate_arn,
                ),
                to_pulumi_object_field(
                    "sslSecurityProtocol",
                    &self.r#ssl_security_protocol,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEndpointRedisSetting {
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
                    r#auth_password: {
                        let field_value = match fields_map.get("authPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'authPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_type: {
                        let field_value = match fields_map.get("authType") {
                            Some(value) => value,
                            None => bail!("Missing field 'authType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_user_name: {
                        let field_value = match fields_map.get("authUserName") {
                            Some(value) => value,
                            None => bail!("Missing field 'authUserName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#server_name: {
                        let field_value = match fields_map.get("serverName") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_ca_certificate_arn: {
                        let field_value = match fields_map.get("sslCaCertificateArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslCaCertificateArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_security_protocol: {
                        let field_value = match fields_map.get("sslSecurityProtocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslSecurityProtocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
