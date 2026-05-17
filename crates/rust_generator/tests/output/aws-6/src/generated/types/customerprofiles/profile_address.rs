#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProfileAddress {
    /// The first line of a customer address.
    #[builder(into)]
    #[serde(rename = "address1")]
    pub r#address_1: Option<String>,
    /// The second line of a customer address.
    #[builder(into)]
    #[serde(rename = "address2")]
    pub r#address_2: Option<String>,
    /// The third line of a customer address.
    #[builder(into)]
    #[serde(rename = "address3")]
    pub r#address_3: Option<String>,
    /// The fourth line of a customer address.
    #[builder(into)]
    #[serde(rename = "address4")]
    pub r#address_4: Option<String>,
    /// The city in which a customer lives.
    #[builder(into)]
    #[serde(rename = "city")]
    pub r#city: Option<String>,
    /// The country in which a customer lives.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// The county in which a customer lives.
    #[builder(into)]
    #[serde(rename = "county")]
    pub r#county: Option<String>,
    /// The postal code of a customer address.
    #[builder(into)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: Option<String>,
    /// The province in which a customer lives.
    #[builder(into)]
    #[serde(rename = "province")]
    pub r#province: Option<String>,
    /// The state in which a customer lives.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProfileAddress {
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
                "address_1".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_1,
                )
                .await,
            );
            map.insert(
                "address_2".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_2,
                )
                .await,
            );
            map.insert(
                "address_3".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_3,
                )
                .await,
            );
            map.insert(
                "address_4".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_4,
                )
                .await,
            );
            map.insert(
                "city".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#city,
                )
                .await,
            );
            map.insert(
                "country".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#country,
                )
                .await,
            );
            map.insert(
                "county".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#county,
                )
                .await,
            );
            map.insert(
                "postal_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#postal_code,
                )
                .await,
            );
            map.insert(
                "province".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#province,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProfileAddress {
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
                    r#address_1: {
                        let field_value = match fields_map.get("address_1") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#address_2: {
                        let field_value = match fields_map.get("address_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#address_3: {
                        let field_value = match fields_map.get("address_3") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#address_4: {
                        let field_value = match fields_map.get("address_4") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_4' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#city: {
                        let field_value = match fields_map.get("city") {
                            Some(value) => value,
                            None => bail!("Missing field 'city' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#country: {
                        let field_value = match fields_map.get("country") {
                            Some(value) => value,
                            None => bail!("Missing field 'country' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#county: {
                        let field_value = match fields_map.get("county") {
                            Some(value) => value,
                            None => bail!("Missing field 'county' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#postal_code: {
                        let field_value = match fields_map.get("postal_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'postal_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#province: {
                        let field_value = match fields_map.get("province") {
                            Some(value) => value,
                            None => bail!("Missing field 'province' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
