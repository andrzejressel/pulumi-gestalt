#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDomainEndpointOptions {
    /// Fully qualified domain for your custom endpoint.
    #[builder(into)]
    #[serde(rename = "customEndpoint")]
    pub r#custom_endpoint: Option<String>,
    /// ACM certificate ARN for your custom endpoint.
    #[builder(into)]
    #[serde(rename = "customEndpointCertificateArn")]
    pub r#custom_endpoint_certificate_arn: Option<String>,
    /// Whether to enable custom endpoint for the OpenSearch domain.
    #[builder(into)]
    #[serde(rename = "customEndpointEnabled")]
    pub r#custom_endpoint_enabled: Option<bool>,
    /// Whether or not to require HTTPS. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enforceHttps")]
    pub r#enforce_https: Option<bool>,
    /// Name of the TLS security policy that needs to be applied to the HTTPS endpoint. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_DomainEndpointOptions.html#opensearchservice-Type-DomainEndpointOptions-TLSSecurityPolicy). Pulumi will only perform drift detection if a configuration value is provided.
    #[builder(into)]
    #[serde(rename = "tlsSecurityPolicy")]
    pub r#tls_security_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDomainEndpointOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("custom_endpoint".to_string(), self.r#custom_endpoint.to_pulumi_value().await);
            map.insert("custom_endpoint_certificate_arn".to_string(), self.r#custom_endpoint_certificate_arn.to_pulumi_value().await);
            map.insert("custom_endpoint_enabled".to_string(), self.r#custom_endpoint_enabled.to_pulumi_value().await);
            map.insert("enforce_https".to_string(), self.r#enforce_https.to_pulumi_value().await);
            map.insert("tls_security_policy".to_string(), self.r#tls_security_policy.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDomainEndpointOptions {
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
                    r#custom_endpoint: {
                        let field_value = match fields_map.get("custom_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_endpoint_certificate_arn: {
                        let field_value = match fields_map.get("custom_endpoint_certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_endpoint_certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_endpoint_enabled: {
                        let field_value = match fields_map.get("custom_endpoint_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_endpoint_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enforce_https: {
                        let field_value = match fields_map.get("enforce_https") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce_https' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tls_security_policy: {
                        let field_value = match fields_map.get("tls_security_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_security_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
