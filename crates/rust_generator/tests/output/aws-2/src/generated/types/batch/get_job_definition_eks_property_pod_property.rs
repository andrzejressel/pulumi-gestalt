#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionEksPropertyPodProperty {
    /// The properties of the container that's used on the Amazon EKS pod. See containers below.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainer>,
    /// The DNS policy for the pod. The default value is ClusterFirst. If the hostNetwork parameter is not specified, the default is ClusterFirstWithHostNet. ClusterFirst indicates that any DNS query that does not match the configured cluster domain suffix is forwarded to the upstream nameserver inherited from the node.
    #[builder(into)]
    #[serde(rename = "dnsPolicy")]
    pub r#dns_policy: String,
    /// Indicates if the pod uses the hosts' network IP address. The default value is true. Setting this to false enables the Kubernetes pod networking model. Most AWS Batch workloads are egress-only and don't require the overhead of IP allocation for each pod for incoming connections.
    #[builder(into)]
    #[serde(rename = "hostNetwork")]
    pub r#host_network: bool,
    #[builder(into)]
    #[serde(rename = "imagePullSecrets")]
    pub r#image_pull_secrets: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyImagePullSecret>,
    /// Containers which run before application containers, always runs to completion, and must complete successfully before the next container starts. These containers are registered with the Amazon EKS Connector agent and persists the registration information in the Kubernetes backend data store. See containers below.
    #[builder(into)]
    #[serde(rename = "initContainers")]
    pub r#init_containers: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyInitContainer>,
    /// Metadata about the Kubernetes pod.
    #[builder(into)]
    #[serde(rename = "metadatas")]
    pub r#metadatas: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyMetadata>,
    /// The name of the service account that's used to run the pod.
    #[builder(into)]
    #[serde(rename = "serviceAccountName")]
    pub r#service_account_name: String,
    /// (Optional) Indicates if the processes in a container are shared, or visible, to other containers in the same pod.
    #[builder(into)]
    #[serde(rename = "shareProcessNamespace")]
    pub r#share_process_namespace: bool,
    /// A list of data volumes used in a job.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolume>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionEksPropertyPodProperty {
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
                "containers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#containers,
                )
                .await,
            );
            map.insert(
                "dns_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_policy,
                )
                .await,
            );
            map.insert(
                "host_network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_network,
                )
                .await,
            );
            map.insert(
                "image_pull_secrets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_pull_secrets,
                )
                .await,
            );
            map.insert(
                "init_containers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#init_containers,
                )
                .await,
            );
            map.insert(
                "metadatas".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metadatas,
                )
                .await,
            );
            map.insert(
                "service_account_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_name,
                )
                .await,
            );
            map.insert(
                "share_process_namespace".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#share_process_namespace,
                )
                .await,
            );
            map.insert(
                "volumes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volumes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionEksPropertyPodProperty {
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
                    r#containers: {
                        let field_value = match fields_map.get("containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_policy: {
                        let field_value = match fields_map.get("dns_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_network: {
                        let field_value = match fields_map.get("host_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_pull_secrets: {
                        let field_value = match fields_map.get("image_pull_secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_pull_secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#init_containers: {
                        let field_value = match fields_map.get("init_containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'init_containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadatas: {
                        let field_value = match fields_map.get("metadatas") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadatas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_name: {
                        let field_value = match fields_map.get("service_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#share_process_namespace: {
                        let field_value = match fields_map.get("share_process_namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'share_process_namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volumes: {
                        let field_value = match fields_map.get("volumes") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
