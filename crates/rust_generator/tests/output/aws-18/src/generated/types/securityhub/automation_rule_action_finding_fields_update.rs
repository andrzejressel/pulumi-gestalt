#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutomationRuleActionFindingFieldsUpdate {
    /// The rule action updates the `Confidence` field of a finding.
    #[builder(into)]
    #[serde(rename = "confidence")]
    pub r#confidence: Option<i32>,
    /// The rule action updates the `Criticality` field of a finding.
    #[builder(into)]
    #[serde(rename = "criticality")]
    pub r#criticality: Option<i32>,
    /// A resource block that updates the note. Documented below.
    #[builder(into)]
    #[serde(rename = "note")]
    pub r#note: Option<Box<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateNote>>,
    /// A resource block that the rule action updates the `RelatedFindings` field of a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "relatedFindings")]
    pub r#related_findings: Option<Vec<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateRelatedFinding>>,
    /// A resource block that updates to the severity information for a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Option<Box<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateSeverity>>,
    /// The rule action updates the `Types` field of a finding.
    #[builder(into)]
    #[serde(rename = "types")]
    pub r#types: Option<Vec<String>>,
    /// The rule action updates the `UserDefinedFields` field of a finding.
    #[builder(into)]
    #[serde(rename = "userDefinedFields")]
    pub r#user_defined_fields: Option<std::collections::HashMap<String, String>>,
    /// The rule action updates the `VerificationState` field of a finding. The allowed values are the following `UNKNOWN`, `TRUE_POSITIVE`, `FALSE_POSITIVE` and `BENIGN_POSITIVE`.
    #[builder(into)]
    #[serde(rename = "verificationState")]
    pub r#verification_state: Option<String>,
    /// A resource block that is used to update information about the investigation into the finding. Documented below.
    #[builder(into)]
    #[serde(rename = "workflow")]
    pub r#workflow: Option<Box<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateWorkflow>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutomationRuleActionFindingFieldsUpdate {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "confidence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#confidence,
                )
                .await,
            );
            map.insert(
                "criticality".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#criticality,
                )
                .await,
            );
            map.insert(
                "note".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#note,
                )
                .await,
            );
            map.insert(
                "related_findings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#related_findings,
                )
                .await,
            );
            map.insert(
                "severity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#severity,
                )
                .await,
            );
            map.insert(
                "types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#types,
                )
                .await,
            );
            map.insert(
                "user_defined_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_defined_fields,
                )
                .await,
            );
            map.insert(
                "verification_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#verification_state,
                )
                .await,
            );
            map.insert(
                "workflow".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workflow,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutomationRuleActionFindingFieldsUpdate {
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
                    r#confidence: {
                        let field_value = match fields_map.get("confidence") {
                            Some(value) => value,
                            None => bail!("Missing field 'confidence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#criticality: {
                        let field_value = match fields_map.get("criticality") {
                            Some(value) => value,
                            None => bail!("Missing field 'criticality' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#note: {
                        let field_value = match fields_map.get("note") {
                            Some(value) => value,
                            None => bail!("Missing field 'note' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#related_findings: {
                        let field_value = match fields_map.get("related_findings") {
                            Some(value) => value,
                            None => bail!("Missing field 'related_findings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#severity: {
                        let field_value = match fields_map.get("severity") {
                            Some(value) => value,
                            None => bail!("Missing field 'severity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#types: {
                        let field_value = match fields_map.get("types") {
                            Some(value) => value,
                            None => bail!("Missing field 'types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_defined_fields: {
                        let field_value = match fields_map.get("user_defined_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_defined_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verification_state: {
                        let field_value = match fields_map.get("verification_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'verification_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workflow: {
                        let field_value = match fields_map.get("workflow") {
                            Some(value) => value,
                            None => bail!("Missing field 'workflow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
