#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterMasterAuth {
    /// Base64 encoded public certificate used by clients to authenticate to the cluster endpoint.
    #[builder(into)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: String,
    /// Whether client certificate authorization is enabled for this cluster.
    #[builder(into)]
    #[serde(rename = "clientCertificateConfigs")]
    pub r#client_certificate_configs: Vec<super::super::types::container::GetClusterMasterAuthClientCertificateConfig>,
    /// Base64 encoded private key used by clients to authenticate to the cluster endpoint.
    #[builder(into)]
    #[serde(rename = "clientKey")]
    pub r#client_key: String,
    /// Base64 encoded public certificate that is the root of trust for the cluster.
    #[builder(into)]
    #[serde(rename = "clusterCaCertificate")]
    pub r#cluster_ca_certificate: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterMasterAuth {
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
                    "client_certificate",
                    &self.r#client_certificate,
                ),
                to_pulumi_object_field(
                    "client_certificate_configs",
                    &self.r#client_certificate_configs,
                ),
                to_pulumi_object_field(
                    "client_key",
                    &self.r#client_key,
                ),
                to_pulumi_object_field(
                    "cluster_ca_certificate",
                    &self.r#cluster_ca_certificate,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterMasterAuth {
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
                    r#client_certificate: {
                        let field_value = match fields_map.get("client_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_certificate_configs: {
                        let field_value = match fields_map.get("client_certificate_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_key: {
                        let field_value = match fields_map.get("client_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_ca_certificate: {
                        let field_value = match fields_map.get("cluster_ca_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_ca_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
