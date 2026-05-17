#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterServiceMeshProfile {
    /// A `certificate_authority` block as documented below.
    #[builder(into)]
    #[serde(rename = "certificateAuthorities")]
    pub r#certificate_authorities: Vec<super::super::types::containerservice::GetKubernetesClusterServiceMeshProfileCertificateAuthority>,
    /// Is Istio External Ingress Gateway enabled?
    #[builder(into)]
    #[serde(rename = "externalIngressGatewayEnabled")]
    pub r#external_ingress_gateway_enabled: bool,
    /// Is Istio Internal Ingress Gateway enabled?
    #[builder(into)]
    #[serde(rename = "internalIngressGatewayEnabled")]
    pub r#internal_ingress_gateway_enabled: bool,
    /// The mode of the service mesh.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    /// List of revisions of the Istio control plane. When an upgrade is not in progress, this holds one value. When canary upgrade is in progress, this can only hold two consecutive values. Learn More.
    #[builder(into)]
    #[serde(rename = "revisions")]
    pub r#revisions: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKubernetesClusterServiceMeshProfile {
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
                "certificate_authorities".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_authorities,
                )
                .await,
            );
            map.insert(
                "external_ingress_gateway_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#external_ingress_gateway_enabled,
                )
                .await,
            );
            map.insert(
                "internal_ingress_gateway_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#internal_ingress_gateway_enabled,
                )
                .await,
            );
            map.insert(
                "mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mode,
                )
                .await,
            );
            map.insert(
                "revisions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#revisions,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetKubernetesClusterServiceMeshProfile {
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
                    r#certificate_authorities: {
                        let field_value = match fields_map.get("certificate_authorities") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_authorities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_ingress_gateway_enabled: {
                        let field_value = match fields_map.get("external_ingress_gateway_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_ingress_gateway_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#internal_ingress_gateway_enabled: {
                        let field_value = match fields_map.get("internal_ingress_gateway_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'internal_ingress_gateway_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revisions: {
                        let field_value = match fields_map.get("revisions") {
                            Some(value) => value,
                            None => bail!("Missing field 'revisions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
