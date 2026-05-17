#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableBiglakeConfiguration {
    /// The connection specifying the credentials to be used to
    /// read and write to external storage, such as Cloud Storage. The connection_id can
    /// have the form "&lt;project\_id&gt;.&lt;location\_id&gt;.&lt;connection\_id&gt;" or
    /// projects/&lt;project\_id&gt;/locations/&lt;location\_id&gt;/connections/&lt;connection\_id&gt;".
    #[builder(into)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: String,
    /// The file format the table data is stored in.
    #[builder(into)]
    #[serde(rename = "fileFormat")]
    pub r#file_format: String,
    /// The fully qualified location prefix of the external folder where table data
    /// is stored. The '*' wildcard character is not allowed. The URI should be in the format "gs://bucket/path_to_table/"
    #[builder(into)]
    #[serde(rename = "storageUri")]
    pub r#storage_uri: String,
    /// The table format the metadata only snapshots are stored in.
    #[builder(into)]
    #[serde(rename = "tableFormat")]
    pub r#table_format: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableBiglakeConfiguration {
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
                    "connection_id",
                    &self.r#connection_id,
                ),
                to_pulumi_object_field(
                    "file_format",
                    &self.r#file_format,
                ),
                to_pulumi_object_field(
                    "storage_uri",
                    &self.r#storage_uri,
                ),
                to_pulumi_object_field(
                    "table_format",
                    &self.r#table_format,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableBiglakeConfiguration {
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
                    r#connection_id: {
                        let field_value = match fields_map.get("connection_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_format: {
                        let field_value = match fields_map.get("file_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_uri: {
                        let field_value = match fields_map.get("storage_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_format: {
                        let field_value = match fields_map.get("table_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
