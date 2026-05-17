#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBudgetResourceGroupNotification {
    /// A list of email addresses to send the budget notification to when the threshold is exceeded.
    #[builder(into)]
    #[serde(rename = "contactEmails")]
    pub r#contact_emails: Vec<String>,
    /// A list of Action Group IDs to send the budget notification to when the threshold is exceeded.
    #[builder(into)]
    #[serde(rename = "contactGroups")]
    pub r#contact_groups: Vec<String>,
    /// A list of contact roles to send the budget notification to when the threshold is exceeded.
    #[builder(into)]
    #[serde(rename = "contactRoles")]
    pub r#contact_roles: Vec<String>,
    /// Whether the notification is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// The operator used for comparison.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// Threshold value associated with the notification.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: i32,
    #[builder(into)]
    #[serde(rename = "thresholdType")]
    pub r#threshold_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBudgetResourceGroupNotification {
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
                    "contact_emails",
                    &self.r#contact_emails,
                ),
                to_pulumi_object_field(
                    "contact_groups",
                    &self.r#contact_groups,
                ),
                to_pulumi_object_field(
                    "contact_roles",
                    &self.r#contact_roles,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "operator",
                    &self.r#operator,
                ),
                to_pulumi_object_field(
                    "threshold",
                    &self.r#threshold,
                ),
                to_pulumi_object_field(
                    "threshold_type",
                    &self.r#threshold_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBudgetResourceGroupNotification {
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
                    r#contact_emails: {
                        let field_value = match fields_map.get("contact_emails") {
                            Some(value) => value,
                            None => bail!("Missing field 'contact_emails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#contact_groups: {
                        let field_value = match fields_map.get("contact_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'contact_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#contact_roles: {
                        let field_value = match fields_map.get("contact_roles") {
                            Some(value) => value,
                            None => bail!("Missing field 'contact_roles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operator: {
                        let field_value = match fields_map.get("operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold: {
                        let field_value = match fields_map.get("threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold_type: {
                        let field_value = match fields_map.get("threshold_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
