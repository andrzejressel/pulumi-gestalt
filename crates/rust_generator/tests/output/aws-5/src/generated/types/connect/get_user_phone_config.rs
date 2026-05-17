#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPhoneConfig {
    /// The After Call Work (ACW) timeout setting, in seconds.
    #[builder(into)]
    #[serde(rename = "afterContactWorkTimeLimit")]
    pub r#after_contact_work_time_limit: i32,
    /// When Auto-Accept Call is enabled for an available agent, the agent connects to contacts automatically.
    #[builder(into)]
    #[serde(rename = "autoAccept")]
    pub r#auto_accept: bool,
    /// The phone number for the user's desk phone.
    #[builder(into)]
    #[serde(rename = "deskPhoneNumber")]
    pub r#desk_phone_number: String,
    /// The phone type. Valid values are `DESK_PHONE` and `SOFT_PHONE`.
    #[builder(into)]
    #[serde(rename = "phoneType")]
    pub r#phone_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserPhoneConfig {
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
                "after_contact_work_time_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#after_contact_work_time_limit,
                )
                .await,
            );
            map.insert(
                "auto_accept".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_accept,
                )
                .await,
            );
            map.insert(
                "desk_phone_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#desk_phone_number,
                )
                .await,
            );
            map.insert(
                "phone_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#phone_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserPhoneConfig {
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
                    r#after_contact_work_time_limit: {
                        let field_value = match fields_map.get("after_contact_work_time_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'after_contact_work_time_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_accept: {
                        let field_value = match fields_map.get("auto_accept") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_accept' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#desk_phone_number: {
                        let field_value = match fields_map.get("desk_phone_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'desk_phone_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phone_type: {
                        let field_value = match fields_map.get("phone_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'phone_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
