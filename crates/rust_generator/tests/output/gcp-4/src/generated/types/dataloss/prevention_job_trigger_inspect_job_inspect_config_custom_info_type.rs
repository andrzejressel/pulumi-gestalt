#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobInspectConfigCustomInfoType {
    /// Dictionary which defines the rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dictionary")]
    pub r#dictionary: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigCustomInfoTypeDictionary>>,
    /// If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching.
    /// Possible values are: `EXCLUSION_TYPE_EXCLUDE`.
    #[builder(into)]
    #[serde(rename = "exclusionType")]
    pub r#exclusion_type: Option<String>,
    /// CustomInfoType can either be a new infoType, or an extension of built-in infoType, when the name matches one of existing
    /// infoTypes and that infoType is specified in `info_types` field. Specifying the latter adds findings to the
    /// one detected by the system. If built-in info type is not specified in `info_types` list then the name is
    /// treated as a custom info type.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "infoType")]
    pub r#info_type: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigCustomInfoTypeInfoType>,
    /// Likelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria
    /// specified by the rule.
    /// Default value is `VERY_LIKELY`.
    /// Possible values are: `VERY_UNLIKELY`, `UNLIKELY`, `POSSIBLE`, `LIKELY`, `VERY_LIKELY`.
    #[builder(into)]
    #[serde(rename = "likelihood")]
    pub r#likelihood: Option<String>,
    /// Regular expression which defines the rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigCustomInfoTypeRegex>>,
    /// Optional custom sensitivity for this InfoType. This only applies to data profiling.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sensitivityScore")]
    pub r#sensitivity_score: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigCustomInfoTypeSensitivityScore>>,
    /// A reference to a StoredInfoType to use with scanning.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "storedType")]
    pub r#stored_type: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigCustomInfoTypeStoredType>>,
    /// Message for detecting output from deidentification transformations that support reversing.
    #[builder(into)]
    #[serde(rename = "surrogateType")]
    pub r#surrogate_type: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigCustomInfoTypeSurrogateType>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobInspectConfigCustomInfoType {
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
                    "dictionary",
                    &self.r#dictionary,
                ),
                to_pulumi_object_field(
                    "exclusion_type",
                    &self.r#exclusion_type,
                ),
                to_pulumi_object_field(
                    "info_type",
                    &self.r#info_type,
                ),
                to_pulumi_object_field(
                    "likelihood",
                    &self.r#likelihood,
                ),
                to_pulumi_object_field(
                    "regex",
                    &self.r#regex,
                ),
                to_pulumi_object_field(
                    "sensitivity_score",
                    &self.r#sensitivity_score,
                ),
                to_pulumi_object_field(
                    "stored_type",
                    &self.r#stored_type,
                ),
                to_pulumi_object_field(
                    "surrogate_type",
                    &self.r#surrogate_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobInspectConfigCustomInfoType {
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
                    r#dictionary: {
                        let field_value = match fields_map.get("dictionary") {
                            Some(value) => value,
                            None => bail!("Missing field 'dictionary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclusion_type: {
                        let field_value = match fields_map.get("exclusion_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusion_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#info_type: {
                        let field_value = match fields_map.get("info_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'info_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#likelihood: {
                        let field_value = match fields_map.get("likelihood") {
                            Some(value) => value,
                            None => bail!("Missing field 'likelihood' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex: {
                        let field_value = match fields_map.get("regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sensitivity_score: {
                        let field_value = match fields_map.get("sensitivity_score") {
                            Some(value) => value,
                            None => bail!("Missing field 'sensitivity_score' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stored_type: {
                        let field_value = match fields_map.get("stored_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'stored_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#surrogate_type: {
                        let field_value = match fields_map.get("surrogate_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'surrogate_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
