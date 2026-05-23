#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterResourceUsageExportConfig {
    /// Parameters for using BigQuery as the destination of resource usage export.
    /// 
    /// * `bigquery_destination.dataset_id` (Required) - The ID of a BigQuery Dataset. For Example:
    /// 
    #[builder(into)]
    pub r#bigquery_destination: Box<super::super::types::container::ClusterResourceUsageExportConfigBigqueryDestination>,
    /// Whether to enable network egress metering for this cluster. If enabled, a daemonset will be created
    /// in the cluster to meter network egress traffic.
    #[builder(into)]
    pub r#enable_network_egress_metering: Option<bool>,
    /// Whether to enable resource
    /// consumption metering on this cluster. When enabled, a table will be created in
    /// the resource export BigQuery dataset to store resource consumption data. The
    /// resulting table can be joined with the resource usage table or with BigQuery
    /// billing export. Defaults to `true`.
    #[builder(into)]
    pub r#enable_resource_consumption_metering: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterResourceUsageExportConfig {
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
                    "bigqueryDestination",
                    &self.r#bigquery_destination,
                ),
                to_pulumi_object_field(
                    "enableNetworkEgressMetering",
                    &self.r#enable_network_egress_metering,
                ),
                to_pulumi_object_field(
                    "enableResourceConsumptionMetering",
                    &self.r#enable_resource_consumption_metering,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterResourceUsageExportConfig {
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
                    r#bigquery_destination: {
                        let field_value = match fields_map.get("bigqueryDestination") {
                            Some(value) => value,
                            None => bail!("Missing field 'bigqueryDestination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_network_egress_metering: {
                        let field_value = match fields_map.get("enableNetworkEgressMetering") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableNetworkEgressMetering' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_resource_consumption_metering: {
                        let field_value = match fields_map.get("enableResourceConsumptionMetering") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableResourceConsumptionMetering' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
