#include <pulumi_gestalt.h>
#include <stdio.h>

void test_provider(PulumiContext* ctx) {
    // 1. Register a provider
    RegisterResourceRequest provider_req = {
        .type_ = "pulumi:providers:random",
        .name = "custom-provider",
        .version = "4.15.1",
        .inputs = NULL,
        .inputs_len = 0,
        .provider = NULL
    };
    CustomCompositeOutputId* provider = pulumi_register_resource(ctx, &provider_req);
    CustomOutputId* provider_urn = pulumi_composite_get_urn(provider);

    // 2. Register a resource using that provider
    CustomOutputId* length = pulumi_create_output(ctx, "16");
    ObjectField fields[] = {
        {.name = "length", .value = length}
    };
    RegisterResourceRequest res_req = {
        .type_ = "random:index/randomString:RandomString",
        .name = "my_name",
        .version = "4.15.1",
        .inputs = fields,
        .inputs_len = 1,
        .provider = provider_urn
    };

    CustomCompositeOutputId* res = pulumi_register_resource(ctx, &res_req);
    CustomOutputId* res_urn = pulumi_composite_get_urn(res);

    pulumi_add_export(ctx, "res_urn", res_urn);
    pulumi_add_export(ctx, "prov_urn", provider_urn);
}
