#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrust {
    /// TLS validation context trust for an AWS Certificate Manager (ACM) certificate.
    #[builder(into)]
    #[serde(rename = "acm")]
    pub r#acm: Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustAcm>>,
    /// TLS validation context trust for a local file certificate.
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustFile>>,
    /// TLS validation context trust for a [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
    #[builder(into)]
    #[serde(rename = "sds")]
    pub r#sds: Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustSds>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrust {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("acm".to_string(), self.r#acm.to_pulumi_value().await);
            map.insert("file".to_string(), self.r#file.to_pulumi_value().await);
            map.insert("sds".to_string(), self.r#sds.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrust {
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
                    r#acm: {
                        let field_value = match fields_map.get("acm") {
                            Some(value) => value,
                            None => bail!("Missing field 'acm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustAcm>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#file: {
                        let field_value = match fields_map.get("file") {
                            Some(value) => value,
                            None => bail!("Missing field 'file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustFile>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sds: {
                        let field_value = match fields_map.get("sds") {
                            Some(value) => value,
                            None => bail!("Missing field 'sds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustSds>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
