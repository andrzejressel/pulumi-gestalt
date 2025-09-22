#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InferenceClusterSsl {
    /// The certificate for the SSL configuration.Conflicts with `ssl[0].leaf_domain_label`,`ssl[0].overwrite_existing_domain`. Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "cert")]
    pub r#cert: Option<String>,
    /// The cname of the SSL configuration.Conflicts with `ssl[0].leaf_domain_label`,`ssl[0].overwrite_existing_domain`. Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "cname")]
    pub r#cname: Option<String>,
    /// The key content for the SSL configuration.Conflicts with `ssl[0].leaf_domain_label`,`ssl[0].overwrite_existing_domain`. Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The leaf domain label for the SSL configuration. Conflicts with `ssl[0].cert`,`ssl[0].key`,`ssl[0].cname`. Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "leafDomainLabel")]
    pub r#leaf_domain_label: Option<String>,
    /// Whether or not to overwrite existing leaf domain. Conflicts with `ssl[0].cert`,`ssl[0].key`,`ssl[0].cname` Changing this forces a new Machine Learning Inference Cluster to be created. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "overwriteExistingDomain")]
    pub r#overwrite_existing_domain: Option<bool>,
}
