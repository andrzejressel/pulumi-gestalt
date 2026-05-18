#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackendServiceFabricCluster {
    /// The client certificate resource id for the management endpoint.
    /// 
    /// > **Note:** At least one of `client_certificate_thumbprint`, and `client_certificate_id` must be set.
    /// >
    #[builder(into)]
    #[serde(rename = "clientCertificateId")]
    pub r#client_certificate_id: Option<String>,
    /// The client certificate thumbprint for the management endpoint.
    #[builder(into)]
    #[serde(rename = "clientCertificateThumbprint")]
    pub r#client_certificate_thumbprint: Option<String>,
    /// A list of cluster management endpoints.
    #[builder(into)]
    #[serde(rename = "managementEndpoints")]
    pub r#management_endpoints: Vec<String>,
    /// The maximum number of retries when attempting resolve the partition.
    #[builder(into)]
    #[serde(rename = "maxPartitionResolutionRetries")]
    pub r#max_partition_resolution_retries: i32,
    /// A list of thumbprints of the server certificates of the Service Fabric cluster.
    #[builder(into)]
    #[serde(rename = "serverCertificateThumbprints")]
    pub r#server_certificate_thumbprints: Option<Vec<String>>,
    /// One or more `server_x509_name` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "serverX509Names")]
    pub r#server_x_509_names: Option<Vec<super::super::types::apimanagement::BackendServiceFabricClusterServerX509Name>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackendServiceFabricCluster {
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
                    "client_certificate_id",
                    &self.r#client_certificate_id,
                ),
                to_pulumi_object_field(
                    "client_certificate_thumbprint",
                    &self.r#client_certificate_thumbprint,
                ),
                to_pulumi_object_field(
                    "management_endpoints",
                    &self.r#management_endpoints,
                ),
                to_pulumi_object_field(
                    "max_partition_resolution_retries",
                    &self.r#max_partition_resolution_retries,
                ),
                to_pulumi_object_field(
                    "server_certificate_thumbprints",
                    &self.r#server_certificate_thumbprints,
                ),
                to_pulumi_object_field(
                    "server_x_509_names",
                    &self.r#server_x_509_names,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackendServiceFabricCluster {
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
                    r#client_certificate_id: {
                        let field_value = match fields_map.get("client_certificate_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_certificate_thumbprint: {
                        let field_value = match fields_map.get("client_certificate_thumbprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate_thumbprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#management_endpoints: {
                        let field_value = match fields_map.get("management_endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'management_endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_partition_resolution_retries: {
                        let field_value = match fields_map.get("max_partition_resolution_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_partition_resolution_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_certificate_thumbprints: {
                        let field_value = match fields_map.get("server_certificate_thumbprints") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_certificate_thumbprints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_x_509_names: {
                        let field_value = match fields_map.get("server_x_509_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_x_509_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
