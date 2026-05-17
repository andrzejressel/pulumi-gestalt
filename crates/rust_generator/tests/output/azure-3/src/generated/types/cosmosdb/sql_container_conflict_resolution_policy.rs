#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SqlContainerConflictResolutionPolicy {
    /// The conflict resolution path in the case of `LastWriterWins` mode.
    #[builder(into)]
    #[serde(rename = "conflictResolutionPath")]
    pub r#conflict_resolution_path: Option<String>,
    /// The procedure to resolve conflicts in the case of `Custom` mode.
    #[builder(into)]
    #[serde(rename = "conflictResolutionProcedure")]
    pub r#conflict_resolution_procedure: Option<String>,
    /// Indicates the conflict resolution mode. Possible values include: `LastWriterWins`, `Custom`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SqlContainerConflictResolutionPolicy {
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
                    "conflict_resolution_path",
                    &self.r#conflict_resolution_path,
                ),
                to_pulumi_object_field(
                    "conflict_resolution_procedure",
                    &self.r#conflict_resolution_procedure,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SqlContainerConflictResolutionPolicy {
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
                    r#conflict_resolution_path: {
                        let field_value = match fields_map.get("conflict_resolution_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'conflict_resolution_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#conflict_resolution_procedure: {
                        let field_value = match fields_map.get("conflict_resolution_procedure") {
                            Some(value) => value,
                            None => bail!("Missing field 'conflict_resolution_procedure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
