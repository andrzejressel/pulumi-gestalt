#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AttachedClusterWorkloadIdentityConfig {
    /// The ID of the OIDC Identity Provider (IdP) associated to
    /// the Workload Identity Pool.
    #[builder(into)]
    #[serde(rename = "identityProvider")]
    pub r#identity_provider: Option<String>,
    /// The OIDC issuer URL for this cluster.
    #[builder(into)]
    #[serde(rename = "issuerUri")]
    pub r#issuer_uri: Option<String>,
    /// The Workload Identity Pool associated to the cluster.
    #[builder(into)]
    #[serde(rename = "workloadPool")]
    pub r#workload_pool: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AttachedClusterWorkloadIdentityConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "identity_provider".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identity_provider,
                )
                .await,
            );
            map.insert(
                "issuer_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#issuer_uri,
                )
                .await,
            );
            map.insert(
                "workload_pool".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workload_pool,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AttachedClusterWorkloadIdentityConfig {
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
                    r#identity_provider: {
                        let field_value = match fields_map.get("identity_provider") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_provider' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer_uri: {
                        let field_value = match fields_map.get("issuer_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workload_pool: {
                        let field_value = match fields_map.get("workload_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'workload_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
