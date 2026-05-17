#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProfileShippingAddress {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProfileShippingAddress {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "address_1",
                    &self.r#address_1,
                ),
                to_pulumi_object_field(
                    "address_2",
                    &self.r#address_2,
                ),
                to_pulumi_object_field(
                    "address_3",
                    &self.r#address_3,
                ),
                to_pulumi_object_field(
                    "address_4",
                    &self.r#address_4,
                ),
                to_pulumi_object_field(
                    "city",
                    &self.r#city,
                ),
                to_pulumi_object_field(
                    "country",
                    &self.r#country,
                ),
                to_pulumi_object_field(
                    "county",
                    &self.r#county,
                ),
                to_pulumi_object_field(
                    "postal_code",
                    &self.r#postal_code,
                ),
                to_pulumi_object_field(
                    "province",
                    &self.r#province,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProfileShippingAddress {
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
