#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableCapacitySpecification {
    /// The throughput capacity specified for read operations defined in read capacity units (RCUs).
    #[builder(into)]
    #[serde(rename = "readCapacityUnits")]
    pub r#read_capacity_units: Option<i32>,
    /// The read/write throughput capacity mode for a table. Valid values: `PAY_PER_REQUEST`, `PROVISIONED`. The default value is `PAY_PER_REQUEST`.
    #[builder(into)]
    #[serde(rename = "throughputMode")]
    pub r#throughput_mode: Option<String>,
    /// The throughput capacity specified for write operations defined in write capacity units (WCUs).
    #[builder(into)]
    #[serde(rename = "writeCapacityUnits")]
    pub r#write_capacity_units: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableCapacitySpecification {
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
                "read_capacity_units".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#read_capacity_units,
                )
                .await,
            );
            map.insert(
                "throughput_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#throughput_mode,
                )
                .await,
            );
            map.insert(
                "write_capacity_units".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#write_capacity_units,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableCapacitySpecification {
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
                    r#read_capacity_units: {
                        let field_value = match fields_map.get("read_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#throughput_mode: {
                        let field_value = match fields_map.get("throughput_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'throughput_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_capacity_units: {
                        let field_value = match fields_map.get("write_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'write_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
