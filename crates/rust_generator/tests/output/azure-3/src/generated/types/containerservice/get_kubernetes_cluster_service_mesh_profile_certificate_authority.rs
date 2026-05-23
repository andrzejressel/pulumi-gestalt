#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterServiceMeshProfileCertificateAuthority {
    /// The certificate chain object name in Azure Key Vault.
    #[builder(into)]
    pub r#cert_chain_object_name: String,
    /// The intermediate certificate object name in Azure Key Vault.
    #[builder(into)]
    pub r#cert_object_name: String,
    /// The intermediate certificate private key object name in Azure Key Vault.
    #[builder(into)]
    pub r#key_object_name: String,
    /// The resource ID of the Key Vault.
    #[builder(into)]
    pub r#key_vault_id: String,
    /// The root certificate object name in Azure Key Vault.
    #[builder(into)]
    pub r#root_cert_object_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKubernetesClusterServiceMeshProfileCertificateAuthority {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "certChainObjectName",
                    &self.r#cert_chain_object_name,
                ),
                to_pulumi_object_field(
                    "certObjectName",
                    &self.r#cert_object_name,
                ),
                to_pulumi_object_field(
                    "keyObjectName",
                    &self.r#key_object_name,
                ),
                to_pulumi_object_field(
                    "keyVaultId",
                    &self.r#key_vault_id,
                ),
                to_pulumi_object_field(
                    "rootCertObjectName",
                    &self.r#root_cert_object_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetKubernetesClusterServiceMeshProfileCertificateAuthority {
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
                    r#cert_chain_object_name: {
                        let field_value = match fields_map.get("certChainObjectName") {
                            Some(value) => value,
                            None => bail!("Missing field 'certChainObjectName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cert_object_name: {
                        let field_value = match fields_map.get("certObjectName") {
                            Some(value) => value,
                            None => bail!("Missing field 'certObjectName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_object_name: {
                        let field_value = match fields_map.get("keyObjectName") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyObjectName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_vault_id: {
                        let field_value = match fields_map.get("keyVaultId") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyVaultId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_cert_object_name: {
                        let field_value = match fields_map.get("rootCertObjectName") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootCertObjectName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
