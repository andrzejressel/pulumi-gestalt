#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ObjectReplicationRule {
    /// The time after which the Block Blobs created will be copies to the destination. Possible values are `OnlyNewObjects`, `Everything` and time in RFC3339 format: `2006-01-02T15:04:00Z`. Defaults to `OnlyNewObjects`.
    #[builder(into)]
    #[serde(rename = "copyBlobsCreatedAfter")]
    pub r#copy_blobs_created_after: Option<String>,
    /// The destination storage container name.
    #[builder(into)]
    #[serde(rename = "destinationContainerName")]
    pub r#destination_container_name: String,
    /// Specifies a list of filters prefixes, the blobs whose names begin with which will be replicated.
    #[builder(into)]
    #[serde(rename = "filterOutBlobsWithPrefixes")]
    pub r#filter_out_blobs_with_prefixes: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The source storage container name.
    #[builder(into)]
    #[serde(rename = "sourceContainerName")]
    pub r#source_container_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ObjectReplicationRule {
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
                    "copy_blobs_created_after",
                    &self.r#copy_blobs_created_after,
                ),
                to_pulumi_object_field(
                    "destination_container_name",
                    &self.r#destination_container_name,
                ),
                to_pulumi_object_field(
                    "filter_out_blobs_with_prefixes",
                    &self.r#filter_out_blobs_with_prefixes,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "source_container_name",
                    &self.r#source_container_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ObjectReplicationRule {
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
                    r#copy_blobs_created_after: {
                        let field_value = match fields_map.get("copy_blobs_created_after") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_blobs_created_after' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_container_name: {
                        let field_value = match fields_map.get("destination_container_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_container_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_out_blobs_with_prefixes: {
                        let field_value = match fields_map.get("filter_out_blobs_with_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_out_blobs_with_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#source_container_name: {
                        let field_value = match fields_map.get("source_container_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_container_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
