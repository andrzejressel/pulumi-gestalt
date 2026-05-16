#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo {
    /// A block that contains EBS volume provisioned throughput information. To provision storage throughput, you must choose broker type kafka.m5.4xlarge or larger. See below.
    #[builder(into)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput>>,
    /// The size in GiB of the EBS volume for the data drive on each broker node. Minimum value of `1` and maximum value of `16384`.
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("provisioned_throughput".to_string(), self.r#provisioned_throughput.to_pulumi_value().await);
            map.insert("volume_size".to_string(), self.r#volume_size.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo {
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
                    r#provisioned_throughput: {
                        let field_value = match fields_map.get("provisioned_throughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_throughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#volume_size: {
                        let field_value = match fields_map.get("volume_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
