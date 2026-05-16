#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrust {
    #[builder(into)]
    #[serde(rename = "acms")]
    pub r#acms: Vec<super::super::types::appmesh::GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustAcm>,
    #[builder(into)]
    #[serde(rename = "files")]
    pub r#files: Vec<super::super::types::appmesh::GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustFile>,
    #[builder(into)]
    #[serde(rename = "sds")]
    pub r#sds: Vec<super::super::types::appmesh::GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustSd>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrust {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("acms".to_string(), self.r#acms.to_pulumi_value().await);
            map.insert("files".to_string(), self.r#files.to_pulumi_value().await);
            map.insert("sds".to_string(), self.r#sds.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrust {
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
                    r#acms: {
                        let field_value = match fields_map.get("acms") {
                            Some(value) => value,
                            None => bail!("Missing field 'acms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustAcm> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#files: {
                        let field_value = match fields_map.get("files") {
                            Some(value) => value,
                            None => bail!("Missing field 'files' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustFile> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sds: {
                        let field_value = match fields_map.get("sds") {
                            Some(value) => value,
                            None => bail!("Missing field 'sds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualGatewaySpecBackendDefaultClientPolicyTlValidationTrustSd> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
