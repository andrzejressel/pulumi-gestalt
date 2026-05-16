#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionViewerCertificate {
    /// ARN of the [AWS Certificate Manager](https://aws.amazon.com/certificate-manager/) certificate that you wish to use with this distribution. Specify this, `cloudfront_default_certificate`, or `iam_certificate_id`.  The ACM certificate must be in  US-EAST-1.
    #[builder(into)]
    #[serde(rename = "acmCertificateArn")]
    pub r#acm_certificate_arn: Option<String>,
    /// `true` if you want viewers to use HTTPS to request your objects and you're using the CloudFront domain name for your distribution. Specify this, `acm_certificate_arn`, or `iam_certificate_id`.
    #[builder(into)]
    #[serde(rename = "cloudfrontDefaultCertificate")]
    pub r#cloudfront_default_certificate: Option<bool>,
    /// IAM certificate identifier of the custom viewer certificate for this distribution if you are using a custom domain. Specify this, `acm_certificate_arn`, or `cloudfront_default_certificate`.
    #[builder(into)]
    #[serde(rename = "iamCertificateId")]
    pub r#iam_certificate_id: Option<String>,
    /// Minimum version of the SSL protocol that you want CloudFront to use for HTTPS connections. Can only be set if `cloudfront_default_certificate = false`. See all possible values in [this](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/secure-connections-supported-viewer-protocols-ciphers.html) table under "Security policy." Some examples include: `TLSv1.2_2019` and `TLSv1.2_2021`. Default: `TLSv1`. **NOTE**: If you are using a custom certificate (specified with `acm_certificate_arn` or `iam_certificate_id`), and have specified `sni-only` in `ssl_support_method`, `TLSv1` or later must be specified. If you have specified `vip` in `ssl_support_method`, only `SSLv3` or `TLSv1` can be specified. If you have specified `cloudfront_default_certificate`, `TLSv1` must be specified.
    #[builder(into)]
    #[serde(rename = "minimumProtocolVersion")]
    pub r#minimum_protocol_version: Option<String>,
    /// How you want CloudFront to serve HTTPS requests. One of `vip`, `sni-only`, or `static-ip`. Required if you specify `acm_certificate_arn` or `iam_certificate_id`. **NOTE:** `vip` causes CloudFront to use a dedicated IP address and may incur extra charges.
    #[builder(into)]
    #[serde(rename = "sslSupportMethod")]
    pub r#ssl_support_method: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionViewerCertificate {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("acm_certificate_arn".to_string(), self.r#acm_certificate_arn.to_pulumi_value().await);
            map.insert("cloudfront_default_certificate".to_string(), self.r#cloudfront_default_certificate.to_pulumi_value().await);
            map.insert("iam_certificate_id".to_string(), self.r#iam_certificate_id.to_pulumi_value().await);
            map.insert("minimum_protocol_version".to_string(), self.r#minimum_protocol_version.to_pulumi_value().await);
            map.insert("ssl_support_method".to_string(), self.r#ssl_support_method.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionViewerCertificate {
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
                    r#acm_certificate_arn: {
                        let field_value = match fields_map.get("acm_certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'acm_certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cloudfront_default_certificate: {
                        let field_value = match fields_map.get("cloudfront_default_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudfront_default_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#iam_certificate_id: {
                        let field_value = match fields_map.get("iam_certificate_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'iam_certificate_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#minimum_protocol_version: {
                        let field_value = match fields_map.get("minimum_protocol_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_protocol_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ssl_support_method: {
                        let field_value = match fields_map.get("ssl_support_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_support_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
