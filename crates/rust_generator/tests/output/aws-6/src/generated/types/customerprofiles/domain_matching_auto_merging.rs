#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainMatchingAutoMerging {
    /// A block that specifies how the auto-merging process should resolve conflicts between different profiles. Documented below.
    #[builder(into)]
    #[serde(rename = "conflictResolution")]
    pub r#conflict_resolution: Option<Box<super::super::types::customerprofiles::DomainMatchingAutoMergingConflictResolution>>,
    /// A block that specifies a list of matching attributes that represent matching criteria. If two profiles meet at least one of the requirements in the matching attributes list, they will be merged. Documented below.
    /// * `min_allowed_confidence_score_for_merging ` - (Optional) A number between 0 and 1 that represents the minimum confidence score required for profiles within a matching group to be merged during the auto-merge process. A higher score means higher similarity required to merge profiles.
    #[builder(into)]
    #[serde(rename = "consolidation")]
    pub r#consolidation: Option<Box<super::super::types::customerprofiles::DomainMatchingAutoMergingConsolidation>>,
    /// The flag that enables the auto-merging of duplicate profiles.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    #[builder(into)]
    #[serde(rename = "minAllowedConfidenceScoreForMerging")]
    pub r#min_allowed_confidence_score_for_merging: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainMatchingAutoMerging {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("conflict_resolution".to_string(), self.r#conflict_resolution.to_pulumi_value().await);
            map.insert("consolidation".to_string(), self.r#consolidation.to_pulumi_value().await);
            map.insert("enabled".to_string(), self.r#enabled.to_pulumi_value().await);
            map.insert("min_allowed_confidence_score_for_merging".to_string(), self.r#min_allowed_confidence_score_for_merging.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainMatchingAutoMerging {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#conflict_resolution: {
                        let field_value = match fields_map.get("conflict_resolution") {
                            Some(value) => value,
                            None => bail!("Missing field 'conflict_resolution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::customerprofiles::DomainMatchingAutoMergingConflictResolution>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#consolidation: {
                        let field_value = match fields_map.get("consolidation") {
                            Some(value) => value,
                            None => bail!("Missing field 'consolidation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::customerprofiles::DomainMatchingAutoMergingConsolidation>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#min_allowed_confidence_score_for_merging: {
                        let field_value = match fields_map.get("min_allowed_confidence_score_for_merging") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_allowed_confidence_score_for_merging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<f64> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
