#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TransferJobTransferSpecTransferOptions {
    /// Whether objects should be deleted from the source after they are transferred to the sink. Note that this option and `delete_objects_unique_in_sink` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "deleteObjectsFromSourceAfterTransfer")]
    pub r#delete_objects_from_source_after_transfer: Option<bool>,
    /// Whether objects that exist only in the sink should be deleted. Note that this option and
    /// `delete_objects_from_source_after_transfer` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "deleteObjectsUniqueInSink")]
    pub r#delete_objects_unique_in_sink: Option<bool>,
    /// Whether overwriting objects that already exist in the sink is allowed.
    #[builder(into)]
    #[serde(rename = "overwriteObjectsAlreadyExistingInSink")]
    pub r#overwrite_objects_already_existing_in_sink: Option<bool>,
    /// When to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by `overwrite_objects_already_existing_in_sink`. Possible values: ALWAYS, DIFFERENT, NEVER.
    #[builder(into)]
    #[serde(rename = "overwriteWhen")]
    pub r#overwrite_when: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TransferJobTransferSpecTransferOptions {
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
                    "delete_objects_from_source_after_transfer",
                    &self.r#delete_objects_from_source_after_transfer,
                ),
                to_pulumi_object_field(
                    "delete_objects_unique_in_sink",
                    &self.r#delete_objects_unique_in_sink,
                ),
                to_pulumi_object_field(
                    "overwrite_objects_already_existing_in_sink",
                    &self.r#overwrite_objects_already_existing_in_sink,
                ),
                to_pulumi_object_field(
                    "overwrite_when",
                    &self.r#overwrite_when,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TransferJobTransferSpecTransferOptions {
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
                    r#delete_objects_from_source_after_transfer: {
                        let field_value = match fields_map.get("delete_objects_from_source_after_transfer") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_objects_from_source_after_transfer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete_objects_unique_in_sink: {
                        let field_value = match fields_map.get("delete_objects_unique_in_sink") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_objects_unique_in_sink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#overwrite_objects_already_existing_in_sink: {
                        let field_value = match fields_map.get("overwrite_objects_already_existing_in_sink") {
                            Some(value) => value,
                            None => bail!("Missing field 'overwrite_objects_already_existing_in_sink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#overwrite_when: {
                        let field_value = match fields_map.get("overwrite_when") {
                            Some(value) => value,
                            None => bail!("Missing field 'overwrite_when' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
