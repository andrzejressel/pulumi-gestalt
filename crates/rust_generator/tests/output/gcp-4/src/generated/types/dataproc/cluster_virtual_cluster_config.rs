#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterVirtualClusterConfig {
    /// Configuration of auxiliary services used by this cluster. 
    /// Structure defined below.
    #[builder(into)]
    #[serde(rename = "auxiliaryServicesConfig")]
    pub r#auxiliary_services_config: Option<Box<super::super::types::dataproc::ClusterVirtualClusterConfigAuxiliaryServicesConfig>>,
    /// The configuration for running the Dataproc cluster on Kubernetes.
    /// Structure defined below.
    /// - - -
    #[builder(into)]
    #[serde(rename = "kubernetesClusterConfig")]
    pub r#kubernetes_cluster_config: Option<Box<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfig>>,
    /// The Cloud Storage staging bucket used to stage files,
    /// such as Hadoop jars, between client machines and the cluster.
    /// Note: If you don't explicitly specify a `staging_bucket`
    /// then GCP will auto create / assign one for you. However, you are not guaranteed
    /// an auto generated bucket which is solely dedicated to your cluster; it may be shared
    /// with other clusters in the same region/zone also choosing to use the auto generation
    /// option.
    #[builder(into)]
    #[serde(rename = "stagingBucket")]
    pub r#staging_bucket: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterVirtualClusterConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auxiliary_services_config",
                    &self.r#auxiliary_services_config,
                ),
                to_pulumi_object_field(
                    "kubernetes_cluster_config",
                    &self.r#kubernetes_cluster_config,
                ),
                to_pulumi_object_field(
                    "staging_bucket",
                    &self.r#staging_bucket,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterVirtualClusterConfig {
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
                    r#auxiliary_services_config: {
                        let field_value = match fields_map.get("auxiliary_services_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'auxiliary_services_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kubernetes_cluster_config: {
                        let field_value = match fields_map.get("kubernetes_cluster_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubernetes_cluster_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#staging_bucket: {
                        let field_value = match fields_map.get("staging_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'staging_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
