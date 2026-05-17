#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceNotification {
    /// A list of additional email addresses to notify when there are alerts in the managed domain.
    #[builder(into)]
    #[serde(rename = "additionalRecipients")]
    pub r#additional_recipients: Vec<String>,
    /// Whethermembers of the _AAD DC Administrators_ group are notified when there are alerts in the managed domain.
    #[builder(into)]
    #[serde(rename = "notifyDcAdmins")]
    pub r#notify_dc_admins: bool,
    /// Whether all Global Administrators are notified when there are alerts in the managed domain.
    #[builder(into)]
    #[serde(rename = "notifyGlobalAdmins")]
    pub r#notify_global_admins: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceNotification {
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
                "additional_recipients".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_recipients,
                )
                .await,
            );
            map.insert(
                "notify_dc_admins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notify_dc_admins,
                )
                .await,
            );
            map.insert(
                "notify_global_admins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notify_global_admins,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceNotification {
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
                    r#additional_recipients: {
                        let field_value = match fields_map.get("additional_recipients") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_recipients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notify_dc_admins: {
                        let field_value = match fields_map.get("notify_dc_admins") {
                            Some(value) => value,
                            None => bail!("Missing field 'notify_dc_admins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notify_global_admins: {
                        let field_value = match fields_map.get("notify_global_admins") {
                            Some(value) => value,
                            None => bail!("Missing field 'notify_global_admins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
