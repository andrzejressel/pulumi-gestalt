#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualGatewaySpecBackendDefaultsClientPolicyTls {
    /// Listener's TLS certificate.
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificate>>,
    /// Whether the policy is enforced. Default is `true`.
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: Option<bool>,
    /// One or more ports that the policy is enforced for.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<i32>>,
    /// Listener's Transport Layer Security (TLS) validation context.
    #[builder(into)]
    #[serde(rename = "validation")]
    pub r#validation: Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualGatewaySpecBackendDefaultsClientPolicyTls {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("certificate".to_string(), self.r#certificate.to_pulumi_value().await);
            map.insert("enforce".to_string(), self.r#enforce.to_pulumi_value().await);
            map.insert("ports".to_string(), self.r#ports.to_pulumi_value().await);
            map.insert("validation".to_string(), self.r#validation.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualGatewaySpecBackendDefaultsClientPolicyTls {
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
                    r#certificate: {
                        let field_value = match fields_map.get("certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificate>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enforce: {
                        let field_value = match fields_map.get("enforce") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ports: {
                        let field_value = match fields_map.get("ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<i32>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#validation: {
                        let field_value = match fields_map.get("validation") {
                            Some(value) => value,
                            None => bail!("Missing field 'validation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
