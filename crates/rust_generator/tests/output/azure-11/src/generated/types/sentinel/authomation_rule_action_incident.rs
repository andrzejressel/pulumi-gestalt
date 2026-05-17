#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthomationRuleActionIncident {
    /// The classification of the incident, when closing it. Possible values are: `BenignPositive_SuspiciousButExpected`, `FalsePositive_InaccurateData`, `FalsePositive_IncorrectAlertLogic`, `TruePositive_SuspiciousActivity` and `Undetermined`.
    /// 
    /// > **Note:** The `classification` is required when `status` is `Closed`.
    #[builder(into)]
    #[serde(rename = "classification")]
    pub r#classification: Option<String>,
    /// The comment why the incident is to be closed.
    /// 
    /// > **Note:** The `classification_comment` is allowed to set only when `status` is `Closed`.
    #[builder(into)]
    #[serde(rename = "classificationComment")]
    pub r#classification_comment: Option<String>,
    /// Specifies a list of labels to add to the incident.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<String>>,
    /// The execution order of this action.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: i32,
    /// The object ID of the entity this incident is assigned to.
    #[builder(into)]
    #[serde(rename = "ownerId")]
    pub r#owner_id: Option<String>,
    /// The severity to add to the incident. Possible values are `High`, `Informational`, `Low` and `Medium`.
    /// 
    /// > **Note:**: At least one of `status`, `labels`, `owner_id` and `severity` has to be set.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Option<String>,
    /// The status to set to the incident. Possible values are: `Active`, `Closed`, `New`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthomationRuleActionIncident {
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
                    "classification",
                    &self.r#classification,
                ),
                to_pulumi_object_field(
                    "classification_comment",
                    &self.r#classification_comment,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "order",
                    &self.r#order,
                ),
                to_pulumi_object_field(
                    "owner_id",
                    &self.r#owner_id,
                ),
                to_pulumi_object_field(
                    "severity",
                    &self.r#severity,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthomationRuleActionIncident {
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
                    r#classification: {
                        let field_value = match fields_map.get("classification") {
                            Some(value) => value,
                            None => bail!("Missing field 'classification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#classification_comment: {
                        let field_value = match fields_map.get("classification_comment") {
                            Some(value) => value,
                            None => bail!("Missing field 'classification_comment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#owner_id: {
                        let field_value = match fields_map.get("owner_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'owner_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
