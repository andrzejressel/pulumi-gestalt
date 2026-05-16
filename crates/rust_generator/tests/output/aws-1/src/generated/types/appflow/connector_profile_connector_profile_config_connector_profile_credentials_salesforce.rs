#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce {
    #[builder(into)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Option<String>,
    /// The secret manager ARN, which contains the client ID and client secret of the connected app.
    #[builder(into)]
    #[serde(rename = "clientCredentialsArn")]
    pub r#client_credentials_arn: Option<String>,
    /// A JSON web token (JWT) that authorizes access to Salesforce records.
    #[builder(into)]
    #[serde(rename = "jwtToken")]
    pub r#jwt_token: Option<String>,
    #[builder(into)]
    #[serde(rename = "oauth2GrantType")]
    pub r#oauth_2_grant_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "oauthRequest")]
    pub r#oauth_request: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest>>,
    #[builder(into)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("access_token".to_string(), self.r#access_token.to_pulumi_value().await);
            map.insert("client_credentials_arn".to_string(), self.r#client_credentials_arn.to_pulumi_value().await);
            map.insert("jwt_token".to_string(), self.r#jwt_token.to_pulumi_value().await);
            map.insert("oauth_2_grant_type".to_string(), self.r#oauth_2_grant_type.to_pulumi_value().await);
            map.insert("oauth_request".to_string(), self.r#oauth_request.to_pulumi_value().await);
            map.insert("refresh_token".to_string(), self.r#refresh_token.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce {
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
                    r#access_token: {
                        let field_value = match fields_map.get("access_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#client_credentials_arn: {
                        let field_value = match fields_map.get("client_credentials_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_credentials_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#jwt_token: {
                        let field_value = match fields_map.get("jwt_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_grant_type: {
                        let field_value = match fields_map.get("oauth_2_grant_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2_grant_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#oauth_request: {
                        let field_value = match fields_map.get("oauth_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#refresh_token: {
                        let field_value = match fields_map.get("refresh_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'refresh_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
