#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkstationClusterPrivateClusterConfig {
    /// Additional project IDs that are allowed to attach to the workstation cluster's service attachment.
    /// By default, the workstation cluster's project and the VPC host project (if different) are allowed.
    #[builder(into)]
    #[serde(rename = "allowedProjects")]
    pub r#allowed_projects: Option<Vec<String>>,
    /// (Output)
    /// Hostname for the workstation cluster.
    /// This field will be populated only when private endpoint is enabled.
    /// To access workstations in the cluster, create a new DNS zone mapping this domain name to an internal IP address and a forwarding rule mapping that address to the service attachment.
    #[builder(into)]
    #[serde(rename = "clusterHostname")]
    pub r#cluster_hostname: Option<String>,
    /// Whether Workstations endpoint is private.
    #[builder(into)]
    #[serde(rename = "enablePrivateEndpoint")]
    pub r#enable_private_endpoint: bool,
    /// (Output)
    /// Service attachment URI for the workstation cluster.
    /// The service attachment is created when private endpoint is enabled.
    /// To access workstations in the cluster, configure access to the managed service using (Private Service Connect)[https://cloud.google.com/vpc/docs/configure-private-service-connect-services].
    #[builder(into)]
    #[serde(rename = "serviceAttachmentUri")]
    pub r#service_attachment_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkstationClusterPrivateClusterConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allowed_projects",
                    &self.r#allowed_projects,
                ),
                to_pulumi_object_field(
                    "cluster_hostname",
                    &self.r#cluster_hostname,
                ),
                to_pulumi_object_field(
                    "enable_private_endpoint",
                    &self.r#enable_private_endpoint,
                ),
                to_pulumi_object_field(
                    "service_attachment_uri",
                    &self.r#service_attachment_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkstationClusterPrivateClusterConfig {
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
                    r#allowed_projects: {
                        let field_value = match fields_map.get("allowed_projects") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_projects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_hostname: {
                        let field_value = match fields_map.get("cluster_hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_private_endpoint: {
                        let field_value = match fields_map.get("enable_private_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_private_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_attachment_uri: {
                        let field_value = match fields_map.get("service_attachment_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_attachment_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
