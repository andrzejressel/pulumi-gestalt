#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerTlsPolicyServerCertificate {
    /// Optional if policy is to be used with Traffic Director. For external HTTPS load balancer must be empty.
    /// Defines a mechanism to provision server identity (public and private keys). Cannot be combined with allowOpen as a permissive mode that allows both plain text and TLS is not supported.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "certificateProviderInstance")]
    pub r#certificate_provider_instance: Option<Box<super::super::types::networksecurity::ServerTlsPolicyServerCertificateCertificateProviderInstance>>,
    /// gRPC specific configuration to access the gRPC server to obtain the cert and private key.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "grpcEndpoint")]
    pub r#grpc_endpoint: Option<Box<super::super::types::networksecurity::ServerTlsPolicyServerCertificateGrpcEndpoint>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServerTlsPolicyServerCertificate {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "certificate_provider_instance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_provider_instance,
                )
                .await,
            );
            map.insert(
                "grpc_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#grpc_endpoint,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServerTlsPolicyServerCertificate {
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
                    r#certificate_provider_instance: {
                        let field_value = match fields_map.get("certificate_provider_instance") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_provider_instance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#grpc_endpoint: {
                        let field_value = match fields_map.get("grpc_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'grpc_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
