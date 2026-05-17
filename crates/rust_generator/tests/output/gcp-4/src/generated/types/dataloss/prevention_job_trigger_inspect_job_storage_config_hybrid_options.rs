#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobStorageConfigHybridOptions {
    /// A short description of where the data is coming from. Will be stored once in the job. 256 max length.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// To organize findings, these labels will be added to each finding.
    /// Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `a-z?`.
    /// Label values must be between 0 and 63 characters long and must conform to the regular expression `(a-z?)?`.
    /// No more than 10 labels can be associated with a given finding.
    /// Examples:
    /// * `"environment" : "production"`
    /// * `"pipeline" : "etl"`
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// These are labels that each inspection request must include within their 'finding_labels' map. Request
    /// may contain others, but any missing one of these will be rejected.
    /// Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `a-z?`.
    /// No more than 10 keys can be required.
    #[builder(into)]
    #[serde(rename = "requiredFindingLabelKeys")]
    pub r#required_finding_label_keys: Option<Vec<String>>,
    /// If the container is a table, additional information to make findings meaningful such as the columns that are primary keys.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tableOptions")]
    pub r#table_options: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigHybridOptionsTableOptions>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobStorageConfigHybridOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "required_finding_label_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#required_finding_label_keys,
                )
                .await,
            );
            map.insert(
                "table_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table_options,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobStorageConfigHybridOptions {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#required_finding_label_keys: {
                        let field_value = match fields_map.get("required_finding_label_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'required_finding_label_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_options: {
                        let field_value = match fields_map.get("table_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
