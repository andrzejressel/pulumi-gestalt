#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DockerBuild {
    /// Custom host-to-IP mappings to use while building (format: "host:ip")
    #[builder(into)]
    #[serde(rename = "addHosts")]
    pub r#add_hosts: Option<Vec<String>>,
    /// An optional map of named build-time argument variables to set during the Docker build. This flag allows you to pass build-time variables that can be accessed like environment variables inside the RUN instruction.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<std::collections::HashMap<String, String>>,
    /// The version of the Docker builder.
    #[builder(into)]
    #[serde(rename = "builderVersion")]
    pub r#builder_version: Option<Box<super::types::BuilderVersion>>,
    /// A list of image names to use as build cache. Images provided must have a cache manifest. Must provide authentication to cache registry.
    #[builder(into)]
    #[serde(rename = "cacheFrom")]
    pub r#cache_from: Option<Box<super::types::CacheFrom>>,
    /// The path to the build context to use.
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: Option<String>,
    /// The path to the Dockerfile to use.
    #[builder(into)]
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Option<String>,
    /// Set the networking mode for RUN instructions
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// The architecture of the platform you want to build this image for, e.g. `linux/arm64`.
    #[builder(into)]
    #[serde(rename = "platform")]
    pub r#platform: Option<String>,
    /// The target of the Dockerfile to build
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
}
