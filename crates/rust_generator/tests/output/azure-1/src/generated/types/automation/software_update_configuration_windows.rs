#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SoftwareUpdateConfigurationWindows {
    /// Specifies the list of update classification. Possible values are `Unclassified`, `Critical`, `Security`, `UpdateRollup`, `FeaturePack`, `ServicePack`, `Definition`, `Tools` and `Updates`.
    /// 
    /// > **NOTE:** The `classifications_included` property will become `Required` in version 4.0 of the Provider.
    #[builder(into)]
    #[serde(rename = "classificationsIncludeds")]
    pub r#classifications_includeds: Vec<String>,
    /// Specifies a list of knowledge base numbers excluded.
    #[builder(into)]
    #[serde(rename = "excludedKnowledgeBaseNumbers")]
    pub r#excluded_knowledge_base_numbers: Option<Vec<String>>,
    /// Specifies a list of knowledge base numbers included.
    #[builder(into)]
    #[serde(rename = "includedKnowledgeBaseNumbers")]
    pub r#included_knowledge_base_numbers: Option<Vec<String>>,
    /// Specifies the reboot settings after software update, possible values are `IfRequired`, `Never`, `RebootOnly` and `Always`. Defaults to `IfRequired`.
    #[builder(into)]
    #[serde(rename = "reboot")]
    pub r#reboot: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SoftwareUpdateConfigurationWindows {
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
                    "classifications_includeds",
                    &self.r#classifications_includeds,
                ),
                to_pulumi_object_field(
                    "excluded_knowledge_base_numbers",
                    &self.r#excluded_knowledge_base_numbers,
                ),
                to_pulumi_object_field(
                    "included_knowledge_base_numbers",
                    &self.r#included_knowledge_base_numbers,
                ),
                to_pulumi_object_field(
                    "reboot",
                    &self.r#reboot,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SoftwareUpdateConfigurationWindows {
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
                    r#classifications_includeds: {
                        let field_value = match fields_map.get("classifications_includeds") {
                            Some(value) => value,
                            None => bail!("Missing field 'classifications_includeds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_knowledge_base_numbers: {
                        let field_value = match fields_map.get("excluded_knowledge_base_numbers") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_knowledge_base_numbers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_knowledge_base_numbers: {
                        let field_value = match fields_map.get("included_knowledge_base_numbers") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_knowledge_base_numbers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reboot: {
                        let field_value = match fields_map.get("reboot") {
                            Some(value) => value,
                            None => bail!("Missing field 'reboot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
