#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTableGlobalSecondaryIndex {
    #[builder(into)]
    #[serde(rename = "hashKey")]
    pub r#hash_key: String,
    /// Name of the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "nonKeyAttributes")]
    pub r#non_key_attributes: Vec<String>,
    #[builder(into)]
    #[serde(rename = "projectionType")]
    pub r#projection_type: String,
    #[builder(into)]
    #[serde(rename = "rangeKey")]
    pub r#range_key: String,
    #[builder(into)]
    #[serde(rename = "readCapacity")]
    pub r#read_capacity: i32,
    #[builder(into)]
    #[serde(rename = "writeCapacity")]
    pub r#write_capacity: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTableGlobalSecondaryIndex {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "hash_key",
                    &self.r#hash_key,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "non_key_attributes",
                    &self.r#non_key_attributes,
                ),
                to_pulumi_object_field(
                    "projection_type",
                    &self.r#projection_type,
                ),
                to_pulumi_object_field(
                    "range_key",
                    &self.r#range_key,
                ),
                to_pulumi_object_field(
                    "read_capacity",
                    &self.r#read_capacity,
                ),
                to_pulumi_object_field(
                    "write_capacity",
                    &self.r#write_capacity,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTableGlobalSecondaryIndex {
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
                    r#hash_key: {
                        let field_value = match fields_map.get("hash_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'hash_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#non_key_attributes: {
                        let field_value = match fields_map.get("non_key_attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'non_key_attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#projection_type: {
                        let field_value = match fields_map.get("projection_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'projection_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#range_key: {
                        let field_value = match fields_map.get("range_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'range_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_capacity: {
                        let field_value = match fields_map.get("read_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_capacity: {
                        let field_value = match fields_map.get("write_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'write_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
