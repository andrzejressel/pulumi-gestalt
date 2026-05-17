#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetBudgetFilter {
    /// A CalendarPeriod represents the abstract concept of a recurring time period that has a
    /// canonical start. Grammatically, "the start of the current CalendarPeriod".
    /// All calendar times begin at 12 AM US and Canadian Pacific Time (UTC-8).
    /// Exactly one of `calendar_period`, `custom_period` must be provided.
    /// Possible values are: `MONTH`, `QUARTER`, `YEAR`, `CALENDAR_PERIOD_UNSPECIFIED`.
    #[builder(into)]
    #[serde(rename = "calendarPeriod")]
    pub r#calendar_period: Option<String>,
    /// Optional. If creditTypesTreatment is INCLUDE_SPECIFIED_CREDITS,
    /// this is a list of credit types to be subtracted from gross cost to determine the spend for threshold calculations. See a list of acceptable credit type values.
    /// If creditTypesTreatment is not INCLUDE_SPECIFIED_CREDITS, this field must be empty.
    /// **Note:** If the field has a value in the config and needs to be removed, the field has to be an empty array in the config.
    #[builder(into)]
    #[serde(rename = "creditTypes")]
    pub r#credit_types: Option<Vec<String>>,
    /// Specifies how credits should be treated when determining spend
    /// for threshold calculations.
    /// Default value is `INCLUDE_ALL_CREDITS`.
    /// Possible values are: `INCLUDE_ALL_CREDITS`, `EXCLUDE_ALL_CREDITS`, `INCLUDE_SPECIFIED_CREDITS`.
    #[builder(into)]
    #[serde(rename = "creditTypesTreatment")]
    pub r#credit_types_treatment: Option<String>,
    /// Specifies to track usage from any start date (required) to any end date (optional).
    /// This time period is static, it does not recur.
    /// Exactly one of `calendar_period`, `custom_period` must be provided.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customPeriod")]
    pub r#custom_period: Option<Box<super::super::types::billing::BudgetBudgetFilterCustomPeriod>>,
    /// A single label and value pair specifying that usage from only
    /// this set of labeled resources should be included in the budget.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// A set of projects of the form projects/{project_number},
    /// specifying that usage from only this set of projects should be
    /// included in the budget. If omitted, the report will include
    /// all usage for the billing account, regardless of which project
    /// the usage occurred on.
    #[builder(into)]
    #[serde(rename = "projects")]
    pub r#projects: Option<Vec<String>>,
    /// A set of folder and organization names of the form folders/{folderId} or organizations/{organizationId},
    /// specifying that usage from only this set of folders and organizations should be included in the budget.
    /// If omitted, the budget includes all usage that the billing account pays for. If the folder or organization
    /// contains projects that are paid for by a different Cloud Billing account, the budget doesn't apply to those projects.
    #[builder(into)]
    #[serde(rename = "resourceAncestors")]
    pub r#resource_ancestors: Option<Vec<String>>,
    /// A set of services of the form services/{service_id},
    /// specifying that usage from only this set of services should be
    /// included in the budget. If omitted, the report will include
    /// usage for all the services. The service names are available
    /// through the Catalog API:
    /// https://cloud.google.com/billing/v1/how-tos/catalog-api.
    #[builder(into)]
    #[serde(rename = "services")]
    pub r#services: Option<Vec<String>>,
    /// A set of subaccounts of the form billingAccounts/{account_id},
    /// specifying that usage from only this set of subaccounts should
    /// be included in the budget. If a subaccount is set to the name of
    /// the parent account, usage from the parent account will be included.
    /// If the field is omitted, the report will include usage from the parent
    /// account and all subaccounts, if they exist.
    /// **Note:** If the field has a value in the config and needs to be removed, the field has to be an empty array in the config.
    #[builder(into)]
    #[serde(rename = "subaccounts")]
    pub r#subaccounts: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetBudgetFilter {
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
                "calendar_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#calendar_period,
                )
                .await,
            );
            map.insert(
                "credit_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#credit_types,
                )
                .await,
            );
            map.insert(
                "credit_types_treatment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#credit_types_treatment,
                )
                .await,
            );
            map.insert(
                "custom_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_period,
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
                "projects".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#projects,
                )
                .await,
            );
            map.insert(
                "resource_ancestors".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_ancestors,
                )
                .await,
            );
            map.insert(
                "services".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#services,
                )
                .await,
            );
            map.insert(
                "subaccounts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subaccounts,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetBudgetFilter {
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
                    r#calendar_period: {
                        let field_value = match fields_map.get("calendar_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'calendar_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credit_types: {
                        let field_value = match fields_map.get("credit_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'credit_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credit_types_treatment: {
                        let field_value = match fields_map.get("credit_types_treatment") {
                            Some(value) => value,
                            None => bail!("Missing field 'credit_types_treatment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_period: {
                        let field_value = match fields_map.get("custom_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#projects: {
                        let field_value = match fields_map.get("projects") {
                            Some(value) => value,
                            None => bail!("Missing field 'projects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_ancestors: {
                        let field_value = match fields_map.get("resource_ancestors") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_ancestors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#services: {
                        let field_value = match fields_map.get("services") {
                            Some(value) => value,
                            None => bail!("Missing field 'services' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subaccounts: {
                        let field_value = match fields_map.get("subaccounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'subaccounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
