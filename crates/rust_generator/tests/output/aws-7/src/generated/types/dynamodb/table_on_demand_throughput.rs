#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableOnDemandThroughput {
    /// Maximum number of read request units for the specified table. To specify set the value greater than or equal to 1. To remove set the value to -1.
    #[builder(into)]
    #[serde(rename = "maxReadRequestUnits")]
    pub r#max_read_request_units: Option<i32>,
    /// Maximum number of write request units for the specified table. To specify set the value greater than or equal to 1. To remove set the value to -1.
    #[builder(into)]
    #[serde(rename = "maxWriteRequestUnits")]
    pub r#max_write_request_units: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableOnDemandThroughput {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "max_read_request_units".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_read_request_units,
                )
                .await,
            );
            map.insert(
                "max_write_request_units".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_write_request_units,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableOnDemandThroughput {
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
                    r#max_read_request_units: {
                        let field_value = match fields_map.get("max_read_request_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_read_request_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_write_request_units: {
                        let field_value = match fields_map.get("max_write_request_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_write_request_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
