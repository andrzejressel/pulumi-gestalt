#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IdentitySourceConfigurationOpenIdConnectConfiguration {
    /// A descriptive string that you want to prefix to user entities from your OIDC identity provider.
    #[builder(into)]
    #[serde(rename = "entityIdPrefix")]
    pub r#entity_id_prefix: Option<String>,
    /// The type of entity that a policy store maps to groups from an Amazon Cognito user pool identity source. See Group Configuration below.
    #[builder(into)]
    #[serde(rename = "groupConfiguration")]
    pub r#group_configuration: Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfiguration>>,
    /// The issuer URL of an OIDC identity provider. This URL must have an OIDC discovery endpoint at the path `.well-known/openid-configuration`.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: String,
    /// The token type that you want to process from your OIDC identity provider. Your policy store can process either identity (ID) or access tokens from a given OIDC identity source. See Token Selection below.
    #[builder(into)]
    #[serde(rename = "tokenSelection")]
    pub r#token_selection: Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IdentitySourceConfigurationOpenIdConnectConfiguration {
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
                    "entity_id_prefix",
                    &self.r#entity_id_prefix,
                ),
                to_pulumi_object_field(
                    "group_configuration",
                    &self.r#group_configuration,
                ),
                to_pulumi_object_field(
                    "issuer",
                    &self.r#issuer,
                ),
                to_pulumi_object_field(
                    "token_selection",
                    &self.r#token_selection,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IdentitySourceConfigurationOpenIdConnectConfiguration {
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
                    r#entity_id_prefix: {
                        let field_value = match fields_map.get("entity_id_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_id_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_configuration: {
                        let field_value = match fields_map.get("group_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer: {
                        let field_value = match fields_map.get("issuer") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_selection: {
                        let field_value = match fields_map.get("token_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
