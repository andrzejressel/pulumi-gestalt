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
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "certificate_authorities",
                    &self.r#certificate_authorities,
                ),
                to_pulumi_object_field(
                    "external_ingress_gateway_enabled",
                    &self.r#external_ingress_gateway_enabled,
                ),
                to_pulumi_object_field(
                    "internal_ingress_gateway_enabled",
                    &self.r#internal_ingress_gateway_enabled,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "revisions",
                    &self.r#revisions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
