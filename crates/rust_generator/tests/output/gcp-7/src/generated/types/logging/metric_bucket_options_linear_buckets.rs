#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricBucketOptionsLinearBuckets {
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "numFiniteBuckets")]
    pub r#num_finite_buckets: i32,
    /// Lower bound of the first bucket.
    #[builder(into)]
    #[serde(rename = "offset")]
    pub r#offset: f64,
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "width")]
    pub r#width: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MetricBucketOptionsLinearBuckets {
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
                "num_finite_buckets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#num_finite_buckets,
                )
                .await,
            );
            map.insert(
                "offset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#offset,
                )
                .await,
            );
            map.insert(
                "width".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#width,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MetricBucketOptionsLinearBuckets {
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
                    r#num_finite_buckets: {
                        let field_value = match fields_map.get("num_finite_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_finite_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#offset: {
                        let field_value = match fields_map.get("offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#width: {
                        let field_value = match fields_map.get("width") {
                            Some(value) => value,
                            None => bail!("Missing field 'width' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
