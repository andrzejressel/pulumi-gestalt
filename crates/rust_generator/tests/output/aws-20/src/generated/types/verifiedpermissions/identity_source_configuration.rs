#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IdentitySourceConfiguration {
    /// Specifies the configuration details of an Amazon Cognito user pool that Verified Permissions can use as a source of authenticated identities as entities. See Cognito User Pool Configuration below.
    #[builder(into)]
    #[serde(rename = "cognitoUserPoolConfiguration")]
    pub r#cognito_user_pool_configuration: Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationCognitoUserPoolConfiguration>>,
    /// Specifies the configuration details of an OpenID Connect (OIDC) identity provider, or identity source, that Verified Permissions can use to generate entities from authenticated identities. See Open ID Connect Configuration below.
    #[builder(into)]
    #[serde(rename = "openIdConnectConfiguration")]
    pub r#open_id_connect_configuration: Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IdentitySourceConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("cognito_user_pool_configuration".to_string(), self.r#cognito_user_pool_configuration.to_pulumi_value().await);
            map.insert("open_id_connect_configuration".to_string(), self.r#open_id_connect_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IdentitySourceConfiguration {
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
                    r#cognito_user_pool_configuration: {
                        let field_value = match fields_map.get("cognito_user_pool_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'cognito_user_pool_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationCognitoUserPoolConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#open_id_connect_configuration: {
                        let field_value = match fields_map.get("open_id_connect_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'open_id_connect_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
