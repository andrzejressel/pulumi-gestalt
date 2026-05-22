#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData {
    /// The location of the SAPOData resource.
    #[builder(into)]
    #[serde(rename = "applicationHostUrl")]
    pub r#application_host_url: String,
    /// The application path to catalog service.
    #[builder(into)]
    #[serde(rename = "applicationServicePath")]
    pub r#application_service_path: String,
    /// The client number for the client creating the connection.
    #[builder(into)]
    #[serde(rename = "clientNumber")]
    pub r#client_number: String,
    /// The logon language of SAPOData instance.
    #[builder(into)]
    #[serde(rename = "logonLanguage")]
    pub r#logon_language: Option<String>,
    /// The SAPOData OAuth properties required for OAuth type authentication.
    #[builder(into)]
    #[serde(rename = "oauthProperties")]
    pub r#oauth_properties: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthProperties>>,
    /// The port number of the SAPOData instance.
    #[builder(into)]
    #[serde(rename = "portNumber")]
    pub r#port_number: i32,
    #[builder(into)]
    #[serde(rename = "privateLinkServiceName")]
    pub r#private_link_service_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData {
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
                    "applicationHostUrl",
                    &self.r#application_host_url,
                ),
                to_pulumi_object_field(
                    "applicationServicePath",
                    &self.r#application_service_path,
                ),
                to_pulumi_object_field(
                    "clientNumber",
                    &self.r#client_number,
                ),
                to_pulumi_object_field(
                    "logonLanguage",
                    &self.r#logon_language,
                ),
                to_pulumi_object_field(
                    "oauthProperties",
                    &self.r#oauth_properties,
                ),
                to_pulumi_object_field(
                    "portNumber",
                    &self.r#port_number,
                ),
                to_pulumi_object_field(
                    "privateLinkServiceName",
                    &self.r#private_link_service_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData {
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
                    r#application_host_url: {
                        let field_value = match fields_map.get("applicationHostUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationHostUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_service_path: {
                        let field_value = match fields_map.get("applicationServicePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationServicePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_number: {
                        let field_value = match fields_map.get("clientNumber") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientNumber' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logon_language: {
                        let field_value = match fields_map.get("logonLanguage") {
                            Some(value) => value,
                            None => bail!("Missing field 'logonLanguage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_properties: {
                        let field_value = match fields_map.get("oauthProperties") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauthProperties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_number: {
                        let field_value = match fields_map.get("portNumber") {
                            Some(value) => value,
                            None => bail!("Missing field 'portNumber' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_link_service_name: {
                        let field_value = match fields_map.get("privateLinkServiceName") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateLinkServiceName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
