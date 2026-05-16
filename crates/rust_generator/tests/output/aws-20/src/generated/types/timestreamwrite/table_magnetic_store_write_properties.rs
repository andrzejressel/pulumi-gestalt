#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableMagneticStoreWriteProperties {
    /// A flag to enable magnetic store writes.
    #[builder(into)]
    #[serde(rename = "enableMagneticStoreWrites")]
    pub r#enable_magnetic_store_writes: Option<bool>,
    /// The location to write error reports for records rejected asynchronously during magnetic store writes. See Magnetic Store Rejected Data Location below for more details.
    #[builder(into)]
    #[serde(rename = "magneticStoreRejectedDataLocation")]
    pub r#magnetic_store_rejected_data_location: Option<Box<super::super::types::timestreamwrite::TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocation>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableMagneticStoreWriteProperties {
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
            map.insert("magnetic_store_rejected_data_location".to_string(), self.r#magnetic_store_rejected_data_location.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableMagneticStoreWriteProperties {
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
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#magnetic_store_rejected_data_location: {
                        let field_value = match fields_map.get("magnetic_store_rejected_data_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'magnetic_store_rejected_data_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::timestreamwrite::TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocation>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
