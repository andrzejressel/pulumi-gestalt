#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterResourceUsageExportConfig {
    /// Parameters for using BigQuery as the destination of resource usage export.
    #[builder(into)]
    #[serde(rename = "bigqueryDestinations")]
    pub r#bigquery_destinations: Vec<super::super::types::container::GetClusterResourceUsageExportConfigBigqueryDestination>,
    /// Whether to enable network egress metering for this cluster. If enabled, a daemonset will be created in the cluster to meter network egress traffic.
    #[builder(into)]
    #[serde(rename = "enableNetworkEgressMetering")]
    pub r#enable_network_egress_metering: bool,
    /// Whether to enable resource consumption metering on this cluster. When enabled, a table will be created in the resource export BigQuery dataset to store resource consumption data. The resulting table can be joined with the resource usage table or with BigQuery billing export. Defaults to true.
    #[builder(into)]
    #[serde(rename = "enableResourceConsumptionMetering")]
    pub r#enable_resource_consumption_metering: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterResourceUsageExportConfig {
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
                    "bigquery_destinations",
                    &self.r#bigquery_destinations,
                ),
                to_pulumi_object_field(
                    "enable_network_egress_metering",
                    &self.r#enable_network_egress_metering,
                ),
                to_pulumi_object_field(
                    "enable_resource_consumption_metering",
                    &self.r#enable_resource_consumption_metering,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterResourceUsageExportConfig {
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
                    r#bigquery_destinations: {
                        let field_value = match fields_map.get("bigquery_destinations") {
                            Some(value) => value,
                            None => bail!("Missing field 'bigquery_destinations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_network_egress_metering: {
                        let field_value = match fields_map.get("enable_network_egress_metering") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_network_egress_metering' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_resource_consumption_metering: {
                        let field_value = match fields_map.get("enable_resource_consumption_metering") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_resource_consumption_metering' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
