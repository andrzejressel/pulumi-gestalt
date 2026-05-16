#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation {
    /// SANs for a TLS validation context.
    #[builder(into)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Option<Box<super::super::types::appmesh::VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames>>,
    /// TLS validation context trust.
    #[builder(into)]
    #[serde(rename = "trust")]
    pub r#trust: Box<super::super::types::appmesh::VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrust>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("subject_alternative_names".to_string(), self.r#subject_alternative_names.to_pulumi_value().await);
            map.insert("trust".to_string(), self.r#trust.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpecBackendDefaultsClientPolicyTlsValidation {
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
                    r#subject_alternative_names: {
                        let field_value = match fields_map.get("subject_alternative_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_alternative_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#trust: {
                        let field_value = match fields_map.get("trust") {
                            Some(value) => value,
                            None => bail!("Missing field 'trust' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::appmesh::VirtualNodeSpecBackendDefaultsClientPolicyTlsValidationTrust> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
