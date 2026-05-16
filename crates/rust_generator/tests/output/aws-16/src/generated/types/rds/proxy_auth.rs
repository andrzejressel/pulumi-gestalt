#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProxyAuth {
    /// The type of authentication that the proxy uses for connections from the proxy to the underlying database. One of `SECRETS`.
    #[builder(into)]
    #[serde(rename = "authScheme")]
    pub r#auth_scheme: Option<String>,
    /// The type of authentication the proxy uses for connections from clients. Valid values are `MYSQL_NATIVE_PASSWORD`, `POSTGRES_SCRAM_SHA_256`, `POSTGRES_MD5`, and `SQL_SERVER_AUTHENTICATION`.
    #[builder(into)]
    #[serde(rename = "clientPasswordAuthType")]
    pub r#client_password_auth_type: Option<String>,
    /// A user-specified description about the authentication used by a proxy to log in as a specific database user.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Whether to require or disallow AWS Identity and Access Management (IAM) authentication for connections to the proxy. One of `DISABLED`, `REQUIRED`.
    #[builder(into)]
    #[serde(rename = "iamAuth")]
    pub r#iam_auth: Option<String>,
    /// The Amazon Resource Name (ARN) representing the secret that the proxy uses to authenticate to the RDS DB instance or Aurora DB cluster. These secrets are stored within Amazon Secrets Manager.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Option<String>,
    /// The name of the database user to which the proxy connects.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProxyAuth {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("auth_scheme".to_string(), self.r#auth_scheme.to_pulumi_value().await);
            map.insert("client_password_auth_type".to_string(), self.r#client_password_auth_type.to_pulumi_value().await);
            map.insert("description".to_string(), self.r#description.to_pulumi_value().await);
            map.insert("iam_auth".to_string(), self.r#iam_auth.to_pulumi_value().await);
            map.insert("secret_arn".to_string(), self.r#secret_arn.to_pulumi_value().await);
            map.insert("username".to_string(), self.r#username.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProxyAuth {
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
                    r#auth_scheme: {
                        let field_value = match fields_map.get("auth_scheme") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_scheme' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#client_password_auth_type: {
                        let field_value = match fields_map.get("client_password_auth_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_password_auth_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#iam_auth: {
                        let field_value = match fields_map.get("iam_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'iam_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#secret_arn: {
                        let field_value = match fields_map.get("secret_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
