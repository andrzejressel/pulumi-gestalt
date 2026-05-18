#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector {
    #[builder(into)]
    #[serde(rename = "apiKey")]
    pub r#api_key: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorApiKey>>,
    /// The authentication type that the custom connector uses for authenticating while creating a connector profile. One of: `APIKEY`, `BASIC`, `CUSTOM`, `OAUTH2`.
    #[builder(into)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: String,
    /// Basic credentials that are required for the authentication of the user.
    #[builder(into)]
    #[serde(rename = "basic")]
    pub r#basic: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorBasic>>,
    /// If the connector uses the custom authentication mechanism, this holds the required credentials.
    #[builder(into)]
    #[serde(rename = "custom")]
    pub r#custom: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorCustom>>,
    /// OAuth 2.0 credentials required for the authentication of the user.
    #[builder(into)]
    #[serde(rename = "oauth2")]
    pub r#oauth_2: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorOauth2>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector {
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
                    "api_key",
                    &self.r#api_key,
                ),
                to_pulumi_object_field(
                    "authentication_type",
                    &self.r#authentication_type,
                ),
                to_pulumi_object_field(
                    "basic",
                    &self.r#basic,
                ),
                to_pulumi_object_field(
                    "custom",
                    &self.r#custom,
                ),
                to_pulumi_object_field(
                    "oauth_2",
                    &self.r#oauth_2,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector {
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
                    r#api_key: {
                        let field_value = match fields_map.get("api_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authentication_type: {
                        let field_value = match fields_map.get("authentication_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#basic: {
                        let field_value = match fields_map.get("basic") {
                            Some(value) => value,
                            None => bail!("Missing field 'basic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom: {
                        let field_value = match fields_map.get("custom") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2: {
                        let field_value = match fields_map.get("oauth_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
