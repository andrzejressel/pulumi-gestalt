#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointAuthenticationOption {
    /// The ID of the Active Directory to be used for authentication if type is `directory-service-authentication`.
    #[builder(into)]
    pub r#active_directory_id: Option<String>,
    /// The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in AWS Certificate Manager (ACM). Only necessary when type is set to `certificate-authentication`.
    #[builder(into)]
    pub r#root_certificate_chain_arn: Option<String>,
    /// The ARN of the IAM SAML identity provider if type is `federated-authentication`.
    #[builder(into)]
    pub r#saml_provider_arn: Option<String>,
    /// The ARN of the IAM SAML identity provider for the self service portal if type is `federated-authentication`.
    #[builder(into)]
    pub r#self_service_saml_provider_arn: Option<String>,
    /// The type of client authentication to be used. Specify `certificate-authentication` to use certificate-based authentication, `directory-service-authentication` to use Active Directory authentication, or `federated-authentication` to use Federated Authentication via SAML 2.0.
    #[builder(into)]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointAuthenticationOption {
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
                    "activeDirectoryId",
                    &self.r#active_directory_id,
                ),
                to_pulumi_object_field(
                    "rootCertificateChainArn",
                    &self.r#root_certificate_chain_arn,
                ),
                to_pulumi_object_field(
                    "samlProviderArn",
                    &self.r#saml_provider_arn,
                ),
                to_pulumi_object_field(
                    "selfServiceSamlProviderArn",
                    &self.r#self_service_saml_provider_arn,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointAuthenticationOption {
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
                    r#active_directory_id: {
                        let field_value = match fields_map.get("activeDirectoryId") {
                            Some(value) => value,
                            None => bail!("Missing field 'activeDirectoryId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_certificate_chain_arn: {
                        let field_value = match fields_map.get("rootCertificateChainArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootCertificateChainArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#saml_provider_arn: {
                        let field_value = match fields_map.get("samlProviderArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'samlProviderArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_service_saml_provider_arn: {
                        let field_value = match fields_map.get("selfServiceSamlProviderArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'selfServiceSamlProviderArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
