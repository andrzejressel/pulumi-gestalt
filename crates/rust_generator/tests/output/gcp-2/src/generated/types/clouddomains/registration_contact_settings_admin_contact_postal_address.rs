#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistrationContactSettingsAdminContactPostalAddress {
    /// Unstructured address lines describing the lower levels of an address.
    /// Because values in addressLines do not have type information and may sometimes contain multiple values in a single
    /// field (e.g. "Austin, TX"), it is important that the line order is clear. The order of address lines should be
    /// "envelope order" for the country/region of the address. In places where this can vary (e.g. Japan), address_language
    /// is used to make it explicit (e.g. "ja" for large-to-small ordering and "ja-Latn" or "en" for small-to-large). This way,
    /// the most specific line of an address can be selected based on the language.
    #[builder(into)]
    pub r#address_lines: Option<Vec<String>>,
    /// Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,
    /// a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community
    /// (e.g. "Barcelona" and not "Catalonia"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland
    /// this should be left unpopulated.
    #[builder(into)]
    pub r#administrative_area: Option<String>,
    /// Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world
    /// where localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines.
    #[builder(into)]
    pub r#locality: Option<String>,
    /// The name of the organization at the address.
    #[builder(into)]
    pub r#organization: Option<String>,
    /// Postal code of the address. Not all countries use or require postal codes to be present, but where they are used,
    /// they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.).
    #[builder(into)]
    pub r#postal_code: Option<String>,
    /// The recipient at the address. This field may, under certain circumstances, contain multiline information. For example,
    /// it might contain "care of" information.
    /// 
    /// - - -
    #[builder(into)]
    pub r#recipients: Option<Vec<String>>,
    /// Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to
    /// ensure the value is correct. See https://cldr.unicode.org/ and
    /// https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: "CH" for Switzerland.
    #[builder(into)]
    pub r#region_code: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistrationContactSettingsAdminContactPostalAddress {
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
                    "addressLines",
                    &self.r#address_lines,
                ),
                to_pulumi_object_field(
                    "administrativeArea",
                    &self.r#administrative_area,
                ),
                to_pulumi_object_field(
                    "locality",
                    &self.r#locality,
                ),
                to_pulumi_object_field(
                    "organization",
                    &self.r#organization,
                ),
                to_pulumi_object_field(
                    "postalCode",
                    &self.r#postal_code,
                ),
                to_pulumi_object_field(
                    "recipients",
                    &self.r#recipients,
                ),
                to_pulumi_object_field(
                    "regionCode",
                    &self.r#region_code,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistrationContactSettingsAdminContactPostalAddress {
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
                    r#address_lines: {
                        let field_value = match fields_map.get("addressLines") {
                            Some(value) => value,
                            None => bail!("Missing field 'addressLines' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#administrative_area: {
                        let field_value = match fields_map.get("administrativeArea") {
                            Some(value) => value,
                            None => bail!("Missing field 'administrativeArea' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#locality: {
                        let field_value = match fields_map.get("locality") {
                            Some(value) => value,
                            None => bail!("Missing field 'locality' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organization: {
                        let field_value = match fields_map.get("organization") {
                            Some(value) => value,
                            None => bail!("Missing field 'organization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#postal_code: {
                        let field_value = match fields_map.get("postalCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'postalCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recipients: {
                        let field_value = match fields_map.get("recipients") {
                            Some(value) => value,
                            None => bail!("Missing field 'recipients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region_code: {
                        let field_value = match fields_map.get("regionCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'regionCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
