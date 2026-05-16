#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector {
    /// The OAuth 2.0 properties required for OAuth 2.0 authentication.
    #[builder(into)]
    #[serde(rename = "oauth2Properties")]
    pub r#oauth_2_properties: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties>>,
    /// A map of properties that are required to create a profile for the custom connector.
    #[builder(into)]
    #[serde(rename = "profileProperties")]
    pub r#profile_properties: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("oauth_2_properties".to_string(), self.r#oauth_2_properties.to_pulumi_value().await);
            map.insert("profile_properties".to_string(), self.r#profile_properties.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector {
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
                    r#oauth_2_properties: {
                        let field_value = match fields_map.get("oauth_2_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnectorOauth2Properties>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#profile_properties: {
                        let field_value = match fields_map.get("profile_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'profile_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
