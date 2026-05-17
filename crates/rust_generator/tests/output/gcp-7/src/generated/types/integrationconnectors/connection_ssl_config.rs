#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionSslConfig {
    /// Additional SSL related field values.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "additionalVariables")]
    pub r#additional_variables: Option<Vec<super::super::types::integrationconnectors::ConnectionSslConfigAdditionalVariable>>,
    /// Type of Client Cert (PEM/JKS/.. etc.)
    /// Possible values are: `PEM`.
    #[builder(into)]
    #[serde(rename = "clientCertType")]
    pub r#client_cert_type: Option<String>,
    /// Client Certificate
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Option<Box<super::super::types::integrationconnectors::ConnectionSslConfigClientCertificate>>,
    /// Client Private Key
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clientPrivateKey")]
    pub r#client_private_key: Option<Box<super::super::types::integrationconnectors::ConnectionSslConfigClientPrivateKey>>,
    /// Secret containing the passphrase protecting the Client Private Key
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clientPrivateKeyPass")]
    pub r#client_private_key_pass: Option<Box<super::super::types::integrationconnectors::ConnectionSslConfigClientPrivateKeyPass>>,
    /// Private Server Certificate. Needs to be specified if trust model is PRIVATE.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "privateServerCertificate")]
    pub r#private_server_certificate: Option<Box<super::super::types::integrationconnectors::ConnectionSslConfigPrivateServerCertificate>>,
    /// Type of Server Cert (PEM/JKS/.. etc.)
    /// Possible values are: `PEM`.
    #[builder(into)]
    #[serde(rename = "serverCertType")]
    pub r#server_cert_type: Option<String>,
    /// Enum for Trust Model
    /// Possible values are: `PUBLIC`, `PRIVATE`, `INSECURE`.
    #[builder(into)]
    #[serde(rename = "trustModel")]
    pub r#trust_model: Option<String>,
    /// Enum for controlling the SSL Type (TLS/MTLS)
    /// Possible values are: `TLS`, `MTLS`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Bool for enabling SSL
    #[builder(into)]
    #[serde(rename = "useSsl")]
    pub r#use_ssl: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionSslConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "additional_variables",
                    &self.r#additional_variables,
                ),
                to_pulumi_object_field(
                    "client_cert_type",
                    &self.r#client_cert_type,
                ),
                to_pulumi_object_field(
                    "client_certificate",
                    &self.r#client_certificate,
                ),
                to_pulumi_object_field(
                    "client_private_key",
                    &self.r#client_private_key,
                ),
                to_pulumi_object_field(
                    "client_private_key_pass",
                    &self.r#client_private_key_pass,
                ),
                to_pulumi_object_field(
                    "private_server_certificate",
                    &self.r#private_server_certificate,
                ),
                to_pulumi_object_field(
                    "server_cert_type",
                    &self.r#server_cert_type,
                ),
                to_pulumi_object_field(
                    "trust_model",
                    &self.r#trust_model,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
                to_pulumi_object_field(
                    "use_ssl",
                    &self.r#use_ssl,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionSslConfig {
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
                    r#additional_variables: {
                        let field_value = match fields_map.get("additional_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_cert_type: {
                        let field_value = match fields_map.get("client_cert_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_cert_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_certificate: {
                        let field_value = match fields_map.get("client_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_private_key: {
                        let field_value = match fields_map.get("client_private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_private_key_pass: {
                        let field_value = match fields_map.get("client_private_key_pass") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_private_key_pass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_server_certificate: {
                        let field_value = match fields_map.get("private_server_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_server_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_cert_type: {
                        let field_value = match fields_map.get("server_cert_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_cert_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trust_model: {
                        let field_value = match fields_map.get("trust_model") {
                            Some(value) => value,
                            None => bail!("Missing field 'trust_model' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_ssl: {
                        let field_value = match fields_map.get("use_ssl") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_ssl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
