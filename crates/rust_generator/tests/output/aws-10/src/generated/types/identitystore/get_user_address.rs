#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserAddress {
    /// The country that this address is in.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: String,
    /// The name that is typically displayed when the name is shown for display.
    #[builder(into)]
    #[serde(rename = "formatted")]
    pub r#formatted: String,
    /// The address locality.
    #[builder(into)]
    #[serde(rename = "locality")]
    pub r#locality: String,
    /// The postal code of the address.
    #[builder(into)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: String,
    /// When `true`, this is the primary phone number associated with the user.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: bool,
    /// The region of the address.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// The street of the address.
    #[builder(into)]
    #[serde(rename = "streetAddress")]
    pub r#street_address: String,
    /// The type of phone number.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserAddress {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("country".to_string(), self.r#country.to_pulumi_value().await);
            map.insert("formatted".to_string(), self.r#formatted.to_pulumi_value().await);
            map.insert("locality".to_string(), self.r#locality.to_pulumi_value().await);
            map.insert("postal_code".to_string(), self.r#postal_code.to_pulumi_value().await);
            map.insert("primary".to_string(), self.r#primary.to_pulumi_value().await);
            map.insert("region".to_string(), self.r#region.to_pulumi_value().await);
            map.insert("street_address".to_string(), self.r#street_address.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserAddress {
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
                    r#country: {
                        let field_value = match fields_map.get("country") {
                            Some(value) => value,
                            None => bail!("Missing field 'country' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#formatted: {
                        let field_value = match fields_map.get("formatted") {
                            Some(value) => value,
                            None => bail!("Missing field 'formatted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#locality: {
                        let field_value = match fields_map.get("locality") {
                            Some(value) => value,
                            None => bail!("Missing field 'locality' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#postal_code: {
                        let field_value = match fields_map.get("postal_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'postal_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#primary: {
                        let field_value = match fields_map.get("primary") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#street_address: {
                        let field_value = match fields_map.get("street_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'street_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
