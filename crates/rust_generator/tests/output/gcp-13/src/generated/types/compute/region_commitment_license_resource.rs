#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionCommitmentLicenseResource {
    /// The number of licenses purchased.
    #[builder(into)]
    #[serde(rename = "amount")]
    pub r#amount: Option<String>,
    /// Specifies the core range of the instance for which this license applies.
    #[builder(into)]
    #[serde(rename = "coresPerLicense")]
    pub r#cores_per_license: Option<String>,
    /// Any applicable license URI.
    #[builder(into)]
    #[serde(rename = "license")]
    pub r#license: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionCommitmentLicenseResource {
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
                "amount".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#amount,
                )
                .await,
            );
            map.insert(
                "cores_per_license".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cores_per_license,
                )
                .await,
            );
            map.insert(
                "license".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#license,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionCommitmentLicenseResource {
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
                    r#amount: {
                        let field_value = match fields_map.get("amount") {
                            Some(value) => value,
                            None => bail!("Missing field 'amount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cores_per_license: {
                        let field_value = match fields_map.get("cores_per_license") {
                            Some(value) => value,
                            None => bail!("Missing field 'cores_per_license' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#license: {
                        let field_value = match fields_map.get("license") {
                            Some(value) => value,
                            None => bail!("Missing field 'license' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
