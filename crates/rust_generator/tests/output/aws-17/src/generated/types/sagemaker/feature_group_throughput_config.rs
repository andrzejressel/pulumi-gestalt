#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureGroupThroughputConfig {
    #[builder(into)]
    #[serde(rename = "provisionedReadCapacityUnits")]
    pub r#provisioned_read_capacity_units: Option<i32>,
    #[builder(into)]
    #[serde(rename = "provisionedWriteCapacityUnits")]
    pub r#provisioned_write_capacity_units: Option<i32>,
    #[builder(into)]
    #[serde(rename = "throughputMode")]
    pub r#throughput_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureGroupThroughputConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("provisioned_read_capacity_units".to_string(), self.r#provisioned_read_capacity_units.to_pulumi_value().await);
            map.insert("provisioned_write_capacity_units".to_string(), self.r#provisioned_write_capacity_units.to_pulumi_value().await);
            map.insert("throughput_mode".to_string(), self.r#throughput_mode.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureGroupThroughputConfig {
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
                    r#provisioned_read_capacity_units: {
                        let field_value = match fields_map.get("provisioned_read_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_read_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#provisioned_write_capacity_units: {
                        let field_value = match fields_map.get("provisioned_write_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_write_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#throughput_mode: {
                        let field_value = match fields_map.get("throughput_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'throughput_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
