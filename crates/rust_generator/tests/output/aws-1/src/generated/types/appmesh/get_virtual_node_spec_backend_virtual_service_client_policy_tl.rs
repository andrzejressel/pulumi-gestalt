#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNodeSpecBackendVirtualServiceClientPolicyTl {
    #[builder(into)]
    #[serde(rename = "certificates")]
    pub r#certificates: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificate>,
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: bool,
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Vec<i32>,
    #[builder(into)]
    #[serde(rename = "validations")]
    pub r#validations: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidation>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNodeSpecBackendVirtualServiceClientPolicyTl {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("certificates".to_string(), self.r#certificates.to_pulumi_value().await);
            map.insert("enforce".to_string(), self.r#enforce.to_pulumi_value().await);
            map.insert("ports".to_string(), self.r#ports.to_pulumi_value().await);
            map.insert("validations".to_string(), self.r#validations.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNodeSpecBackendVirtualServiceClientPolicyTl {
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
                    r#certificates: {
                        let field_value = match fields_map.get("certificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificate> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enforce: {
                        let field_value = match fields_map.get("enforce") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ports: {
                        let field_value = match fields_map.get("ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#validations: {
                        let field_value = match fields_map.get("validations") {
                            Some(value) => value,
                            None => bail!("Missing field 'validations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidation> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
