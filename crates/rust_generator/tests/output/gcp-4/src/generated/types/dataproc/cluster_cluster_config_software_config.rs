#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfigSoftwareConfig {
    /// The Cloud Dataproc image version to use
    /// for the cluster - this controls the sets of software versions
    /// installed onto the nodes when you create clusters. If not specified, defaults to the
    /// latest version. For a list of valid versions see
    /// [Cloud Dataproc versions](https://cloud.google.com/dataproc/docs/concepts/dataproc-versions)
    #[builder(into)]
    #[serde(rename = "imageVersion")]
    pub r#image_version: Option<String>,
    /// The set of optional components to activate on the cluster. See [Available Optional Components](https://cloud.google.com/dataproc/docs/concepts/components/overview#available_optional_components).
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "optionalComponents")]
    pub r#optional_components: Option<Vec<String>>,
    /// A list of override and additional properties (key/value pairs)
    /// used to modify various aspects of the common configuration files used when creating
    /// a cluster. For a list of valid properties please see
    /// [Cluster properties](https://cloud.google.com/dataproc/docs/concepts/cluster-properties)
    #[builder(into)]
    #[serde(rename = "overrideProperties")]
    pub r#override_properties: Option<std::collections::HashMap<String, String>>,
    /// A list of the properties used to set the daemon config files.
    /// This will include any values supplied by the user via `cluster_config.software_config.override_properties`
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterConfigSoftwareConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "image_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_version,
                )
                .await,
            );
            map.insert(
                "optional_components".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#optional_components,
                )
                .await,
            );
            map.insert(
                "override_properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#override_properties,
                )
                .await,
            );
            map.insert(
                "properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#properties,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterConfigSoftwareConfig {
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
                    r#override_properties: {
                        let field_value = match fields_map.get("override_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'override_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
