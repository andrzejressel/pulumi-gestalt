#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowTemplatePlacementManagedClusterConfigSoftwareConfig {
    /// The version of software inside the cluster. It must be one of the supported [Dataproc Versions](https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#supported_dataproc_versions), such as "1.2" (including a subminor version, such as "1.2.29"), or the ["preview" version](https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#other_versions). If unspecified, it defaults to the latest Debian version.
    #[builder(into)]
    #[serde(rename = "imageVersion")]
    pub r#image_version: Option<String>,
    /// The set of components to activate on the cluster.
    #[builder(into)]
    #[serde(rename = "optionalComponents")]
    pub r#optional_components: Option<Vec<String>>,
    /// The properties to set on daemon config files.
    /// 
    /// Property keys are specified in `prefix:property` format, for example `core:hadoop.tmp.dir`. The following are supported prefixes and their mappings:
    /// 
    /// * capacity-scheduler: `capacity-scheduler.xml`
    /// * core: `core-site.xml`
    /// * distcp: `distcp-default.xml`
    /// * hdfs: `hdfs-site.xml`
    /// * hive: `hive-site.xml`
    /// * mapred: `mapred-site.xml`
    /// * pig: `pig.properties`
    /// * spark: `spark-defaults.conf`
    /// * yarn: `yarn-site.xml`
    /// 
    /// 
    /// For more information, see [Cluster properties](https://cloud.google.com/dataproc/docs/concepts/cluster-properties).
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowTemplatePlacementManagedClusterConfigSoftwareConfig {
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
                    "image_version",
                    &self.r#image_version,
                ),
                to_pulumi_object_field(
                    "optional_components",
                    &self.r#optional_components,
                ),
                to_pulumi_object_field(
                    "properties",
                    &self.r#properties,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowTemplatePlacementManagedClusterConfigSoftwareConfig {
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
                    r#image_version: {
                        let field_value = match fields_map.get("image_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#optional_components: {
                        let field_value = match fields_map.get("optional_components") {
                            Some(value) => value,
                            None => bail!("Missing field 'optional_components' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#properties: {
                        let field_value = match fields_map.get("properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
