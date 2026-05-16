#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProactiveEngagementEmergencyContact {
    /// Additional notes regarding the contact.
    #[builder(into)]
    #[serde(rename = "contactNotes")]
    pub r#contact_notes: Option<String>,
    /// A valid email address that will be used for this contact.
    #[builder(into)]
    #[serde(rename = "emailAddress")]
    pub r#email_address: String,
    /// A phone number, starting with `+` and up to 15 digits that will be used for this contact.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProactiveEngagementEmergencyContact {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("contact_notes".to_string(), self.r#contact_notes.to_pulumi_value().await);
            map.insert("email_address".to_string(), self.r#email_address.to_pulumi_value().await);
            map.insert("phone_number".to_string(), self.r#phone_number.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProactiveEngagementEmergencyContact {
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
                    r#contact_notes: {
                        let field_value = match fields_map.get("contact_notes") {
                            Some(value) => value,
                            None => bail!("Missing field 'contact_notes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#email_address: {
                        let field_value = match fields_map.get("email_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#phone_number: {
                        let field_value = match fields_map.get("phone_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'phone_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
