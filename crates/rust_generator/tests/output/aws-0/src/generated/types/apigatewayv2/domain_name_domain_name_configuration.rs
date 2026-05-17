#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainNameDomainNameConfiguration {
    /// ARN of an AWS-managed certificate that will be used by the endpoint for the domain name. AWS Certificate Manager is the only supported source. Use the `aws.acm.Certificate` resource to configure an ACM certificate.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: String,
    /// Endpoint type. Valid values: `REGIONAL`.
    #[builder(into)]
    #[serde(rename = "endpointType")]
    pub r#endpoint_type: String,
    /// Amazon Route 53 Hosted Zone ID of the endpoint.
    #[builder(into)]
    #[serde(rename = "hostedZoneId")]
    pub r#hosted_zone_id: Option<String>,
    /// ARN of the AWS-issued certificate used to validate custom domain ownership (when `certificate_arn` is issued via an ACM Private CA or `mutual_tls_authentication` is configured with an ACM-imported certificate.)
    #[builder(into)]
    #[serde(rename = "ownershipVerificationCertificateArn")]
    pub r#ownership_verification_certificate_arn: Option<String>,
    /// Transport Layer Security (TLS) version of the [security policy](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-custom-domain-tls-version.html) for the domain name. Valid values: `TLS_1_2`.
    #[builder(into)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: String,
    /// Target domain name.
    #[builder(into)]
    #[serde(rename = "targetDomainName")]
    pub r#target_domain_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainNameDomainNameConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "certificate_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_arn,
                )
                .await,
            );
            map.insert(
                "endpoint_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint_type,
                )
                .await,
            );
            map.insert(
                "hosted_zone_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hosted_zone_id,
                )
                .await,
            );
            map.insert(
                "ownership_verification_certificate_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ownership_verification_certificate_arn,
                )
                .await,
            );
            map.insert(
                "security_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_policy,
                )
                .await,
            );
            map.insert(
                "target_domain_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_domain_name,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainNameDomainNameConfiguration {
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
                    r#certificate_arn: {
                        let field_value = match fields_map.get("certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoint_type: {
                        let field_value = match fields_map.get("endpoint_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hosted_zone_id: {
                        let field_value = match fields_map.get("hosted_zone_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'hosted_zone_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ownership_verification_certificate_arn: {
                        let field_value = match fields_map.get("ownership_verification_certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'ownership_verification_certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_policy: {
                        let field_value = match fields_map.get("security_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_domain_name: {
                        let field_value = match fields_map.get("target_domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
