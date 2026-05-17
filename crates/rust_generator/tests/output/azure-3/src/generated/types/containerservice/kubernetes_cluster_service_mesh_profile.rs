#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterServiceMeshProfile {
    /// A `certificate_authority` block as defined below. When this property is specified, `key_vault_secrets_provider` is also required to be set. This configuration allows you to bring your own root certificate and keys for Istio CA in the Istio-based service mesh add-on for Azure Kubernetes Service.
    #[builder(into)]
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Option<Box<super::super::types::containerservice::KubernetesClusterServiceMeshProfileCertificateAuthority>>,
    /// Is Istio External Ingress Gateway enabled?
    /// 
    /// > **NOTE:** Currently only one Internal Ingress Gateway and one External Ingress Gateway are allowed per cluster
    #[builder(into)]
    #[serde(rename = "externalIngressGatewayEnabled")]
    pub r#external_ingress_gateway_enabled: Option<bool>,
    /// Is Istio Internal Ingress Gateway enabled?
    #[builder(into)]
    #[serde(rename = "internalIngressGatewayEnabled")]
    pub r#internal_ingress_gateway_enabled: Option<bool>,
    /// The mode of the service mesh. Possible value is `Istio`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    /// Specify 1 or 2 Istio control plane revisions for managing minor upgrades using the canary upgrade process. For example, create the resource with `revisions` set to `["asm-1-20"]`, or leave it empty (the `revisions` will only be known after apply). To start the canary upgrade, change `revisions` to `["asm-1-20", "asm-1-21"]`. To roll back the canary upgrade, revert to `["asm-1-20"]`. To confirm the upgrade, change to `["asm-1-21"]`.
    /// 
    /// > **NOTE:** Upgrading to a new (canary) revision does not affect existing sidecar proxies. You need to apply the canary revision label to selected namespaces and restart pods with kubectl to inject the new sidecar proxy. [Learn more](https://istio.io/latest/docs/setup/upgrade/canary/#data-plane).
    #[builder(into)]
    #[serde(rename = "revisions")]
    pub r#revisions: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterServiceMeshProfile {
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
                "certificate_authority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_authority,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterServiceMeshProfile {
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
                    r#certificate_authority: {
                        let field_value = match fields_map.get("certificate_authority") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_authority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
