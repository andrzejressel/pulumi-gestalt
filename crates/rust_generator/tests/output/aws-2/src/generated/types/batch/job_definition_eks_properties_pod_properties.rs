#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
