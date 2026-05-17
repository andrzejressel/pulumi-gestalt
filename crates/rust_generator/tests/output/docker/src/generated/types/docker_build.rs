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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DockerBuild {
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
                    "add_hosts",
                    &self.r#add_hosts,
                ),
                to_pulumi_object_field(
                    "args",
                    &self.r#args,
                ),
                to_pulumi_object_field(
                    "builder_version",
                    &self.r#builder_version,
                ),
                to_pulumi_object_field(
                    "cache_from",
                    &self.r#cache_from,
                ),
                to_pulumi_object_field(
                    "context",
                    &self.r#context,
                ),
                to_pulumi_object_field(
                    "dockerfile",
                    &self.r#dockerfile,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "platform",
                    &self.r#platform,
                ),
                to_pulumi_object_field(
                    "target",
                    &self.r#target,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DockerBuild {
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
                    r#add_hosts: {
                        let field_value = match fields_map.get("add_hosts") {
                            Some(value) => value,
                            None => bail!("Missing field 'add_hosts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#args: {
                        let field_value = match fields_map.get("args") {
                            Some(value) => value,
                            None => bail!("Missing field 'args' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#builder_version: {
                        let field_value = match fields_map.get("builder_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'builder_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_from: {
                        let field_value = match fields_map.get("cache_from") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_from' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#context: {
                        let field_value = match fields_map.get("context") {
                            Some(value) => value,
                            None => bail!("Missing field 'context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dockerfile: {
                        let field_value = match fields_map.get("dockerfile") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerfile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#platform: {
                        let field_value = match fields_map.get("platform") {
                            Some(value) => value,
                            None => bail!("Missing field 'platform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
