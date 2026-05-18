#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DevicePostureIntegrationConfig {
    /// The Access client ID to be used as the `Cf-Access-Client-ID` header when making a request to the `api_url`.
    #[builder(into)]
    #[serde(rename = "accessClientId")]
    pub r#access_client_id: Option<String>,
    /// The Access client secret to be used as the `Cf-Access-Client-Secret` header when making a request to the `api_url`.
    #[builder(into)]
    #[serde(rename = "accessClientSecret")]
    pub r#access_client_secret: Option<String>,
    /// The third-party API's URL.
    #[builder(into)]
    #[serde(rename = "apiUrl")]
    pub r#api_url: Option<String>,
    /// The third-party authorization API URL.
    #[builder(into)]
    #[serde(rename = "authUrl")]
    pub r#auth_url: Option<String>,
    /// The client identifier for authenticating API calls.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// The client key for authenticating API calls.
    #[builder(into)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Option<String>,
    /// The client secret for authenticating API calls.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// The customer identifier for authenticating API calls.
    #[builder(into)]
    #[serde(rename = "customerId")]
    pub r#customer_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DevicePostureIntegrationConfig {
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
                    "access_client_id",
                    &self.r#access_client_id,
                ),
                to_pulumi_object_field(
                    "access_client_secret",
                    &self.r#access_client_secret,
                ),
                to_pulumi_object_field(
                    "api_url",
                    &self.r#api_url,
                ),
                to_pulumi_object_field(
                    "auth_url",
                    &self.r#auth_url,
                ),
                to_pulumi_object_field(
                    "client_id",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "client_key",
                    &self.r#client_key,
                ),
                to_pulumi_object_field(
                    "client_secret",
                    &self.r#client_secret,
                ),
                to_pulumi_object_field(
                    "customer_id",
                    &self.r#customer_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DevicePostureIntegrationConfig {
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
                    r#access_client_id: {
                        let field_value = match fields_map.get("access_client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_client_secret: {
                        let field_value = match fields_map.get("access_client_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_client_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_url: {
                        let field_value = match fields_map.get("api_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_url: {
                        let field_value = match fields_map.get("auth_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_id: {
                        let field_value = match fields_map.get("client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_key: {
                        let field_value = match fields_map.get("client_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret: {
                        let field_value = match fields_map.get("client_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_id: {
                        let field_value = match fields_map.get("customer_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
