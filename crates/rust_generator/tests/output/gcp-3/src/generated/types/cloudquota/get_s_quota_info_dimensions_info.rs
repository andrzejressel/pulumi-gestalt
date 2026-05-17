#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSQuotaInfoDimensionsInfo {
    /// The applicable regions or zones of this dimensions info. The field will be set to `['global']` for quotas that are not per region or per zone. Otherwise, it will be set to the list of locations this dimension info is applicable to.
    #[builder(into)]
    #[serde(rename = "applicableLocations")]
    pub r#applicable_locations: Vec<String>,
    /// The quota details for a map of dimensions.
    #[builder(into)]
    #[serde(rename = "details")]
    pub r#details: Vec<super::super::types::cloudquota::GetSQuotaInfoDimensionsInfoDetail>,
    /// The map of dimensions for this dimensions info. The key of a map entry is "region", "zone" or the name of a service specific dimension, and the value of a map entry is the value of the dimension. If a dimension does not appear in the map of dimensions, the dimensions info applies to all the dimension values except for those that have another DimenisonInfo instance configured for the specific value. Example: {"provider" : "Foo Inc"} where "provider" is a service specific dimension of a quota.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: std::collections::HashMap<String, String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSQuotaInfoDimensionsInfo {
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
                "applicable_locations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#applicable_locations,
                )
                .await,
            );
            map.insert(
                "details".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#details,
                )
                .await,
            );
            map.insert(
                "dimensions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dimensions,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSQuotaInfoDimensionsInfo {
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
                    r#applicable_locations: {
                        let field_value = match fields_map.get("applicable_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicable_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#details: {
                        let field_value = match fields_map.get("details") {
                            Some(value) => value,
                            None => bail!("Missing field 'details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
