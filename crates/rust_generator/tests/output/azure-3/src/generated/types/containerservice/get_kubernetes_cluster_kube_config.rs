#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterKubeConfig {
    /// Base64 encoded public certificate used by clients to authenticate to the Kubernetes cluster.
    #[builder(into)]
    pub r#client_certificate: String,
    /// Base64 encoded private key used by clients to authenticate to the Kubernetes cluster.
    #[builder(into)]
    pub r#client_key: String,
    /// Base64 encoded public CA certificate used as the root of trust for the Kubernetes cluster.
    #[builder(into)]
    pub r#cluster_ca_certificate: String,
    /// The Kubernetes cluster server host.
    #[builder(into)]
    pub r#host: String,
    /// A password or token used to authenticate to the Kubernetes cluster.
    #[builder(into)]
    pub r#password: String,
    /// A username used to authenticate to the Kubernetes cluster.
    #[builder(into)]
    pub r#username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKubernetesClusterKubeConfig {
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
                    "clientCertificate",
                    &self.r#client_certificate,
                ),
                to_pulumi_object_field(
                    "clientKey",
                    &self.r#client_key,
                ),
                to_pulumi_object_field(
                    "clusterCaCertificate",
                    &self.r#cluster_ca_certificate,
                ),
                to_pulumi_object_field(
                    "host",
                    &self.r#host,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
                to_pulumi_object_field(
                    "username",
                    &self.r#username,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetKubernetesClusterKubeConfig {
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
                        let field_value = match fields_map.get("clientCertificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientCertificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_key: {
                        let field_value = match fields_map.get("clientKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_ca_certificate: {
                        let field_value = match fields_map.get("clusterCaCertificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterCaCertificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host: {
                        let field_value = match fields_map.get("host") {
                            Some(value) => value,
                            None => bail!("Missing field 'host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
