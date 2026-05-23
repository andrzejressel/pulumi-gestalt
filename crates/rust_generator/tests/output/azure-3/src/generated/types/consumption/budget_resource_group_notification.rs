#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetResourceGroupNotification {
    /// Specifies a list of email addresses to send the budget notification to when the threshold is exceeded.
    #[builder(into)]
    pub r#contact_emails: Option<Vec<String>>,
    /// Specifies a list of Action Group IDs to send the budget notification to when the threshold is exceeded.
    #[builder(into)]
    pub r#contact_groups: Option<Vec<String>>,
    /// Specifies a list of contact roles to send the budget notification to when the threshold is exceeded.
    #[builder(into)]
    pub r#contact_roles: Option<Vec<String>>,
    /// Should the notification be enabled? Defaults to `true`.
    /// 
    /// > **NOTE:** A `notification` block cannot have all of `contact_emails`, `contact_roles`, and `contact_groups` empty. This means that at least one of the three must be specified.
    #[builder(into)]
    pub r#enabled: Option<bool>,
    /// The comparison operator for the notification. Must be one of `EqualTo`, `GreaterThan`, or `GreaterThanOrEqualTo`.
    #[builder(into)]
    pub r#operator: String,
    /// Threshold value associated with a notification. Notification is sent when the cost exceeded the threshold. It is always percent and has to be between 0 and 1000.
    #[builder(into)]
    pub r#threshold: i32,
    /// The type of threshold for the notification. This determines whether the notification is triggered by forecasted costs or actual costs. The allowed values are `Actual` and `Forecasted`. Default is `Actual`.
    #[builder(into)]
    pub r#threshold_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetResourceGroupNotification {
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
                    "contactEmails",
                    &self.r#contact_emails,
                ),
                to_pulumi_object_field(
                    "contactGroups",
                    &self.r#contact_groups,
                ),
                to_pulumi_object_field(
                    "contactRoles",
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
                    "thresholdType",
                    &self.r#threshold_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetResourceGroupNotification {
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
                        let field_value = match fields_map.get("contactEmails") {
                            Some(value) => value,
                            None => bail!("Missing field 'contactEmails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#contact_groups: {
                        let field_value = match fields_map.get("contactGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'contactGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#contact_roles: {
                        let field_value = match fields_map.get("contactRoles") {
                            Some(value) => value,
                            None => bail!("Missing field 'contactRoles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("thresholdType") {
                            Some(value) => value,
                            None => bail!("Missing field 'thresholdType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
