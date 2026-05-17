#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCatalogTableStorageDescriptorSkewedInfo {
    /// List of names of columns that contain skewed values.
    #[builder(into)]
    #[serde(rename = "skewedColumnNames")]
    pub r#skewed_column_names: Vec<String>,
    /// List of values that appear so frequently as to be considered skewed.
    #[builder(into)]
    #[serde(rename = "skewedColumnValueLocationMaps")]
    pub r#skewed_column_value_location_maps: std::collections::HashMap<String, String>,
    /// Map of skewed values to the columns that contain them.
    #[builder(into)]
    #[serde(rename = "skewedColumnValues")]
    pub r#skewed_column_values: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCatalogTableStorageDescriptorSkewedInfo {
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
                    "skewed_column_names",
                    &self.r#skewed_column_names,
                ),
                to_pulumi_object_field(
                    "skewed_column_value_location_maps",
                    &self.r#skewed_column_value_location_maps,
                ),
                to_pulumi_object_field(
                    "skewed_column_values",
                    &self.r#skewed_column_values,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCatalogTableStorageDescriptorSkewedInfo {
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
                    r#skewed_column_names: {
                        let field_value = match fields_map.get("skewed_column_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'skewed_column_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skewed_column_value_location_maps: {
                        let field_value = match fields_map.get("skewed_column_value_location_maps") {
                            Some(value) => value,
                            None => bail!("Missing field 'skewed_column_value_location_maps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skewed_column_values: {
                        let field_value = match fields_map.get("skewed_column_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'skewed_column_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
