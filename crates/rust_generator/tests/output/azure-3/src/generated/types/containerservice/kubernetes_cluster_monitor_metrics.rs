#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterMonitorMetrics {
    /// Specifies a comma-separated list of Kubernetes annotation keys that will be used in the resource's labels metric.
    #[builder(into)]
    #[serde(rename = "annotationsAllowed")]
    pub r#annotations_allowed: Option<String>,
    /// Specifies a Comma-separated list of additional Kubernetes label keys that will be used in the resource's labels metric.
    /// 
    /// > **Note:** Both properties `annotations_allowed` and `labels_allowed` are required if you are enabling Managed Prometheus with an existing Azure Monitor Workspace.
    #[builder(into)]
    #[serde(rename = "labelsAllowed")]
    pub r#labels_allowed: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterMonitorMetrics {
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
                "annotations_allowed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#annotations_allowed,
                )
                .await,
            );
            map.insert(
                "labels_allowed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels_allowed,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterMonitorMetrics {
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
                    r#annotations_allowed: {
                        let field_value = match fields_map.get("annotations_allowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'annotations_allowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels_allowed: {
                        let field_value = match fields_map.get("labels_allowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels_allowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
