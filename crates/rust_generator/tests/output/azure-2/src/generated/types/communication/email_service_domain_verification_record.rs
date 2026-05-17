#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EmailServiceDomainVerificationRecord {
    /// (Optional) An `dkim2` block as defined below.
    #[builder(into)]
    #[serde(rename = "dkim2s")]
    pub r#dkim_2_s: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDkim2>>,
    /// (Optional) An `dkim` block as defined below.
    #[builder(into)]
    #[serde(rename = "dkims")]
    pub r#dkims: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDkim>>,
    /// (Optional) An `dmarc` block as defined below.
    #[builder(into)]
    #[serde(rename = "dmarcs")]
    pub r#dmarcs: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDmarc>>,
    /// (Optional) An `domain` block as defined below.
    #[builder(into)]
    #[serde(rename = "domains")]
    pub r#domains: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDomain>>,
    /// (Optional) An `spf` block as defined below.
    #[builder(into)]
    #[serde(rename = "spfs")]
    pub r#spfs: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordSpf>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EmailServiceDomainVerificationRecord {
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
                "dkim_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dkim_2_s,
                )
                .await,
            );
            map.insert(
                "dkims".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dkims,
                )
                .await,
            );
            map.insert(
                "dmarcs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dmarcs,
                )
                .await,
            );
            map.insert(
                "domains".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domains,
                )
                .await,
            );
            map.insert(
                "spfs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spfs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EmailServiceDomainVerificationRecord {
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
                    r#dkim_2_s: {
                        let field_value = match fields_map.get("dkim_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'dkim_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dkims: {
                        let field_value = match fields_map.get("dkims") {
                            Some(value) => value,
                            None => bail!("Missing field 'dkims' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dmarcs: {
                        let field_value = match fields_map.get("dmarcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'dmarcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domains: {
                        let field_value = match fields_map.get("domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spfs: {
                        let field_value = match fields_map.get("spfs") {
                            Some(value) => value,
                            None => bail!("Missing field 'spfs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
