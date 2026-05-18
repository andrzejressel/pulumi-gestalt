#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTableMagneticStoreWriteProperty {
    /// Flag that is set based on if magnetic store writes are enabled.
    #[builder(into)]
    #[serde(rename = "enableMagneticStoreWrites")]
    pub r#enable_magnetic_store_writes: bool,
    /// Object containing the following attributes to describe error reports for records rejected during magnetic store writes.
    #[builder(into)]
    #[serde(rename = "magneticStoreRejectedDataLocations")]
    pub r#magnetic_store_rejected_data_locations: Vec<super::super::types::timestreamwrite::GetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocation>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTableMagneticStoreWriteProperty {
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
                    "enable_magnetic_store_writes",
                    &self.r#enable_magnetic_store_writes,
                ),
                to_pulumi_object_field(
                    "magnetic_store_rejected_data_locations",
                    &self.r#magnetic_store_rejected_data_locations,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTableMagneticStoreWriteProperty {
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
                    r#enable_magnetic_store_writes: {
                        let field_value = match fields_map.get("enable_magnetic_store_writes") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_magnetic_store_writes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#magnetic_store_rejected_data_locations: {
                        let field_value = match fields_map.get("magnetic_store_rejected_data_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'magnetic_store_rejected_data_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
