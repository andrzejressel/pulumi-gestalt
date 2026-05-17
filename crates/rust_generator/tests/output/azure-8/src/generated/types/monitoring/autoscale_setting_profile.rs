#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscaleSettingProfile {
    /// A `capacity` block as defined below.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<super::super::types::monitoring::AutoscaleSettingProfileCapacity>,
    /// A `fixed_date` block as defined below. This cannot be specified if a `recurrence` block is specified.
    #[builder(into)]
    #[serde(rename = "fixedDate")]
    pub r#fixed_date: Option<Box<super::super::types::monitoring::AutoscaleSettingProfileFixedDate>>,
    /// Specifies the name of the profile.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A `recurrence` block as defined below. This cannot be specified if a `fixed_date` block is specified.
    #[builder(into)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Option<Box<super::super::types::monitoring::AutoscaleSettingProfileRecurrence>>,
    /// One or more (up to 10) `rule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Option<Vec<super::super::types::monitoring::AutoscaleSettingProfileRule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscaleSettingProfile {
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
                "capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#capacity,
                )
                .await,
            );
            map.insert(
                "fixed_date".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fixed_date,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "recurrence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recurrence,
                )
                .await,
            );
            map.insert(
                "rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rules,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutoscaleSettingProfile {
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
                    r#capacity: {
                        let field_value = match fields_map.get("capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_date: {
                        let field_value = match fields_map.get("fixed_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixed_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recurrence: {
                        let field_value = match fields_map.get("recurrence") {
                            Some(value) => value,
                            None => bail!("Missing field 'recurrence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rules: {
                        let field_value = match fields_map.get("rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
