#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFileSystemLifecyclePolicy {
    #[builder(into)]
    #[serde(rename = "transitionToArchive")]
    pub r#transition_to_archive: String,
    #[builder(into)]
    #[serde(rename = "transitionToIa")]
    pub r#transition_to_ia: String,
    #[builder(into)]
    #[serde(rename = "transitionToPrimaryStorageClass")]
    pub r#transition_to_primary_storage_class: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFileSystemLifecyclePolicy {
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
                    "transition_to_archive",
                    &self.r#transition_to_archive,
                ),
                to_pulumi_object_field(
                    "transition_to_ia",
                    &self.r#transition_to_ia,
                ),
                to_pulumi_object_field(
                    "transition_to_primary_storage_class",
                    &self.r#transition_to_primary_storage_class,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFileSystemLifecyclePolicy {
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
                    r#transition_to_archive: {
                        let field_value = match fields_map.get("transition_to_archive") {
                            Some(value) => value,
                            None => bail!("Missing field 'transition_to_archive' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transition_to_ia: {
                        let field_value = match fields_map.get("transition_to_ia") {
                            Some(value) => value,
                            None => bail!("Missing field 'transition_to_ia' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transition_to_primary_storage_class: {
                        let field_value = match fields_map.get("transition_to_primary_storage_class") {
                            Some(value) => value,
                            None => bail!("Missing field 'transition_to_primary_storage_class' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
