#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMasterInstanceFleetInstanceTypeConfig {
    /// Bid price for each EC2 Spot instance type as defined by `instance_type`. Expressed in USD. If neither `bid_price` nor `bid_price_as_percentage_of_on_demand_price` is provided, `bid_price_as_percentage_of_on_demand_price` defaults to 100%.
    #[builder(into)]
    #[serde(rename = "bidPrice")]
    pub r#bid_price: Option<String>,
    /// Bid price, as a percentage of On-Demand price, for each EC2 Spot instance as defined by `instance_type`. Expressed as a number (for example, 20 specifies 20%). If neither `bid_price` nor `bid_price_as_percentage_of_on_demand_price` is provided, `bid_price_as_percentage_of_on_demand_price` defaults to 100%.
    #[builder(into)]
    #[serde(rename = "bidPriceAsPercentageOfOnDemandPrice")]
    pub r#bid_price_as_percentage_of_on_demand_price: Option<f64>,
    /// Configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software that run on the cluster. List of `configuration` blocks.
    #[builder(into)]
    #[serde(rename = "configurations")]
    pub r#configurations: Option<Vec<super::super::types::emr::ClusterMasterInstanceFleetInstanceTypeConfigConfiguration>>,
    /// Configuration block(s) for EBS volumes attached to each instance in the instance group. Detailed below.
    #[builder(into)]
    #[serde(rename = "ebsConfigs")]
    pub r#ebs_configs: Option<Vec<super::super::types::emr::ClusterMasterInstanceFleetInstanceTypeConfigEbsConfig>>,
    /// EC2 instance type, such as m4.xlarge.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    /// Number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in `aws.emr.InstanceFleet`.
    #[builder(into)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterMasterInstanceFleetInstanceTypeConfig {
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
                    "bid_price",
                    &self.r#bid_price,
                ),
                to_pulumi_object_field(
                    "bid_price_as_percentage_of_on_demand_price",
                    &self.r#bid_price_as_percentage_of_on_demand_price,
                ),
                to_pulumi_object_field(
                    "configurations",
                    &self.r#configurations,
                ),
                to_pulumi_object_field(
                    "ebs_configs",
                    &self.r#ebs_configs,
                ),
                to_pulumi_object_field(
                    "instance_type",
                    &self.r#instance_type,
                ),
                to_pulumi_object_field(
                    "weighted_capacity",
                    &self.r#weighted_capacity,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterMasterInstanceFleetInstanceTypeConfig {
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
                    r#bid_price: {
                        let field_value = match fields_map.get("bid_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'bid_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bid_price_as_percentage_of_on_demand_price: {
                        let field_value = match fields_map.get("bid_price_as_percentage_of_on_demand_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'bid_price_as_percentage_of_on_demand_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configurations: {
                        let field_value = match fields_map.get("configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebs_configs: {
                        let field_value = match fields_map.get("ebs_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebs_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weighted_capacity: {
                        let field_value = match fields_map.get("weighted_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'weighted_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
