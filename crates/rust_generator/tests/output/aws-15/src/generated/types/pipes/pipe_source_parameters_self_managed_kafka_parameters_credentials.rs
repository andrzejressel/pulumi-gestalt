#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeSourceParametersSelfManagedKafkaParametersCredentials {
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into)]
    #[serde(rename = "basicAuth")]
    pub r#basic_auth: Option<String>,
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into)]
    #[serde(rename = "clientCertificateTlsAuth")]
    pub r#client_certificate_tls_auth: Option<String>,
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into)]
    #[serde(rename = "saslScram256Auth")]
    pub r#sasl_scram_256_auth: Option<String>,
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into)]
    #[serde(rename = "saslScram512Auth")]
    pub r#sasl_scram_512_auth: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeSourceParametersSelfManagedKafkaParametersCredentials {
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
                    "basicAuth",
                    &self.r#basic_auth,
                ),
                to_pulumi_object_field(
                    "clientCertificateTlsAuth",
                    &self.r#client_certificate_tls_auth,
                ),
                to_pulumi_object_field(
                    "saslScram256Auth",
                    &self.r#sasl_scram_256_auth,
                ),
                to_pulumi_object_field(
                    "saslScram512Auth",
                    &self.r#sasl_scram_512_auth,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeSourceParametersSelfManagedKafkaParametersCredentials {
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
                    r#basic_auth: {
                        let field_value = match fields_map.get("basicAuth") {
                            Some(value) => value,
                            None => bail!("Missing field 'basicAuth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_certificate_tls_auth: {
                        let field_value = match fields_map.get("clientCertificateTlsAuth") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientCertificateTlsAuth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sasl_scram_256_auth: {
                        let field_value = match fields_map.get("saslScram256Auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'saslScram256Auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sasl_scram_512_auth: {
                        let field_value = match fields_map.get("saslScram512Auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'saslScram512Auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
