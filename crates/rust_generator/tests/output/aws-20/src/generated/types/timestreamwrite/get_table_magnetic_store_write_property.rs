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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("enable_magnetic_store_writes".to_string(), self.r#enable_magnetic_store_writes.to_pulumi_value().await);
            map.insert("magnetic_store_rejected_data_locations".to_string(), self.r#magnetic_store_rejected_data_locations.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTableMagneticStoreWriteProperty {
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
                    r#enable_magnetic_store_writes: {
                        let field_value = match fields_map.get("enable_magnetic_store_writes") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_magnetic_store_writes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#magnetic_store_rejected_data_locations: {
                        let field_value = match fields_map.get("magnetic_store_rejected_data_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'magnetic_store_rejected_data_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::timestreamwrite::GetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocation> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
