#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificateCertificatePolicyLifetimeActionTrigger {
    /// The number of days before the Certificate expires that the action associated with this Trigger should run.
    #[builder(into)]
    #[serde(rename = "daysBeforeExpiry")]
    pub r#days_before_expiry: i32,
    /// The percentage at which during the Certificates Lifetime the action associated with this Trigger should run.
    #[builder(into)]
    #[serde(rename = "lifetimePercentage")]
    pub r#lifetime_percentage: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCertificateCertificatePolicyLifetimeActionTrigger {
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
                "days_before_expiry".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#days_before_expiry,
                )
                .await,
            );
            map.insert(
                "lifetime_percentage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lifetime_percentage,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCertificateCertificatePolicyLifetimeActionTrigger {
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
                    r#days_before_expiry: {
                        let field_value = match fields_map.get("days_before_expiry") {
                            Some(value) => value,
                            None => bail!("Missing field 'days_before_expiry' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifetime_percentage: {
                        let field_value = match fields_map.get("lifetime_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifetime_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
