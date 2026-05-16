#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection {
    /// The OIDC configuration for processing access tokens. See Access Token Only below.
    #[builder(into)]
    #[serde(rename = "accessTokenOnly")]
    pub r#access_token_only: Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly>>,
    /// The OIDC configuration for processing identity (ID) tokens. See Identity Token Only below.
    #[builder(into)]
    #[serde(rename = "identityTokenOnly")]
    pub r#identity_token_only: Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("access_token_only".to_string(), self.r#access_token_only.to_pulumi_value().await);
            map.insert("identity_token_only".to_string(), self.r#identity_token_only.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection {
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
                    r#access_token_only: {
                        let field_value = match fields_map.get("access_token_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_token_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#identity_token_only: {
                        let field_value = match fields_map.get("identity_token_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_token_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
