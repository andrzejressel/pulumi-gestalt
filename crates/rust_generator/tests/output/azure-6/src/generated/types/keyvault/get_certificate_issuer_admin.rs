#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificateIssuerAdmin {
    /// E-mail address of the admin.
    #[builder(into)]
    #[serde(rename = "emailAddress")]
    pub r#email_address: String,
    /// First name of the admin.
    #[builder(into)]
    #[serde(rename = "firstName")]
    pub r#first_name: String,
    /// Last name of the admin.
    #[builder(into)]
    #[serde(rename = "lastName")]
    pub r#last_name: String,
    /// Phone number of the admin.
    #[builder(into)]
    #[serde(rename = "phone")]
    pub r#phone: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCertificateIssuerAdmin {
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
                "email_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_address,
                )
                .await,
            );
            map.insert(
                "first_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#first_name,
                )
                .await,
            );
            map.insert(
                "last_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_name,
                )
                .await,
            );
            map.insert(
                "phone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#phone,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCertificateIssuerAdmin {
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
                    r#email_address: {
                        let field_value = match fields_map.get("email_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#first_name: {
                        let field_value = match fields_map.get("first_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'first_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_name: {
                        let field_value = match fields_map.get("last_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phone: {
                        let field_value = match fields_map.get("phone") {
                            Some(value) => value,
                            None => bail!("Missing field 'phone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
