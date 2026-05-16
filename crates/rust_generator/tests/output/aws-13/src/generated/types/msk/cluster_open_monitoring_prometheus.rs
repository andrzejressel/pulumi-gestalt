#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterOpenMonitoringPrometheus {
    /// Configuration block for JMX Exporter. See below.
    #[builder(into)]
    #[serde(rename = "jmxExporter")]
    pub r#jmx_exporter: Option<Box<super::super::types::msk::ClusterOpenMonitoringPrometheusJmxExporter>>,
    /// Configuration block for Node Exporter. See below.
    #[builder(into)]
    #[serde(rename = "nodeExporter")]
    pub r#node_exporter: Option<Box<super::super::types::msk::ClusterOpenMonitoringPrometheusNodeExporter>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterOpenMonitoringPrometheus {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("jmx_exporter".to_string(), self.r#jmx_exporter.to_pulumi_value().await);
            map.insert("node_exporter".to_string(), self.r#node_exporter.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterOpenMonitoringPrometheus {
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
                    r#jmx_exporter: {
                        let field_value = match fields_map.get("jmx_exporter") {
                            Some(value) => value,
                            None => bail!("Missing field 'jmx_exporter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::msk::ClusterOpenMonitoringPrometheusJmxExporter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#node_exporter: {
                        let field_value = match fields_map.get("node_exporter") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_exporter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::msk::ClusterOpenMonitoringPrometheusNodeExporter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
