#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceFleetInstanceTypeConfig {
    /// The bid price for each EC2 Spot instance type as defined by `instance_type`. Expressed in USD. If neither `bid_price` nor `bid_price_as_percentage_of_on_demand_price` is provided, `bid_price_as_percentage_of_on_demand_price` defaults to 100%.
    #[builder(into, default)]
    #[serde(rename = "bidPrice")]
    pub r#bid_price: Box<Option<String>>,
    /// The bid price, as a percentage of On-Demand price, for each EC2 Spot instance as defined by `instance_type`. Expressed as a number (for example, 20 specifies 20%). If neither `bid_price` nor `bid_price_as_percentage_of_on_demand_price` is provided, `bid_price_as_percentage_of_on_demand_price` defaults to 100%.
    #[builder(into, default)]
    #[serde(rename = "bidPriceAsPercentageOfOnDemandPrice")]
    pub r#bid_price_as_percentage_of_on_demand_price: Box<Option<f64>>,
    /// A configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software that run on the cluster. List of `configuration` blocks.
    #[builder(into, default)]
    #[serde(rename = "configurations")]
    pub r#configurations: Box<Option<Vec<super::super::types::emr::InstanceFleetInstanceTypeConfigConfiguration>>>,
    /// Configuration block(s) for EBS volumes attached to each instance in the instance group. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "ebsConfigs")]
    pub r#ebs_configs: Box<Option<Vec<super::super::types::emr::InstanceFleetInstanceTypeConfigEbsConfig>>>,
    /// An EC2 instance type, such as m4.xlarge.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<String>,
    /// The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in `aws.emr.InstanceFleet`.
    #[builder(into, default)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Box<Option<i32>>,
}
