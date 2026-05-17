#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolCertificate {
    /// The fully qualified ID of the certificate installed on the pool.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The location of the certificate store on the compute node into which the certificate is installed, either `CurrentUser` or `LocalMachine`.
    #[builder(into)]
    #[serde(rename = "storeLocation")]
    pub r#store_location: String,
    /// The name of the certificate store on the compute node into which the certificate is installed.
    #[builder(into)]
    #[serde(rename = "storeName")]
    pub r#store_name: String,
    /// Which user accounts on the compute node have access to the private data of the certificate.
    #[builder(into)]
    #[serde(rename = "visibilities")]
    pub r#visibilities: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPoolCertificate {
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
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "store_location".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#store_location,
                )
                .await,
            );
            map.insert(
                "store_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#store_name,
                )
                .await,
            );
            map.insert(
                "visibilities".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#visibilities,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPoolCertificate {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#store_location: {
                        let field_value = match fields_map.get("store_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'store_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#store_name: {
                        let field_value = match fields_map.get("store_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'store_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#visibilities: {
                        let field_value = match fields_map.get("visibilities") {
                            Some(value) => value,
                            None => bail!("Missing field 'visibilities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
