#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ObjectReplicationRule {
    /// The time after which the Block Blobs created will be copies to the destination. Possible values are `OnlyNewObjects`, `Everything` and time in RFC3339 format: `2006-01-02T15:04:00Z`. Defaults to `OnlyNewObjects`.
    #[builder(into)]
    pub r#copy_blobs_created_after: Option<String>,
    /// The destination storage container name.
    #[builder(into)]
    pub r#destination_container_name: String,
    /// Specifies a list of filters prefixes, the blobs whose names begin with which will be replicated.
    #[builder(into)]
    pub r#filter_out_blobs_with_prefixes: Option<Vec<String>>,
    #[builder(into)]
    pub r#name: Option<String>,
    /// The source storage container name.
    #[builder(into)]
    pub r#source_container_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ObjectReplicationRule {
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
                    "copyBlobsCreatedAfter",
                    &self.r#copy_blobs_created_after,
                ),
                to_pulumi_object_field(
                    "destinationContainerName",
                    &self.r#destination_container_name,
                ),
                to_pulumi_object_field(
                    "filterOutBlobsWithPrefixes",
                    &self.r#filter_out_blobs_with_prefixes,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "sourceContainerName",
                    &self.r#source_container_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("copyBlobsCreatedAfter") {
                            Some(value) => value,
                            None => bail!("Missing field 'copyBlobsCreatedAfter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_container_name: {
                        let field_value = match fields_map.get("destinationContainerName") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationContainerName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_out_blobs_with_prefixes: {
                        let field_value = match fields_map.get("filterOutBlobsWithPrefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'filterOutBlobsWithPrefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("sourceContainerName") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceContainerName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
