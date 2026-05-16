#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpcOriginVpcOriginEndpointConfig {
    /// The VPC origin ARN.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// The HTTP port for the CloudFront VPC origin endpoint configuration.
    #[builder(into)]
    #[serde(rename = "httpPort")]
    pub r#http_port: i32,
    /// The HTTPS port for the CloudFront VPC origin endpoint configuration.
    #[builder(into)]
    #[serde(rename = "httpsPort")]
    pub r#https_port: i32,
    /// The name of the CloudFront VPC origin endpoint configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The origin protocol policy for the CloudFront VPC origin endpoint configuration.
    #[builder(into)]
    #[serde(rename = "originProtocolPolicy")]
    pub r#origin_protocol_policy: String,
    /// A complex type that contains information about the SSL/TLS protocols that CloudFront can use when establishing an HTTPS connection with your origin.
    #[builder(into)]
    #[serde(rename = "originSslProtocols")]
    pub r#origin_ssl_protocols: Option<Box<super::super::types::cloudfront::VpcOriginVpcOriginEndpointConfigOriginSslProtocols>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpcOriginVpcOriginEndpointConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("arn".to_string(), self.r#arn.to_pulumi_value().await);
            map.insert("http_port".to_string(), self.r#http_port.to_pulumi_value().await);
            map.insert("https_port".to_string(), self.r#https_port.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("origin_protocol_policy".to_string(), self.r#origin_protocol_policy.to_pulumi_value().await);
            map.insert("origin_ssl_protocols".to_string(), self.r#origin_ssl_protocols.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VpcOriginVpcOriginEndpointConfig {
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
                    r#arn: {
                        let field_value = match fields_map.get("arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#http_port: {
                        let field_value = match fields_map.get("http_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#https_port: {
                        let field_value = match fields_map.get("https_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#origin_protocol_policy: {
                        let field_value = match fields_map.get("origin_protocol_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_protocol_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#origin_ssl_protocols: {
                        let field_value = match fields_map.get("origin_ssl_protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_ssl_protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::cloudfront::VpcOriginVpcOriginEndpointConfigOriginSslProtocols>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
