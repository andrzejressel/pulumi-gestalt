#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobDefinitionEksPropertiesPodProperties {
    /// Properties of the container that's used on the Amazon EKS pod. See containers below.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Box<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesContainers>,
    /// DNS policy for the pod. The default value is `ClusterFirst`. If the `host_network` argument is not specified, the default is `ClusterFirstWithHostNet`. `ClusterFirst` indicates that any DNS query that does not match the configured cluster domain suffix is forwarded to the upstream nameserver inherited from the node. For more information, see Pod's DNS policy in the Kubernetes documentation.
    #[builder(into)]
    #[serde(rename = "dnsPolicy")]
    pub r#dns_policy: Option<String>,
    /// Whether the pod uses the hosts' network IP address. The default value is `true`. Setting this to `false` enables the Kubernetes pod networking model. Most AWS Batch workloads are egress-only and don't require the overhead of IP allocation for each pod for incoming connections.
    #[builder(into)]
    #[serde(rename = "hostNetwork")]
    pub r#host_network: Option<bool>,
    /// List of Kubernetes secret resources. See `image_pull_secret` below.
    #[builder(into)]
    #[serde(rename = "imagePullSecrets")]
    pub r#image_pull_secrets: Option<Vec<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesImagePullSecret>>,
    /// Containers which run before application containers, always runs to completion, and must complete successfully before the next container starts. These containers are registered with the Amazon EKS Connector agent and persists the registration information in the Kubernetes backend data store. See containers below.
    #[builder(into)]
    #[serde(rename = "initContainers")]
    pub r#init_containers: Option<Vec<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesInitContainer>>,
    /// Metadata about the Kubernetes pod.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Option<Box<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesMetadata>>,
    /// Name of the service account that's used to run the pod.
    #[builder(into)]
    #[serde(rename = "serviceAccountName")]
    pub r#service_account_name: Option<String>,
    /// Indicates if the processes in a container are shared, or visible, to other containers in the same pod.
    #[builder(into)]
    #[serde(rename = "shareProcessNamespace")]
    pub r#share_process_namespace: Option<bool>,
    /// Volumes for a job definition that uses Amazon EKS resources. AWS Batch supports emptyDir, hostPath, and secret volume types.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Option<Vec<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesVolume>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobDefinitionEksPropertiesPodProperties {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("containers".to_string(), self.r#containers.to_pulumi_value().await);
            map.insert("dns_policy".to_string(), self.r#dns_policy.to_pulumi_value().await);
            map.insert("host_network".to_string(), self.r#host_network.to_pulumi_value().await);
            map.insert("image_pull_secrets".to_string(), self.r#image_pull_secrets.to_pulumi_value().await);
            map.insert("init_containers".to_string(), self.r#init_containers.to_pulumi_value().await);
            map.insert("metadata".to_string(), self.r#metadata.to_pulumi_value().await);
            map.insert("service_account_name".to_string(), self.r#service_account_name.to_pulumi_value().await);
            map.insert("share_process_namespace".to_string(), self.r#share_process_namespace.to_pulumi_value().await);
            map.insert("volumes".to_string(), self.r#volumes.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobDefinitionEksPropertiesPodProperties {
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
                    r#containers: {
                        let field_value = match fields_map.get("containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesContainers> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dns_policy: {
                        let field_value = match fields_map.get("dns_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#host_network: {
                        let field_value = match fields_map.get("host_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_pull_secrets: {
                        let field_value = match fields_map.get("image_pull_secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_pull_secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesImagePullSecret>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#init_containers: {
                        let field_value = match fields_map.get("init_containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'init_containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesInitContainer>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#metadata: {
                        let field_value = match fields_map.get("metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesMetadata>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service_account_name: {
                        let field_value = match fields_map.get("service_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#share_process_namespace: {
                        let field_value = match fields_map.get("share_process_namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'share_process_namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#volumes: {
                        let field_value = match fields_map.get("volumes") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesVolume>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
