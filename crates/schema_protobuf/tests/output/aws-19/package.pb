
awsAWS"6.66.2*�
j
sesv2AccountSuppressionAttributesCaws:sesv2/accountSuppressionAttributes:AccountSuppressionAttributes�Manages AWS SESv2 (Simple Email V2) account-level suppression attributes.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.AccountSuppressionAttributes("example", {suppressedReasons: ["COMPLAINT"]});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.AccountSuppressionAttributes("example", suppressed_reasons=["COMPLAINT"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.AccountSuppressionAttributes("example", new()
    {
        SuppressedReasons = new[]
        {
            "COMPLAINT",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewAccountSuppressionAttributes(ctx, "example", &sesv2.AccountSuppressionAttributesArgs{
			SuppressedReasons: pulumi.StringArray{
				pulumi.String("COMPLAINT"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.AccountSuppressionAttributes;
import com.pulumi.aws.sesv2.AccountSuppressionAttributesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new AccountSuppressionAttributes("example", AccountSuppressionAttributesArgs.builder()
            .suppressedReasons("COMPLAINT")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:AccountSuppressionAttributes
    properties:
      suppressedReasons:
        - COMPLAINT
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import account-level suppression attributes using the account ID. For example:

```sh
$ pulumi import aws:sesv2/accountSuppressionAttributes:AccountSuppressionAttributes example 123456789012
```
�
suppressedReasons*" �A list that contains the reasons that email addresses will be automatically added to the suppression list for your account. Valid values: `COMPLAINT`, `BOUNCE`.
"�
suppressedReasons*" �A list that contains the reasons that email addresses will be automatically added to the suppression list for your account. Valid values: `COMPLAINT`, `BOUNCE`.
*�(
R
sesv2AccountVdmAttributes3aws:sesv2/accountVdmAttributes:AccountVdmAttributes�Resource for managing an AWS SESv2 (Simple Email V2) Account VDM Attributes.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.AccountVdmAttributes("example", {
    vdmEnabled: "ENABLED",
    dashboardAttributes: {
        engagementMetrics: "ENABLED",
    },
    guardianAttributes: {
        optimizedSharedDelivery: "ENABLED",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.AccountVdmAttributes("example",
    vdm_enabled="ENABLED",
    dashboard_attributes={
        "engagement_metrics": "ENABLED",
    },
    guardian_attributes={
        "optimized_shared_delivery": "ENABLED",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.AccountVdmAttributes("example", new()
    {
        VdmEnabled = "ENABLED",
        DashboardAttributes = new Aws.SesV2.Inputs.AccountVdmAttributesDashboardAttributesArgs
        {
            EngagementMetrics = "ENABLED",
        },
        GuardianAttributes = new Aws.SesV2.Inputs.AccountVdmAttributesGuardianAttributesArgs
        {
            OptimizedSharedDelivery = "ENABLED",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewAccountVdmAttributes(ctx, "example", &sesv2.AccountVdmAttributesArgs{
			VdmEnabled: pulumi.String("ENABLED"),
			DashboardAttributes: &sesv2.AccountVdmAttributesDashboardAttributesArgs{
				EngagementMetrics: pulumi.String("ENABLED"),
			},
			GuardianAttributes: &sesv2.AccountVdmAttributesGuardianAttributesArgs{
				OptimizedSharedDelivery: pulumi.String("ENABLED"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.AccountVdmAttributes;
import com.pulumi.aws.sesv2.AccountVdmAttributesArgs;
import com.pulumi.aws.sesv2.inputs.AccountVdmAttributesDashboardAttributesArgs;
import com.pulumi.aws.sesv2.inputs.AccountVdmAttributesGuardianAttributesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new AccountVdmAttributes("example", AccountVdmAttributesArgs.builder()
            .vdmEnabled("ENABLED")
            .dashboardAttributes(AccountVdmAttributesDashboardAttributesArgs.builder()
                .engagementMetrics("ENABLED")
                .build())
            .guardianAttributes(AccountVdmAttributesGuardianAttributesArgs.builder()
                .optimizedSharedDelivery("ENABLED")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:AccountVdmAttributes
    properties:
      vdmEnabled: ENABLED
      dashboardAttributes:
        engagementMetrics: ENABLED
      guardianAttributes:
        optimizedSharedDelivery: ENABLED
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Account VDM Attributes using the word `ses-account-vdm-attributes`. For example:

```sh
$ pulumi import aws:sesv2/accountVdmAttributes:AccountVdmAttributes example ses-account-vdm-attributes
```
�
dashboardAttributes�B�:�
�
sesv2'AccountVdmAttributesDashboardAttributesYaws:sesv2/AccountVdmAttributesDashboardAttributes:AccountVdmAttributesDashboardAttributesYSpecifies additional settings for your VDM configuration as applicable to the Dashboard.
�
guardianAttributes�B�:�
�
sesv2&AccountVdmAttributesGuardianAttributesWaws:sesv2/AccountVdmAttributesGuardianAttributes:AccountVdmAttributesGuardianAttributesXSpecifies additional settings for your VDM configuration as applicable to the Guardian.
�

vdmEnabled" |Specifies the status of your VDM configuration. Valid values: `ENABLED`, `DISABLED`.

The following arguments are optional:
"�
dashboardAttributes�:�
�
sesv2'AccountVdmAttributesDashboardAttributesYaws:sesv2/AccountVdmAttributesDashboardAttributes:AccountVdmAttributesDashboardAttributesYSpecifies additional settings for your VDM configuration as applicable to the Dashboard.
"�
guardianAttributes�:�
�
sesv2&AccountVdmAttributesGuardianAttributesWaws:sesv2/AccountVdmAttributesGuardianAttributes:AccountVdmAttributesGuardianAttributesXSpecifies additional settings for your VDM configuration as applicable to the Guardian.
"�

vdmEnabled" |Specifies the status of your VDM configuration. Valid values: `ENABLED`, `DISABLED`.

The following arguments are optional:
*�S
F
sesv2ConfigurationSet+aws:sesv2/configurationSet:ConfigurationSet�2Resource for managing an AWS SESv2 (Simple Email V2) Configuration Set.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.ConfigurationSet("example", {
    configurationSetName: "example",
    deliveryOptions: {
        maxDeliverySeconds: 300,
        tlsPolicy: "REQUIRE",
    },
    reputationOptions: {
        reputationMetricsEnabled: false,
    },
    sendingOptions: {
        sendingEnabled: true,
    },
    suppressionOptions: {
        suppressedReasons: [
            "BOUNCE",
            "COMPLAINT",
        ],
    },
    trackingOptions: {
        customRedirectDomain: "example.com",
        httpsPolicy: "REQUIRE",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.ConfigurationSet("example",
    configuration_set_name="example",
    delivery_options={
        "max_delivery_seconds": 300,
        "tls_policy": "REQUIRE",
    },
    reputation_options={
        "reputation_metrics_enabled": False,
    },
    sending_options={
        "sending_enabled": True,
    },
    suppression_options={
        "suppressed_reasons": [
            "BOUNCE",
            "COMPLAINT",
        ],
    },
    tracking_options={
        "custom_redirect_domain": "example.com",
        "https_policy": "REQUIRE",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.ConfigurationSet("example", new()
    {
        ConfigurationSetName = "example",
        DeliveryOptions = new Aws.SesV2.Inputs.ConfigurationSetDeliveryOptionsArgs
        {
            MaxDeliverySeconds = 300,
            TlsPolicy = "REQUIRE",
        },
        ReputationOptions = new Aws.SesV2.Inputs.ConfigurationSetReputationOptionsArgs
        {
            ReputationMetricsEnabled = false,
        },
        SendingOptions = new Aws.SesV2.Inputs.ConfigurationSetSendingOptionsArgs
        {
            SendingEnabled = true,
        },
        SuppressionOptions = new Aws.SesV2.Inputs.ConfigurationSetSuppressionOptionsArgs
        {
            SuppressedReasons = new[]
            {
                "BOUNCE",
                "COMPLAINT",
            },
        },
        TrackingOptions = new Aws.SesV2.Inputs.ConfigurationSetTrackingOptionsArgs
        {
            CustomRedirectDomain = "example.com",
            HttpsPolicy = "REQUIRE",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewConfigurationSet(ctx, "example", &sesv2.ConfigurationSetArgs{
			ConfigurationSetName: pulumi.String("example"),
			DeliveryOptions: &sesv2.ConfigurationSetDeliveryOptionsArgs{
				MaxDeliverySeconds: pulumi.Int(300),
				TlsPolicy:          pulumi.String("REQUIRE"),
			},
			ReputationOptions: &sesv2.ConfigurationSetReputationOptionsArgs{
				ReputationMetricsEnabled: pulumi.Bool(false),
			},
			SendingOptions: &sesv2.ConfigurationSetSendingOptionsArgs{
				SendingEnabled: pulumi.Bool(true),
			},
			SuppressionOptions: &sesv2.ConfigurationSetSuppressionOptionsArgs{
				SuppressedReasons: pulumi.StringArray{
					pulumi.String("BOUNCE"),
					pulumi.String("COMPLAINT"),
				},
			},
			TrackingOptions: &sesv2.ConfigurationSetTrackingOptionsArgs{
				CustomRedirectDomain: pulumi.String("example.com"),
				HttpsPolicy:          pulumi.String("REQUIRE"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.ConfigurationSet;
import com.pulumi.aws.sesv2.ConfigurationSetArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetDeliveryOptionsArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetReputationOptionsArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetSendingOptionsArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetSuppressionOptionsArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetTrackingOptionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ConfigurationSet("example", ConfigurationSetArgs.builder()
            .configurationSetName("example")
            .deliveryOptions(ConfigurationSetDeliveryOptionsArgs.builder()
                .maxDeliverySeconds(300)
                .tlsPolicy("REQUIRE")
                .build())
            .reputationOptions(ConfigurationSetReputationOptionsArgs.builder()
                .reputationMetricsEnabled(false)
                .build())
            .sendingOptions(ConfigurationSetSendingOptionsArgs.builder()
                .sendingEnabled(true)
                .build())
            .suppressionOptions(ConfigurationSetSuppressionOptionsArgs.builder()
                .suppressedReasons(                
                    "BOUNCE",
                    "COMPLAINT")
                .build())
            .trackingOptions(ConfigurationSetTrackingOptionsArgs.builder()
                .customRedirectDomain("example.com")
                .httpsPolicy("REQUIRE")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:ConfigurationSet
    properties:
      configurationSetName: example
      deliveryOptions:
        maxDeliverySeconds: 300
        tlsPolicy: REQUIRE
      reputationOptions:
        reputationMetricsEnabled: false
      sendingOptions:
        sendingEnabled: true
      suppressionOptions:
        suppressedReasons:
          - BOUNCE
          - COMPLAINT
      trackingOptions:
        customRedirectDomain: example.com
        httpsPolicy: REQUIRE
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Configuration Set using the `configuration_set_name`. For example:

```sh
$ pulumi import aws:sesv2/configurationSet:ConfigurationSet example example
```
?
configurationSetName" #The name of the configuration set.
�
deliveryOptionsyBw:u
s
sesv2ConfigurationSetDeliveryOptionsIaws:sesv2/ConfigurationSetDeliveryOptions:ConfigurationSetDeliveryOptions�An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set. See `delivery_options` Block for details.
�
reputationOptionsB}:{
y
sesv2!ConfigurationSetReputationOptionsMaws:sesv2/ConfigurationSetReputationOptions:ConfigurationSetReputationOptions�An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set. See `reputation_options` Block for details.
�
sendingOptionsvBt:r
p
sesv2ConfigurationSetSendingOptionsGaws:sesv2/ConfigurationSetSendingOptions:ConfigurationSetSendingOptions�An object that defines whether or not Amazon SES can send email that you send using the configuration set. See `sending_options` Block for details.
�
suppressionOptions�B�:~
|
sesv2"ConfigurationSetSuppressionOptionsOaws:sesv2/ConfigurationSetSuppressionOptions:ConfigurationSetSuppressionOptions�An object that contains information about the suppression list preferences for your account. See `suppression_options` Block for details.
�
tagsB2" �A map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
trackingOptionsyBw:u
s
sesv2ConfigurationSetTrackingOptionsIaws:sesv2/ConfigurationSetTrackingOptions:ConfigurationSetTrackingOptions�An object that defines the open and click tracking options for emails that you send using the configuration set. See `tracking_options` Block for details.
�

vdmOptionsjBh:f
d
sesv2ConfigurationSetVdmOptions?aws:sesv2/ConfigurationSetVdmOptions:ConfigurationSetVdmOptions�An object that defines the VDM settings that apply to emails that you send using the configuration set. See `vdm_options` Block for details.
")
arn" ARN of the Configuration Set.
"?
configurationSetName" #The name of the configuration set.
"�
deliveryOptionsyBw:u
s
sesv2ConfigurationSetDeliveryOptionsIaws:sesv2/ConfigurationSetDeliveryOptions:ConfigurationSetDeliveryOptions�An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set. See `delivery_options` Block for details.
"�
reputationOptions}:{
y
sesv2!ConfigurationSetReputationOptionsMaws:sesv2/ConfigurationSetReputationOptions:ConfigurationSetReputationOptions�An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set. See `reputation_options` Block for details.
"�
sendingOptionst:r
p
sesv2ConfigurationSetSendingOptionsGaws:sesv2/ConfigurationSetSendingOptions:ConfigurationSetSendingOptions�An object that defines whether or not Amazon SES can send email that you send using the configuration set. See `sending_options` Block for details.
"�
suppressionOptions�B�:~
|
sesv2"ConfigurationSetSuppressionOptionsOaws:sesv2/ConfigurationSetSuppressionOptions:ConfigurationSetSuppressionOptions�An object that contains information about the suppression list preferences for your account. See `suppression_options` Block for details.
"�
tagsB2" �A map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "�
trackingOptionsyBw:u
s
sesv2ConfigurationSetTrackingOptionsIaws:sesv2/ConfigurationSetTrackingOptions:ConfigurationSetTrackingOptions�An object that defines the open and click tracking options for emails that you send using the configuration set. See `tracking_options` Block for details.
"�

vdmOptionsjBh:f
d
sesv2ConfigurationSetVdmOptions?aws:sesv2/ConfigurationSetVdmOptions:ConfigurationSetVdmOptions�An object that defines the VDM settings that apply to emails that you send using the configuration set. See `vdm_options` Block for details.
*��
v
sesv2 ConfigurationSetEventDestinationKaws:sesv2/configurationSetEventDestination:ConfigurationSetEventDestination��Resource for managing an AWS SESv2 (Simple Email V2) Configuration Set Event Destination.

## Example Usage

### CloudWatch Destination

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.ConfigurationSet("example", {configurationSetName: "example"});
const exampleConfigurationSetEventDestination = new aws.sesv2.ConfigurationSetEventDestination("example", {
    configurationSetName: example.configurationSetName,
    eventDestinationName: "example",
    eventDestination: {
        cloudWatchDestination: {
            dimensionConfigurations: [{
                defaultDimensionValue: "example",
                dimensionName: "example",
                dimensionValueSource: "MESSAGE_TAG",
            }],
        },
        enabled: true,
        matchingEventTypes: ["SEND"],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.ConfigurationSet("example", configuration_set_name="example")
example_configuration_set_event_destination = aws.sesv2.ConfigurationSetEventDestination("example",
    configuration_set_name=example.configuration_set_name,
    event_destination_name="example",
    event_destination={
        "cloud_watch_destination": {
            "dimension_configurations": [{
                "default_dimension_value": "example",
                "dimension_name": "example",
                "dimension_value_source": "MESSAGE_TAG",
            }],
        },
        "enabled": True,
        "matching_event_types": ["SEND"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.ConfigurationSet("example", new()
    {
        ConfigurationSetName = "example",
    });

    var exampleConfigurationSetEventDestination = new Aws.SesV2.ConfigurationSetEventDestination("example", new()
    {
        ConfigurationSetName = example.ConfigurationSetName,
        EventDestinationName = "example",
        EventDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationArgs
        {
            CloudWatchDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationArgs
            {
                DimensionConfigurations = new[]
                {
                    new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfigurationArgs
                    {
                        DefaultDimensionValue = "example",
                        DimensionName = "example",
                        DimensionValueSource = "MESSAGE_TAG",
                    },
                },
            },
            Enabled = true,
            MatchingEventTypes = new[]
            {
                "SEND",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sesv2.NewConfigurationSet(ctx, "example", &sesv2.ConfigurationSetArgs{
			ConfigurationSetName: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = sesv2.NewConfigurationSetEventDestination(ctx, "example", &sesv2.ConfigurationSetEventDestinationArgs{
			ConfigurationSetName: example.ConfigurationSetName,
			EventDestinationName: pulumi.String("example"),
			EventDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationArgs{
				CloudWatchDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationArgs{
					DimensionConfigurations: sesv2.ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfigurationArray{
						&sesv2.ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfigurationArgs{
							DefaultDimensionValue: pulumi.String("example"),
							DimensionName:         pulumi.String("example"),
							DimensionValueSource:  pulumi.String("MESSAGE_TAG"),
						},
					},
				},
				Enabled: pulumi.Bool(true),
				MatchingEventTypes: pulumi.StringArray{
					pulumi.String("SEND"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.ConfigurationSet;
import com.pulumi.aws.sesv2.ConfigurationSetArgs;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestination;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ConfigurationSet("example", ConfigurationSetArgs.builder()
            .configurationSetName("example")
            .build());

        var exampleConfigurationSetEventDestination = new ConfigurationSetEventDestination("exampleConfigurationSetEventDestination", ConfigurationSetEventDestinationArgs.builder()
            .configurationSetName(example.configurationSetName())
            .eventDestinationName("example")
            .eventDestination(ConfigurationSetEventDestinationEventDestinationArgs.builder()
                .cloudWatchDestination(ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationArgs.builder()
                    .dimensionConfigurations(ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfigurationArgs.builder()
                        .defaultDimensionValue("example")
                        .dimensionName("example")
                        .dimensionValueSource("MESSAGE_TAG")
                        .build())
                    .build())
                .enabled(true)
                .matchingEventTypes("SEND")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:ConfigurationSet
    properties:
      configurationSetName: example
  exampleConfigurationSetEventDestination:
    type: aws:sesv2:ConfigurationSetEventDestination
    name: example
    properties:
      configurationSetName: ${example.configurationSetName}
      eventDestinationName: example
      eventDestination:
        cloudWatchDestination:
          dimensionConfigurations:
            - defaultDimensionValue: example
              dimensionName: example
              dimensionValueSource: MESSAGE_TAG
        enabled: true
        matchingEventTypes:
          - SEND
```
<!--End PulumiCodeChooser -->

### EventBridge Destination

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const default = aws.cloudwatch.getEventBus({
    name: "default",
});
const example = new aws.sesv2.ConfigurationSetEventDestination("example", {
    configurationSetName: exampleAwsSesv2ConfigurationSet.configurationSetName,
    eventDestinationName: "example",
    eventDestination: {
        eventBridgeDestination: {
            eventBusArn: _default.then(_default => _default.arn),
        },
        enabled: true,
        matchingEventTypes: ["SEND"],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

default = aws.cloudwatch.get_event_bus(name="default")
example = aws.sesv2.ConfigurationSetEventDestination("example",
    configuration_set_name=example_aws_sesv2_configuration_set["configurationSetName"],
    event_destination_name="example",
    event_destination={
        "event_bridge_destination": {
            "event_bus_arn": default.arn,
        },
        "enabled": True,
        "matching_event_types": ["SEND"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var @default = Aws.CloudWatch.GetEventBus.Invoke(new()
    {
        Name = "default",
    });

    var example = new Aws.SesV2.ConfigurationSetEventDestination("example", new()
    {
        ConfigurationSetName = exampleAwsSesv2ConfigurationSet.ConfigurationSetName,
        EventDestinationName = "example",
        EventDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationArgs
        {
            EventBridgeDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationEventBridgeDestinationArgs
            {
                EventBusArn = @default.Apply(@default => @default.Apply(getEventBusResult => getEventBusResult.Arn)),
            },
            Enabled = true,
            MatchingEventTypes = new[]
            {
                "SEND",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudwatch"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_default, err := cloudwatch.LookupEventBus(ctx, &cloudwatch.LookupEventBusArgs{
			Name: "default",
		}, nil)
		if err != nil {
			return err
		}
		_, err = sesv2.NewConfigurationSetEventDestination(ctx, "example", &sesv2.ConfigurationSetEventDestinationArgs{
			ConfigurationSetName: pulumi.Any(exampleAwsSesv2ConfigurationSet.ConfigurationSetName),
			EventDestinationName: pulumi.String("example"),
			EventDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationArgs{
				EventBridgeDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationEventBridgeDestinationArgs{
					EventBusArn: pulumi.String(_default.Arn),
				},
				Enabled: pulumi.Bool(true),
				MatchingEventTypes: pulumi.StringArray{
					pulumi.String("SEND"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cloudwatch.CloudwatchFunctions;
import com.pulumi.aws.cloudwatch.inputs.GetEventBusArgs;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestination;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationEventBridgeDestinationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var default = CloudwatchFunctions.getEventBus(GetEventBusArgs.builder()
            .name("default")
            .build());

        var example = new ConfigurationSetEventDestination("example", ConfigurationSetEventDestinationArgs.builder()
            .configurationSetName(exampleAwsSesv2ConfigurationSet.configurationSetName())
            .eventDestinationName("example")
            .eventDestination(ConfigurationSetEventDestinationEventDestinationArgs.builder()
                .eventBridgeDestination(ConfigurationSetEventDestinationEventDestinationEventBridgeDestinationArgs.builder()
                    .eventBusArn(default_.arn())
                    .build())
                .enabled(true)
                .matchingEventTypes("SEND")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:ConfigurationSetEventDestination
    properties:
      configurationSetName: ${exampleAwsSesv2ConfigurationSet.configurationSetName}
      eventDestinationName: example
      eventDestination:
        eventBridgeDestination:
          eventBusArn: ${default.arn}
        enabled: true
        matchingEventTypes:
          - SEND
variables:
  default:
    fn::invoke:
      function: aws:cloudwatch:getEventBus
      arguments:
        name: default
```
<!--End PulumiCodeChooser -->

### Kinesis Firehose Destination

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.ConfigurationSet("example", {configurationSetName: "example"});
const exampleConfigurationSetEventDestination = new aws.sesv2.ConfigurationSetEventDestination("example", {
    configurationSetName: example.configurationSetName,
    eventDestinationName: "example",
    eventDestination: {
        kinesisFirehoseDestination: {
            deliveryStreamArn: exampleAwsKinesisFirehoseDeliveryStream.arn,
            iamRoleArn: exampleAwsIamRole.arn,
        },
        enabled: true,
        matchingEventTypes: ["SEND"],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.ConfigurationSet("example", configuration_set_name="example")
example_configuration_set_event_destination = aws.sesv2.ConfigurationSetEventDestination("example",
    configuration_set_name=example.configuration_set_name,
    event_destination_name="example",
    event_destination={
        "kinesis_firehose_destination": {
            "delivery_stream_arn": example_aws_kinesis_firehose_delivery_stream["arn"],
            "iam_role_arn": example_aws_iam_role["arn"],
        },
        "enabled": True,
        "matching_event_types": ["SEND"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.ConfigurationSet("example", new()
    {
        ConfigurationSetName = "example",
    });

    var exampleConfigurationSetEventDestination = new Aws.SesV2.ConfigurationSetEventDestination("example", new()
    {
        ConfigurationSetName = example.ConfigurationSetName,
        EventDestinationName = "example",
        EventDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationArgs
        {
            KinesisFirehoseDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestinationArgs
            {
                DeliveryStreamArn = exampleAwsKinesisFirehoseDeliveryStream.Arn,
                IamRoleArn = exampleAwsIamRole.Arn,
            },
            Enabled = true,
            MatchingEventTypes = new[]
            {
                "SEND",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sesv2.NewConfigurationSet(ctx, "example", &sesv2.ConfigurationSetArgs{
			ConfigurationSetName: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = sesv2.NewConfigurationSetEventDestination(ctx, "example", &sesv2.ConfigurationSetEventDestinationArgs{
			ConfigurationSetName: example.ConfigurationSetName,
			EventDestinationName: pulumi.String("example"),
			EventDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationArgs{
				KinesisFirehoseDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestinationArgs{
					DeliveryStreamArn: pulumi.Any(exampleAwsKinesisFirehoseDeliveryStream.Arn),
					IamRoleArn:        pulumi.Any(exampleAwsIamRole.Arn),
				},
				Enabled: pulumi.Bool(true),
				MatchingEventTypes: pulumi.StringArray{
					pulumi.String("SEND"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.ConfigurationSet;
import com.pulumi.aws.sesv2.ConfigurationSetArgs;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestination;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestinationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ConfigurationSet("example", ConfigurationSetArgs.builder()
            .configurationSetName("example")
            .build());

        var exampleConfigurationSetEventDestination = new ConfigurationSetEventDestination("exampleConfigurationSetEventDestination", ConfigurationSetEventDestinationArgs.builder()
            .configurationSetName(example.configurationSetName())
            .eventDestinationName("example")
            .eventDestination(ConfigurationSetEventDestinationEventDestinationArgs.builder()
                .kinesisFirehoseDestination(ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestinationArgs.builder()
                    .deliveryStreamArn(exampleAwsKinesisFirehoseDeliveryStream.arn())
                    .iamRoleArn(exampleAwsIamRole.arn())
                    .build())
                .enabled(true)
                .matchingEventTypes("SEND")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:ConfigurationSet
    properties:
      configurationSetName: example
  exampleConfigurationSetEventDestination:
    type: aws:sesv2:ConfigurationSetEventDestination
    name: example
    properties:
      configurationSetName: ${example.configurationSetName}
      eventDestinationName: example
      eventDestination:
        kinesisFirehoseDestination:
          deliveryStreamArn: ${exampleAwsKinesisFirehoseDeliveryStream.arn}
          iamRoleArn: ${exampleAwsIamRole.arn}
        enabled: true
        matchingEventTypes:
          - SEND
```
<!--End PulumiCodeChooser -->

### Pinpoint Destination

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.ConfigurationSet("example", {configurationSetName: "example"});
const exampleConfigurationSetEventDestination = new aws.sesv2.ConfigurationSetEventDestination("example", {
    configurationSetName: example.configurationSetName,
    eventDestinationName: "example",
    eventDestination: {
        pinpointDestination: {
            applicationArn: exampleAwsPinpointApp.arn,
        },
        enabled: true,
        matchingEventTypes: ["SEND"],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.ConfigurationSet("example", configuration_set_name="example")
example_configuration_set_event_destination = aws.sesv2.ConfigurationSetEventDestination("example",
    configuration_set_name=example.configuration_set_name,
    event_destination_name="example",
    event_destination={
        "pinpoint_destination": {
            "application_arn": example_aws_pinpoint_app["arn"],
        },
        "enabled": True,
        "matching_event_types": ["SEND"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.ConfigurationSet("example", new()
    {
        ConfigurationSetName = "example",
    });

    var exampleConfigurationSetEventDestination = new Aws.SesV2.ConfigurationSetEventDestination("example", new()
    {
        ConfigurationSetName = example.ConfigurationSetName,
        EventDestinationName = "example",
        EventDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationArgs
        {
            PinpointDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationPinpointDestinationArgs
            {
                ApplicationArn = exampleAwsPinpointApp.Arn,
            },
            Enabled = true,
            MatchingEventTypes = new[]
            {
                "SEND",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sesv2.NewConfigurationSet(ctx, "example", &sesv2.ConfigurationSetArgs{
			ConfigurationSetName: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = sesv2.NewConfigurationSetEventDestination(ctx, "example", &sesv2.ConfigurationSetEventDestinationArgs{
			ConfigurationSetName: example.ConfigurationSetName,
			EventDestinationName: pulumi.String("example"),
			EventDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationArgs{
				PinpointDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationPinpointDestinationArgs{
					ApplicationArn: pulumi.Any(exampleAwsPinpointApp.Arn),
				},
				Enabled: pulumi.Bool(true),
				MatchingEventTypes: pulumi.StringArray{
					pulumi.String("SEND"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.ConfigurationSet;
import com.pulumi.aws.sesv2.ConfigurationSetArgs;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestination;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationPinpointDestinationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ConfigurationSet("example", ConfigurationSetArgs.builder()
            .configurationSetName("example")
            .build());

        var exampleConfigurationSetEventDestination = new ConfigurationSetEventDestination("exampleConfigurationSetEventDestination", ConfigurationSetEventDestinationArgs.builder()
            .configurationSetName(example.configurationSetName())
            .eventDestinationName("example")
            .eventDestination(ConfigurationSetEventDestinationEventDestinationArgs.builder()
                .pinpointDestination(ConfigurationSetEventDestinationEventDestinationPinpointDestinationArgs.builder()
                    .applicationArn(exampleAwsPinpointApp.arn())
                    .build())
                .enabled(true)
                .matchingEventTypes("SEND")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:ConfigurationSet
    properties:
      configurationSetName: example
  exampleConfigurationSetEventDestination:
    type: aws:sesv2:ConfigurationSetEventDestination
    name: example
    properties:
      configurationSetName: ${example.configurationSetName}
      eventDestinationName: example
      eventDestination:
        pinpointDestination:
          applicationArn: ${exampleAwsPinpointApp.arn}
        enabled: true
        matchingEventTypes:
          - SEND
```
<!--End PulumiCodeChooser -->

### SNS Destination

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.ConfigurationSet("example", {configurationSetName: "example"});
const exampleConfigurationSetEventDestination = new aws.sesv2.ConfigurationSetEventDestination("example", {
    configurationSetName: example.configurationSetName,
    eventDestinationName: "example",
    eventDestination: {
        snsDestination: {
            topicArn: exampleAwsSnsTopic.arn,
        },
        enabled: true,
        matchingEventTypes: ["SEND"],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.ConfigurationSet("example", configuration_set_name="example")
example_configuration_set_event_destination = aws.sesv2.ConfigurationSetEventDestination("example",
    configuration_set_name=example.configuration_set_name,
    event_destination_name="example",
    event_destination={
        "sns_destination": {
            "topic_arn": example_aws_sns_topic["arn"],
        },
        "enabled": True,
        "matching_event_types": ["SEND"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.ConfigurationSet("example", new()
    {
        ConfigurationSetName = "example",
    });

    var exampleConfigurationSetEventDestination = new Aws.SesV2.ConfigurationSetEventDestination("example", new()
    {
        ConfigurationSetName = example.ConfigurationSetName,
        EventDestinationName = "example",
        EventDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationArgs
        {
            SnsDestination = new Aws.SesV2.Inputs.ConfigurationSetEventDestinationEventDestinationSnsDestinationArgs
            {
                TopicArn = exampleAwsSnsTopic.Arn,
            },
            Enabled = true,
            MatchingEventTypes = new[]
            {
                "SEND",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sesv2.NewConfigurationSet(ctx, "example", &sesv2.ConfigurationSetArgs{
			ConfigurationSetName: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = sesv2.NewConfigurationSetEventDestination(ctx, "example", &sesv2.ConfigurationSetEventDestinationArgs{
			ConfigurationSetName: example.ConfigurationSetName,
			EventDestinationName: pulumi.String("example"),
			EventDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationArgs{
				SnsDestination: &sesv2.ConfigurationSetEventDestinationEventDestinationSnsDestinationArgs{
					TopicArn: pulumi.Any(exampleAwsSnsTopic.Arn),
				},
				Enabled: pulumi.Bool(true),
				MatchingEventTypes: pulumi.StringArray{
					pulumi.String("SEND"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.ConfigurationSet;
import com.pulumi.aws.sesv2.ConfigurationSetArgs;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestination;
import com.pulumi.aws.sesv2.ConfigurationSetEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationArgs;
import com.pulumi.aws.sesv2.inputs.ConfigurationSetEventDestinationEventDestinationSnsDestinationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ConfigurationSet("example", ConfigurationSetArgs.builder()
            .configurationSetName("example")
            .build());

        var exampleConfigurationSetEventDestination = new ConfigurationSetEventDestination("exampleConfigurationSetEventDestination", ConfigurationSetEventDestinationArgs.builder()
            .configurationSetName(example.configurationSetName())
            .eventDestinationName("example")
            .eventDestination(ConfigurationSetEventDestinationEventDestinationArgs.builder()
                .snsDestination(ConfigurationSetEventDestinationEventDestinationSnsDestinationArgs.builder()
                    .topicArn(exampleAwsSnsTopic.arn())
                    .build())
                .enabled(true)
                .matchingEventTypes("SEND")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:ConfigurationSet
    properties:
      configurationSetName: example
  exampleConfigurationSetEventDestination:
    type: aws:sesv2:ConfigurationSetEventDestination
    name: example
    properties:
      configurationSetName: ${example.configurationSetName}
      eventDestinationName: example
      eventDestination:
        snsDestination:
          topicArn: ${exampleAwsSnsTopic.arn}
        enabled: true
        matchingEventTypes:
          - SEND
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Configuration Set Event Destination using the `id` (`configuration_set_name|event_destination_name`). For example:

```sh
$ pulumi import aws:sesv2/configurationSetEventDestination:ConfigurationSetEventDestination example example_configuration_set|example_event_destination
```
?
configurationSetName" #The name of the configuration set.
�
eventDestination�:�
�
sesv20ConfigurationSetEventDestinationEventDestinationkaws:sesv2/ConfigurationSetEventDestinationEventDestination:ConfigurationSetEventDestinationEventDestinationKA name that identifies the event destination within the configuration set.
u
eventDestinationName" YAn object that defines the event destination. See `event_destination` Block for details.
"?
configurationSetName" #The name of the configuration set.
"�
eventDestination�:�
�
sesv20ConfigurationSetEventDestinationEventDestinationkaws:sesv2/ConfigurationSetEventDestinationEventDestination:ConfigurationSetEventDestinationEventDestinationKA name that identifies the event destination within the configuration set.
"u
eventDestinationName" YAn object that defines the event destination. See `event_destination` Block for details.
*�6
7
sesv2ContactList!aws:sesv2/contactList:ContactList�+Resource for managing an AWS SESv2 (Simple Email V2) Contact List.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.ContactList("example", {contactListName: "example"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.ContactList("example", contact_list_name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.ContactList("example", new()
    {
        ContactListName = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewContactList(ctx, "example", &sesv2.ContactListArgs{
			ContactListName: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.ContactList;
import com.pulumi.aws.sesv2.ContactListArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ContactList("example", ContactListArgs.builder()
            .contactListName("example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:ContactList
    properties:
      contactListName: example
```
<!--End PulumiCodeChooser -->

### Extended Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.ContactList("example", {
    contactListName: "example",
    description: "description",
    topics: [{
        defaultSubscriptionStatus: "OPT_IN",
        description: "topic description",
        displayName: "Example Topic",
        topicName: "example-topic",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.ContactList("example",
    contact_list_name="example",
    description="description",
    topics=[{
        "default_subscription_status": "OPT_IN",
        "description": "topic description",
        "display_name": "Example Topic",
        "topic_name": "example-topic",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.ContactList("example", new()
    {
        ContactListName = "example",
        Description = "description",
        Topics = new[]
        {
            new Aws.SesV2.Inputs.ContactListTopicArgs
            {
                DefaultSubscriptionStatus = "OPT_IN",
                Description = "topic description",
                DisplayName = "Example Topic",
                TopicName = "example-topic",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewContactList(ctx, "example", &sesv2.ContactListArgs{
			ContactListName: pulumi.String("example"),
			Description:     pulumi.String("description"),
			Topics: sesv2.ContactListTopicArray{
				&sesv2.ContactListTopicArgs{
					DefaultSubscriptionStatus: pulumi.String("OPT_IN"),
					Description:               pulumi.String("topic description"),
					DisplayName:               pulumi.String("Example Topic"),
					TopicName:                 pulumi.String("example-topic"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.ContactList;
import com.pulumi.aws.sesv2.ContactListArgs;
import com.pulumi.aws.sesv2.inputs.ContactListTopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ContactList("example", ContactListArgs.builder()
            .contactListName("example")
            .description("description")
            .topics(ContactListTopicArgs.builder()
                .defaultSubscriptionStatus("OPT_IN")
                .description("topic description")
                .displayName("Example Topic")
                .topicName("example-topic")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:ContactList
    properties:
      contactListName: example
      description: description
      topics:
        - defaultSubscriptionStatus: OPT_IN
          description: topic description
          displayName: Example Topic
          topicName: example-topic
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Contact List using the `id`. For example:

```sh
$ pulumi import aws:sesv2/contactList:ContactList example example
```
X
contactListName" AName of the contact list.

The following arguments are optional:
D
descriptionB" /Description of what the contact list is about.
�
tagsB2" �Key-value map of resource tags for the contact list. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
topicsNBL*J:H
F
sesv2ContactListTopic+aws:sesv2/ContactListTopic:ContactListTopicHConfiguration block(s) with topic for the contact list. Detailed below.
"	
arn" "X
contactListName" AName of the contact list.

The following arguments are optional:
"_
createdTimestamp" GTimestamp noting when the contact list was created in ISO 8601 format.
"D
descriptionB" /Description of what the contact list is about.
"l
lastUpdatedTimestamp" PTimestamp noting the last time the contact list was updated in ISO 8601 format.
"�
tagsB2" �Key-value map of resource tags for the contact list. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "�
topicsNBL*J:H
F
sesv2ContactListTopic+aws:sesv2/ContactListTopic:ContactListTopicHConfiguration block(s) with topic for the contact list. Detailed below.
*�
U
sesv2DedicatedIpAssignment5aws:sesv2/dedicatedIpAssignment:DedicatedIpAssignment�Resource for managing an AWS SESv2 (Simple Email V2) Dedicated IP Assignment.

This resource is used with "Standard" dedicated IP addresses. This includes addresses [requested and relinquished manually](https://docs.aws.amazon.com/ses/latest/dg/dedicated-ip-case.html) via an AWS support case, or [Bring Your Own IP](https://docs.aws.amazon.com/ses/latest/dg/dedicated-ip-byo.html) addresses. Once no longer assigned, this resource returns the IP to the [`ses-default-dedicated-pool`](https://docs.aws.amazon.com/ses/latest/dg/managing-ip-pools.html), managed by AWS.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.DedicatedIpAssignment("example", {
    ip: "0.0.0.0",
    destinationPoolName: "my-pool",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.DedicatedIpAssignment("example",
    ip="0.0.0.0",
    destination_pool_name="my-pool")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.DedicatedIpAssignment("example", new()
    {
        Ip = "0.0.0.0",
        DestinationPoolName = "my-pool",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewDedicatedIpAssignment(ctx, "example", &sesv2.DedicatedIpAssignmentArgs{
			Ip:                  pulumi.String("0.0.0.0"),
			DestinationPoolName: pulumi.String("my-pool"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.DedicatedIpAssignment;
import com.pulumi.aws.sesv2.DedicatedIpAssignmentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new DedicatedIpAssignment("example", DedicatedIpAssignmentArgs.builder()
            .ip("0.0.0.0")
            .destinationPoolName("my-pool")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:DedicatedIpAssignment
    properties:
      ip: 0.0.0.0
      destinationPoolName: my-pool
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Dedicated IP Assignment using the `id`, which is a comma-separated string made up of `ip` and `destination_pool_name`. For example:

```sh
$ pulumi import aws:sesv2/dedicatedIpAssignment:DedicatedIpAssignment example "0.0.0.0,my-pool"
```
1
destinationPoolName" Dedicated IP address.
 
ip" Dedicated IP address.
"1
destinationPoolName" Dedicated IP address.
" 
ip" Dedicated IP address.
*�(
C
sesv2DedicatedIpPool)aws:sesv2/dedicatedIpPool:DedicatedIpPool� Resource for managing an AWS SESv2 (Simple Email V2) Dedicated IP Pool.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.DedicatedIpPool("example", {poolName: "my-pool"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.DedicatedIpPool("example", pool_name="my-pool")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.DedicatedIpPool("example", new()
    {
        PoolName = "my-pool",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewDedicatedIpPool(ctx, "example", &sesv2.DedicatedIpPoolArgs{
			PoolName: pulumi.String("my-pool"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.DedicatedIpPool;
import com.pulumi.aws.sesv2.DedicatedIpPoolArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new DedicatedIpPool("example", DedicatedIpPoolArgs.builder()
            .poolName("my-pool")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:DedicatedIpPool
    properties:
      poolName: my-pool
```
<!--End PulumiCodeChooser -->

### Managed Pool

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.DedicatedIpPool("example", {
    poolName: "my-managed-pool",
    scalingMode: "MANAGED",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.DedicatedIpPool("example",
    pool_name="my-managed-pool",
    scaling_mode="MANAGED")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.DedicatedIpPool("example", new()
    {
        PoolName = "my-managed-pool",
        ScalingMode = "MANAGED",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewDedicatedIpPool(ctx, "example", &sesv2.DedicatedIpPoolArgs{
			PoolName:    pulumi.String("my-managed-pool"),
			ScalingMode: pulumi.String("MANAGED"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.DedicatedIpPool;
import com.pulumi.aws.sesv2.DedicatedIpPoolArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new DedicatedIpPool("example", DedicatedIpPoolArgs.builder()
            .poolName("my-managed-pool")
            .scalingMode("MANAGED")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:DedicatedIpPool
    properties:
      poolName: my-managed-pool
      scalingMode: MANAGED
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Dedicated IP Pool using the `pool_name`. For example:

```sh
$ pulumi import aws:sesv2/dedicatedIpPool:DedicatedIpPool example my-pool
```
V
poolName" FName of the dedicated IP pool.

The following arguments are optional:
�
scalingModeB" tIP pool scaling mode. Valid values: `STANDARD`, `MANAGED`. If omitted, the AWS API will default to a standard pool.
�
tagsB2" �A map of tags to assign to the pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
")
arn" ARN of the Dedicated IP Pool.
"V
poolName" FName of the dedicated IP pool.

The following arguments are optional:
"�
scalingMode" tIP pool scaling mode. Valid values: `STANDARD`, `MANAGED`. If omitted, the AWS API will default to a standard pool.
"�
tagsB2" �A map of tags to assign to the pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" *�`
=
sesv2EmailIdentity%aws:sesv2/emailIdentity:EmailIdentity�QResource for managing an AWS SESv2 (Simple Email V2) Email Identity.

## Example Usage

### Basic Usage

### Email Address Identity

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.EmailIdentity("example", {emailIdentity: "testing@example.com"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.EmailIdentity("example", email_identity="testing@example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.EmailIdentity("example", new()
    {
        EmailIdentityDetails = "testing@example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewEmailIdentity(ctx, "example", &sesv2.EmailIdentityArgs{
			EmailIdentity: pulumi.String("testing@example.com"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.EmailIdentity;
import com.pulumi.aws.sesv2.EmailIdentityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new EmailIdentity("example", EmailIdentityArgs.builder()
            .emailIdentity("testing@example.com")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:EmailIdentity
    properties:
      emailIdentity: testing@example.com
```
<!--End PulumiCodeChooser -->

### Domain Identity

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.EmailIdentity("example", {emailIdentity: "example.com"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.EmailIdentity("example", email_identity="example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.EmailIdentity("example", new()
    {
        EmailIdentityDetails = "example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewEmailIdentity(ctx, "example", &sesv2.EmailIdentityArgs{
			EmailIdentity: pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.EmailIdentity;
import com.pulumi.aws.sesv2.EmailIdentityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new EmailIdentity("example", EmailIdentityArgs.builder()
            .emailIdentity("example.com")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:EmailIdentity
    properties:
      emailIdentity: example.com
```
<!--End PulumiCodeChooser -->

### Configuration Set

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.ConfigurationSet("example", {configurationSetName: "example"});
const exampleEmailIdentity = new aws.sesv2.EmailIdentity("example", {
    emailIdentity: "example.com",
    configurationSetName: example.configurationSetName,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.ConfigurationSet("example", configuration_set_name="example")
example_email_identity = aws.sesv2.EmailIdentity("example",
    email_identity="example.com",
    configuration_set_name=example.configuration_set_name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.ConfigurationSet("example", new()
    {
        ConfigurationSetName = "example",
    });

    var exampleEmailIdentity = new Aws.SesV2.EmailIdentity("example", new()
    {
        EmailIdentityDetails = "example.com",
        ConfigurationSetName = example.ConfigurationSetName,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sesv2.NewConfigurationSet(ctx, "example", &sesv2.ConfigurationSetArgs{
			ConfigurationSetName: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = sesv2.NewEmailIdentity(ctx, "example", &sesv2.EmailIdentityArgs{
			EmailIdentity:        pulumi.String("example.com"),
			ConfigurationSetName: example.ConfigurationSetName,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.ConfigurationSet;
import com.pulumi.aws.sesv2.ConfigurationSetArgs;
import com.pulumi.aws.sesv2.EmailIdentity;
import com.pulumi.aws.sesv2.EmailIdentityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ConfigurationSet("example", ConfigurationSetArgs.builder()
            .configurationSetName("example")
            .build());

        var exampleEmailIdentity = new EmailIdentity("exampleEmailIdentity", EmailIdentityArgs.builder()
            .emailIdentity("example.com")
            .configurationSetName(example.configurationSetName())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:ConfigurationSet
    properties:
      configurationSetName: example
  exampleEmailIdentity:
    type: aws:sesv2:EmailIdentity
    name: example
    properties:
      emailIdentity: example.com
      configurationSetName: ${example.configurationSetName}
```
<!--End PulumiCodeChooser -->

### DKIM Signing Attributes (BYODKIM)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.EmailIdentity("example", {
    emailIdentity: "example.com",
    dkimSigningAttributes: {
        domainSigningPrivateKey: "MIIJKAIBAAKCAgEA2Se7p8zvnI4yh+Gh9j2rG5e2aRXjg03Y8saiupLnadPH9xvM...",
        domainSigningSelector: "example",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.EmailIdentity("example",
    email_identity="example.com",
    dkim_signing_attributes={
        "domain_signing_private_key": "MIIJKAIBAAKCAgEA2Se7p8zvnI4yh+Gh9j2rG5e2aRXjg03Y8saiupLnadPH9xvM...",
        "domain_signing_selector": "example",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.EmailIdentity("example", new()
    {
        EmailIdentityDetails = "example.com",
        DkimSigningAttributes = new Aws.SesV2.Inputs.EmailIdentityDkimSigningAttributesArgs
        {
            DomainSigningPrivateKey = "MIIJKAIBAAKCAgEA2Se7p8zvnI4yh+Gh9j2rG5e2aRXjg03Y8saiupLnadPH9xvM...",
            DomainSigningSelector = "example",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.NewEmailIdentity(ctx, "example", &sesv2.EmailIdentityArgs{
			EmailIdentity: pulumi.String("example.com"),
			DkimSigningAttributes: &sesv2.EmailIdentityDkimSigningAttributesArgs{
				DomainSigningPrivateKey: pulumi.String("MIIJKAIBAAKCAgEA2Se7p8zvnI4yh+Gh9j2rG5e2aRXjg03Y8saiupLnadPH9xvM..."),
				DomainSigningSelector:   pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.EmailIdentity;
import com.pulumi.aws.sesv2.EmailIdentityArgs;
import com.pulumi.aws.sesv2.inputs.EmailIdentityDkimSigningAttributesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new EmailIdentity("example", EmailIdentityArgs.builder()
            .emailIdentity("example.com")
            .dkimSigningAttributes(EmailIdentityDkimSigningAttributesArgs.builder()
                .domainSigningPrivateKey("MIIJKAIBAAKCAgEA2Se7p8zvnI4yh+Gh9j2rG5e2aRXjg03Y8saiupLnadPH9xvM...")
                .domainSigningSelector("example")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:EmailIdentity
    properties:
      emailIdentity: example.com
      dkimSigningAttributes:
        domainSigningPrivateKey: MIIJKAIBAAKCAgEA2Se7p8zvnI4yh+Gh9j2rG5e2aRXjg03Y8saiupLnadPH9xvM...
        domainSigningSelector: example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Email Identity using the `email_identity`. For example:

```sh
$ pulumi import aws:sesv2/emailIdentity:EmailIdentity example example.com
```
�
configurationSetNameB" �The configuration set to use by default when sending from this identity. Note that any configuration set defined in the email sending request takes precedence.
�
dkimSigningAttributes�B�:~
|
sesv2"EmailIdentityDkimSigningAttributesOaws:sesv2/EmailIdentityDkimSigningAttributes:EmailIdentityDkimSigningAttributesTThe configuration of the DKIM authentication settings for an email domain identity.
c
emailIdentity" NThe email address or domain to verify.

The following arguments are optional:
�
tagsB2" �Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"&
arn" ARN of the Email Identity.
"�
configurationSetNameB" �The configuration set to use by default when sending from this identity. Note that any configuration set defined in the email sending request takes precedence.
"�
dkimSigningAttributes�:~
|
sesv2"EmailIdentityDkimSigningAttributesOaws:sesv2/EmailIdentityDkimSigningAttributes:EmailIdentityDkimSigningAttributesTThe configuration of the DKIM authentication settings for an email domain identity.
"c
emailIdentity" NThe email address or domain to verify.

The following arguments are optional:
"V
identityType" BThe email identity type. Valid values: `EMAIL_ADDRESS`, `DOMAIN`.
"�
tagsB2" �Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"S
verifiedForSendingStatus
 3Specifies whether or not the identity is verified.
*�
s
sesv2EmailIdentityFeedbackAttributesIaws:sesv2/emailIdentityFeedbackAttributes:EmailIdentityFeedbackAttributes�Resource for managing an AWS SESv2 (Simple Email V2) Email Identity Feedback Attributes.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.EmailIdentity("example", {emailIdentity: "example.com"});
const exampleEmailIdentityFeedbackAttributes = new aws.sesv2.EmailIdentityFeedbackAttributes("example", {
    emailIdentity: example.emailIdentity,
    emailForwardingEnabled: true,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.EmailIdentity("example", email_identity="example.com")
example_email_identity_feedback_attributes = aws.sesv2.EmailIdentityFeedbackAttributes("example",
    email_identity=example.email_identity,
    email_forwarding_enabled=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.EmailIdentity("example", new()
    {
        EmailIdentityDetails = "example.com",
    });

    var exampleEmailIdentityFeedbackAttributes = new Aws.SesV2.EmailIdentityFeedbackAttributes("example", new()
    {
        EmailIdentity = example.EmailIdentityDetails,
        EmailForwardingEnabled = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sesv2.NewEmailIdentity(ctx, "example", &sesv2.EmailIdentityArgs{
			EmailIdentity: pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		_, err = sesv2.NewEmailIdentityFeedbackAttributes(ctx, "example", &sesv2.EmailIdentityFeedbackAttributesArgs{
			EmailIdentity:          example.EmailIdentity,
			EmailForwardingEnabled: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.EmailIdentity;
import com.pulumi.aws.sesv2.EmailIdentityArgs;
import com.pulumi.aws.sesv2.EmailIdentityFeedbackAttributes;
import com.pulumi.aws.sesv2.EmailIdentityFeedbackAttributesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new EmailIdentity("example", EmailIdentityArgs.builder()
            .emailIdentity("example.com")
            .build());

        var exampleEmailIdentityFeedbackAttributes = new EmailIdentityFeedbackAttributes("exampleEmailIdentityFeedbackAttributes", EmailIdentityFeedbackAttributesArgs.builder()
            .emailIdentity(example.emailIdentity())
            .emailForwardingEnabled(true)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:EmailIdentity
    properties:
      emailIdentity: example.com
  exampleEmailIdentityFeedbackAttributes:
    type: aws:sesv2:EmailIdentityFeedbackAttributes
    name: example
    properties:
      emailIdentity: ${example.emailIdentity}
      emailForwardingEnabled: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Email Identity Feedback Attributes using the `email_identity`. For example:

```sh
$ pulumi import aws:sesv2/emailIdentityFeedbackAttributes:EmailIdentityFeedbackAttributes example example.com
```
]
emailForwardingEnabledB
 =Sets the feedback forwarding configuration for the identity.
)
emailIdentity" The email identity.
"]
emailForwardingEnabledB
 =Sets the feedback forwarding configuration for the identity.
")
emailIdentity" The email identity.
*�)
s
sesv2EmailIdentityMailFromAttributesIaws:sesv2/emailIdentityMailFromAttributes:EmailIdentityMailFromAttributes�"Resource for managing an AWS SESv2 (Simple Email V2) Email Identity Mail From Attributes.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.EmailIdentity("example", {emailIdentity: "example.com"});
const exampleEmailIdentityMailFromAttributes = new aws.sesv2.EmailIdentityMailFromAttributes("example", {
    emailIdentity: example.emailIdentity,
    behaviorOnMxFailure: "REJECT_MESSAGE",
    mailFromDomain: pulumi.interpolate`subdomain.${example.emailIdentity}`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.EmailIdentity("example", email_identity="example.com")
example_email_identity_mail_from_attributes = aws.sesv2.EmailIdentityMailFromAttributes("example",
    email_identity=example.email_identity,
    behavior_on_mx_failure="REJECT_MESSAGE",
    mail_from_domain=example.email_identity.apply(lambda email_identity: f"subdomain.{email_identity}"))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.EmailIdentity("example", new()
    {
        EmailIdentityDetails = "example.com",
    });

    var exampleEmailIdentityMailFromAttributes = new Aws.SesV2.EmailIdentityMailFromAttributes("example", new()
    {
        EmailIdentity = example.EmailIdentityDetails,
        BehaviorOnMxFailure = "REJECT_MESSAGE",
        MailFromDomain = example.EmailIdentityDetails.Apply(emailIdentity => $"subdomain.{emailIdentity}"),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sesv2.NewEmailIdentity(ctx, "example", &sesv2.EmailIdentityArgs{
			EmailIdentity: pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		_, err = sesv2.NewEmailIdentityMailFromAttributes(ctx, "example", &sesv2.EmailIdentityMailFromAttributesArgs{
			EmailIdentity:       example.EmailIdentity,
			BehaviorOnMxFailure: pulumi.String("REJECT_MESSAGE"),
			MailFromDomain: example.EmailIdentity.ApplyT(func(emailIdentity string) (string, error) {
				return fmt.Sprintf("subdomain.%v", emailIdentity), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.EmailIdentity;
import com.pulumi.aws.sesv2.EmailIdentityArgs;
import com.pulumi.aws.sesv2.EmailIdentityMailFromAttributes;
import com.pulumi.aws.sesv2.EmailIdentityMailFromAttributesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new EmailIdentity("example", EmailIdentityArgs.builder()
            .emailIdentity("example.com")
            .build());

        var exampleEmailIdentityMailFromAttributes = new EmailIdentityMailFromAttributes("exampleEmailIdentityMailFromAttributes", EmailIdentityMailFromAttributesArgs.builder()
            .emailIdentity(example.emailIdentity())
            .behaviorOnMxFailure("REJECT_MESSAGE")
            .mailFromDomain(example.emailIdentity().applyValue(emailIdentity -> String.format("subdomain.%s", emailIdentity)))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:EmailIdentity
    properties:
      emailIdentity: example.com
  exampleEmailIdentityMailFromAttributes:
    type: aws:sesv2:EmailIdentityMailFromAttributes
    name: example
    properties:
      emailIdentity: ${example.emailIdentity}
      behaviorOnMxFailure: REJECT_MESSAGE
      mailFromDomain: subdomain.${example.emailIdentity}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Email Identity Mail From Attributes using the `email_identity`. For example:

```sh
$ pulumi import aws:sesv2/emailIdentityMailFromAttributes:EmailIdentityMailFromAttributes example example.com
```
�
behaviorOnMxFailureB" �The action to take if the required MX record isn't found when you send an email. Valid values: `USE_DEFAULT_VALUE`, `REJECT_MESSAGE`.
2
emailIdentity" The verified email identity.
�
mailFromDomainB" �The custom MAIL FROM domain that you want the verified identity to use. Required if `behavior_on_mx_failure` is `REJECT_MESSAGE`.
"�
behaviorOnMxFailureB" �The action to take if the required MX record isn't found when you send an email. Valid values: `USE_DEFAULT_VALUE`, `REJECT_MESSAGE`.
"2
emailIdentity" The verified email identity.
"�
mailFromDomainB" �The custom MAIL FROM domain that you want the verified identity to use. Required if `behavior_on_mx_failure` is `REJECT_MESSAGE`.
*�7
O
sesv2EmailIdentityPolicy1aws:sesv2/emailIdentityPolicy:EmailIdentityPolicy�4Resource for managing an AWS SESv2 (Simple Email V2) Email Identity Policy.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sesv2.EmailIdentity("example", {emailIdentity: "testing@example.com"});
const exampleEmailIdentityPolicy = new aws.sesv2.EmailIdentityPolicy("example", {
    emailIdentity: example.emailIdentity,
    policyName: "example",
    policy: pulumi.interpolate`{
  "Id":"ExampleAuthorizationPolicy",
  "Version":"2012-10-17",
  "Statement":[
    {
      "Sid":"AuthorizeIAMUser",
      "Effect":"Allow",
      "Resource":"${example.arn}",
      "Principal":{
        "AWS":[
          "arn:aws:iam::123456789012:user/John",
          "arn:aws:iam::123456789012:user/Jane"
        ]
      },
      "Action":[
        "ses:DeleteEmailIdentity",
        "ses:PutEmailIdentityDkimSigningAttributes"
      ]
    }
  ]
}
`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.EmailIdentity("example", email_identity="testing@example.com")
example_email_identity_policy = aws.sesv2.EmailIdentityPolicy("example",
    email_identity=example.email_identity,
    policy_name="example",
    policy=example.arn.apply(lambda arn: f"""{{
  "Id":"ExampleAuthorizationPolicy",
  "Version":"2012-10-17",
  "Statement":[
    {{
      "Sid":"AuthorizeIAMUser",
      "Effect":"Allow",
      "Resource":"{arn}",
      "Principal":{{
        "AWS":[
          "arn:aws:iam::123456789012:user/John",
          "arn:aws:iam::123456789012:user/Jane"
        ]
      }},
      "Action":[
        "ses:DeleteEmailIdentity",
        "ses:PutEmailIdentityDkimSigningAttributes"
      ]
    }}
  ]
}}
"""))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SesV2.EmailIdentity("example", new()
    {
        EmailIdentityDetails = "testing@example.com",
    });

    var exampleEmailIdentityPolicy = new Aws.SesV2.EmailIdentityPolicy("example", new()
    {
        EmailIdentity = example.EmailIdentityDetails,
        PolicyName = "example",
        Policy = example.Arn.Apply(arn => @$"{{
  ""Id"":""ExampleAuthorizationPolicy"",
  ""Version"":""2012-10-17"",
  ""Statement"":[
    {{
      ""Sid"":""AuthorizeIAMUser"",
      ""Effect"":""Allow"",
      ""Resource"":""{arn}"",
      ""Principal"":{{
        ""AWS"":[
          ""arn:aws:iam::123456789012:user/John"",
          ""arn:aws:iam::123456789012:user/Jane""
        ]
      }},
      ""Action"":[
        ""ses:DeleteEmailIdentity"",
        ""ses:PutEmailIdentityDkimSigningAttributes""
      ]
    }}
  ]
}}
"),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sesv2.NewEmailIdentity(ctx, "example", &sesv2.EmailIdentityArgs{
			EmailIdentity: pulumi.String("testing@example.com"),
		})
		if err != nil {
			return err
		}
		_, err = sesv2.NewEmailIdentityPolicy(ctx, "example", &sesv2.EmailIdentityPolicyArgs{
			EmailIdentity: example.EmailIdentity,
			PolicyName:    pulumi.String("example"),
			Policy: example.Arn.ApplyT(func(arn string) (string, error) {
				return fmt.Sprintf(`{
  "Id":"ExampleAuthorizationPolicy",
  "Version":"2012-10-17",
  "Statement":[
    {
      "Sid":"AuthorizeIAMUser",
      "Effect":"Allow",
      "Resource":"%v",
      "Principal":{
        "AWS":[
          "arn:aws:iam::123456789012:user/John",
          "arn:aws:iam::123456789012:user/Jane"
        ]
      },
      "Action":[
        "ses:DeleteEmailIdentity",
        "ses:PutEmailIdentityDkimSigningAttributes"
      ]
    }
  ]
}
`, arn), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.EmailIdentity;
import com.pulumi.aws.sesv2.EmailIdentityArgs;
import com.pulumi.aws.sesv2.EmailIdentityPolicy;
import com.pulumi.aws.sesv2.EmailIdentityPolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new EmailIdentity("example", EmailIdentityArgs.builder()
            .emailIdentity("testing@example.com")
            .build());

        var exampleEmailIdentityPolicy = new EmailIdentityPolicy("exampleEmailIdentityPolicy", EmailIdentityPolicyArgs.builder()
            .emailIdentity(example.emailIdentity())
            .policyName("example")
            .policy(example.arn().applyValue(arn -> """
{
  "Id":"ExampleAuthorizationPolicy",
  "Version":"2012-10-17",
  "Statement":[
    {
      "Sid":"AuthorizeIAMUser",
      "Effect":"Allow",
      "Resource":"%s",
      "Principal":{
        "AWS":[
          "arn:aws:iam::123456789012:user/John",
          "arn:aws:iam::123456789012:user/Jane"
        ]
      },
      "Action":[
        "ses:DeleteEmailIdentity",
        "ses:PutEmailIdentityDkimSigningAttributes"
      ]
    }
  ]
}
", arn)))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sesv2:EmailIdentity
    properties:
      emailIdentity: testing@example.com
  exampleEmailIdentityPolicy:
    type: aws:sesv2:EmailIdentityPolicy
    name: example
    properties:
      emailIdentity: ${example.emailIdentity}
      policyName: example
      policy: |
        {
          "Id":"ExampleAuthorizationPolicy",
          "Version":"2012-10-17",
          "Statement":[
            {
              "Sid":"AuthorizeIAMUser",
              "Effect":"Allow",
              "Resource":"${example.arn}",
              "Principal":{
                "AWS":[
                  "arn:aws:iam::123456789012:user/John",
                  "arn:aws:iam::123456789012:user/Jane"
                ]
              },
              "Action":[
                "ses:DeleteEmailIdentity",
                "ses:PutEmailIdentityDkimSigningAttributes"
              ]
            }
          ]
        }
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SESv2 (Simple Email V2) Email Identity Policy using the `example_id_arg`. For example:

```sh
$ pulumi import aws:sesv2/emailIdentityPolicy:EmailIdentityPolicy example example_email_identity|example_policy_name
```
)
emailIdentity" The email identity.
5
policy" 'The text of the policy in JSON format.
*

policyName" The name of the policy.
")
emailIdentity" The email identity.
"5
policy" 'The text of the policy in JSON format.
"*

policyName" The name of the policy.
*�7
*
sfnActivityaws:sfn/activity:Activity�*Provides a Step Function Activity resource

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const sfnActivity = new aws.sfn.Activity("sfn_activity", {name: "my-activity"});
```
```python
import pulumi
import pulumi_aws as aws

sfn_activity = aws.sfn.Activity("sfn_activity", name="my-activity")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var sfnActivity = new Aws.Sfn.Activity("sfn_activity", new()
    {
        Name = "my-activity",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sfn.NewActivity(ctx, "sfn_activity", &sfn.ActivityArgs{
			Name: pulumi.String("my-activity"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.Activity;
import com.pulumi.aws.sfn.ActivityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var sfnActivity = new Activity("sfnActivity", ActivityArgs.builder()
            .name("my-activity")
            .build());

    }
}
```
```yaml
resources:
  sfnActivity:
    type: aws:sfn:Activity
    name: sfn_activity
    properties:
      name: my-activity
```
<!--End PulumiCodeChooser -->

### Encryption

> *NOTE:* See the section [Data at rest encyption](https://docs.aws.amazon.com/step-functions/latest/dg/encryption-at-rest.html) in the [AWS Step Functions Developer Guide](https://docs.aws.amazon.com/step-functions/latest/dg/welcome.html) for more information about enabling encryption of data using a customer-managed key for Step Functions State Machines data.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const sfnActivity = new aws.sfn.Activity("sfn_activity", {
    name: "my-activity",
    encryptionConfiguration: {
        kmsKeyId: kmsKeyForSfn.arn,
        type: "CUSTOMER_MANAGED_KMS_KEY",
        kmsDataKeyReusePeriodSeconds: 900,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

sfn_activity = aws.sfn.Activity("sfn_activity",
    name="my-activity",
    encryption_configuration={
        "kms_key_id": kms_key_for_sfn["arn"],
        "type": "CUSTOMER_MANAGED_KMS_KEY",
        "kms_data_key_reuse_period_seconds": 900,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var sfnActivity = new Aws.Sfn.Activity("sfn_activity", new()
    {
        Name = "my-activity",
        EncryptionConfiguration = new Aws.Sfn.Inputs.ActivityEncryptionConfigurationArgs
        {
            KmsKeyId = kmsKeyForSfn.Arn,
            Type = "CUSTOMER_MANAGED_KMS_KEY",
            KmsDataKeyReusePeriodSeconds = 900,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sfn.NewActivity(ctx, "sfn_activity", &sfn.ActivityArgs{
			Name: pulumi.String("my-activity"),
			EncryptionConfiguration: &sfn.ActivityEncryptionConfigurationArgs{
				KmsKeyId:                     pulumi.Any(kmsKeyForSfn.Arn),
				Type:                         pulumi.String("CUSTOMER_MANAGED_KMS_KEY"),
				KmsDataKeyReusePeriodSeconds: pulumi.Int(900),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.Activity;
import com.pulumi.aws.sfn.ActivityArgs;
import com.pulumi.aws.sfn.inputs.ActivityEncryptionConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var sfnActivity = new Activity("sfnActivity", ActivityArgs.builder()
            .name("my-activity")
            .encryptionConfiguration(ActivityEncryptionConfigurationArgs.builder()
                .kmsKeyId(kmsKeyForSfn.arn())
                .type("CUSTOMER_MANAGED_KMS_KEY")
                .kmsDataKeyReusePeriodSeconds(900)
                .build())
            .build());

    }
}
```
```yaml
resources:
  sfnActivity:
    type: aws:sfn:Activity
    name: sfn_activity
    properties:
      name: my-activity
      encryptionConfiguration:
        kmsKeyId: ${kmsKeyForSfn.arn}
        type: CUSTOMER_MANAGED_KMS_KEY
        kmsDataKeyReusePeriodSeconds: 900
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import activities using the `arn`. For example:

```sh
$ pulumi import aws:sfn/activity:Activity foo arn:aws:states:eu-west-1:123456789098:activity:bar
```
�
encryptionConfigurationuBs:q
o
sfnActivityEncryptionConfigurationGaws:sfn/ActivityEncryptionConfiguration:ActivityEncryptionConfiguration�Defines what encryption configuration is used to encrypt data in the Activity. For more information see the section [Data at rest encyption](https://docs.aws.amazon.com/step-functions/latest/dg/encryption-at-rest.html) in the AWS Step Functions User Guide.
2
nameB" $The name of the activity to create.
�
tagsB2" �Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"7
creationDate" #The date the activity was created.
"�
encryptionConfigurations:q
o
sfnActivityEncryptionConfigurationGaws:sfn/ActivityEncryptionConfiguration:ActivityEncryptionConfiguration�Defines what encryption configuration is used to encrypt data in the Activity. For more information see the section [Data at rest encyption](https://docs.aws.amazon.com/step-functions/latest/dg/encryption-at-rest.html) in the AWS Step Functions User Guide.
"0
name" $The name of the activity to create.
"�
tagsB2" �Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�5
!
sfnAliasaws:sfn/alias:Alias�/Provides a Step Function State Machine Alias.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const sfnAlias = new aws.sfn.Alias("sfn_alias", {
    name: "my_sfn_alias",
    routingConfigurations: [{
        stateMachineVersionArn: sfnTest.stateMachineVersionArn,
        weight: 100,
    }],
});
const mySfnAlias = new aws.sfn.Alias("my_sfn_alias", {
    name: "my_sfn_alias",
    routingConfigurations: [
        {
            stateMachineVersionArn: "arn:aws:states:us-east-1:12345:stateMachine:demo:3",
            weight: 50,
        },
        {
            stateMachineVersionArn: "arn:aws:states:us-east-1:12345:stateMachine:demo:2",
            weight: 50,
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

sfn_alias = aws.sfn.Alias("sfn_alias",
    name="my_sfn_alias",
    routing_configurations=[{
        "state_machine_version_arn": sfn_test["stateMachineVersionArn"],
        "weight": 100,
    }])
my_sfn_alias = aws.sfn.Alias("my_sfn_alias",
    name="my_sfn_alias",
    routing_configurations=[
        {
            "state_machine_version_arn": "arn:aws:states:us-east-1:12345:stateMachine:demo:3",
            "weight": 50,
        },
        {
            "state_machine_version_arn": "arn:aws:states:us-east-1:12345:stateMachine:demo:2",
            "weight": 50,
        },
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var sfnAlias = new Aws.Sfn.Alias("sfn_alias", new()
    {
        Name = "my_sfn_alias",
        RoutingConfigurations = new[]
        {
            new Aws.Sfn.Inputs.AliasRoutingConfigurationArgs
            {
                StateMachineVersionArn = sfnTest.StateMachineVersionArn,
                Weight = 100,
            },
        },
    });

    var mySfnAlias = new Aws.Sfn.Alias("my_sfn_alias", new()
    {
        Name = "my_sfn_alias",
        RoutingConfigurations = new[]
        {
            new Aws.Sfn.Inputs.AliasRoutingConfigurationArgs
            {
                StateMachineVersionArn = "arn:aws:states:us-east-1:12345:stateMachine:demo:3",
                Weight = 50,
            },
            new Aws.Sfn.Inputs.AliasRoutingConfigurationArgs
            {
                StateMachineVersionArn = "arn:aws:states:us-east-1:12345:stateMachine:demo:2",
                Weight = 50,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sfn.NewAlias(ctx, "sfn_alias", &sfn.AliasArgs{
			Name: pulumi.String("my_sfn_alias"),
			RoutingConfigurations: sfn.AliasRoutingConfigurationArray{
				&sfn.AliasRoutingConfigurationArgs{
					StateMachineVersionArn: pulumi.Any(sfnTest.StateMachineVersionArn),
					Weight:                 pulumi.Int(100),
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = sfn.NewAlias(ctx, "my_sfn_alias", &sfn.AliasArgs{
			Name: pulumi.String("my_sfn_alias"),
			RoutingConfigurations: sfn.AliasRoutingConfigurationArray{
				&sfn.AliasRoutingConfigurationArgs{
					StateMachineVersionArn: pulumi.String("arn:aws:states:us-east-1:12345:stateMachine:demo:3"),
					Weight:                 pulumi.Int(50),
				},
				&sfn.AliasRoutingConfigurationArgs{
					StateMachineVersionArn: pulumi.String("arn:aws:states:us-east-1:12345:stateMachine:demo:2"),
					Weight:                 pulumi.Int(50),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.Alias;
import com.pulumi.aws.sfn.AliasArgs;
import com.pulumi.aws.sfn.inputs.AliasRoutingConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var sfnAlias = new Alias("sfnAlias", AliasArgs.builder()
            .name("my_sfn_alias")
            .routingConfigurations(AliasRoutingConfigurationArgs.builder()
                .stateMachineVersionArn(sfnTest.stateMachineVersionArn())
                .weight(100)
                .build())
            .build());

        var mySfnAlias = new Alias("mySfnAlias", AliasArgs.builder()
            .name("my_sfn_alias")
            .routingConfigurations(            
                AliasRoutingConfigurationArgs.builder()
                    .stateMachineVersionArn("arn:aws:states:us-east-1:12345:stateMachine:demo:3")
                    .weight(50)
                    .build(),
                AliasRoutingConfigurationArgs.builder()
                    .stateMachineVersionArn("arn:aws:states:us-east-1:12345:stateMachine:demo:2")
                    .weight(50)
                    .build())
            .build());

    }
}
```
```yaml
resources:
  sfnAlias:
    type: aws:sfn:Alias
    name: sfn_alias
    properties:
      name: my_sfn_alias
      routingConfigurations:
        - stateMachineVersionArn: ${sfnTest.stateMachineVersionArn}
          weight: 100
  mySfnAlias:
    type: aws:sfn:Alias
    name: my_sfn_alias
    properties:
      name: my_sfn_alias
      routingConfigurations:
        - stateMachineVersionArn: arn:aws:states:us-east-1:12345:stateMachine:demo:3
          weight: 50
        - stateMachineVersionArn: arn:aws:states:us-east-1:12345:stateMachine:demo:2
          weight: 50
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SFN (Step Functions) Alias using the `arn`. For example:

```sh
$ pulumi import aws:sfn/alias:Alias foo arn:aws:states:us-east-1:123456789098:stateMachine:myStateMachine:foo
```
/
descriptionB" Description of the alias.
3
nameB" %Name for the alias you are creating.
�
routingConfigurationsc*a:_
]
sfnAliasRoutingConfiguration;aws:sfn/AliasRoutingConfiguration:AliasRoutingConfigurationNThe StateMachine alias' route configuration settings. Fields documented below
"P
arn" EThe Amazon Resource Name (ARN) identifying your state machine alias.
"B
creationDate" .The date the state machine alias was created.
"/
descriptionB" Description of the alias.
"1
name" %Name for the alias you are creating.
"�
routingConfigurationsc*a:_
]
sfnAliasRoutingConfiguration;aws:sfn/AliasRoutingConfiguration:AliasRoutingConfigurationNThe StateMachine alias' route configuration settings. Fields documented below
*��
6
sfnStateMachine!aws:sfn/stateMachine:StateMachine��Provides a Step Function State Machine resource

## Example Usage

### Basic (Standard Workflow)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// ...
const sfnStateMachine = new aws.sfn.StateMachine("sfn_state_machine", {
    name: "my-state-machine",
    roleArn: iamForSfn.arn,
    definition: `{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "${lambda.arn}",
      "End": true
    }
  }
}
`,
});
```
```python
import pulumi
import pulumi_aws as aws

# ...
sfn_state_machine = aws.sfn.StateMachine("sfn_state_machine",
    name="my-state-machine",
    role_arn=iam_for_sfn["arn"],
    definition=f"""{{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {{
    "HelloWorld": {{
      "Type": "Task",
      "Resource": "{lambda_["arn"]}",
      "End": true
    }}
  }}
}}
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // ...
    var sfnStateMachine = new Aws.Sfn.StateMachine("sfn_state_machine", new()
    {
        Name = "my-state-machine",
        RoleArn = iamForSfn.Arn,
        Definition = @$"{{
  ""Comment"": ""A Hello World example of the Amazon States Language using an AWS Lambda Function"",
  ""StartAt"": ""HelloWorld"",
  ""States"": {{
    ""HelloWorld"": {{
      ""Type"": ""Task"",
      ""Resource"": ""{lambda.Arn}"",
      ""End"": true
    }}
  }}
}}
",
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// ...
		_, err := sfn.NewStateMachine(ctx, "sfn_state_machine", &sfn.StateMachineArgs{
			Name:    pulumi.String("my-state-machine"),
			RoleArn: pulumi.Any(iamForSfn.Arn),
			Definition: pulumi.Sprintf(`{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%v",
      "End": true
    }
  }
}
`, lambda.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.StateMachine;
import com.pulumi.aws.sfn.StateMachineArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // ...
        var sfnStateMachine = new StateMachine("sfnStateMachine", StateMachineArgs.builder()
            .name("my-state-machine")
            .roleArn(iamForSfn.arn())
            .definition("""
{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%s",
      "End": true
    }
  }
}
", lambda.arn()))
            .build());

    }
}
```
```yaml
resources:
  # ...
  sfnStateMachine:
    type: aws:sfn:StateMachine
    name: sfn_state_machine
    properties:
      name: my-state-machine
      roleArn: ${iamForSfn.arn}
      definition: |
        {
          "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
          "StartAt": "HelloWorld",
          "States": {
            "HelloWorld": {
              "Type": "Task",
              "Resource": "${lambda.arn}",
              "End": true
            }
          }
        }
```
<!--End PulumiCodeChooser -->

### Basic (Express Workflow)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// ...
const sfnStateMachine = new aws.sfn.StateMachine("sfn_state_machine", {
    name: "my-state-machine",
    roleArn: iamForSfn.arn,
    type: "EXPRESS",
    definition: `{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "${lambda.arn}",
      "End": true
    }
  }
}
`,
});
```
```python
import pulumi
import pulumi_aws as aws

# ...
sfn_state_machine = aws.sfn.StateMachine("sfn_state_machine",
    name="my-state-machine",
    role_arn=iam_for_sfn["arn"],
    type="EXPRESS",
    definition=f"""{{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {{
    "HelloWorld": {{
      "Type": "Task",
      "Resource": "{lambda_["arn"]}",
      "End": true
    }}
  }}
}}
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // ...
    var sfnStateMachine = new Aws.Sfn.StateMachine("sfn_state_machine", new()
    {
        Name = "my-state-machine",
        RoleArn = iamForSfn.Arn,
        Type = "EXPRESS",
        Definition = @$"{{
  ""Comment"": ""A Hello World example of the Amazon States Language using an AWS Lambda Function"",
  ""StartAt"": ""HelloWorld"",
  ""States"": {{
    ""HelloWorld"": {{
      ""Type"": ""Task"",
      ""Resource"": ""{lambda.Arn}"",
      ""End"": true
    }}
  }}
}}
",
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// ...
		_, err := sfn.NewStateMachine(ctx, "sfn_state_machine", &sfn.StateMachineArgs{
			Name:    pulumi.String("my-state-machine"),
			RoleArn: pulumi.Any(iamForSfn.Arn),
			Type:    pulumi.String("EXPRESS"),
			Definition: pulumi.Sprintf(`{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%v",
      "End": true
    }
  }
}
`, lambda.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.StateMachine;
import com.pulumi.aws.sfn.StateMachineArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // ...
        var sfnStateMachine = new StateMachine("sfnStateMachine", StateMachineArgs.builder()
            .name("my-state-machine")
            .roleArn(iamForSfn.arn())
            .type("EXPRESS")
            .definition("""
{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%s",
      "End": true
    }
  }
}
", lambda.arn()))
            .build());

    }
}
```
```yaml
resources:
  # ...
  sfnStateMachine:
    type: aws:sfn:StateMachine
    name: sfn_state_machine
    properties:
      name: my-state-machine
      roleArn: ${iamForSfn.arn}
      type: EXPRESS
      definition: |
        {
          "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
          "StartAt": "HelloWorld",
          "States": {
            "HelloWorld": {
              "Type": "Task",
              "Resource": "${lambda.arn}",
              "End": true
            }
          }
        }
```
<!--End PulumiCodeChooser -->

### Publish (Publish SFN version)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// ...
const sfnStateMachine = new aws.sfn.StateMachine("sfn_state_machine", {
    name: "my-state-machine",
    roleArn: iamForSfn.arn,
    publish: true,
    type: "EXPRESS",
    definition: `{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "${lambda.arn}",
      "End": true
    }
  }
}
`,
});
```
```python
import pulumi
import pulumi_aws as aws

# ...
sfn_state_machine = aws.sfn.StateMachine("sfn_state_machine",
    name="my-state-machine",
    role_arn=iam_for_sfn["arn"],
    publish=True,
    type="EXPRESS",
    definition=f"""{{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {{
    "HelloWorld": {{
      "Type": "Task",
      "Resource": "{lambda_["arn"]}",
      "End": true
    }}
  }}
}}
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // ...
    var sfnStateMachine = new Aws.Sfn.StateMachine("sfn_state_machine", new()
    {
        Name = "my-state-machine",
        RoleArn = iamForSfn.Arn,
        Publish = true,
        Type = "EXPRESS",
        Definition = @$"{{
  ""Comment"": ""A Hello World example of the Amazon States Language using an AWS Lambda Function"",
  ""StartAt"": ""HelloWorld"",
  ""States"": {{
    ""HelloWorld"": {{
      ""Type"": ""Task"",
      ""Resource"": ""{lambda.Arn}"",
      ""End"": true
    }}
  }}
}}
",
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// ...
		_, err := sfn.NewStateMachine(ctx, "sfn_state_machine", &sfn.StateMachineArgs{
			Name:    pulumi.String("my-state-machine"),
			RoleArn: pulumi.Any(iamForSfn.Arn),
			Publish: pulumi.Bool(true),
			Type:    pulumi.String("EXPRESS"),
			Definition: pulumi.Sprintf(`{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%v",
      "End": true
    }
  }
}
`, lambda.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.StateMachine;
import com.pulumi.aws.sfn.StateMachineArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // ...
        var sfnStateMachine = new StateMachine("sfnStateMachine", StateMachineArgs.builder()
            .name("my-state-machine")
            .roleArn(iamForSfn.arn())
            .publish(true)
            .type("EXPRESS")
            .definition("""
{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%s",
      "End": true
    }
  }
}
", lambda.arn()))
            .build());

    }
}
```
```yaml
resources:
  # ...
  sfnStateMachine:
    type: aws:sfn:StateMachine
    name: sfn_state_machine
    properties:
      name: my-state-machine
      roleArn: ${iamForSfn.arn}
      publish: true
      type: EXPRESS
      definition: |
        {
          "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
          "StartAt": "HelloWorld",
          "States": {
            "HelloWorld": {
              "Type": "Task",
              "Resource": "${lambda.arn}",
              "End": true
            }
          }
        }
```
<!--End PulumiCodeChooser -->

### Logging

> *NOTE:* See the [AWS Step Functions Developer Guide](https://docs.aws.amazon.com/step-functions/latest/dg/welcome.html) for more information about enabling Step Function logging.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// ...
const sfnStateMachine = new aws.sfn.StateMachine("sfn_state_machine", {
    name: "my-state-machine",
    roleArn: iamForSfn.arn,
    definition: `{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "${lambda.arn}",
      "End": true
    }
  }
}
`,
    loggingConfiguration: {
        logDestination: `${logGroupForSfn.arn}:*`,
        includeExecutionData: true,
        level: "ERROR",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

# ...
sfn_state_machine = aws.sfn.StateMachine("sfn_state_machine",
    name="my-state-machine",
    role_arn=iam_for_sfn["arn"],
    definition=f"""{{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {{
    "HelloWorld": {{
      "Type": "Task",
      "Resource": "{lambda_["arn"]}",
      "End": true
    }}
  }}
}}
""",
    logging_configuration={
        "log_destination": f"{log_group_for_sfn['arn']}:*",
        "include_execution_data": True,
        "level": "ERROR",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // ...
    var sfnStateMachine = new Aws.Sfn.StateMachine("sfn_state_machine", new()
    {
        Name = "my-state-machine",
        RoleArn = iamForSfn.Arn,
        Definition = @$"{{
  ""Comment"": ""A Hello World example of the Amazon States Language using an AWS Lambda Function"",
  ""StartAt"": ""HelloWorld"",
  ""States"": {{
    ""HelloWorld"": {{
      ""Type"": ""Task"",
      ""Resource"": ""{lambda.Arn}"",
      ""End"": true
    }}
  }}
}}
",
        LoggingConfiguration = new Aws.Sfn.Inputs.StateMachineLoggingConfigurationArgs
        {
            LogDestination = $"{logGroupForSfn.Arn}:*",
            IncludeExecutionData = true,
            Level = "ERROR",
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// ...
		_, err := sfn.NewStateMachine(ctx, "sfn_state_machine", &sfn.StateMachineArgs{
			Name:    pulumi.String("my-state-machine"),
			RoleArn: pulumi.Any(iamForSfn.Arn),
			Definition: pulumi.Sprintf(`{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%v",
      "End": true
    }
  }
}
`, lambda.Arn),
			LoggingConfiguration: &sfn.StateMachineLoggingConfigurationArgs{
				LogDestination:       pulumi.Sprintf("%v:*", logGroupForSfn.Arn),
				IncludeExecutionData: pulumi.Bool(true),
				Level:                pulumi.String("ERROR"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.StateMachine;
import com.pulumi.aws.sfn.StateMachineArgs;
import com.pulumi.aws.sfn.inputs.StateMachineLoggingConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // ...
        var sfnStateMachine = new StateMachine("sfnStateMachine", StateMachineArgs.builder()
            .name("my-state-machine")
            .roleArn(iamForSfn.arn())
            .definition("""
{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%s",
      "End": true
    }
  }
}
", lambda.arn()))
            .loggingConfiguration(StateMachineLoggingConfigurationArgs.builder()
                .logDestination(String.format("%s:*", logGroupForSfn.arn()))
                .includeExecutionData(true)
                .level("ERROR")
                .build())
            .build());

    }
}
```
```yaml
resources:
  # ...
  sfnStateMachine:
    type: aws:sfn:StateMachine
    name: sfn_state_machine
    properties:
      name: my-state-machine
      roleArn: ${iamForSfn.arn}
      definition: |
        {
          "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
          "StartAt": "HelloWorld",
          "States": {
            "HelloWorld": {
              "Type": "Task",
              "Resource": "${lambda.arn}",
              "End": true
            }
          }
        }
      loggingConfiguration:
        logDestination: ${logGroupForSfn.arn}:*
        includeExecutionData: true
        level: ERROR
```
<!--End PulumiCodeChooser -->

### Encryption

> *NOTE:* See the section [Data at rest encyption](https://docs.aws.amazon.com/step-functions/latest/dg/encryption-at-rest.html) in the [AWS Step Functions Developer Guide](https://docs.aws.amazon.com/step-functions/latest/dg/welcome.html) for more information about enabling encryption of data using a customer-managed key for Step Functions State Machines data.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// ...
const sfnStateMachine = new aws.sfn.StateMachine("sfn_state_machine", {
    name: "my-state-machine",
    roleArn: iamForSfn.arn,
    definition: `{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "${lambda.arn}",
      "End": true
    }
  }
}
`,
    encryptionConfiguration: {
        kmsKeyId: kmsKeyForSfn.arn,
        type: "CUSTOMER_MANAGED_KMS_KEY",
        kmsDataKeyReusePeriodSeconds: 900,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

# ...
sfn_state_machine = aws.sfn.StateMachine("sfn_state_machine",
    name="my-state-machine",
    role_arn=iam_for_sfn["arn"],
    definition=f"""{{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {{
    "HelloWorld": {{
      "Type": "Task",
      "Resource": "{lambda_["arn"]}",
      "End": true
    }}
  }}
}}
""",
    encryption_configuration={
        "kms_key_id": kms_key_for_sfn["arn"],
        "type": "CUSTOMER_MANAGED_KMS_KEY",
        "kms_data_key_reuse_period_seconds": 900,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // ...
    var sfnStateMachine = new Aws.Sfn.StateMachine("sfn_state_machine", new()
    {
        Name = "my-state-machine",
        RoleArn = iamForSfn.Arn,
        Definition = @$"{{
  ""Comment"": ""A Hello World example of the Amazon States Language using an AWS Lambda Function"",
  ""StartAt"": ""HelloWorld"",
  ""States"": {{
    ""HelloWorld"": {{
      ""Type"": ""Task"",
      ""Resource"": ""{lambda.Arn}"",
      ""End"": true
    }}
  }}
}}
",
        EncryptionConfiguration = new Aws.Sfn.Inputs.StateMachineEncryptionConfigurationArgs
        {
            KmsKeyId = kmsKeyForSfn.Arn,
            Type = "CUSTOMER_MANAGED_KMS_KEY",
            KmsDataKeyReusePeriodSeconds = 900,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// ...
		_, err := sfn.NewStateMachine(ctx, "sfn_state_machine", &sfn.StateMachineArgs{
			Name:    pulumi.String("my-state-machine"),
			RoleArn: pulumi.Any(iamForSfn.Arn),
			Definition: pulumi.Sprintf(`{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%v",
      "End": true
    }
  }
}
`, lambda.Arn),
			EncryptionConfiguration: &sfn.StateMachineEncryptionConfigurationArgs{
				KmsKeyId:                     pulumi.Any(kmsKeyForSfn.Arn),
				Type:                         pulumi.String("CUSTOMER_MANAGED_KMS_KEY"),
				KmsDataKeyReusePeriodSeconds: pulumi.Int(900),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.StateMachine;
import com.pulumi.aws.sfn.StateMachineArgs;
import com.pulumi.aws.sfn.inputs.StateMachineEncryptionConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // ...
        var sfnStateMachine = new StateMachine("sfnStateMachine", StateMachineArgs.builder()
            .name("my-state-machine")
            .roleArn(iamForSfn.arn())
            .definition("""
{
  "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
  "StartAt": "HelloWorld",
  "States": {
    "HelloWorld": {
      "Type": "Task",
      "Resource": "%s",
      "End": true
    }
  }
}
", lambda.arn()))
            .encryptionConfiguration(StateMachineEncryptionConfigurationArgs.builder()
                .kmsKeyId(kmsKeyForSfn.arn())
                .type("CUSTOMER_MANAGED_KMS_KEY")
                .kmsDataKeyReusePeriodSeconds(900)
                .build())
            .build());

    }
}
```
```yaml
resources:
  # ...
  sfnStateMachine:
    type: aws:sfn:StateMachine
    name: sfn_state_machine
    properties:
      name: my-state-machine
      roleArn: ${iamForSfn.arn}
      definition: |
        {
          "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
          "StartAt": "HelloWorld",
          "States": {
            "HelloWorld": {
              "Type": "Task",
              "Resource": "${lambda.arn}",
              "End": true
            }
          }
        }
      encryptionConfiguration:
        kmsKeyId: ${kmsKeyForSfn.arn}
        type: CUSTOMER_MANAGED_KMS_KEY
        kmsDataKeyReusePeriodSeconds: 900
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import State Machines using the `arn`. For example:

```sh
$ pulumi import aws:sfn/stateMachine:StateMachine foo arn:aws:states:eu-west-1:123456789098:stateMachine:bar
```
�

definition" �The [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html) definition of the state machine.
�
encryptionConfiguration�B:}
{
sfn#StateMachineEncryptionConfigurationOaws:sfn/StateMachineEncryptionConfiguration:StateMachineEncryptionConfiguration�Defines what encryption configuration is used to encrypt data in the State Machine. For more information see [TBD] in the AWS Step Functions User Guide.
�
loggingConfigurationxBv:t
r
sfn StateMachineLoggingConfigurationIaws:sfn/StateMachineLoggingConfiguration:StateMachineLoggingConfiguration�Defines what execution history events are logged and where they are logged. The `logging_configuration` parameter is valid when `type` is set to `STANDARD` or `EXPRESS`. Defaults to `OFF`. For more information see [Logging Express Workflows](https://docs.aws.amazon.com/step-functions/latest/dg/cw-logs.html), [Log Levels](https://docs.aws.amazon.com/step-functions/latest/dg/cloudwatch-log-level.html) and [Logging Configuration](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html) in the AWS Step Functions User Guide.
�
nameB" �The name of the state machine. The name should only contain `0`-`9`, `A`-`Z`, `a`-`z`, `-` and `_`. If omitted, the provider will assign a random, unique name.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
h
publishB
 WSet to true to publish a version of the state machine during creation. Default: false.
]
roleArn" NThe Amazon Resource Name (ARN) of the IAM role to use for this state machine.
�
tagsB2" �Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
tracingConfigurationxBv:t
r
sfn StateMachineTracingConfigurationIaws:sfn/StateMachineTracingConfiguration:StateMachineTracingConfiguration.Selects whether AWS X-Ray tracing is enabled.
�
typeB" �Determines whether a Standard or Express state machine is created. The default is `STANDARD`. You cannot update the type of a state machine once it has been created. Valid values: `STANDARD`, `EXPRESS`.
")
arn" The ARN of the state machine.
"<
creationDate" (The date the state machine was created.
"�

definition" �The [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html) definition of the state machine.
"
description" "�
encryptionConfiguration:}
{
sfn#StateMachineEncryptionConfigurationOaws:sfn/StateMachineEncryptionConfiguration:StateMachineEncryptionConfiguration�Defines what encryption configuration is used to encrypt data in the State Machine. For more information see [TBD] in the AWS Step Functions User Guide.
"�
loggingConfigurationv:t
r
sfn StateMachineLoggingConfigurationIaws:sfn/StateMachineLoggingConfiguration:StateMachineLoggingConfiguration�Defines what execution history events are logged and where they are logged. The `logging_configuration` parameter is valid when `type` is set to `STANDARD` or `EXPRESS`. Defaults to `OFF`. For more information see [Logging Express Workflows](https://docs.aws.amazon.com/step-functions/latest/dg/cw-logs.html), [Log Levels](https://docs.aws.amazon.com/step-functions/latest/dg/cloudwatch-log-level.html) and [Logging Configuration](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html) in the AWS Step Functions User Guide.
"�
name" �The name of the state machine. The name should only contain `0`-`9`, `A`-`Z`, `a`-`z`, `-` and `_`. If omitted, the provider will assign a random, unique name.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"h
publishB
 WSet to true to publish a version of the state machine during creation. Default: false.
"

revisionId" "]
roleArn" NThe Amazon Resource Name (ARN) of the IAM role to use for this state machine.
"D
stateMachineVersionArn" &The ARN of the state machine version.
"V
status" HThe current status of the state machine. Either `ACTIVE` or `DELETING`.
"�
tagsB2" �Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
tracingConfigurationv:t
r
sfn StateMachineTracingConfigurationIaws:sfn/StateMachineTracingConfiguration:StateMachineTracingConfiguration.Selects whether AWS X-Ray tracing is enabled.
"�
typeB" �Determines whether a Standard or Express state machine is created. The default is `STANDARD`. You cannot update the type of a state machine once it has been created. Valid values: `STANDARD`, `EXPRESS`.
"
versionDescription" *�4
{
shield!ApplicationLayerAutomaticResponseNaws:shield/applicationLayerAutomaticResponse:ApplicationLayerAutomaticResponse�.Resource for managing an AWS Shield Application Layer Automatic Response for automatic DDoS mitigation.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
const currentGetCallerIdentity = aws.getCallerIdentity({});
const currentGetPartition = aws.getPartition({});
const config = new pulumi.Config();
// The Cloudfront Distribution on which to enable the Application Layer Automatic Response.
const distributionId = config.require("distributionId");
const example = new aws.shield.ApplicationLayerAutomaticResponse("example", {
    resourceArn: Promise.all([currentGetPartition, currentGetCallerIdentity]).then(([currentGetPartition, currentGetCallerIdentity]) => `arn:${currentGetPartition.partition}:cloudfront:${currentGetCallerIdentity.accountId}:distribution/${distributionId}`),
    action: "COUNT",
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
current_get_caller_identity = aws.get_caller_identity()
current_get_partition = aws.get_partition()
config = pulumi.Config()
# The Cloudfront Distribution on which to enable the Application Layer Automatic Response.
distribution_id = config.require("distributionId")
example = aws.shield.ApplicationLayerAutomaticResponse("example",
    resource_arn=f"arn:{current_get_partition.partition}:cloudfront:{current_get_caller_identity.account_id}:distribution/{distribution_id}",
    action="COUNT")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

    var currentGetCallerIdentity = Aws.GetCallerIdentity.Invoke();

    var currentGetPartition = Aws.GetPartition.Invoke();

    var config = new Config();
    // The Cloudfront Distribution on which to enable the Application Layer Automatic Response.
    var distributionId = config.Require("distributionId");
    var example = new Aws.Shield.ApplicationLayerAutomaticResponse("example", new()
    {
        ResourceArn = Output.Tuple(currentGetPartition, currentGetCallerIdentity).Apply(values =>
        {
            var currentGetPartition = values.Item1;
            var currentGetCallerIdentity = values.Item2;
            return $"arn:{currentGetPartition.Apply(getPartitionResult => getPartitionResult.Partition)}:cloudfront:{currentGetCallerIdentity.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:distribution/{distributionId}";
        }),
        Action = "COUNT",
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		currentGetCallerIdentity, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		currentGetPartition, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
		if err != nil {
			return err
		}
		cfg := config.New(ctx, "")
		// The Cloudfront Distribution on which to enable the Application Layer Automatic Response.
		distributionId := cfg.Require("distributionId")
		_, err = shield.NewApplicationLayerAutomaticResponse(ctx, "example", &shield.ApplicationLayerAutomaticResponseArgs{
			ResourceArn: pulumi.Sprintf("arn:%v:cloudfront:%v:distribution/%v", currentGetPartition.Partition, currentGetCallerIdentity.AccountId, distributionId),
			Action:      pulumi.String("COUNT"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.shield.ApplicationLayerAutomaticResponse;
import com.pulumi.aws.shield.ApplicationLayerAutomaticResponseArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var config = ctx.config();
        final var current = AwsFunctions.getRegion();

        final var currentGetCallerIdentity = AwsFunctions.getCallerIdentity();

        final var currentGetPartition = AwsFunctions.getPartition();

        final var distributionId = config.get("distributionId");
        var example = new ApplicationLayerAutomaticResponse("example", ApplicationLayerAutomaticResponseArgs.builder()
            .resourceArn(String.format("arn:%s:cloudfront:%s:distribution/%s", currentGetPartition.applyValue(getPartitionResult -> getPartitionResult.partition()),currentGetCallerIdentity.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()),distributionId))
            .action("COUNT")
            .build());

    }
}
```
```yaml
configuration:
  distributionId:
    type: string
resources:
  example:
    type: aws:shield:ApplicationLayerAutomaticResponse
    properties:
      resourceArn: arn:${currentGetPartition.partition}:cloudfront:${currentGetCallerIdentity.accountId}:distribution/${distributionId}
      action: COUNT
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
  currentGetCallerIdentity:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
  currentGetPartition:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
```
<!--End PulumiCodeChooser -->
(
action" One of `COUNT` or `BLOCK`
i
resourceArn" VARN of the resource to protect (Cloudfront Distributions and ALBs only at this time).
�
timeouts�B�:�
�
shield)ApplicationLayerAutomaticResponseTimeouts^aws:shield/ApplicationLayerAutomaticResponseTimeouts:ApplicationLayerAutomaticResponseTimeouts"(
action" One of `COUNT` or `BLOCK`
"i
resourceArn" VARN of the resource to protect (Cloudfront Distributions and ALBs only at this time).
"�
timeouts�B�:�
�
shield)ApplicationLayerAutomaticResponseTimeouts^aws:shield/ApplicationLayerAutomaticResponseTimeouts:ApplicationLayerAutomaticResponseTimeouts*�'
o
shieldDrtAccessLogBucketAssociationFaws:shield/drtAccessLogBucketAssociation:DrtAccessLogBucketAssociation�!Resource for managing an AWS Shield DRT Access Log Bucket Association.
Up to 10 log buckets can be associated for DRT Access sharing with the Shield Response Team (SRT).

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.shield.DrtAccessRoleArnAssociation("test", {roleArn: `arn:aws:iam:${current.name}:${currentAwsCallerIdentity.accountId}:${shieldDrtAccessRoleName}`});
const testDrtAccessLogBucketAssociation = new aws.shield.DrtAccessLogBucketAssociation("test", {
    logBucket: shieldDrtAccessLogBucket,
    roleArnAssociationId: test.id,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.shield.DrtAccessRoleArnAssociation("test", role_arn=f"arn:aws:iam:{current['name']}:{current_aws_caller_identity['accountId']}:{shield_drt_access_role_name}")
test_drt_access_log_bucket_association = aws.shield.DrtAccessLogBucketAssociation("test",
    log_bucket=shield_drt_access_log_bucket,
    role_arn_association_id=test.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Shield.DrtAccessRoleArnAssociation("test", new()
    {
        RoleArn = $"arn:aws:iam:{current.Name}:{currentAwsCallerIdentity.AccountId}:{shieldDrtAccessRoleName}",
    });

    var testDrtAccessLogBucketAssociation = new Aws.Shield.DrtAccessLogBucketAssociation("test", new()
    {
        LogBucket = shieldDrtAccessLogBucket,
        RoleArnAssociationId = test.Id,
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		test, err := shield.NewDrtAccessRoleArnAssociation(ctx, "test", &shield.DrtAccessRoleArnAssociationArgs{
			RoleArn: pulumi.Sprintf("arn:aws:iam:%v:%v:%v", current.Name, currentAwsCallerIdentity.AccountId, shieldDrtAccessRoleName),
		})
		if err != nil {
			return err
		}
		_, err = shield.NewDrtAccessLogBucketAssociation(ctx, "test", &shield.DrtAccessLogBucketAssociationArgs{
			LogBucket:            pulumi.Any(shieldDrtAccessLogBucket),
			RoleArnAssociationId: test.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.shield.DrtAccessRoleArnAssociation;
import com.pulumi.aws.shield.DrtAccessRoleArnAssociationArgs;
import com.pulumi.aws.shield.DrtAccessLogBucketAssociation;
import com.pulumi.aws.shield.DrtAccessLogBucketAssociationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new DrtAccessRoleArnAssociation("test", DrtAccessRoleArnAssociationArgs.builder()
            .roleArn(String.format("arn:aws:iam:%s:%s:%s", current.name(),currentAwsCallerIdentity.accountId(),shieldDrtAccessRoleName))
            .build());

        var testDrtAccessLogBucketAssociation = new DrtAccessLogBucketAssociation("testDrtAccessLogBucketAssociation", DrtAccessLogBucketAssociationArgs.builder()
            .logBucket(shieldDrtAccessLogBucket)
            .roleArnAssociationId(test.id())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:shield:DrtAccessRoleArnAssociation
    properties:
      roleArn: arn:aws:iam:${current.name}:${currentAwsCallerIdentity.accountId}:${shieldDrtAccessRoleName}
  testDrtAccessLogBucketAssociation:
    type: aws:shield:DrtAccessLogBucketAssociation
    name: test
    properties:
      logBucket: ${shieldDrtAccessLogBucket}
      roleArnAssociationId: ${test.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Shield DRT access log bucket associations using the `log_bucket`. For example:

```sh
$ pulumi import aws:shield/drtAccessLogBucketAssociation:DrtAccessLogBucketAssociation example example-bucket
```
U
	logBucket" DThe Amazon S3 bucket that contains the logs that you want to share.
d
roleArnAssociationId" HThe ID of the Role Arn association used for allowing Shield DRT Access.
�
timeouts�B�:�
�
shield%DrtAccessLogBucketAssociationTimeoutsVaws:shield/DrtAccessLogBucketAssociationTimeouts:DrtAccessLogBucketAssociationTimeouts"U
	logBucket" DThe Amazon S3 bucket that contains the logs that you want to share.
"d
roleArnAssociationId" HThe ID of the Role Arn association used for allowing Shield DRT Access.
"�
timeouts�B�:�
�
shield%DrtAccessLogBucketAssociationTimeoutsVaws:shield/DrtAccessLogBucketAssociationTimeouts:DrtAccessLogBucketAssociationTimeouts*�?
i
shieldDrtAccessRoleArnAssociationBaws:shield/drtAccessRoleArnAssociation:DrtAccessRoleArnAssociation�8Authorizes the Shield Response Team (SRT) using the specified role, to access your AWS account to assist with DDoS attack mitigation during potential attacks.
For more information see [Configure AWS SRT Support](https://docs.aws.amazon.com/waf/latest/developerguide/authorize-srt.html)

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleRole = new aws.iam.Role("example", {
    name: "example-role",
    assumeRolePolicy: JSON.stringify({
        Version: "2012-10-17",
        Statement: [{
            Sid: "",
            Effect: "Allow",
            Principal: {
                Service: "drt.shield.amazonaws.com",
            },
            Action: "sts:AssumeRole",
        }],
    }),
});
const example = new aws.shield.DrtAccessRoleArnAssociation("example", {roleArn: exampleRole.arn});
const exampleRolePolicyAttachment = new aws.iam.RolePolicyAttachment("example", {
    role: exampleRole.name,
    policyArn: "arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy",
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example_role = aws.iam.Role("example",
    name="example-role",
    assume_role_policy=json.dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Sid": "",
            "Effect": "Allow",
            "Principal": {
                "Service": "drt.shield.amazonaws.com",
            },
            "Action": "sts:AssumeRole",
        }],
    }))
example = aws.shield.DrtAccessRoleArnAssociation("example", role_arn=example_role.arn)
example_role_policy_attachment = aws.iam.RolePolicyAttachment("example",
    role=example_role.name,
    policy_arn="arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleRole = new Aws.Iam.Role("example", new()
    {
        Name = "example-role",
        AssumeRolePolicy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Sid"] = "",
                    ["Effect"] = "Allow",
                    ["Principal"] = new Dictionary<string, object?>
                    {
                        ["Service"] = "drt.shield.amazonaws.com",
                    },
                    ["Action"] = "sts:AssumeRole",
                },
            },
        }),
    });

    var example = new Aws.Shield.DrtAccessRoleArnAssociation("example", new()
    {
        RoleArn = exampleRole.Arn,
    });

    var exampleRolePolicyAttachment = new Aws.Iam.RolePolicyAttachment("example", new()
    {
        Role = exampleRole.Name,
        PolicyArn = "arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy",
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"Sid":    "",
					"Effect": "Allow",
					"Principal": map[string]interface{}{
						"Service": "drt.shield.amazonaws.com",
					},
					"Action": "sts:AssumeRole",
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		exampleRole, err := iam.NewRole(ctx, "example", &iam.RoleArgs{
			Name:             pulumi.String("example-role"),
			AssumeRolePolicy: pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		_, err = shield.NewDrtAccessRoleArnAssociation(ctx, "example", &shield.DrtAccessRoleArnAssociationArgs{
			RoleArn: exampleRole.Arn,
		})
		if err != nil {
			return err
		}
		_, err = iam.NewRolePolicyAttachment(ctx, "example", &iam.RolePolicyAttachmentArgs{
			Role:      exampleRole.Name,
			PolicyArn: pulumi.String("arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.shield.DrtAccessRoleArnAssociation;
import com.pulumi.aws.shield.DrtAccessRoleArnAssociationArgs;
import com.pulumi.aws.iam.RolePolicyAttachment;
import com.pulumi.aws.iam.RolePolicyAttachmentArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("example-role")
            .assumeRolePolicy(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Sid", ""),
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Principal", jsonObject(
                            jsonProperty("Service", "drt.shield.amazonaws.com")
                        )),
                        jsonProperty("Action", "sts:AssumeRole")
                    )))
                )))
            .build());

        var example = new DrtAccessRoleArnAssociation("example", DrtAccessRoleArnAssociationArgs.builder()
            .roleArn(exampleRole.arn())
            .build());

        var exampleRolePolicyAttachment = new RolePolicyAttachment("exampleRolePolicyAttachment", RolePolicyAttachmentArgs.builder()
            .role(exampleRole.name())
            .policyArn("arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:shield:DrtAccessRoleArnAssociation
    properties:
      roleArn: ${exampleRole.arn}
  exampleRole:
    type: aws:iam:Role
    name: example
    properties:
      name: example-role
      assumeRolePolicy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Sid: ""
              Effect: Allow
              Principal:
                Service: drt.shield.amazonaws.com
              Action: sts:AssumeRole
  exampleRolePolicyAttachment:
    type: aws:iam:RolePolicyAttachment
    name: example
    properties:
      role: ${exampleRole.name}
      policyArn: arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Shield DRT access role ARN association using the AWS account ID. For example:

```sh
$ pulumi import aws:shield/drtAccessRoleArnAssociation:DrtAccessRoleArnAssociation example 123456789012
```
�
roleArn" �The Amazon Resource Name (ARN) of the role the SRT will use to access your AWS account. Prior to making the AssociateDRTRole request, you must attach the `AWSShieldDRTAccessPolicy` managed policy to this role.
�
timeouts�B�:�
�
shield#DrtAccessRoleArnAssociationTimeoutsRaws:shield/DrtAccessRoleArnAssociationTimeouts:DrtAccessRoleArnAssociationTimeouts"�
roleArn" �The Amazon Resource Name (ARN) of the role the SRT will use to access your AWS account. Prior to making the AssociateDRTRole request, you must attach the `AWSShieldDRTAccessPolicy` managed policy to this role.
"�
timeouts�B�:�
�
shield#DrtAccessRoleArnAssociationTimeoutsRaws:shield/DrtAccessRoleArnAssociationTimeouts:DrtAccessRoleArnAssociationTimeouts*�i
Q
shieldProactiveEngagement2aws:shield/proactiveEngagement:ProactiveEngagement�bResource for managing a AWS Shield Proactive Engagement.
Proactive engagement authorizes the Shield Response Team (SRT) to use email and phone to notify contacts about escalations to the SRT and to initiate proactive customer support.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleRole = new aws.iam.Role("example", {
    name: "example-role",
    assumeRolePolicy: JSON.stringify({
        Version: "2012-10-17",
        Statement: [{
            Sid: "",
            Effect: "Allow",
            Principal: {
                Service: "drt.shield.amazonaws.com",
            },
            Action: "sts:AssumeRole",
        }],
    }),
});
const exampleDrtAccessRoleArnAssociation = new aws.shield.DrtAccessRoleArnAssociation("example", {roleArn: exampleRole.arn});
const example = new aws.shield.ProactiveEngagement("example", {
    enabled: true,
    emergencyContacts: [
        {
            contactNotes: "Notes",
            emailAddress: "contact1@example.com",
            phoneNumber: "+12358132134",
        },
        {
            contactNotes: "Notes 2",
            emailAddress: "contact2@example.com",
            phoneNumber: "+12358132134",
        },
    ],
}, {
    dependsOn: [exampleDrtAccessRoleArnAssociation],
});
const exampleRolePolicyAttachment = new aws.iam.RolePolicyAttachment("example", {
    role: exampleRole.name,
    policyArn: "arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy",
});
const exampleProtectionGroup = new aws.shield.ProtectionGroup("example", {
    protectionGroupId: "example",
    aggregation: "MAX",
    pattern: "ALL",
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example_role = aws.iam.Role("example",
    name="example-role",
    assume_role_policy=json.dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Sid": "",
            "Effect": "Allow",
            "Principal": {
                "Service": "drt.shield.amazonaws.com",
            },
            "Action": "sts:AssumeRole",
        }],
    }))
example_drt_access_role_arn_association = aws.shield.DrtAccessRoleArnAssociation("example", role_arn=example_role.arn)
example = aws.shield.ProactiveEngagement("example",
    enabled=True,
    emergency_contacts=[
        {
            "contact_notes": "Notes",
            "email_address": "contact1@example.com",
            "phone_number": "+12358132134",
        },
        {
            "contact_notes": "Notes 2",
            "email_address": "contact2@example.com",
            "phone_number": "+12358132134",
        },
    ],
    opts = pulumi.ResourceOptions(depends_on=[example_drt_access_role_arn_association]))
example_role_policy_attachment = aws.iam.RolePolicyAttachment("example",
    role=example_role.name,
    policy_arn="arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy")
example_protection_group = aws.shield.ProtectionGroup("example",
    protection_group_id="example",
    aggregation="MAX",
    pattern="ALL")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleRole = new Aws.Iam.Role("example", new()
    {
        Name = "example-role",
        AssumeRolePolicy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Sid"] = "",
                    ["Effect"] = "Allow",
                    ["Principal"] = new Dictionary<string, object?>
                    {
                        ["Service"] = "drt.shield.amazonaws.com",
                    },
                    ["Action"] = "sts:AssumeRole",
                },
            },
        }),
    });

    var exampleDrtAccessRoleArnAssociation = new Aws.Shield.DrtAccessRoleArnAssociation("example", new()
    {
        RoleArn = exampleRole.Arn,
    });

    var example = new Aws.Shield.ProactiveEngagement("example", new()
    {
        Enabled = true,
        EmergencyContacts = new[]
        {
            new Aws.Shield.Inputs.ProactiveEngagementEmergencyContactArgs
            {
                ContactNotes = "Notes",
                EmailAddress = "contact1@example.com",
                PhoneNumber = "+12358132134",
            },
            new Aws.Shield.Inputs.ProactiveEngagementEmergencyContactArgs
            {
                ContactNotes = "Notes 2",
                EmailAddress = "contact2@example.com",
                PhoneNumber = "+12358132134",
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleDrtAccessRoleArnAssociation,
        },
    });

    var exampleRolePolicyAttachment = new Aws.Iam.RolePolicyAttachment("example", new()
    {
        Role = exampleRole.Name,
        PolicyArn = "arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy",
    });

    var exampleProtectionGroup = new Aws.Shield.ProtectionGroup("example", new()
    {
        ProtectionGroupId = "example",
        Aggregation = "MAX",
        Pattern = "ALL",
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"Sid":    "",
					"Effect": "Allow",
					"Principal": map[string]interface{}{
						"Service": "drt.shield.amazonaws.com",
					},
					"Action": "sts:AssumeRole",
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		exampleRole, err := iam.NewRole(ctx, "example", &iam.RoleArgs{
			Name:             pulumi.String("example-role"),
			AssumeRolePolicy: pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		exampleDrtAccessRoleArnAssociation, err := shield.NewDrtAccessRoleArnAssociation(ctx, "example", &shield.DrtAccessRoleArnAssociationArgs{
			RoleArn: exampleRole.Arn,
		})
		if err != nil {
			return err
		}
		_, err = shield.NewProactiveEngagement(ctx, "example", &shield.ProactiveEngagementArgs{
			Enabled: pulumi.Bool(true),
			EmergencyContacts: shield.ProactiveEngagementEmergencyContactArray{
				&shield.ProactiveEngagementEmergencyContactArgs{
					ContactNotes: pulumi.String("Notes"),
					EmailAddress: pulumi.String("contact1@example.com"),
					PhoneNumber:  pulumi.String("+12358132134"),
				},
				&shield.ProactiveEngagementEmergencyContactArgs{
					ContactNotes: pulumi.String("Notes 2"),
					EmailAddress: pulumi.String("contact2@example.com"),
					PhoneNumber:  pulumi.String("+12358132134"),
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleDrtAccessRoleArnAssociation,
		}))
		if err != nil {
			return err
		}
		_, err = iam.NewRolePolicyAttachment(ctx, "example", &iam.RolePolicyAttachmentArgs{
			Role:      exampleRole.Name,
			PolicyArn: pulumi.String("arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy"),
		})
		if err != nil {
			return err
		}
		_, err = shield.NewProtectionGroup(ctx, "example", &shield.ProtectionGroupArgs{
			ProtectionGroupId: pulumi.String("example"),
			Aggregation:       pulumi.String("MAX"),
			Pattern:           pulumi.String("ALL"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.shield.DrtAccessRoleArnAssociation;
import com.pulumi.aws.shield.DrtAccessRoleArnAssociationArgs;
import com.pulumi.aws.shield.ProactiveEngagement;
import com.pulumi.aws.shield.ProactiveEngagementArgs;
import com.pulumi.aws.shield.inputs.ProactiveEngagementEmergencyContactArgs;
import com.pulumi.aws.iam.RolePolicyAttachment;
import com.pulumi.aws.iam.RolePolicyAttachmentArgs;
import com.pulumi.aws.shield.ProtectionGroup;
import com.pulumi.aws.shield.ProtectionGroupArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("example-role")
            .assumeRolePolicy(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Sid", ""),
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Principal", jsonObject(
                            jsonProperty("Service", "drt.shield.amazonaws.com")
                        )),
                        jsonProperty("Action", "sts:AssumeRole")
                    )))
                )))
            .build());

        var exampleDrtAccessRoleArnAssociation = new DrtAccessRoleArnAssociation("exampleDrtAccessRoleArnAssociation", DrtAccessRoleArnAssociationArgs.builder()
            .roleArn(exampleRole.arn())
            .build());

        var example = new ProactiveEngagement("example", ProactiveEngagementArgs.builder()
            .enabled(true)
            .emergencyContacts(            
                ProactiveEngagementEmergencyContactArgs.builder()
                    .contactNotes("Notes")
                    .emailAddress("contact1@example.com")
                    .phoneNumber("+12358132134")
                    .build(),
                ProactiveEngagementEmergencyContactArgs.builder()
                    .contactNotes("Notes 2")
                    .emailAddress("contact2@example.com")
                    .phoneNumber("+12358132134")
                    .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleDrtAccessRoleArnAssociation)
                .build());

        var exampleRolePolicyAttachment = new RolePolicyAttachment("exampleRolePolicyAttachment", RolePolicyAttachmentArgs.builder()
            .role(exampleRole.name())
            .policyArn("arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy")
            .build());

        var exampleProtectionGroup = new ProtectionGroup("exampleProtectionGroup", ProtectionGroupArgs.builder()
            .protectionGroupId("example")
            .aggregation("MAX")
            .pattern("ALL")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:shield:ProactiveEngagement
    properties:
      enabled: true
      emergencyContacts:
        - contactNotes: Notes
          emailAddress: contact1@example.com
          phoneNumber: '+12358132134'
        - contactNotes: Notes 2
          emailAddress: contact2@example.com
          phoneNumber: '+12358132134'
    options:
      dependsOn:
        - ${exampleDrtAccessRoleArnAssociation}
  exampleRole:
    type: aws:iam:Role
    name: example
    properties:
      name: example-role
      assumeRolePolicy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Sid: ""
              Effect: Allow
              Principal:
                Service: drt.shield.amazonaws.com
              Action: sts:AssumeRole
  exampleRolePolicyAttachment:
    type: aws:iam:RolePolicyAttachment
    name: example
    properties:
      role: ${exampleRole.name}
      policyArn: arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy
  exampleDrtAccessRoleArnAssociation:
    type: aws:shield:DrtAccessRoleArnAssociation
    name: example
    properties:
      roleArn: ${exampleRole.arn}
  exampleProtectionGroup:
    type: aws:shield:ProtectionGroup
    name: example
    properties:
      protectionGroupId: example
      aggregation: MAX
      pattern: ALL
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Shield proactive engagement using the AWS account ID. For example:

```sh
$ pulumi import aws:shield/proactiveEngagement:ProactiveEngagement example 123456789012
```
�
emergencyContacts�B�*�:�
�
shield#ProactiveEngagementEmergencyContactRaws:shield/ProactiveEngagementEmergencyContact:ProactiveEngagementEmergencyContact�One or more emergency contacts. You must provide at least one phone number in the emergency contact list. See `emergency_contacts`.
Z
enabled
 KBoolean value indicating if Proactive Engagement should be enabled or not.
"�
emergencyContacts�B�*�:�
�
shield#ProactiveEngagementEmergencyContactRaws:shield/ProactiveEngagementEmergencyContact:ProactiveEngagementEmergencyContact�One or more emergency contacts. You must provide at least one phone number in the emergency contact list. See `emergency_contacts`.
"Z
enabled
 KBoolean value indicating if Proactive Engagement should be enabled or not.
*�7
6
shield
Protection aws:shield/protection:Protection�0Enables AWS Shield Advanced for a specific AWS resource.
The resource can be an Amazon CloudFront distribution, Elastic Load Balancing load balancer, AWS Global Accelerator accelerator, Elastic IP Address, or an Amazon Route 53 hosted zone.

## Example Usage

### Create protection

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const available = aws.getAvailabilityZones({});
const current = aws.getRegion({});
const currentGetCallerIdentity = aws.getCallerIdentity({});
const example = new aws.ec2.Eip("example", {domain: "vpc"});
const exampleProtection = new aws.shield.Protection("example", {
    name: "example",
    resourceArn: pulumi.all([current, currentGetCallerIdentity, example.id]).apply(([current, currentGetCallerIdentity, id]) => `arn:aws:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${id}`),
    tags: {
        Environment: "Dev",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

available = aws.get_availability_zones()
current = aws.get_region()
current_get_caller_identity = aws.get_caller_identity()
example = aws.ec2.Eip("example", domain="vpc")
example_protection = aws.shield.Protection("example",
    name="example",
    resource_arn=example.id.apply(lambda id: f"arn:aws:ec2:{current.name}:{current_get_caller_identity.account_id}:eip-allocation/{id}"),
    tags={
        "Environment": "Dev",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var available = Aws.GetAvailabilityZones.Invoke();

    var current = Aws.GetRegion.Invoke();

    var currentGetCallerIdentity = Aws.GetCallerIdentity.Invoke();

    var example = new Aws.Ec2.Eip("example", new()
    {
        Domain = "vpc",
    });

    var exampleProtection = new Aws.Shield.Protection("example", new()
    {
        Name = "example",
        ResourceArn = Output.Tuple(current, currentGetCallerIdentity, example.Id).Apply(values =>
        {
            var current = values.Item1;
            var currentGetCallerIdentity = values.Item2;
            var id = values.Item3;
            return $"arn:aws:ec2:{current.Apply(getRegionResult => getRegionResult.Name)}:{currentGetCallerIdentity.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:eip-allocation/{id}";
        }),
        Tags = 
        {
            { "Environment", "Dev" },
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{}, nil)
		if err != nil {
			return err
		}
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		currentGetCallerIdentity, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		example, err := ec2.NewEip(ctx, "example", &ec2.EipArgs{
			Domain: pulumi.String("vpc"),
		})
		if err != nil {
			return err
		}
		_, err = shield.NewProtection(ctx, "example", &shield.ProtectionArgs{
			Name: pulumi.String("example"),
			ResourceArn: example.ID().ApplyT(func(id string) (string, error) {
				return fmt.Sprintf("arn:aws:ec2:%v:%v:eip-allocation/%v", current.Name, currentGetCallerIdentity.AccountId, id), nil
			}).(pulumi.StringOutput),
			Tags: pulumi.StringMap{
				"Environment": pulumi.String("Dev"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.ec2.Eip;
import com.pulumi.aws.ec2.EipArgs;
import com.pulumi.aws.shield.Protection;
import com.pulumi.aws.shield.ProtectionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var available = AwsFunctions.getAvailabilityZones();

        final var current = AwsFunctions.getRegion();

        final var currentGetCallerIdentity = AwsFunctions.getCallerIdentity();

        var example = new Eip("example", EipArgs.builder()
            .domain("vpc")
            .build());

        var exampleProtection = new Protection("exampleProtection", ProtectionArgs.builder()
            .name("example")
            .resourceArn(example.id().applyValue(id -> String.format("arn:aws:ec2:%s:%s:eip-allocation/%s", current.applyValue(getRegionResult -> getRegionResult.name()),currentGetCallerIdentity.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()),id)))
            .tags(Map.of("Environment", "Dev"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ec2:Eip
    properties:
      domain: vpc
  exampleProtection:
    type: aws:shield:Protection
    name: example
    properties:
      name: example
      resourceArn: arn:aws:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${example.id}
      tags:
        Environment: Dev
variables:
  available:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments: {}
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
  currentGetCallerIdentity:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Shield protection resources using specifying their ID. For example:

```sh
$ pulumi import aws:shield/protection:Protection example ff9592dc-22f3-4e88-afa1-7b29fde9669a
```
C
nameB" 5A friendly name for the Protection you are creating.
S
resourceArn" @The ARN (Amazon Resource Name) of the resource to be protected.
�
tagsB2" �Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"&
arn" The ARN of the Protection.
"A
name" 5A friendly name for the Protection you are creating.
"S
resourceArn" @The ARN (Amazon Resource Name) of the resource to be protected.
"�
tagsB2" �Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�{
E
shieldProtectionGroup*aws:shield/protectionGroup:ProtectionGroup�jCreates a grouping of protected resources so they can be handled as a collective.
This resource grouping improves the accuracy of detection and reduces false positives. For more information see
[Managing AWS Shield Advanced protection groups](https://docs.aws.amazon.com/waf/latest/developerguide/manage-protection-group.html)

## Example Usage

### Create protection group for all resources

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.shield.ProtectionGroup("example", {
    protectionGroupId: "example",
    aggregation: "MAX",
    pattern: "ALL",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.shield.ProtectionGroup("example",
    protection_group_id="example",
    aggregation="MAX",
    pattern="ALL")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Shield.ProtectionGroup("example", new()
    {
        ProtectionGroupId = "example",
        Aggregation = "MAX",
        Pattern = "ALL",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := shield.NewProtectionGroup(ctx, "example", &shield.ProtectionGroupArgs{
			ProtectionGroupId: pulumi.String("example"),
			Aggregation:       pulumi.String("MAX"),
			Pattern:           pulumi.String("ALL"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.shield.ProtectionGroup;
import com.pulumi.aws.shield.ProtectionGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ProtectionGroup("example", ProtectionGroupArgs.builder()
            .protectionGroupId("example")
            .aggregation("MAX")
            .pattern("ALL")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:shield:ProtectionGroup
    properties:
      protectionGroupId: example
      aggregation: MAX
      pattern: ALL
```
<!--End PulumiCodeChooser -->

### Create protection group for arbitrary number of resources

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
const currentGetCallerIdentity = aws.getCallerIdentity({});
const example = new aws.ec2.Eip("example", {domain: "vpc"});
const exampleProtection = new aws.shield.Protection("example", {
    name: "example",
    resourceArn: pulumi.all([current, currentGetCallerIdentity, example.id]).apply(([current, currentGetCallerIdentity, id]) => `arn:aws:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${id}`),
});
const exampleProtectionGroup = new aws.shield.ProtectionGroup("example", {
    protectionGroupId: "example",
    aggregation: "MEAN",
    pattern: "ARBITRARY",
    members: [pulumi.all([current, currentGetCallerIdentity, example.id]).apply(([current, currentGetCallerIdentity, id]) => `arn:aws:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${id}`)],
}, {
    dependsOn: [exampleProtection],
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
current_get_caller_identity = aws.get_caller_identity()
example = aws.ec2.Eip("example", domain="vpc")
example_protection = aws.shield.Protection("example",
    name="example",
    resource_arn=example.id.apply(lambda id: f"arn:aws:ec2:{current.name}:{current_get_caller_identity.account_id}:eip-allocation/{id}"))
example_protection_group = aws.shield.ProtectionGroup("example",
    protection_group_id="example",
    aggregation="MEAN",
    pattern="ARBITRARY",
    members=[example.id.apply(lambda id: f"arn:aws:ec2:{current.name}:{current_get_caller_identity.account_id}:eip-allocation/{id}")],
    opts = pulumi.ResourceOptions(depends_on=[example_protection]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

    var currentGetCallerIdentity = Aws.GetCallerIdentity.Invoke();

    var example = new Aws.Ec2.Eip("example", new()
    {
        Domain = "vpc",
    });

    var exampleProtection = new Aws.Shield.Protection("example", new()
    {
        Name = "example",
        ResourceArn = Output.Tuple(current, currentGetCallerIdentity, example.Id).Apply(values =>
        {
            var current = values.Item1;
            var currentGetCallerIdentity = values.Item2;
            var id = values.Item3;
            return $"arn:aws:ec2:{current.Apply(getRegionResult => getRegionResult.Name)}:{currentGetCallerIdentity.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:eip-allocation/{id}";
        }),
    });

    var exampleProtectionGroup = new Aws.Shield.ProtectionGroup("example", new()
    {
        ProtectionGroupId = "example",
        Aggregation = "MEAN",
        Pattern = "ARBITRARY",
        Members = new[]
        {
            Output.Tuple(current, currentGetCallerIdentity, example.Id).Apply(values =>
            {
                var current = values.Item1;
                var currentGetCallerIdentity = values.Item2;
                var id = values.Item3;
                return $"arn:aws:ec2:{current.Apply(getRegionResult => getRegionResult.Name)}:{currentGetCallerIdentity.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:eip-allocation/{id}";
            }),
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleProtection,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		currentGetCallerIdentity, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		example, err := ec2.NewEip(ctx, "example", &ec2.EipArgs{
			Domain: pulumi.String("vpc"),
		})
		if err != nil {
			return err
		}
		exampleProtection, err := shield.NewProtection(ctx, "example", &shield.ProtectionArgs{
			Name: pulumi.String("example"),
			ResourceArn: example.ID().ApplyT(func(id string) (string, error) {
				return fmt.Sprintf("arn:aws:ec2:%v:%v:eip-allocation/%v", current.Name, currentGetCallerIdentity.AccountId, id), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		_, err = shield.NewProtectionGroup(ctx, "example", &shield.ProtectionGroupArgs{
			ProtectionGroupId: pulumi.String("example"),
			Aggregation:       pulumi.String("MEAN"),
			Pattern:           pulumi.String("ARBITRARY"),
			Members: pulumi.StringArray{
				example.ID().ApplyT(func(id string) (string, error) {
					return fmt.Sprintf("arn:aws:ec2:%v:%v:eip-allocation/%v", current.Name, currentGetCallerIdentity.AccountId, id), nil
				}).(pulumi.StringOutput),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleProtection,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.ec2.Eip;
import com.pulumi.aws.ec2.EipArgs;
import com.pulumi.aws.shield.Protection;
import com.pulumi.aws.shield.ProtectionArgs;
import com.pulumi.aws.shield.ProtectionGroup;
import com.pulumi.aws.shield.ProtectionGroupArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

        final var currentGetCallerIdentity = AwsFunctions.getCallerIdentity();

        var example = new Eip("example", EipArgs.builder()
            .domain("vpc")
            .build());

        var exampleProtection = new Protection("exampleProtection", ProtectionArgs.builder()
            .name("example")
            .resourceArn(example.id().applyValue(id -> String.format("arn:aws:ec2:%s:%s:eip-allocation/%s", current.applyValue(getRegionResult -> getRegionResult.name()),currentGetCallerIdentity.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()),id)))
            .build());

        var exampleProtectionGroup = new ProtectionGroup("exampleProtectionGroup", ProtectionGroupArgs.builder()
            .protectionGroupId("example")
            .aggregation("MEAN")
            .pattern("ARBITRARY")
            .members(example.id().applyValue(id -> String.format("arn:aws:ec2:%s:%s:eip-allocation/%s", current.applyValue(getRegionResult -> getRegionResult.name()),currentGetCallerIdentity.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()),id)))
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleProtection)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ec2:Eip
    properties:
      domain: vpc
  exampleProtection:
    type: aws:shield:Protection
    name: example
    properties:
      name: example
      resourceArn: arn:aws:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${example.id}
  exampleProtectionGroup:
    type: aws:shield:ProtectionGroup
    name: example
    properties:
      protectionGroupId: example
      aggregation: MEAN
      pattern: ARBITRARY
      members:
        - arn:aws:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${example.id}
    options:
      dependsOn:
        - ${exampleProtection}
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
  currentGetCallerIdentity:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
```
<!--End PulumiCodeChooser -->

### Create protection group for a type of resource

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.shield.ProtectionGroup("example", {
    protectionGroupId: "example",
    aggregation: "SUM",
    pattern: "BY_RESOURCE_TYPE",
    resourceType: "ELASTIC_IP_ALLOCATION",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.shield.ProtectionGroup("example",
    protection_group_id="example",
    aggregation="SUM",
    pattern="BY_RESOURCE_TYPE",
    resource_type="ELASTIC_IP_ALLOCATION")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Shield.ProtectionGroup("example", new()
    {
        ProtectionGroupId = "example",
        Aggregation = "SUM",
        Pattern = "BY_RESOURCE_TYPE",
        ResourceType = "ELASTIC_IP_ALLOCATION",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := shield.NewProtectionGroup(ctx, "example", &shield.ProtectionGroupArgs{
			ProtectionGroupId: pulumi.String("example"),
			Aggregation:       pulumi.String("SUM"),
			Pattern:           pulumi.String("BY_RESOURCE_TYPE"),
			ResourceType:      pulumi.String("ELASTIC_IP_ALLOCATION"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.shield.ProtectionGroup;
import com.pulumi.aws.shield.ProtectionGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ProtectionGroup("example", ProtectionGroupArgs.builder()
            .protectionGroupId("example")
            .aggregation("SUM")
            .pattern("BY_RESOURCE_TYPE")
            .resourceType("ELASTIC_IP_ALLOCATION")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:shield:ProtectionGroup
    properties:
      protectionGroupId: example
      aggregation: SUM
      pattern: BY_RESOURCE_TYPE
      resourceType: ELASTIC_IP_ALLOCATION
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Shield protection group resources using their protection group id. For example:

```sh
$ pulumi import aws:shield/protectionGroup:ProtectionGroup example example
```
�
aggregation" mDefines how AWS Shield combines resource data for the group in order to detect, mitigate, and report events.
�
membersB*" �The Amazon Resource Names (ARNs) of the resources to include in the protection group. You must set this when you set `pattern` to ARBITRARY and you must not set it for any other `pattern` setting.
a
pattern" RThe criteria to use to choose the protected resources for inclusion in the group.
;
protectionGroupId" "The name of the protection group.
�
resourceTypeB" �The resource type to include in the protection group. You must set this when you set `pattern` to BY_RESOURCE_TYPE and you must not set it for any other `pattern` setting.
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
aggregation" mDefines how AWS Shield combines resource data for the group in order to detect, mitigate, and report events.
"�
membersB*" �The Amazon Resource Names (ARNs) of the resources to include in the protection group. You must set this when you set `pattern` to ARBITRARY and you must not set it for any other `pattern` setting.
"a
pattern" RThe criteria to use to choose the protected resources for inclusion in the group.
"R
protectionGroupArn" 8The ARN (Amazon Resource Name) of the protection group.
";
protectionGroupId" "The name of the protection group.
"�
resourceTypeB" �The resource type to include in the protection group. You must set this when you set `pattern` to BY_RESOURCE_TYPE and you must not set it for any other `pattern` setting.
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�X
x
shield ProtectionHealthCheckAssociationLaws:shield/protectionHealthCheckAssociation:ProtectionHealthCheckAssociation�TCreates an association between a Route53 Health Check and a Shield Advanced protected resource.
This association uses the health of your applications to improve responsiveness and accuracy in attack detection and mitigation.

Blog post: [AWS Shield Advanced now supports Health Based Detection](https://aws.amazon.com/about-aws/whats-new/2020/02/aws-shield-advanced-now-supports-health-based-detection/)

## Example Usage

### Create an association between a protected EIP and a Route53 Health Check

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
const currentGetCallerIdentity = aws.getCallerIdentity({});
const currentGetPartition = aws.getPartition({});
const example = new aws.ec2.Eip("example", {
    domain: "vpc",
    tags: {
        Name: "example",
    },
});
const exampleProtection = new aws.shield.Protection("example", {
    name: "example-protection",
    resourceArn: pulumi.all([currentGetPartition, current, currentGetCallerIdentity, example.id]).apply(([currentGetPartition, current, currentGetCallerIdentity, id]) => `arn:${currentGetPartition.partition}:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${id}`),
});
const exampleHealthCheck = new aws.route53.HealthCheck("example", {
    ipAddress: example.publicIp,
    port: 80,
    type: "HTTP",
    resourcePath: "/ready",
    failureThreshold: 3,
    requestInterval: 30,
    tags: {
        Name: "tf-example-health-check",
    },
});
const exampleProtectionHealthCheckAssociation = new aws.shield.ProtectionHealthCheckAssociation("example", {
    healthCheckArn: exampleHealthCheck.arn,
    shieldProtectionId: exampleProtection.id,
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
current_get_caller_identity = aws.get_caller_identity()
current_get_partition = aws.get_partition()
example = aws.ec2.Eip("example",
    domain="vpc",
    tags={
        "Name": "example",
    })
example_protection = aws.shield.Protection("example",
    name="example-protection",
    resource_arn=example.id.apply(lambda id: f"arn:{current_get_partition.partition}:ec2:{current.name}:{current_get_caller_identity.account_id}:eip-allocation/{id}"))
example_health_check = aws.route53.HealthCheck("example",
    ip_address=example.public_ip,
    port=80,
    type="HTTP",
    resource_path="/ready",
    failure_threshold=3,
    request_interval=30,
    tags={
        "Name": "tf-example-health-check",
    })
example_protection_health_check_association = aws.shield.ProtectionHealthCheckAssociation("example",
    health_check_arn=example_health_check.arn,
    shield_protection_id=example_protection.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

    var currentGetCallerIdentity = Aws.GetCallerIdentity.Invoke();

    var currentGetPartition = Aws.GetPartition.Invoke();

    var example = new Aws.Ec2.Eip("example", new()
    {
        Domain = "vpc",
        Tags = 
        {
            { "Name", "example" },
        },
    });

    var exampleProtection = new Aws.Shield.Protection("example", new()
    {
        Name = "example-protection",
        ResourceArn = Output.Tuple(currentGetPartition, current, currentGetCallerIdentity, example.Id).Apply(values =>
        {
            var currentGetPartition = values.Item1;
            var current = values.Item2;
            var currentGetCallerIdentity = values.Item3;
            var id = values.Item4;
            return $"arn:{currentGetPartition.Apply(getPartitionResult => getPartitionResult.Partition)}:ec2:{current.Apply(getRegionResult => getRegionResult.Name)}:{currentGetCallerIdentity.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:eip-allocation/{id}";
        }),
    });

    var exampleHealthCheck = new Aws.Route53.HealthCheck("example", new()
    {
        IpAddress = example.PublicIp,
        Port = 80,
        Type = "HTTP",
        ResourcePath = "/ready",
        FailureThreshold = 3,
        RequestInterval = 30,
        Tags = 
        {
            { "Name", "tf-example-health-check" },
        },
    });

    var exampleProtectionHealthCheckAssociation = new Aws.Shield.ProtectionHealthCheckAssociation("example", new()
    {
        HealthCheckArn = exampleHealthCheck.Arn,
        ShieldProtectionId = exampleProtection.Id,
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/route53"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		currentGetCallerIdentity, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		currentGetPartition, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
		if err != nil {
			return err
		}
		example, err := ec2.NewEip(ctx, "example", &ec2.EipArgs{
			Domain: pulumi.String("vpc"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		exampleProtection, err := shield.NewProtection(ctx, "example", &shield.ProtectionArgs{
			Name: pulumi.String("example-protection"),
			ResourceArn: example.ID().ApplyT(func(id string) (string, error) {
				return fmt.Sprintf("arn:%v:ec2:%v:%v:eip-allocation/%v", currentGetPartition.Partition, current.Name, currentGetCallerIdentity.AccountId, id), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		exampleHealthCheck, err := route53.NewHealthCheck(ctx, "example", &route53.HealthCheckArgs{
			IpAddress:        example.PublicIp,
			Port:             pulumi.Int(80),
			Type:             pulumi.String("HTTP"),
			ResourcePath:     pulumi.String("/ready"),
			FailureThreshold: pulumi.Int(3),
			RequestInterval:  pulumi.Int(30),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("tf-example-health-check"),
			},
		})
		if err != nil {
			return err
		}
		_, err = shield.NewProtectionHealthCheckAssociation(ctx, "example", &shield.ProtectionHealthCheckAssociationArgs{
			HealthCheckArn:     exampleHealthCheck.Arn,
			ShieldProtectionId: exampleProtection.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.ec2.Eip;
import com.pulumi.aws.ec2.EipArgs;
import com.pulumi.aws.shield.Protection;
import com.pulumi.aws.shield.ProtectionArgs;
import com.pulumi.aws.route53.HealthCheck;
import com.pulumi.aws.route53.HealthCheckArgs;
import com.pulumi.aws.shield.ProtectionHealthCheckAssociation;
import com.pulumi.aws.shield.ProtectionHealthCheckAssociationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

        final var currentGetCallerIdentity = AwsFunctions.getCallerIdentity();

        final var currentGetPartition = AwsFunctions.getPartition();

        var example = new Eip("example", EipArgs.builder()
            .domain("vpc")
            .tags(Map.of("Name", "example"))
            .build());

        var exampleProtection = new Protection("exampleProtection", ProtectionArgs.builder()
            .name("example-protection")
            .resourceArn(example.id().applyValue(id -> String.format("arn:%s:ec2:%s:%s:eip-allocation/%s", currentGetPartition.applyValue(getPartitionResult -> getPartitionResult.partition()),current.applyValue(getRegionResult -> getRegionResult.name()),currentGetCallerIdentity.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()),id)))
            .build());

        var exampleHealthCheck = new HealthCheck("exampleHealthCheck", HealthCheckArgs.builder()
            .ipAddress(example.publicIp())
            .port(80)
            .type("HTTP")
            .resourcePath("/ready")
            .failureThreshold("3")
            .requestInterval("30")
            .tags(Map.of("Name", "tf-example-health-check"))
            .build());

        var exampleProtectionHealthCheckAssociation = new ProtectionHealthCheckAssociation("exampleProtectionHealthCheckAssociation", ProtectionHealthCheckAssociationArgs.builder()
            .healthCheckArn(exampleHealthCheck.arn())
            .shieldProtectionId(exampleProtection.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ec2:Eip
    properties:
      domain: vpc
      tags:
        Name: example
  exampleProtection:
    type: aws:shield:Protection
    name: example
    properties:
      name: example-protection
      resourceArn: arn:${currentGetPartition.partition}:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${example.id}
  exampleHealthCheck:
    type: aws:route53:HealthCheck
    name: example
    properties:
      ipAddress: ${example.publicIp}
      port: 80
      type: HTTP
      resourcePath: /ready
      failureThreshold: '3'
      requestInterval: '30'
      tags:
        Name: tf-example-health-check
  exampleProtectionHealthCheckAssociation:
    type: aws:shield:ProtectionHealthCheckAssociation
    name: example
    properties:
      healthCheckArn: ${exampleHealthCheck.arn}
      shieldProtectionId: ${exampleProtection.id}
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
  currentGetCallerIdentity:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
  currentGetPartition:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Shield protection health check association resources using the `shield_protection_id` and `health_check_arn`. For example:

```sh
$ pulumi import aws:shield/protectionHealthCheckAssociation:ProtectionHealthCheckAssociation example ff9592dc-22f3-4e88-afa1-7b29fde9669a+arn:aws:route53:::healthcheck/3742b175-edb9-46bc-9359-f53e3b794b1b
```
�
healthCheckArn" xThe ARN (Amazon Resource Name) of the Route53 Health Check resource which will be associated to the protected resource.
<
shieldProtectionId" "The ID of the protected resource.
"�
healthCheckArn" xThe ARN (Amazon Resource Name) of the Route53 Health Check resource which will be associated to the protected resource.
"<
shieldProtectionId" "The ID of the protected resource.
*�
<
shieldSubscription$aws:shield/subscription:Subscription�Resource for managing an AWS Shield Subscription.

> This resource creates a subscription to AWS Shield Advanced, which requires a 1 year subscription commitment with a monthly fee. Refer to the [AWS Shield Pricing](https://aws.amazon.com/shield/pricing/) page for more details.

> Destruction of this resource will set `auto_renew` to `DISABLED`. Automatic renewal can only be disabled during the last 30 days of a subscription. To unsubscribe outside of this window, you must contact AWS Support. Set `skip_destroy` to `true` to skip modifying the `auto_renew` argument during destruction.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.shield.Subscription("example", {autoRenew: "ENABLED"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.shield.Subscription("example", auto_renew="ENABLED")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Shield.Subscription("example", new()
    {
        AutoRenew = "ENABLED",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := shield.NewSubscription(ctx, "example", &shield.SubscriptionArgs{
			AutoRenew: pulumi.String("ENABLED"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.shield.Subscription;
import com.pulumi.aws.shield.SubscriptionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Subscription("example", SubscriptionArgs.builder()
            .autoRenew("ENABLED")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:shield:Subscription
    properties:
      autoRenew: ENABLED
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Shield Subscription using the `id`. For example:

```sh
$ pulumi import aws:shield/subscription:Subscription example 123456789012
```
�
	autoRenewB" rToggle for automated renewal of the subscription. Valid values are `ENABLED` or `DISABLED`. Default is `ENABLED`.
�
skipDestroyB
 �Skip attempting to disable automated renewal upon destruction. If set to `true`, the `auto_renew` value will be left as-is and the resource will simply be removed from state.
"�
	autoRenew" rToggle for automated renewal of the subscription. Valid values are `ENABLED` or `DISABLED`. Default is `ENABLED`.
"�
skipDestroyB
 �Skip attempting to disable automated renewal upon destruction. If set to `true`, the `auto_renew` value will be left as-is and the resource will simply be removed from state.
*�D
6
signer
SigningJob aws:signer/signingJob:SigningJob�.Creates a Signer Signing Job.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testSp = new aws.signer.SigningProfile("test_sp", {platformId: "AWSLambda-SHA384-ECDSA"});
const buildSigningJob = new aws.signer.SigningJob("build_signing_job", {
    profileName: testSp.name,
    source: {
        s3: {
            bucket: "s3-bucket-name",
            key: "object-to-be-signed.zip",
            version: "jADjFYYYEXAMPLETszPjOmCMFDzd9dN1",
        },
    },
    destination: {
        s3: {
            bucket: "s3-bucket-name",
            prefix: "signed/",
        },
    },
    ignoreSigningJobFailure: true,
});
```
```python
import pulumi
import pulumi_aws as aws

test_sp = aws.signer.SigningProfile("test_sp", platform_id="AWSLambda-SHA384-ECDSA")
build_signing_job = aws.signer.SigningJob("build_signing_job",
    profile_name=test_sp.name,
    source={
        "s3": {
            "bucket": "s3-bucket-name",
            "key": "object-to-be-signed.zip",
            "version": "jADjFYYYEXAMPLETszPjOmCMFDzd9dN1",
        },
    },
    destination={
        "s3": {
            "bucket": "s3-bucket-name",
            "prefix": "signed/",
        },
    },
    ignore_signing_job_failure=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testSp = new Aws.Signer.SigningProfile("test_sp", new()
    {
        PlatformId = "AWSLambda-SHA384-ECDSA",
    });

    var buildSigningJob = new Aws.Signer.SigningJob("build_signing_job", new()
    {
        ProfileName = testSp.Name,
        Source = new Aws.Signer.Inputs.SigningJobSourceArgs
        {
            S3 = new Aws.Signer.Inputs.SigningJobSourceS3Args
            {
                Bucket = "s3-bucket-name",
                Key = "object-to-be-signed.zip",
                Version = "jADjFYYYEXAMPLETszPjOmCMFDzd9dN1",
            },
        },
        Destination = new Aws.Signer.Inputs.SigningJobDestinationArgs
        {
            S3 = new Aws.Signer.Inputs.SigningJobDestinationS3Args
            {
                Bucket = "s3-bucket-name",
                Prefix = "signed/",
            },
        },
        IgnoreSigningJobFailure = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/signer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		testSp, err := signer.NewSigningProfile(ctx, "test_sp", &signer.SigningProfileArgs{
			PlatformId: pulumi.String("AWSLambda-SHA384-ECDSA"),
		})
		if err != nil {
			return err
		}
		_, err = signer.NewSigningJob(ctx, "build_signing_job", &signer.SigningJobArgs{
			ProfileName: testSp.Name,
			Source: &signer.SigningJobSourceArgs{
				S3: &signer.SigningJobSourceS3Args{
					Bucket:  pulumi.String("s3-bucket-name"),
					Key:     pulumi.String("object-to-be-signed.zip"),
					Version: pulumi.String("jADjFYYYEXAMPLETszPjOmCMFDzd9dN1"),
				},
			},
			Destination: &signer.SigningJobDestinationArgs{
				S3: &signer.SigningJobDestinationS3Args{
					Bucket: pulumi.String("s3-bucket-name"),
					Prefix: pulumi.String("signed/"),
				},
			},
			IgnoreSigningJobFailure: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.signer.SigningProfile;
import com.pulumi.aws.signer.SigningProfileArgs;
import com.pulumi.aws.signer.SigningJob;
import com.pulumi.aws.signer.SigningJobArgs;
import com.pulumi.aws.signer.inputs.SigningJobSourceArgs;
import com.pulumi.aws.signer.inputs.SigningJobSourceS3Args;
import com.pulumi.aws.signer.inputs.SigningJobDestinationArgs;
import com.pulumi.aws.signer.inputs.SigningJobDestinationS3Args;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var testSp = new SigningProfile("testSp", SigningProfileArgs.builder()
            .platformId("AWSLambda-SHA384-ECDSA")
            .build());

        var buildSigningJob = new SigningJob("buildSigningJob", SigningJobArgs.builder()
            .profileName(testSp.name())
            .source(SigningJobSourceArgs.builder()
                .s3(SigningJobSourceS3Args.builder()
                    .bucket("s3-bucket-name")
                    .key("object-to-be-signed.zip")
                    .version("jADjFYYYEXAMPLETszPjOmCMFDzd9dN1")
                    .build())
                .build())
            .destination(SigningJobDestinationArgs.builder()
                .s3(SigningJobDestinationS3Args.builder()
                    .bucket("s3-bucket-name")
                    .prefix("signed/")
                    .build())
                .build())
            .ignoreSigningJobFailure(true)
            .build());

    }
}
```
```yaml
resources:
  testSp:
    type: aws:signer:SigningProfile
    name: test_sp
    properties:
      platformId: AWSLambda-SHA384-ECDSA
  buildSigningJob:
    type: aws:signer:SigningJob
    name: build_signing_job
    properties:
      profileName: ${testSp.name}
      source:
        s3:
          bucket: s3-bucket-name
          key: object-to-be-signed.zip
          version: jADjFYYYEXAMPLETszPjOmCMFDzd9dN1
      destination:
        s3:
          bucket: s3-bucket-name
          prefix: signed/
      ignoreSigningJobFailure: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Signer signing jobs using the `job_id`. For example:

```sh
$ pulumi import aws:signer/signingJob:SigningJob test_signer_signing_job 9ed7e5c3-b8d4-4da0-8459-44e0b068f7ee
```
�
destination[:Y
W
signerSigningJobDestination6aws:signer/SigningJobDestination:SigningJobDestinationVThe S3 bucket in which to save your signed object. See Destination below for details.
�
ignoreSigningJobFailureB
 sSet this argument to `true` to ignore signing job failures and retrieve failed status and reason. Default `false`.
N
profileName" ;The name of the profile to initiate the signing operation.
�
sourceL:J
H
signerSigningJobSource,aws:signer/SigningJobSource:SigningJobSourceNThe S3 bucket that contains the object to sign. See Source below for details.
"�
completedAt" wDate and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the signing job was completed.
"�
	createdAt" uDate and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the signing job was created.
"�
destination[:Y
W
signerSigningJobDestination6aws:signer/SigningJobDestination:SigningJobDestinationVThe S3 bucket in which to save your signed object. See Destination below for details.
"�
ignoreSigningJobFailureB
 sSet this argument to `true` to ignore signing job failures and retrieve failed status and reason. Default `false`.
"2
jobId" %The ID of the signing job on output.
"A

jobInvoker" /The IAM entity that initiated the signing job.
"5
jobOwner" %The AWS account ID of the job owner.
"k
platformDisplayName" PA human-readable name for the signing platform associated with the signing job.
"T

platformId" BThe platform to which your signed code image will be distributed.
"N
profileName" ;The name of the profile to initiate the signing operation.
"[
profileVersion" EThe version of the signing profile used to initiate the signing job.
"E
requestedBy" 2The IAM principal that requested the signing job.
"�
revocationRecordsl*j:h
f
signerSigningJobRevocationRecord@aws:signer/SigningJobRevocationRecord:SigningJobRevocationRecord�A revocation record if the signature generated by the signing job has been revoked. Contains a timestamp and the ID of the IAM entity that revoked the signature.
"P
signatureExpiresAt" 6The time when the signature of a signing job expires.
"�
signedObjects`*^:\
Z
signerSigningJobSignedObject8aws:signer/SigningJobSignedObject:SigningJobSignedObjectLName of the S3 bucket where the signed code image is saved by code signing.
"�
sourceL:J
H
signerSigningJobSource,aws:signer/SigningJobSource:SigningJobSourceNThe S3 bucket that contains the object to sign. See Source below for details.
")
status" Status of the signing job.
"B
statusReason" .String value that contains the status reason.
*�8
B
signerSigningProfile(aws:signer/signingProfile:SigningProfile�$Creates a Signer Signing Profile. A signing profile contains information about the code signing configuration parameters that can be used by a given code signing user.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testSp = new aws.signer.SigningProfile("test_sp", {platformId: "AWSLambda-SHA384-ECDSA"});
const prodSp = new aws.signer.SigningProfile("prod_sp", {
    platformId: "AWSLambda-SHA384-ECDSA",
    namePrefix: "prod_sp_",
    signatureValidityPeriod: {
        value: 5,
        type: "YEARS",
    },
    tags: {
        tag1: "value1",
        tag2: "value2",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test_sp = aws.signer.SigningProfile("test_sp", platform_id="AWSLambda-SHA384-ECDSA")
prod_sp = aws.signer.SigningProfile("prod_sp",
    platform_id="AWSLambda-SHA384-ECDSA",
    name_prefix="prod_sp_",
    signature_validity_period={
        "value": 5,
        "type": "YEARS",
    },
    tags={
        "tag1": "value1",
        "tag2": "value2",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testSp = new Aws.Signer.SigningProfile("test_sp", new()
    {
        PlatformId = "AWSLambda-SHA384-ECDSA",
    });

    var prodSp = new Aws.Signer.SigningProfile("prod_sp", new()
    {
        PlatformId = "AWSLambda-SHA384-ECDSA",
        NamePrefix = "prod_sp_",
        SignatureValidityPeriod = new Aws.Signer.Inputs.SigningProfileSignatureValidityPeriodArgs
        {
            Value = 5,
            Type = "YEARS",
        },
        Tags = 
        {
            { "tag1", "value1" },
            { "tag2", "value2" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/signer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := signer.NewSigningProfile(ctx, "test_sp", &signer.SigningProfileArgs{
			PlatformId: pulumi.String("AWSLambda-SHA384-ECDSA"),
		})
		if err != nil {
			return err
		}
		_, err = signer.NewSigningProfile(ctx, "prod_sp", &signer.SigningProfileArgs{
			PlatformId: pulumi.String("AWSLambda-SHA384-ECDSA"),
			NamePrefix: pulumi.String("prod_sp_"),
			SignatureValidityPeriod: &signer.SigningProfileSignatureValidityPeriodArgs{
				Value: pulumi.Int(5),
				Type:  pulumi.String("YEARS"),
			},
			Tags: pulumi.StringMap{
				"tag1": pulumi.String("value1"),
				"tag2": pulumi.String("value2"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.signer.SigningProfile;
import com.pulumi.aws.signer.SigningProfileArgs;
import com.pulumi.aws.signer.inputs.SigningProfileSignatureValidityPeriodArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var testSp = new SigningProfile("testSp", SigningProfileArgs.builder()
            .platformId("AWSLambda-SHA384-ECDSA")
            .build());

        var prodSp = new SigningProfile("prodSp", SigningProfileArgs.builder()
            .platformId("AWSLambda-SHA384-ECDSA")
            .namePrefix("prod_sp_")
            .signatureValidityPeriod(SigningProfileSignatureValidityPeriodArgs.builder()
                .value(5)
                .type("YEARS")
                .build())
            .tags(Map.ofEntries(
                Map.entry("tag1", "value1"),
                Map.entry("tag2", "value2")
            ))
            .build());

    }
}
```
```yaml
resources:
  testSp:
    type: aws:signer:SigningProfile
    name: test_sp
    properties:
      platformId: AWSLambda-SHA384-ECDSA
  prodSp:
    type: aws:signer:SigningProfile
    name: prod_sp
    properties:
      platformId: AWSLambda-SHA384-ECDSA
      namePrefix: prod_sp_
      signatureValidityPeriod:
        value: 5
        type: YEARS
      tags:
        tag1: value1
        tag2: value2
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Signer signing profiles using the `name`. For example:

```sh
$ pulumi import aws:signer/signingProfile:SigningProfile test_signer_signing_profile test_sp_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK
```

nameB" 

namePrefixB" U

platformId" CThe ID of the platform that is used by the target signing profile.
�
signatureValidityPeriod�B�:�
�
signer%SigningProfileSignatureValidityPeriodVaws:signer/SigningProfileSignatureValidityPeriod:SigningProfileSignatureValidityPeriod`The validity period for a signing job. See `signature_validity_period` Block below for details.
�
signingMaterialuBs:q
o
signerSigningProfileSigningMaterialFaws:signer/SigningProfileSigningMaterial:SigningProfileSigningMaterial�The AWS Certificate Manager certificate that will be used to sign code with the new signing profile. See `signing_material` Block below for details.
�
tagsB2" �A list of tags associated with the signing profile. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"C
arn" 8The Amazon Resource Name (ARN) for the signing profile.
"

name" "

namePrefix" "o
platformDisplayName" TA human-readable name for the signing platform associated with the signing profile.
"U

platformId" CThe ID of the platform that is used by the target signing profile.
"�
revocationRecordsx*v:t
r
signerSigningProfileRevocationRecordHaws:signer/SigningProfileRevocationRecord:SigningProfileRevocationRecord_Revocation information for a signing profile. See `revocation_record` Block below for details.
"�
signatureValidityPeriod�:�
�
signer%SigningProfileSignatureValidityPeriodVaws:signer/SigningProfileSignatureValidityPeriod:SigningProfileSignatureValidityPeriod`The validity period for a signing job. See `signature_validity_period` Block below for details.
"�
signingMaterials:q
o
signerSigningProfileSigningMaterialFaws:signer/SigningProfileSigningMaterial:SigningProfileSigningMaterial�The AWS Certificate Manager certificate that will be used to sign code with the new signing profile. See `signing_material` Block below for details.
"8
status" *The status of the target signing profile.
"�
tagsB2" �A list of tags associated with the signing profile. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
";
version" ,The current version of the signing profile.
"J

versionArn" 8The signing profile ARN, including the profile version.
*�Q
`
signerSigningProfilePermission<aws:signer/signingProfilePermission:SigningProfilePermission�ECreates a Signer Signing Profile Permission. That is, a cross-account permission for a signing profile.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const prodSp = new aws.signer.SigningProfile("prod_sp", {
    platformId: "AWSLambda-SHA384-ECDSA",
    namePrefix: "prod_sp_",
    signatureValidityPeriod: {
        value: 5,
        type: "YEARS",
    },
    tags: {
        tag1: "value1",
        tag2: "value2",
    },
});
const spPermission1 = new aws.signer.SigningProfilePermission("sp_permission_1", {
    profileName: prodSp.name,
    action: "signer:StartSigningJob",
    principal: awsAccount,
});
const spPermission2 = new aws.signer.SigningProfilePermission("sp_permission_2", {
    profileName: prodSp.name,
    action: "signer:GetSigningProfile",
    principal: awsTeamRoleArn,
    statementId: "ProdAccountStartSigningJob_StatementId",
});
const spPermission3 = new aws.signer.SigningProfilePermission("sp_permission_3", {
    profileName: prodSp.name,
    action: "signer:RevokeSignature",
    principal: "123456789012",
    profileVersion: prodSp.version,
    statementIdPrefix: "version-permission-",
});
```
```python
import pulumi
import pulumi_aws as aws

prod_sp = aws.signer.SigningProfile("prod_sp",
    platform_id="AWSLambda-SHA384-ECDSA",
    name_prefix="prod_sp_",
    signature_validity_period={
        "value": 5,
        "type": "YEARS",
    },
    tags={
        "tag1": "value1",
        "tag2": "value2",
    })
sp_permission1 = aws.signer.SigningProfilePermission("sp_permission_1",
    profile_name=prod_sp.name,
    action="signer:StartSigningJob",
    principal=aws_account)
sp_permission2 = aws.signer.SigningProfilePermission("sp_permission_2",
    profile_name=prod_sp.name,
    action="signer:GetSigningProfile",
    principal=aws_team_role_arn,
    statement_id="ProdAccountStartSigningJob_StatementId")
sp_permission3 = aws.signer.SigningProfilePermission("sp_permission_3",
    profile_name=prod_sp.name,
    action="signer:RevokeSignature",
    principal="123456789012",
    profile_version=prod_sp.version,
    statement_id_prefix="version-permission-")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var prodSp = new Aws.Signer.SigningProfile("prod_sp", new()
    {
        PlatformId = "AWSLambda-SHA384-ECDSA",
        NamePrefix = "prod_sp_",
        SignatureValidityPeriod = new Aws.Signer.Inputs.SigningProfileSignatureValidityPeriodArgs
        {
            Value = 5,
            Type = "YEARS",
        },
        Tags = 
        {
            { "tag1", "value1" },
            { "tag2", "value2" },
        },
    });

    var spPermission1 = new Aws.Signer.SigningProfilePermission("sp_permission_1", new()
    {
        ProfileName = prodSp.Name,
        Action = "signer:StartSigningJob",
        Principal = awsAccount,
    });

    var spPermission2 = new Aws.Signer.SigningProfilePermission("sp_permission_2", new()
    {
        ProfileName = prodSp.Name,
        Action = "signer:GetSigningProfile",
        Principal = awsTeamRoleArn,
        StatementId = "ProdAccountStartSigningJob_StatementId",
    });

    var spPermission3 = new Aws.Signer.SigningProfilePermission("sp_permission_3", new()
    {
        ProfileName = prodSp.Name,
        Action = "signer:RevokeSignature",
        Principal = "123456789012",
        ProfileVersion = prodSp.Version,
        StatementIdPrefix = "version-permission-",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/signer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		prodSp, err := signer.NewSigningProfile(ctx, "prod_sp", &signer.SigningProfileArgs{
			PlatformId: pulumi.String("AWSLambda-SHA384-ECDSA"),
			NamePrefix: pulumi.String("prod_sp_"),
			SignatureValidityPeriod: &signer.SigningProfileSignatureValidityPeriodArgs{
				Value: pulumi.Int(5),
				Type:  pulumi.String("YEARS"),
			},
			Tags: pulumi.StringMap{
				"tag1": pulumi.String("value1"),
				"tag2": pulumi.String("value2"),
			},
		})
		if err != nil {
			return err
		}
		_, err = signer.NewSigningProfilePermission(ctx, "sp_permission_1", &signer.SigningProfilePermissionArgs{
			ProfileName: prodSp.Name,
			Action:      pulumi.String("signer:StartSigningJob"),
			Principal:   pulumi.Any(awsAccount),
		})
		if err != nil {
			return err
		}
		_, err = signer.NewSigningProfilePermission(ctx, "sp_permission_2", &signer.SigningProfilePermissionArgs{
			ProfileName: prodSp.Name,
			Action:      pulumi.String("signer:GetSigningProfile"),
			Principal:   pulumi.Any(awsTeamRoleArn),
			StatementId: pulumi.String("ProdAccountStartSigningJob_StatementId"),
		})
		if err != nil {
			return err
		}
		_, err = signer.NewSigningProfilePermission(ctx, "sp_permission_3", &signer.SigningProfilePermissionArgs{
			ProfileName:       prodSp.Name,
			Action:            pulumi.String("signer:RevokeSignature"),
			Principal:         pulumi.String("123456789012"),
			ProfileVersion:    prodSp.Version,
			StatementIdPrefix: pulumi.String("version-permission-"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.signer.SigningProfile;
import com.pulumi.aws.signer.SigningProfileArgs;
import com.pulumi.aws.signer.inputs.SigningProfileSignatureValidityPeriodArgs;
import com.pulumi.aws.signer.SigningProfilePermission;
import com.pulumi.aws.signer.SigningProfilePermissionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var prodSp = new SigningProfile("prodSp", SigningProfileArgs.builder()
            .platformId("AWSLambda-SHA384-ECDSA")
            .namePrefix("prod_sp_")
            .signatureValidityPeriod(SigningProfileSignatureValidityPeriodArgs.builder()
                .value(5)
                .type("YEARS")
                .build())
            .tags(Map.ofEntries(
                Map.entry("tag1", "value1"),
                Map.entry("tag2", "value2")
            ))
            .build());

        var spPermission1 = new SigningProfilePermission("spPermission1", SigningProfilePermissionArgs.builder()
            .profileName(prodSp.name())
            .action("signer:StartSigningJob")
            .principal(awsAccount)
            .build());

        var spPermission2 = new SigningProfilePermission("spPermission2", SigningProfilePermissionArgs.builder()
            .profileName(prodSp.name())
            .action("signer:GetSigningProfile")
            .principal(awsTeamRoleArn)
            .statementId("ProdAccountStartSigningJob_StatementId")
            .build());

        var spPermission3 = new SigningProfilePermission("spPermission3", SigningProfilePermissionArgs.builder()
            .profileName(prodSp.name())
            .action("signer:RevokeSignature")
            .principal("123456789012")
            .profileVersion(prodSp.version())
            .statementIdPrefix("version-permission-")
            .build());

    }
}
```
```yaml
resources:
  prodSp:
    type: aws:signer:SigningProfile
    name: prod_sp
    properties:
      platformId: AWSLambda-SHA384-ECDSA
      namePrefix: prod_sp_
      signatureValidityPeriod:
        value: 5
        type: YEARS
      tags:
        tag1: value1
        tag2: value2
  spPermission1:
    type: aws:signer:SigningProfilePermission
    name: sp_permission_1
    properties:
      profileName: ${prodSp.name}
      action: signer:StartSigningJob
      principal: ${awsAccount}
  spPermission2:
    type: aws:signer:SigningProfilePermission
    name: sp_permission_2
    properties:
      profileName: ${prodSp.name}
      action: signer:GetSigningProfile
      principal: ${awsTeamRoleArn}
      statementId: ProdAccountStartSigningJob_StatementId
  spPermission3:
    type: aws:signer:SigningProfilePermission
    name: sp_permission_3
    properties:
      profileName: ${prodSp.name}
      action: signer:RevokeSignature
      principal: '123456789012'
      profileVersion: ${prodSp.version}
      statementIdPrefix: version-permission-
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Signer signing profile permission statements using profile_name/statement_id. For example:

```sh
$ pulumi import aws:signer/signingProfilePermission:SigningProfilePermission test_signer_signing_profile_permission prod_profile_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK/ProdAccountStartSigningJobStatementId
```
�
action" �An AWS Signer action permitted as part of cross-account permissions. Valid values: `signer:StartSigningJob`, `signer:GetSigningProfile`, `signer:RevokeSignature`, or `signer:SignPayload`.
M
	principal" <The AWS principal to be granted a cross-account permission.
U
profileName" BName of the signing profile to add the cross-account permissions.
R
profileVersionB" :The signing profile version that a permission applies to.
Z
statementIdB" EA unique statement identifier. By default generated by the provider.
�
statementIdPrefixB" jA statement identifier prefix. The provider will generate a unique suffix. Conflicts with `statement_id`.
"�
action" �An AWS Signer action permitted as part of cross-account permissions. Valid values: `signer:StartSigningJob`, `signer:GetSigningProfile`, `signer:RevokeSignature`, or `signer:SignPayload`.
"M
	principal" <The AWS principal to be granted a cross-account permission.
"U
profileName" BName of the signing profile to add the cross-account permissions.
"P
profileVersion" :The signing profile version that a permission applies to.
"X
statementId" EA unique statement identifier. By default generated by the provider.
"�
statementIdPrefix" jA statement identifier prefix. The provider will generate a unique suffix. Conflicts with `statement_id`.
*�
.
simpledbDomainaws:simpledb/domain:Domain�Provides a SimpleDB domain resource.

!> **WARNING:** The `aws.simpledb.Domain` resource has been deprecated and will be removed in a future version. Use Amazon DynamoDB instead.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const users = new aws.simpledb.Domain("users", {name: "users"});
```
```python
import pulumi
import pulumi_aws as aws

users = aws.simpledb.Domain("users", name="users")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var users = new Aws.SimpleDB.Domain("users", new()
    {
        Name = "users",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/simpledb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := simpledb.NewDomain(ctx, "users", &simpledb.DomainArgs{
			Name: pulumi.String("users"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.simpledb.Domain;
import com.pulumi.aws.simpledb.DomainArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var users = new Domain("users", DomainArgs.builder()
            .name("users")
            .build());

    }
}
```
```yaml
resources:
  users:
    type: aws:simpledb:Domain
    properties:
      name: users
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SimpleDB Domains using the `name`. For example:

```sh
$ pulumi import aws:simpledb/domain:Domain users users
```
.
nameB"  The name of the SimpleDB domain
",
name"  The name of the SimpleDB domain
*�;
N
snsDataProtectionPolicy1aws:sns/dataProtectionPolicy:DataProtectionPolicy�7Provides an SNS data protection topic policy resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sns.Topic("example", {name: "example"});
const exampleDataProtectionPolicy = new aws.sns.DataProtectionPolicy("example", {
    arn: example.arn,
    policy: JSON.stringify({
        Description: "Example data protection policy",
        Name: "__example_data_protection_policy",
        Statement: [{
            DataDirection: "Inbound",
            DataIdentifier: ["arn:aws:dataprotection::aws:data-identifier/EmailAddress"],
            Operation: {
                Deny: {},
            },
            Principal: ["*"],
            Sid: "__deny_statement_11ba9d96",
        }],
        Version: "2021-06-01",
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.sns.Topic("example", name="example")
example_data_protection_policy = aws.sns.DataProtectionPolicy("example",
    arn=example.arn,
    policy=json.dumps({
        "Description": "Example data protection policy",
        "Name": "__example_data_protection_policy",
        "Statement": [{
            "DataDirection": "Inbound",
            "DataIdentifier": ["arn:aws:dataprotection::aws:data-identifier/EmailAddress"],
            "Operation": {
                "Deny": {},
            },
            "Principal": ["*"],
            "Sid": "__deny_statement_11ba9d96",
        }],
        "Version": "2021-06-01",
    }))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Sns.Topic("example", new()
    {
        Name = "example",
    });

    var exampleDataProtectionPolicy = new Aws.Sns.DataProtectionPolicy("example", new()
    {
        Arn = example.Arn,
        Policy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Description"] = "Example data protection policy",
            ["Name"] = "__example_data_protection_policy",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["DataDirection"] = "Inbound",
                    ["DataIdentifier"] = new[]
                    {
                        "arn:aws:dataprotection::aws:data-identifier/EmailAddress",
                    },
                    ["Operation"] = new Dictionary<string, object?>
                    {
                        ["Deny"] = new Dictionary<string, object?>
                        {
                        },
                    },
                    ["Principal"] = new[]
                    {
                        "*",
                    },
                    ["Sid"] = "__deny_statement_11ba9d96",
                },
            },
            ["Version"] = "2021-06-01",
        }),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sns.NewTopic(ctx, "example", &sns.TopicArgs{
			Name: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Description": "Example data protection policy",
			"Name":        "__example_data_protection_policy",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"DataDirection": "Inbound",
					"DataIdentifier": []string{
						"arn:aws:dataprotection::aws:data-identifier/EmailAddress",
					},
					"Operation": map[string]interface{}{
						"Deny": map[string]interface{}{},
					},
					"Principal": []string{
						"*",
					},
					"Sid": "__deny_statement_11ba9d96",
				},
			},
			"Version": "2021-06-01",
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = sns.NewDataProtectionPolicy(ctx, "example", &sns.DataProtectionPolicyArgs{
			Arn:    example.Arn,
			Policy: pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import com.pulumi.aws.sns.DataProtectionPolicy;
import com.pulumi.aws.sns.DataProtectionPolicyArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Topic("example", TopicArgs.builder()
            .name("example")
            .build());

        var exampleDataProtectionPolicy = new DataProtectionPolicy("exampleDataProtectionPolicy", DataProtectionPolicyArgs.builder()
            .arn(example.arn())
            .policy(serializeJson(
                jsonObject(
                    jsonProperty("Description", "Example data protection policy"),
                    jsonProperty("Name", "__example_data_protection_policy"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("DataDirection", "Inbound"),
                        jsonProperty("DataIdentifier", jsonArray("arn:aws:dataprotection::aws:data-identifier/EmailAddress")),
                        jsonProperty("Operation", jsonObject(
                            jsonProperty("Deny", jsonObject(

                            ))
                        )),
                        jsonProperty("Principal", jsonArray("*")),
                        jsonProperty("Sid", "__deny_statement_11ba9d96")
                    ))),
                    jsonProperty("Version", "2021-06-01")
                )))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sns:Topic
    properties:
      name: example
  exampleDataProtectionPolicy:
    type: aws:sns:DataProtectionPolicy
    name: example
    properties:
      arn: ${example.arn}
      policy:
        fn::toJSON:
          Description: Example data protection policy
          Name: __example_data_protection_policy
          Statement:
            - DataDirection: Inbound
              DataIdentifier:
                - arn:aws:dataprotection::aws:data-identifier/EmailAddress
              Operation:
                Deny: {}
              Principal:
                - '*'
              Sid: __deny_statement_11ba9d96
          Version: 2021-06-01
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SNS Data Protection Topic Policy using the topic ARN. For example:

```sh
$ pulumi import aws:sns/dataProtectionPolicy:DataProtectionPolicy example arn:aws:sns:us-west-2:123456789012:example
```
$
arn" The ARN of the SNS topic
�
policy" �The fully-formed AWS policy as JSON. For more information about building AWS IAM policy documents with this provider, see the AWS IAM Policy Document Guide.
"$
arn" The ARN of the SNS topic
"�
policy" �The fully-formed AWS policy as JSON. For more information about building AWS IAM policy documents with this provider, see the AWS IAM Policy Document Guide.
*�m
K
snsPlatformApplication/aws:sns/platformApplication:PlatformApplication�FProvides an SNS platform application resource

## Example Usage

### Apple Push Notification Service (APNS) using certificate-based authentication

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const apnsApplication = new aws.sns.PlatformApplication("apns_application", {
    name: "apns_application",
    platform: "APNS",
    platformCredential: "<APNS PRIVATE KEY>",
    platformPrincipal: "<APNS CERTIFICATE>",
});
```
```python
import pulumi
import pulumi_aws as aws

apns_application = aws.sns.PlatformApplication("apns_application",
    name="apns_application",
    platform="APNS",
    platform_credential="<APNS PRIVATE KEY>",
    platform_principal="<APNS CERTIFICATE>")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var apnsApplication = new Aws.Sns.PlatformApplication("apns_application", new()
    {
        Name = "apns_application",
        Platform = "APNS",
        PlatformCredential = "<APNS PRIVATE KEY>",
        PlatformPrincipal = "<APNS CERTIFICATE>",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.NewPlatformApplication(ctx, "apns_application", &sns.PlatformApplicationArgs{
			Name:               pulumi.String("apns_application"),
			Platform:           pulumi.String("APNS"),
			PlatformCredential: pulumi.String("<APNS PRIVATE KEY>"),
			PlatformPrincipal:  pulumi.String("<APNS CERTIFICATE>"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.PlatformApplication;
import com.pulumi.aws.sns.PlatformApplicationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var apnsApplication = new PlatformApplication("apnsApplication", PlatformApplicationArgs.builder()
            .name("apns_application")
            .platform("APNS")
            .platformCredential("<APNS PRIVATE KEY>")
            .platformPrincipal("<APNS CERTIFICATE>")
            .build());

    }
}
```
```yaml
resources:
  apnsApplication:
    type: aws:sns:PlatformApplication
    name: apns_application
    properties:
      name: apns_application
      platform: APNS
      platformCredential: <APNS PRIVATE KEY>
      platformPrincipal: <APNS CERTIFICATE>
```
<!--End PulumiCodeChooser -->

### Apple Push Notification Service (APNS) using token-based authentication

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const apnsApplication = new aws.sns.PlatformApplication("apns_application", {
    name: "apns_application",
    platform: "APNS",
    platformCredential: "<APNS SIGNING KEY>",
    platformPrincipal: "<APNS SIGNING KEY ID>",
    applePlatformTeamId: "<APPLE TEAM ID>",
    applePlatformBundleId: "<APPLE BUNDLE ID>",
});
```
```python
import pulumi
import pulumi_aws as aws

apns_application = aws.sns.PlatformApplication("apns_application",
    name="apns_application",
    platform="APNS",
    platform_credential="<APNS SIGNING KEY>",
    platform_principal="<APNS SIGNING KEY ID>",
    apple_platform_team_id="<APPLE TEAM ID>",
    apple_platform_bundle_id="<APPLE BUNDLE ID>")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var apnsApplication = new Aws.Sns.PlatformApplication("apns_application", new()
    {
        Name = "apns_application",
        Platform = "APNS",
        PlatformCredential = "<APNS SIGNING KEY>",
        PlatformPrincipal = "<APNS SIGNING KEY ID>",
        ApplePlatformTeamId = "<APPLE TEAM ID>",
        ApplePlatformBundleId = "<APPLE BUNDLE ID>",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.NewPlatformApplication(ctx, "apns_application", &sns.PlatformApplicationArgs{
			Name:                  pulumi.String("apns_application"),
			Platform:              pulumi.String("APNS"),
			PlatformCredential:    pulumi.String("<APNS SIGNING KEY>"),
			PlatformPrincipal:     pulumi.String("<APNS SIGNING KEY ID>"),
			ApplePlatformTeamId:   pulumi.String("<APPLE TEAM ID>"),
			ApplePlatformBundleId: pulumi.String("<APPLE BUNDLE ID>"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.PlatformApplication;
import com.pulumi.aws.sns.PlatformApplicationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var apnsApplication = new PlatformApplication("apnsApplication", PlatformApplicationArgs.builder()
            .name("apns_application")
            .platform("APNS")
            .platformCredential("<APNS SIGNING KEY>")
            .platformPrincipal("<APNS SIGNING KEY ID>")
            .applePlatformTeamId("<APPLE TEAM ID>")
            .applePlatformBundleId("<APPLE BUNDLE ID>")
            .build());

    }
}
```
```yaml
resources:
  apnsApplication:
    type: aws:sns:PlatformApplication
    name: apns_application
    properties:
      name: apns_application
      platform: APNS
      platformCredential: <APNS SIGNING KEY>
      platformPrincipal: <APNS SIGNING KEY ID>
      applePlatformTeamId: <APPLE TEAM ID>
      applePlatformBundleId: <APPLE BUNDLE ID>
```
<!--End PulumiCodeChooser -->

### Google Cloud Messaging (GCM)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const gcmApplication = new aws.sns.PlatformApplication("gcm_application", {
    name: "gcm_application",
    platform: "GCM",
    platformCredential: "<GCM API KEY>",
});
```
```python
import pulumi
import pulumi_aws as aws

gcm_application = aws.sns.PlatformApplication("gcm_application",
    name="gcm_application",
    platform="GCM",
    platform_credential="<GCM API KEY>")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var gcmApplication = new Aws.Sns.PlatformApplication("gcm_application", new()
    {
        Name = "gcm_application",
        Platform = "GCM",
        PlatformCredential = "<GCM API KEY>",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.NewPlatformApplication(ctx, "gcm_application", &sns.PlatformApplicationArgs{
			Name:               pulumi.String("gcm_application"),
			Platform:           pulumi.String("GCM"),
			PlatformCredential: pulumi.String("<GCM API KEY>"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.PlatformApplication;
import com.pulumi.aws.sns.PlatformApplicationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var gcmApplication = new PlatformApplication("gcmApplication", PlatformApplicationArgs.builder()
            .name("gcm_application")
            .platform("GCM")
            .platformCredential("<GCM API KEY>")
            .build());

    }
}
```
```yaml
resources:
  gcmApplication:
    type: aws:sns:PlatformApplication
    name: gcm_application
    properties:
      name: gcm_application
      platform: GCM
      platformCredential: <GCM API KEY>
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SNS platform applications using the ARN. For example:

```sh
$ pulumi import aws:sns/platformApplication:PlatformApplication gcm_application arn:aws:sns:us-west-2:123456789012:app/GCM/gcm_application
```
�
applePlatformBundleIdB" The bundle identifier that's assigned to your iOS app. May only include alphanumeric characters, hyphens (-), and periods (.).
�
applePlatformTeamIdB" iThe identifier that's assigned to your Apple developer account team. Must be 10 alphanumeric characters.
�
eventDeliveryFailureTopicArnB" �The ARN of the SNS Topic triggered when a delivery to any of the platform endpoints associated with your platform application encounters a permanent failure.
�
eventEndpointCreatedTopicArnB" gThe ARN of the SNS Topic triggered when a new platform endpoint is added to your platform application.
�
eventEndpointDeletedTopicArnB" qThe ARN of the SNS Topic triggered when an existing platform endpoint is deleted from your platform application.
�
eventEndpointUpdatedTopicArnB" qThe ARN of the SNS Topic triggered when an existing platform endpoint is changed from your platform application.
�
failureFeedbackRoleArnB" �The IAM role ARN permitted to receive failure feedback for this application and give SNS write access to use CloudWatch logs on your behalf.
A
nameB" 3The friendly name for the SNS platform application
�
platform" �The platform that the app is registered with. See [Platform](http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-register.html) for supported platforms.
�
platformCredential" �Application Platform credential. See [Credential](http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-register.html) for type of credential required for platform. The value of this attribute when stored into the state is only a hash of the real value, so therefore it is not practical to use this as an attribute for other resources.
�
platformPrincipalB" �Application Platform principal. See [Principal](http://docs.aws.amazon.com/sns/latest/api/API_CreatePlatformApplication.html) for type of principal required for platform. The value of this attribute when stored into the state is only a hash of the real value, so therefore it is not practical to use this as an attribute for other resources.
�
successFeedbackRoleArnB" �The IAM role ARN permitted to receive success feedback for this application and give SNS write access to use CloudWatch logs on your behalf.
�
successFeedbackSampleRateB" �The sample rate percentage (0-100) of successfully delivered messages.

The following attributes are needed only when using APNS token credentials:
"�
applePlatformBundleIdB" The bundle identifier that's assigned to your iOS app. May only include alphanumeric characters, hyphens (-), and periods (.).
"�
applePlatformTeamIdB" iThe identifier that's assigned to your Apple developer account team. Must be 10 alphanumeric characters.
"3
arn" (The ARN of the SNS platform application
"�
eventDeliveryFailureTopicArnB" �The ARN of the SNS Topic triggered when a delivery to any of the platform endpoints associated with your platform application encounters a permanent failure.
"�
eventEndpointCreatedTopicArnB" gThe ARN of the SNS Topic triggered when a new platform endpoint is added to your platform application.
"�
eventEndpointDeletedTopicArnB" qThe ARN of the SNS Topic triggered when an existing platform endpoint is deleted from your platform application.
"�
eventEndpointUpdatedTopicArnB" qThe ARN of the SNS Topic triggered when an existing platform endpoint is changed from your platform application.
"�
failureFeedbackRoleArnB" �The IAM role ARN permitted to receive failure feedback for this application and give SNS write access to use CloudWatch logs on your behalf.
"?
name" 3The friendly name for the SNS platform application
"�
platform" �The platform that the app is registered with. See [Platform](http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-register.html) for supported platforms.
"�
platformCredential" �Application Platform credential. See [Credential](http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-register.html) for type of credential required for platform. The value of this attribute when stored into the state is only a hash of the real value, so therefore it is not practical to use this as an attribute for other resources.
"�
platformPrincipalB" �Application Platform principal. See [Principal](http://docs.aws.amazon.com/sns/latest/api/API_CreatePlatformApplication.html) for type of principal required for platform. The value of this attribute when stored into the state is only a hash of the real value, so therefore it is not practical to use this as an attribute for other resources.
"�
successFeedbackRoleArnB" �The IAM role ARN permitted to receive success feedback for this application and give SNS write access to use CloudWatch logs on your behalf.
"�
successFeedbackSampleRateB" �The sample rate percentage (0-100) of successfully delivered messages.

The following attributes are needed only when using APNS token credentials:
*�
<
snsSmsPreferences%aws:sns/smsPreferences:SmsPreferences�Provides a way to set SNS SMS preferences.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const updateSmsPrefs = new aws.sns.SmsPreferences("update_sms_prefs", {});
```
```python
import pulumi
import pulumi_aws as aws

update_sms_prefs = aws.sns.SmsPreferences("update_sms_prefs")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var updateSmsPrefs = new Aws.Sns.SmsPreferences("update_sms_prefs");

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.NewSmsPreferences(ctx, "update_sms_prefs", nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.SmsPreferences;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var updateSmsPrefs = new SmsPreferences("updateSmsPrefs");

    }
}
```
```yaml
resources:
  updateSmsPrefs:
    type: aws:sns:SmsPreferences
    name: update_sms_prefs
```
<!--End PulumiCodeChooser -->

## Import

You cannot import the SMS preferences.

y
defaultSenderIdB" `A string, such as your business brand, that is displayed as the sender on the receiving device.

defaultSmsTypeB" gThe type of SMS message that you will send by default. Possible values are: Promotional, Transactional
�
deliveryStatusIamRoleArnB" fThe ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs.
�
!deliveryStatusSuccessSamplingRateB" �The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value must be between 0 and 100.
t
monthlySpendLimitB YThe maximum amount in USD that you are willing to spend each month to send SMS messages.
r
usageReportS3BucketB" UThe name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS.
"y
defaultSenderIdB" `A string, such as your business brand, that is displayed as the sender on the receiving device.
"
defaultSmsTypeB" gThe type of SMS message that you will send by default. Possible values are: Promotional, Transactional
"�
deliveryStatusIamRoleArnB" fThe ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs.
"�
!deliveryStatusSuccessSamplingRateB" �The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value must be between 0 and 100.
"r
monthlySpendLimit YThe maximum amount in USD that you are willing to spend each month to send SMS messages.
"r
usageReportS3BucketB" UThe name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS.
*ϔ
!
snsTopicaws:sns/topic:Topic�XProvides an SNS topic resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const userUpdates = new aws.sns.Topic("user_updates", {name: "user-updates-topic"});
```
```python
import pulumi
import pulumi_aws as aws

user_updates = aws.sns.Topic("user_updates", name="user-updates-topic")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var userUpdates = new Aws.Sns.Topic("user_updates", new()
    {
        Name = "user-updates-topic",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.NewTopic(ctx, "user_updates", &sns.TopicArgs{
			Name: pulumi.String("user-updates-topic"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var userUpdates = new Topic("userUpdates", TopicArgs.builder()
            .name("user-updates-topic")
            .build());

    }
}
```
```yaml
resources:
  userUpdates:
    type: aws:sns:Topic
    name: user_updates
    properties:
      name: user-updates-topic
```
<!--End PulumiCodeChooser -->

## Example with Delivery Policy

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const userUpdates = new aws.sns.Topic("user_updates", {
    name: "user-updates-topic",
    deliveryPolicy: `{
  "http": {
    "defaultHealthyRetryPolicy": {
      "minDelayTarget": 20,
      "maxDelayTarget": 20,
      "numRetries": 3,
      "numMaxDelayRetries": 0,
      "numNoDelayRetries": 0,
      "numMinDelayRetries": 0,
      "backoffFunction": "linear"
    },
    "disableSubscriptionOverrides": false,
    "defaultThrottlePolicy": {
      "maxReceivesPerSecond": 1
    }
  }
}
`,
});
```
```python
import pulumi
import pulumi_aws as aws

user_updates = aws.sns.Topic("user_updates",
    name="user-updates-topic",
    delivery_policy="""{
  "http": {
    "defaultHealthyRetryPolicy": {
      "minDelayTarget": 20,
      "maxDelayTarget": 20,
      "numRetries": 3,
      "numMaxDelayRetries": 0,
      "numNoDelayRetries": 0,
      "numMinDelayRetries": 0,
      "backoffFunction": "linear"
    },
    "disableSubscriptionOverrides": false,
    "defaultThrottlePolicy": {
      "maxReceivesPerSecond": 1
    }
  }
}
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var userUpdates = new Aws.Sns.Topic("user_updates", new()
    {
        Name = "user-updates-topic",
        DeliveryPolicy = @"{
  ""http"": {
    ""defaultHealthyRetryPolicy"": {
      ""minDelayTarget"": 20,
      ""maxDelayTarget"": 20,
      ""numRetries"": 3,
      ""numMaxDelayRetries"": 0,
      ""numNoDelayRetries"": 0,
      ""numMinDelayRetries"": 0,
      ""backoffFunction"": ""linear""
    },
    ""disableSubscriptionOverrides"": false,
    ""defaultThrottlePolicy"": {
      ""maxReceivesPerSecond"": 1
    }
  }
}
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.NewTopic(ctx, "user_updates", &sns.TopicArgs{
			Name: pulumi.String("user-updates-topic"),
			DeliveryPolicy: pulumi.String(`{
  "http": {
    "defaultHealthyRetryPolicy": {
      "minDelayTarget": 20,
      "maxDelayTarget": 20,
      "numRetries": 3,
      "numMaxDelayRetries": 0,
      "numNoDelayRetries": 0,
      "numMinDelayRetries": 0,
      "backoffFunction": "linear"
    },
    "disableSubscriptionOverrides": false,
    "defaultThrottlePolicy": {
      "maxReceivesPerSecond": 1
    }
  }
}
`),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var userUpdates = new Topic("userUpdates", TopicArgs.builder()
            .name("user-updates-topic")
            .deliveryPolicy("""
{
  "http": {
    "defaultHealthyRetryPolicy": {
      "minDelayTarget": 20,
      "maxDelayTarget": 20,
      "numRetries": 3,
      "numMaxDelayRetries": 0,
      "numNoDelayRetries": 0,
      "numMinDelayRetries": 0,
      "backoffFunction": "linear"
    },
    "disableSubscriptionOverrides": false,
    "defaultThrottlePolicy": {
      "maxReceivesPerSecond": 1
    }
  }
}
            """)
            .build());

    }
}
```
```yaml
resources:
  userUpdates:
    type: aws:sns:Topic
    name: user_updates
    properties:
      name: user-updates-topic
      deliveryPolicy: |
        {
          "http": {
            "defaultHealthyRetryPolicy": {
              "minDelayTarget": 20,
              "maxDelayTarget": 20,
              "numRetries": 3,
              "numMaxDelayRetries": 0,
              "numNoDelayRetries": 0,
              "numMinDelayRetries": 0,
              "backoffFunction": "linear"
            },
            "disableSubscriptionOverrides": false,
            "defaultThrottlePolicy": {
              "maxReceivesPerSecond": 1
            }
          }
        }
```
<!--End PulumiCodeChooser -->

## Example with Server-side encryption (SSE)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const userUpdates = new aws.sns.Topic("user_updates", {
    name: "user-updates-topic",
    kmsMasterKeyId: "alias/aws/sns",
});
```
```python
import pulumi
import pulumi_aws as aws

user_updates = aws.sns.Topic("user_updates",
    name="user-updates-topic",
    kms_master_key_id="alias/aws/sns")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var userUpdates = new Aws.Sns.Topic("user_updates", new()
    {
        Name = "user-updates-topic",
        KmsMasterKeyId = "alias/aws/sns",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.NewTopic(ctx, "user_updates", &sns.TopicArgs{
			Name:           pulumi.String("user-updates-topic"),
			KmsMasterKeyId: pulumi.String("alias/aws/sns"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var userUpdates = new Topic("userUpdates", TopicArgs.builder()
            .name("user-updates-topic")
            .kmsMasterKeyId("alias/aws/sns")
            .build());

    }
}
```
```yaml
resources:
  userUpdates:
    type: aws:sns:Topic
    name: user_updates
    properties:
      name: user-updates-topic
      kmsMasterKeyId: alias/aws/sns
```
<!--End PulumiCodeChooser -->

## Example with First-In-First-Out (FIFO)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const userUpdates = new aws.sns.Topic("user_updates", {
    name: "user-updates-topic.fifo",
    fifoTopic: true,
    contentBasedDeduplication: true,
});
```
```python
import pulumi
import pulumi_aws as aws

user_updates = aws.sns.Topic("user_updates",
    name="user-updates-topic.fifo",
    fifo_topic=True,
    content_based_deduplication=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var userUpdates = new Aws.Sns.Topic("user_updates", new()
    {
        Name = "user-updates-topic.fifo",
        FifoTopic = true,
        ContentBasedDeduplication = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.NewTopic(ctx, "user_updates", &sns.TopicArgs{
			Name:                      pulumi.String("user-updates-topic.fifo"),
			FifoTopic:                 pulumi.Bool(true),
			ContentBasedDeduplication: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var userUpdates = new Topic("userUpdates", TopicArgs.builder()
            .name("user-updates-topic.fifo")
            .fifoTopic(true)
            .contentBasedDeduplication(true)
            .build());

    }
}
```
```yaml
resources:
  userUpdates:
    type: aws:sns:Topic
    name: user_updates
    properties:
      name: user-updates-topic.fifo
      fifoTopic: true
      contentBasedDeduplication: true
```
<!--End PulumiCodeChooser -->

## Message Delivery Status Arguments

The `<endpoint>_success_feedback_role_arn` and `<endpoint>_failure_feedback_role_arn` arguments are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The `<endpoint>_success_feedback_sample_rate` argument is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the  `<endpoint>_failure_feedback_role_arn` argument, then all failed message deliveries generate CloudWatch Logs.

## Import

Using `pulumi import`, import SNS Topics using the topic `arn`. For example:

```sh
$ pulumi import aws:sns/topic:Topic user_updates arn:aws:sns:us-west-2:123456789012:my-topic
```
I
!applicationFailureFeedbackRoleArnB" IAM role for failure feedback
m
!applicationSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
N
$applicationSuccessFeedbackSampleRateB  Percentage of success to sample
�
archivePolicyB" �The message archive policy for FIFO topics. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-topic-owner.html).
�
contentBasedDeduplicationB
 �Enables content-based deduplication for FIFO topics. For more information, see the [related documentation](https://docs.aws.amazon.com/sns/latest/dg/fifo-message-dedup.html)
�
deliveryPolicyB" �The SNS delivery policy. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html).
4
displayNameB" The display name for the topic
�
	fifoTopicB
 �Boolean indicating whether or not to create a FIFO (first-in-first-out) topic. FIFO topics can't deliver messages to customer managed endpoints, such as email addresses, mobile apps, SMS, or HTTP(S) endpoints. These endpoint types aren't guaranteed to preserve strict message ordering. Default is `false`.
F
firehoseFailureFeedbackRoleArnB" IAM role for failure feedback
j
firehoseSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
K
!firehoseSuccessFeedbackSampleRateB  Percentage of success to sample
B
httpFailureFeedbackRoleArnB" IAM role for failure feedback
f
httpSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
G
httpSuccessFeedbackSampleRateB  Percentage of success to sample
�
kmsMasterKeyIdB" �The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see [Key Terms](https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms)
D
lambdaFailureFeedbackRoleArnB" IAM role for failure feedback
h
lambdaSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
I
lambdaSuccessFeedbackSampleRateB  Percentage of success to sample
�
nameB" �The name of the topic. Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long. For a FIFO (first-in-first-out) topic, the name must end with the `.fifo` suffix. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`
e

namePrefixB" QCreates a unique name beginning with the specified prefix. Conflicts with `name`
5
policyB" %The fully-formed AWS policy as JSON.
�
signatureVersionB �If `SignatureVersion` should be [1 (SHA1) or 2 (SHA256)](https://docs.aws.amazon.com/sns/latest/dg/sns-verify-signature-of-message.html). The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS.
A
sqsFailureFeedbackRoleArnB" IAM role for failure feedback
e
sqsSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
F
sqsSuccessFeedbackSampleRateB  Percentage of success to sample
�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
g
tracingConfigB" PTracing mode of an Amazon SNS topic. Valid values: `"PassThrough"`, `"Active"`.
"I
!applicationFailureFeedbackRoleArnB" IAM role for failure feedback
"m
!applicationSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
"N
$applicationSuccessFeedbackSampleRateB  Percentage of success to sample
"�
archivePolicyB" �The message archive policy for FIFO topics. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-topic-owner.html).
"N
arn" CThe ARN of the SNS topic, as a more obvious property (clone of id)
"f
beginningArchiveTime" JThe oldest timestamp at which a FIFO topic subscriber can start a replay.
"�
contentBasedDeduplicationB
 �Enables content-based deduplication for FIFO topics. For more information, see the [related documentation](https://docs.aws.amazon.com/sns/latest/dg/fifo-message-dedup.html)
"�
deliveryPolicyB" �The SNS delivery policy. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html).
"4
displayNameB" The display name for the topic
"�
	fifoTopicB
 �Boolean indicating whether or not to create a FIFO (first-in-first-out) topic. FIFO topics can't deliver messages to customer managed endpoints, such as email addresses, mobile apps, SMS, or HTTP(S) endpoints. These endpoint types aren't guaranteed to preserve strict message ordering. Default is `false`.
"F
firehoseFailureFeedbackRoleArnB" IAM role for failure feedback
"j
firehoseSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
"K
!firehoseSuccessFeedbackSampleRateB  Percentage of success to sample
"B
httpFailureFeedbackRoleArnB" IAM role for failure feedback
"f
httpSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
"G
httpSuccessFeedbackSampleRateB  Percentage of success to sample
"�
kmsMasterKeyIdB" �The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see [Key Terms](https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms)
"D
lambdaFailureFeedbackRoleArnB" IAM role for failure feedback
"h
lambdaSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
"I
lambdaSuccessFeedbackSampleRateB  Percentage of success to sample
"�
name" �The name of the topic. Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long. For a FIFO (first-in-first-out) topic, the name must end with the `.fifo` suffix. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`
"c

namePrefix" QCreates a unique name beginning with the specified prefix. Conflicts with `name`
"7
owner" *The AWS Account ID of the SNS topic owner
"3
policy" %The fully-formed AWS policy as JSON.
"�
signatureVersion �If `SignatureVersion` should be [1 (SHA1) or 2 (SHA256)](https://docs.aws.amazon.com/sns/latest/dg/sns-verify-signature-of-message.html). The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS.
"A
sqsFailureFeedbackRoleArnB" IAM role for failure feedback
"e
sqsSuccessFeedbackRoleArnB" BThe IAM role permitted to receive success feedback for this topic
"F
sqsSuccessFeedbackSampleRateB  Percentage of success to sample
"�
tagsB2" �Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"e
tracingConfig" PTracing mode of an Amazon SNS topic. Valid values: `"PassThrough"`, `"Active"`.
*�O
3
snsTopicPolicyaws:sns/topicPolicy:TopicPolicy�MProvides an SNS topic policy resource

> **NOTE:** If a Principal is specified as just an AWS account ID rather than an ARN, AWS silently converts it to the ARN for the root user, causing future deployments to differ. To avoid this problem, just specify the full ARN, e.g. `arn:aws:iam::123456789012:root`

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.sns.Topic("test", {name: "my-topic-with-policy"});
const snsTopicPolicy = test.arn.apply(arn => aws.iam.getPolicyDocumentOutput({
    policyId: "__default_policy_ID",
    statements: [{
        actions: [
            "SNS:Subscribe",
            "SNS:SetTopicAttributes",
            "SNS:RemovePermission",
            "SNS:Receive",
            "SNS:Publish",
            "SNS:ListSubscriptionsByTopic",
            "SNS:GetTopicAttributes",
            "SNS:DeleteTopic",
            "SNS:AddPermission",
        ],
        conditions: [{
            test: "StringEquals",
            variable: "AWS:SourceOwner",
            values: [account_id],
        }],
        effect: "Allow",
        principals: [{
            type: "AWS",
            identifiers: ["*"],
        }],
        resources: [arn],
        sid: "__default_statement_ID",
    }],
}));
const _default = new aws.sns.TopicPolicy("default", {
    arn: test.arn,
    policy: snsTopicPolicy.apply(snsTopicPolicy => snsTopicPolicy.json),
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.sns.Topic("test", name="my-topic-with-policy")
sns_topic_policy = test.arn.apply(lambda arn: aws.iam.get_policy_document_output(policy_id="__default_policy_ID",
    statements=[{
        "actions": [
            "SNS:Subscribe",
            "SNS:SetTopicAttributes",
            "SNS:RemovePermission",
            "SNS:Receive",
            "SNS:Publish",
            "SNS:ListSubscriptionsByTopic",
            "SNS:GetTopicAttributes",
            "SNS:DeleteTopic",
            "SNS:AddPermission",
        ],
        "conditions": [{
            "test": "StringEquals",
            "variable": "AWS:SourceOwner",
            "values": [account_id],
        }],
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": ["*"],
        }],
        "resources": [arn],
        "sid": "__default_statement_ID",
    }]))
default = aws.sns.TopicPolicy("default",
    arn=test.arn,
    policy=sns_topic_policy.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Sns.Topic("test", new()
    {
        Name = "my-topic-with-policy",
    });

    var snsTopicPolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        PolicyId = "__default_policy_ID",
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "SNS:Subscribe",
                    "SNS:SetTopicAttributes",
                    "SNS:RemovePermission",
                    "SNS:Receive",
                    "SNS:Publish",
                    "SNS:ListSubscriptionsByTopic",
                    "SNS:GetTopicAttributes",
                    "SNS:DeleteTopic",
                    "SNS:AddPermission",
                },
                Conditions = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementConditionInputArgs
                    {
                        Test = "StringEquals",
                        Variable = "AWS:SourceOwner",
                        Values = new[]
                        {
                            account_id,
                        },
                    },
                },
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            "*",
                        },
                    },
                },
                Resources = new[]
                {
                    test.Arn,
                },
                Sid = "__default_statement_ID",
            },
        },
    });

    var @default = new Aws.Sns.TopicPolicy("default", new()
    {
        Arn = test.Arn,
        Policy = snsTopicPolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
test, err := sns.NewTopic(ctx, "test", &sns.TopicArgs{
Name: pulumi.String("my-topic-with-policy"),
})
if err != nil {
return err
}
snsTopicPolicy := test.Arn.ApplyT(func(arn string) (iam.GetPolicyDocumentResult, error) {
return iam.GetPolicyDocumentResult(interface{}(iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
PolicyId: "__default_policy_ID",
Statements: []iam.GetPolicyDocumentStatement{
{
Actions: []string{
"SNS:Subscribe",
"SNS:SetTopicAttributes",
"SNS:RemovePermission",
"SNS:Receive",
"SNS:Publish",
"SNS:ListSubscriptionsByTopic",
"SNS:GetTopicAttributes",
"SNS:DeleteTopic",
"SNS:AddPermission",
},
Conditions: []iam.GetPolicyDocumentStatementCondition{
{
Test: "StringEquals",
Variable: "AWS:SourceOwner",
Values: interface{}{
account_id,
},
},
},
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: []string{
"*",
},
},
},
Resources: interface{}{
arn,
},
Sid: "__default_statement_ID",
},
},
}, nil))), nil
}).(iam.GetPolicyDocumentResultOutput)
_, err = sns.NewTopicPolicy(ctx, "default", &sns.TopicPolicyArgs{
Arn: test.Arn,
Policy: pulumi.String(snsTopicPolicy.ApplyT(func(snsTopicPolicy iam.GetPolicyDocumentResult) (*string, error) {
return &snsTopicPolicy.Json, nil
}).(pulumi.StringPtrOutput)),
})
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.sns.TopicPolicy;
import com.pulumi.aws.sns.TopicPolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Topic("test", TopicArgs.builder()
            .name("my-topic-with-policy")
            .build());

        final var snsTopicPolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .policyId("__default_policy_ID")
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions(                
                    "SNS:Subscribe",
                    "SNS:SetTopicAttributes",
                    "SNS:RemovePermission",
                    "SNS:Receive",
                    "SNS:Publish",
                    "SNS:ListSubscriptionsByTopic",
                    "SNS:GetTopicAttributes",
                    "SNS:DeleteTopic",
                    "SNS:AddPermission")
                .conditions(GetPolicyDocumentStatementConditionArgs.builder()
                    .test("StringEquals")
                    .variable("AWS:SourceOwner")
                    .values(account_id)
                    .build())
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("AWS")
                    .identifiers("*")
                    .build())
                .resources(test.arn())
                .sid("__default_statement_ID")
                .build())
            .build());

        var default_ = new TopicPolicy("default", TopicPolicyArgs.builder()
            .arn(test.arn())
            .policy(snsTopicPolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(snsTopicPolicy -> snsTopicPolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:sns:Topic
    properties:
      name: my-topic-with-policy
  default:
    type: aws:sns:TopicPolicy
    properties:
      arn: ${test.arn}
      policy: ${snsTopicPolicy.json}
variables:
  snsTopicPolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        policyId: __default_policy_ID
        statements:
          - actions:
              - SNS:Subscribe
              - SNS:SetTopicAttributes
              - SNS:RemovePermission
              - SNS:Receive
              - SNS:Publish
              - SNS:ListSubscriptionsByTopic
              - SNS:GetTopicAttributes
              - SNS:DeleteTopic
              - SNS:AddPermission
            conditions:
              - test: StringEquals
                variable: AWS:SourceOwner
                values:
                  - ${["account-id"]}
            effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - '*'
            resources:
              - ${test.arn}
            sid: __default_statement_ID
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SNS Topic Policy using the topic ARN. For example:

```sh
$ pulumi import aws:sns/topicPolicy:TopicPolicy user_updates arn:aws:sns:us-west-2:123456789012:my-topic
```
$
arn" The ARN of the SNS topic
3
policy" %The fully-formed AWS policy as JSON.
"$
arn" The ARN of the SNS topic
"7
owner" *The AWS Account ID of the SNS topic owner
"3
policy" %The fully-formed AWS policy as JSON.
*�
E
snsTopicSubscription+aws:sns/topicSubscription:TopicSubscription��Provides a resource for subscribing to SNS topics. Requires that an SNS topic exist for the subscription to attach to. This resource allows you to automatically place messages sent to SNS topics in SQS queues, send them as HTTP(S) POST requests to a given endpoint, send SMS messages, or notify devices / applications. The most likely use case for provider users will probably be SQS queues.

> **NOTE:** If the SNS topic and SQS queue are in different AWS regions, the `aws.sns.TopicSubscription` must use an AWS provider that is in the same region as the SNS topic. If the `aws.sns.TopicSubscription` uses a provider with a different region than the SNS topic, this provider will fail to create the subscription.

> **NOTE:** Setup of cross-account subscriptions from SNS topics to SQS queues requires the provider to have access to BOTH accounts.

> **NOTE:** If an SNS topic and SQS queue are in different AWS accounts but the same region, the `aws.sns.TopicSubscription` must use the AWS provider for the account with the SQS queue. If `aws.sns.TopicSubscription` uses a Provider with a different account than the SQS queue, this provider creates the subscription but does not keep state and tries to re-create the subscription at every `apply`.

> **NOTE:** If an SNS topic and SQS queue are in different AWS accounts and different AWS regions, the subscription needs to be initiated from the account with the SQS queue but in the region of the SNS topic.

> **NOTE:** You cannot unsubscribe to a subscription that is pending confirmation. If you use `email`, `email-json`, or `http`/`https` (without auto-confirmation enabled), until the subscription is confirmed (e.g., outside of this provider), AWS does not allow this provider to delete / unsubscribe the subscription. If you `destroy` an unconfirmed subscription, this provider will remove the subscription from its state but the subscription will still exist in AWS. However, if you delete an SNS topic, SNS [deletes all the subscriptions](https://docs.aws.amazon.com/sns/latest/dg/sns-delete-subscription-topic.html) associated with the topic. Also, you can import a subscription after confirmation and then have the capability to delete it.

## Example Usage

You can directly supply a topic and ARN by hand in the `topic_arn` property along with the queue ARN:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const userUpdatesSqsTarget = new aws.sns.TopicSubscription("user_updates_sqs_target", {
    topic: "arn:aws:sns:us-west-2:432981146916:user-updates-topic",
    protocol: "sqs",
    endpoint: "arn:aws:sqs:us-west-2:432981146916:queue-too",
});
```
```python
import pulumi
import pulumi_aws as aws

user_updates_sqs_target = aws.sns.TopicSubscription("user_updates_sqs_target",
    topic="arn:aws:sns:us-west-2:432981146916:user-updates-topic",
    protocol="sqs",
    endpoint="arn:aws:sqs:us-west-2:432981146916:queue-too")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var userUpdatesSqsTarget = new Aws.Sns.TopicSubscription("user_updates_sqs_target", new()
    {
        Topic = "arn:aws:sns:us-west-2:432981146916:user-updates-topic",
        Protocol = "sqs",
        Endpoint = "arn:aws:sqs:us-west-2:432981146916:queue-too",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.NewTopicSubscription(ctx, "user_updates_sqs_target", &sns.TopicSubscriptionArgs{
			Topic:    pulumi.Any("arn:aws:sns:us-west-2:432981146916:user-updates-topic"),
			Protocol: pulumi.String("sqs"),
			Endpoint: pulumi.String("arn:aws:sqs:us-west-2:432981146916:queue-too"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.TopicSubscription;
import com.pulumi.aws.sns.TopicSubscriptionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var userUpdatesSqsTarget = new TopicSubscription("userUpdatesSqsTarget", TopicSubscriptionArgs.builder()
            .topic("arn:aws:sns:us-west-2:432981146916:user-updates-topic")
            .protocol("sqs")
            .endpoint("arn:aws:sqs:us-west-2:432981146916:queue-too")
            .build());

    }
}
```
```yaml
resources:
  userUpdatesSqsTarget:
    type: aws:sns:TopicSubscription
    name: user_updates_sqs_target
    properties:
      topic: arn:aws:sns:us-west-2:432981146916:user-updates-topic
      protocol: sqs
      endpoint: arn:aws:sqs:us-west-2:432981146916:queue-too
```
<!--End PulumiCodeChooser -->

Alternatively you can use the ARN properties of a managed SNS topic and SQS queue:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const userUpdates = new aws.sns.Topic("user_updates", {name: "user-updates-topic"});
const userUpdatesQueue = new aws.sqs.Queue("user_updates_queue", {name: "user-updates-queue"});
const userUpdatesSqsTarget = new aws.sns.TopicSubscription("user_updates_sqs_target", {
    topic: userUpdates.arn,
    protocol: "sqs",
    endpoint: userUpdatesQueue.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

user_updates = aws.sns.Topic("user_updates", name="user-updates-topic")
user_updates_queue = aws.sqs.Queue("user_updates_queue", name="user-updates-queue")
user_updates_sqs_target = aws.sns.TopicSubscription("user_updates_sqs_target",
    topic=user_updates.arn,
    protocol="sqs",
    endpoint=user_updates_queue.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var userUpdates = new Aws.Sns.Topic("user_updates", new()
    {
        Name = "user-updates-topic",
    });

    var userUpdatesQueue = new Aws.Sqs.Queue("user_updates_queue", new()
    {
        Name = "user-updates-queue",
    });

    var userUpdatesSqsTarget = new Aws.Sns.TopicSubscription("user_updates_sqs_target", new()
    {
        Topic = userUpdates.Arn,
        Protocol = "sqs",
        Endpoint = userUpdatesQueue.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		userUpdates, err := sns.NewTopic(ctx, "user_updates", &sns.TopicArgs{
			Name: pulumi.String("user-updates-topic"),
		})
		if err != nil {
			return err
		}
		userUpdatesQueue, err := sqs.NewQueue(ctx, "user_updates_queue", &sqs.QueueArgs{
			Name: pulumi.String("user-updates-queue"),
		})
		if err != nil {
			return err
		}
		_, err = sns.NewTopicSubscription(ctx, "user_updates_sqs_target", &sns.TopicSubscriptionArgs{
			Topic:    userUpdates.Arn,
			Protocol: pulumi.String("sqs"),
			Endpoint: userUpdatesQueue.Arn,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import com.pulumi.aws.sns.TopicSubscription;
import com.pulumi.aws.sns.TopicSubscriptionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var userUpdates = new Topic("userUpdates", TopicArgs.builder()
            .name("user-updates-topic")
            .build());

        var userUpdatesQueue = new Queue("userUpdatesQueue", QueueArgs.builder()
            .name("user-updates-queue")
            .build());

        var userUpdatesSqsTarget = new TopicSubscription("userUpdatesSqsTarget", TopicSubscriptionArgs.builder()
            .topic(userUpdates.arn())
            .protocol("sqs")
            .endpoint(userUpdatesQueue.arn())
            .build());

    }
}
```
```yaml
resources:
  userUpdates:
    type: aws:sns:Topic
    name: user_updates
    properties:
      name: user-updates-topic
  userUpdatesQueue:
    type: aws:sqs:Queue
    name: user_updates_queue
    properties:
      name: user-updates-queue
  userUpdatesSqsTarget:
    type: aws:sns:TopicSubscription
    name: user_updates_sqs_target
    properties:
      topic: ${userUpdates.arn}
      protocol: sqs
      endpoint: ${userUpdatesQueue.arn}
```
<!--End PulumiCodeChooser -->

You can subscribe SNS topics to SQS queues in different Amazon accounts and regions:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const config = new pulumi.Config();
const sns = config.getObject("sns") || {
    "account-id": "111111111111",
    displayName: "example",
    name: "example-sns-topic",
    region: "us-west-1",
    "role-name": "service/service",
};
const sqs = config.getObject("sqs") || {
    "account-id": "222222222222",
    name: "example-sqs-queue",
    region: "us-east-1",
    "role-name": "service/service",
};
const sns-topic-policy = aws.iam.getPolicyDocument({
    policyId: "__default_policy_ID",
    statements: [
        {
            actions: [
                "SNS:Subscribe",
                "SNS:SetTopicAttributes",
                "SNS:RemovePermission",
                "SNS:Publish",
                "SNS:ListSubscriptionsByTopic",
                "SNS:GetTopicAttributes",
                "SNS:DeleteTopic",
                "SNS:AddPermission",
            ],
            conditions: [{
                test: "StringEquals",
                variable: "AWS:SourceOwner",
                values: [sns["account-id"]],
            }],
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: ["*"],
            }],
            resources: [`arn:aws:sns:${sns.region}:${sns["account-id"]}:${sns.name}`],
            sid: "__default_statement_ID",
        },
        {
            actions: [
                "SNS:Subscribe",
                "SNS:Receive",
            ],
            conditions: [{
                test: "StringLike",
                variable: "SNS:Endpoint",
                values: [`arn:aws:sqs:${sqs.region}:${sqs["account-id"]}:${sqs.name}`],
            }],
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: ["*"],
            }],
            resources: [`arn:aws:sns:${sns.region}:${sns["account-id"]}:${sns.name}`],
            sid: "__console_sub_0",
        },
    ],
});
const sqs-queue-policy = aws.iam.getPolicyDocument({
    policyId: `arn:aws:sqs:${sqs.region}:${sqs["account-id"]}:${sqs.name}/SQSDefaultPolicy`,
    statements: [{
        sid: "example-sns-topic",
        effect: "Allow",
        principals: [{
            type: "AWS",
            identifiers: ["*"],
        }],
        actions: ["SQS:SendMessage"],
        resources: [`arn:aws:sqs:${sqs.region}:${sqs["account-id"]}:${sqs.name}`],
        conditions: [{
            test: "ArnEquals",
            variable: "aws:SourceArn",
            values: [`arn:aws:sns:${sns.region}:${sns["account-id"]}:${sns.name}`],
        }],
    }],
});
const sns_topic = new aws.sns.Topic("sns-topic", {
    name: sns.name,
    displayName: sns.display_name,
    policy: sns_topic_policy.then(sns_topic_policy => sns_topic_policy.json),
});
const sqs_queue = new aws.sqs.Queue("sqs-queue", {
    name: sqs.name,
    policy: sqs_queue_policy.then(sqs_queue_policy => sqs_queue_policy.json),
});
const sns_topicTopicSubscription = new aws.sns.TopicSubscription("sns-topic", {
    topic: sns_topic.arn,
    protocol: "sqs",
    endpoint: sqs_queue.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

config = pulumi.Config()
sns = config.get_object("sns")
if sns is None:
    sns = {
        "account-id": "111111111111",
        "displayName": "example",
        "name": "example-sns-topic",
        "region": "us-west-1",
        "role-name": "service/service",
    }
sqs = config.get_object("sqs")
if sqs is None:
    sqs = {
        "account-id": "222222222222",
        "name": "example-sqs-queue",
        "region": "us-east-1",
        "role-name": "service/service",
    }
sns_topic_policy = aws.iam.get_policy_document(policy_id="__default_policy_ID",
    statements=[
        {
            "actions": [
                "SNS:Subscribe",
                "SNS:SetTopicAttributes",
                "SNS:RemovePermission",
                "SNS:Publish",
                "SNS:ListSubscriptionsByTopic",
                "SNS:GetTopicAttributes",
                "SNS:DeleteTopic",
                "SNS:AddPermission",
            ],
            "conditions": [{
                "test": "StringEquals",
                "variable": "AWS:SourceOwner",
                "values": [sns["account-id"]],
            }],
            "effect": "Allow",
            "principals": [{
                "type": "AWS",
                "identifiers": ["*"],
            }],
            "resources": [f"arn:aws:sns:{sns['region']}:{sns['account-id']}:{sns['name']}"],
            "sid": "__default_statement_ID",
        },
        {
            "actions": [
                "SNS:Subscribe",
                "SNS:Receive",
            ],
            "conditions": [{
                "test": "StringLike",
                "variable": "SNS:Endpoint",
                "values": [f"arn:aws:sqs:{sqs['region']}:{sqs['account-id']}:{sqs['name']}"],
            }],
            "effect": "Allow",
            "principals": [{
                "type": "AWS",
                "identifiers": ["*"],
            }],
            "resources": [f"arn:aws:sns:{sns['region']}:{sns['account-id']}:{sns['name']}"],
            "sid": "__console_sub_0",
        },
    ])
sqs_queue_policy = aws.iam.get_policy_document(policy_id=f"arn:aws:sqs:{sqs['region']}:{sqs['account-id']}:{sqs['name']}/SQSDefaultPolicy",
    statements=[{
        "sid": "example-sns-topic",
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": ["*"],
        }],
        "actions": ["SQS:SendMessage"],
        "resources": [f"arn:aws:sqs:{sqs['region']}:{sqs['account-id']}:{sqs['name']}"],
        "conditions": [{
            "test": "ArnEquals",
            "variable": "aws:SourceArn",
            "values": [f"arn:aws:sns:{sns['region']}:{sns['account-id']}:{sns['name']}"],
        }],
    }])
sns_topic = aws.sns.Topic("sns-topic",
    name=sns["name"],
    display_name=sns["display_name"],
    policy=sns_topic_policy.json)
sqs_queue = aws.sqs.Queue("sqs-queue",
    name=sqs["name"],
    policy=sqs_queue_policy.json)
sns_topic_topic_subscription = aws.sns.TopicSubscription("sns-topic",
    topic=sns_topic.arn,
    protocol="sqs",
    endpoint=sqs_queue.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var config = new Config();
    var sns = config.GetObject<dynamic>("sns") ?? 
    {
        { "account-id", "111111111111" },
        { "displayName", "example" },
        { "name", "example-sns-topic" },
        { "region", "us-west-1" },
        { "role-name", "service/service" },
    };
    var sqs = config.GetObject<dynamic>("sqs") ?? 
    {
        { "account-id", "222222222222" },
        { "name", "example-sqs-queue" },
        { "region", "us-east-1" },
        { "role-name", "service/service" },
    };
    var sns_topic_policy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        PolicyId = "__default_policy_ID",
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "SNS:Subscribe",
                    "SNS:SetTopicAttributes",
                    "SNS:RemovePermission",
                    "SNS:Publish",
                    "SNS:ListSubscriptionsByTopic",
                    "SNS:GetTopicAttributes",
                    "SNS:DeleteTopic",
                    "SNS:AddPermission",
                },
                Conditions = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementConditionInputArgs
                    {
                        Test = "StringEquals",
                        Variable = "AWS:SourceOwner",
                        Values = new[]
                        {
                            sns.Account_id,
                        },
                    },
                },
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            "*",
                        },
                    },
                },
                Resources = new[]
                {
                    $"arn:aws:sns:{sns.Region}:{sns.Account_id}:{sns.Name}",
                },
                Sid = "__default_statement_ID",
            },
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "SNS:Subscribe",
                    "SNS:Receive",
                },
                Conditions = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementConditionInputArgs
                    {
                        Test = "StringLike",
                        Variable = "SNS:Endpoint",
                        Values = new[]
                        {
                            $"arn:aws:sqs:{sqs.Region}:{sqs.Account_id}:{sqs.Name}",
                        },
                    },
                },
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            "*",
                        },
                    },
                },
                Resources = new[]
                {
                    $"arn:aws:sns:{sns.Region}:{sns.Account_id}:{sns.Name}",
                },
                Sid = "__console_sub_0",
            },
        },
    });

    var sqs_queue_policy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        PolicyId = $"arn:aws:sqs:{sqs.Region}:{sqs.Account_id}:{sqs.Name}/SQSDefaultPolicy",
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "example-sns-topic",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            "*",
                        },
                    },
                },
                Actions = new[]
                {
                    "SQS:SendMessage",
                },
                Resources = new[]
                {
                    $"arn:aws:sqs:{sqs.Region}:{sqs.Account_id}:{sqs.Name}",
                },
                Conditions = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementConditionInputArgs
                    {
                        Test = "ArnEquals",
                        Variable = "aws:SourceArn",
                        Values = new[]
                        {
                            $"arn:aws:sns:{sns.Region}:{sns.Account_id}:{sns.Name}",
                        },
                    },
                },
            },
        },
    });

    var sns_topic = new Aws.Sns.Topic("sns-topic", new()
    {
        Name = sns.Name,
        DisplayName = sns.Display_name,
        Policy = sns_topic_policy.Apply(sns_topic_policy => sns_topic_policy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json)),
    });

    var sqs_queue = new Aws.Sqs.Queue("sqs-queue", new()
    {
        Name = sqs.Name,
        Policy = sqs_queue_policy.Apply(sqs_queue_policy => sqs_queue_policy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json)),
    });

    var sns_topicTopicSubscription = new Aws.Sns.TopicSubscription("sns-topic", new()
    {
        Topic = sns_topic.Arn,
        Protocol = "sqs",
        Endpoint = sqs_queue.Arn,
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
cfg := config.New(ctx, "")
sns := map[string]interface{}{
"account-id": "111111111111",
"displayName": "example",
"name": "example-sns-topic",
"region": "us-west-1",
"role-name": "service/service",
};
if param := cfg.GetObject("sns"); param != nil {
sns = param
}
sqs := map[string]interface{}{
"account-id": "222222222222",
"name": "example-sqs-queue",
"region": "us-east-1",
"role-name": "service/service",
};
if param := cfg.GetObject("sqs"); param != nil {
sqs = param
}
sns_topic_policy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
PolicyId: pulumi.StringRef("__default_policy_ID"),
Statements: []iam.GetPolicyDocumentStatement{
{
Actions: []string{
"SNS:Subscribe",
"SNS:SetTopicAttributes",
"SNS:RemovePermission",
"SNS:Publish",
"SNS:ListSubscriptionsByTopic",
"SNS:GetTopicAttributes",
"SNS:DeleteTopic",
"SNS:AddPermission",
},
Conditions: []iam.GetPolicyDocumentStatementCondition{
{
Test: "StringEquals",
Variable: "AWS:SourceOwner",
Values: interface{}{
sns.AccountId,
},
},
},
Effect: pulumi.StringRef("Allow"),
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: []string{
"*",
},
},
},
Resources: []string{
fmt.Sprintf("arn:aws:sns:%v:%v:%v", sns.Region, sns.AccountId, sns.Name),
},
Sid: pulumi.StringRef("__default_statement_ID"),
},
{
Actions: []string{
"SNS:Subscribe",
"SNS:Receive",
},
Conditions: []iam.GetPolicyDocumentStatementCondition{
{
Test: "StringLike",
Variable: "SNS:Endpoint",
Values: []string{
fmt.Sprintf("arn:aws:sqs:%v:%v:%v", sqs.Region, sqs.AccountId, sqs.Name),
},
},
},
Effect: pulumi.StringRef("Allow"),
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: []string{
"*",
},
},
},
Resources: []string{
fmt.Sprintf("arn:aws:sns:%v:%v:%v", sns.Region, sns.AccountId, sns.Name),
},
Sid: pulumi.StringRef("__console_sub_0"),
},
},
}, nil);
if err != nil {
return err
}
sqs_queue_policy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
PolicyId: pulumi.StringRef(fmt.Sprintf("arn:aws:sqs:%v:%v:%v/SQSDefaultPolicy", sqs.Region, sqs.AccountId, sqs.Name)),
Statements: []iam.GetPolicyDocumentStatement{
{
Sid: pulumi.StringRef("example-sns-topic"),
Effect: pulumi.StringRef("Allow"),
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: []string{
"*",
},
},
},
Actions: []string{
"SQS:SendMessage",
},
Resources: []string{
fmt.Sprintf("arn:aws:sqs:%v:%v:%v", sqs.Region, sqs.AccountId, sqs.Name),
},
Conditions: []iam.GetPolicyDocumentStatementCondition{
{
Test: "ArnEquals",
Variable: "aws:SourceArn",
Values: []string{
fmt.Sprintf("arn:aws:sns:%v:%v:%v", sns.Region, sns.AccountId, sns.Name),
},
},
},
},
},
}, nil);
if err != nil {
return err
}
_, err = sns.NewTopic(ctx, "sns-topic", &sns.TopicArgs{
Name: pulumi.Any(sns.Name),
DisplayName: pulumi.Any(sns.Display_name),
Policy: pulumi.String(sns_topic_policy.Json),
})
if err != nil {
return err
}
_, err = sqs.NewQueue(ctx, "sqs-queue", &sqs.QueueArgs{
Name: pulumi.Any(sqs.Name),
Policy: pulumi.String(sqs_queue_policy.Json),
})
if err != nil {
return err
}
_, err = sns.NewTopicSubscription(ctx, "sns-topic", &sns.TopicSubscriptionArgs{
Topic: sns_topic.Arn,
Protocol: pulumi.String("sqs"),
Endpoint: sqs_queue.Arn,
})
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.sns.Topic;
import com.pulumi.aws.sns.TopicArgs;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import com.pulumi.aws.sns.TopicSubscription;
import com.pulumi.aws.sns.TopicSubscriptionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var config = ctx.config();
        final var sns = config.get("sns").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        final var sqs = config.get("sqs").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        final var sns-topic-policy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .policyId("__default_policy_ID")
            .statements(            
                GetPolicyDocumentStatementArgs.builder()
                    .actions(                    
                        "SNS:Subscribe",
                        "SNS:SetTopicAttributes",
                        "SNS:RemovePermission",
                        "SNS:Publish",
                        "SNS:ListSubscriptionsByTopic",
                        "SNS:GetTopicAttributes",
                        "SNS:DeleteTopic",
                        "SNS:AddPermission")
                    .conditions(GetPolicyDocumentStatementConditionArgs.builder()
                        .test("StringEquals")
                        .variable("AWS:SourceOwner")
                        .values(sns.account-id())
                        .build())
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers("*")
                        .build())
                    .resources(String.format("arn:aws:sns:%s:%s:%s", sns.region(),sns.account-id(),sns.name()))
                    .sid("__default_statement_ID")
                    .build(),
                GetPolicyDocumentStatementArgs.builder()
                    .actions(                    
                        "SNS:Subscribe",
                        "SNS:Receive")
                    .conditions(GetPolicyDocumentStatementConditionArgs.builder()
                        .test("StringLike")
                        .variable("SNS:Endpoint")
                        .values(String.format("arn:aws:sqs:%s:%s:%s", sqs.region(),sqs.account-id(),sqs.name()))
                        .build())
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers("*")
                        .build())
                    .resources(String.format("arn:aws:sns:%s:%s:%s", sns.region(),sns.account-id(),sns.name()))
                    .sid("__console_sub_0")
                    .build())
            .build());

        final var sqs-queue-policy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .policyId(String.format("arn:aws:sqs:%s:%s:%s/SQSDefaultPolicy", sqs.region(),sqs.account-id(),sqs.name()))
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("example-sns-topic")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("AWS")
                    .identifiers("*")
                    .build())
                .actions("SQS:SendMessage")
                .resources(String.format("arn:aws:sqs:%s:%s:%s", sqs.region(),sqs.account-id(),sqs.name()))
                .conditions(GetPolicyDocumentStatementConditionArgs.builder()
                    .test("ArnEquals")
                    .variable("aws:SourceArn")
                    .values(String.format("arn:aws:sns:%s:%s:%s", sns.region(),sns.account-id(),sns.name()))
                    .build())
                .build())
            .build());

        var sns_topic = new Topic("sns-topic", TopicArgs.builder()
            .name(sns.name())
            .displayName(sns.display_name())
            .policy(sns_topic_policy.json())
            .build());

        var sqs_queue = new Queue("sqs-queue", QueueArgs.builder()
            .name(sqs.name())
            .policy(sqs_queue_policy.json())
            .build());

        var sns_topicTopicSubscription = new TopicSubscription("sns-topicTopicSubscription", TopicSubscriptionArgs.builder()
            .topic(sns_topic.arn())
            .protocol("sqs")
            .endpoint(sqs_queue.arn())
            .build());

    }
}
```
```yaml
configuration:
  sns:
    type: dynamic
    default:
      account-id: '111111111111'
      displayName: example
      name: example-sns-topic
      region: us-west-1
      role-name: service/service
  sqs:
    type: dynamic
    default:
      account-id: '222222222222'
      name: example-sqs-queue
      region: us-east-1
      role-name: service/service
resources:
  sns-topic:
    type: aws:sns:Topic
    properties:
      name: ${sns.name}
      displayName: ${sns.display_name}
      policy: ${["sns-topic-policy"].json}
  sqs-queue:
    type: aws:sqs:Queue
    properties:
      name: ${sqs.name}
      policy: ${["sqs-queue-policy"].json}
  sns-topicTopicSubscription:
    type: aws:sns:TopicSubscription
    name: sns-topic
    properties:
      topic: ${["sns-topic"].arn}
      protocol: sqs
      endpoint: ${["sqs-queue"].arn}
variables:
  sns-topic-policy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        policyId: __default_policy_ID
        statements:
          - actions:
              - SNS:Subscribe
              - SNS:SetTopicAttributes
              - SNS:RemovePermission
              - SNS:Publish
              - SNS:ListSubscriptionsByTopic
              - SNS:GetTopicAttributes
              - SNS:DeleteTopic
              - SNS:AddPermission
            conditions:
              - test: StringEquals
                variable: AWS:SourceOwner
                values:
                  - ${sns"account-id"[%!s(MISSING)]}
            effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - '*'
            resources:
              - arn:aws:sns:${sns.region}:${sns"account-id"[%!s(MISSING)]}:${sns.name}
            sid: __default_statement_ID
          - actions:
              - SNS:Subscribe
              - SNS:Receive
            conditions:
              - test: StringLike
                variable: SNS:Endpoint
                values:
                  - arn:aws:sqs:${sqs.region}:${sqs"account-id"[%!s(MISSING)]}:${sqs.name}
            effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - '*'
            resources:
              - arn:aws:sns:${sns.region}:${sns"account-id"[%!s(MISSING)]}:${sns.name}
            sid: __console_sub_0
  sqs-queue-policy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        policyId: arn:aws:sqs:${sqs.region}:${sqs"account-id"[%!s(MISSING)]}:${sqs.name}/SQSDefaultPolicy
        statements:
          - sid: example-sns-topic
            effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - '*'
            actions:
              - SQS:SendMessage
            resources:
              - arn:aws:sqs:${sqs.region}:${sqs"account-id"[%!s(MISSING)]}:${sqs.name}
            conditions:
              - test: ArnEquals
                variable: aws:SourceArn
                values:
                  - arn:aws:sns:${sns.region}:${sns"account-id"[%!s(MISSING)]}:${sns.name}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SNS Topic Subscriptions using the subscription `arn`. For example:

```sh
$ pulumi import aws:sns/topicSubscription:TopicSubscription user_updates_sqs_target arn:aws:sns:us-west-2:123456789012:my-topic:8a21d249-4329-4871-acc6-7be709c6ea7f
```
�
confirmationTimeoutInMinutesB �Integer indicating number of minutes to wait in retrying mode for fetching subscription arn before marking it as failure. Only applicable for http and https protocols. Default is `1`.
�
deliveryPolicyB" �JSON String with the delivery policy (retries, backoff, etc.) that will be used in the subscription - this only applies to HTTP/S subscriptions. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html) for more details.
b
endpoint" REndpoint to send data to. The contents vary with the protocol. See details below.
�
endpointAutoConfirmsB
 �Whether the endpoint is capable of [auto confirming subscription](http://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.html#SendMessageToHttp.prepare) (e.g., PagerDuty). Default is `false`.
�
filterPolicyB" �JSON String with the filter policy that will be used in the subscription to filter messages seen by the target resource. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-filtering.html) for more details.
r
filterPolicyScopeB" WWhether the `filter_policy` applies to `MessageAttributes` (default) or `MessageBody`.
�
protocol" �Protocol to use. Valid values are: `sqs`, `sms`, `lambda`, `firehose`, and `application`. Protocols `email`, `email-json`, `http` and `https` are also valid but partially supported. See details below.
�
rawMessageDeliveryB
 �Whether to enable raw message delivery (the original message is directly passed, not wrapped in JSON with the original message in the message property). Default is `false`.
�
redrivePolicyB" �JSON String with the redrive policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-dead-letter-queues.html#how-messages-moved-into-dead-letter-queue) for more details.
�
replayPolicyB" �JSON String with the archived message replay policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-subscriber.html) for more details.
�
subscriptionRoleArnB" �ARN of the IAM role to publish to Kinesis Data Firehose delivery stream. Refer to [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html).
Z
topic" MARN of the SNS topic to subscribe to.

The following arguments are optional:
"$
arn" ARN of the subscription.
"�
confirmationTimeoutInMinutesB �Integer indicating number of minutes to wait in retrying mode for fetching subscription arn before marking it as failure. Only applicable for http and https protocols. Default is `1`.
"e
confirmationWasAuthenticated
 AWhether the subscription confirmation request was authenticated.
"�
deliveryPolicyB" �JSON String with the delivery policy (retries, backoff, etc.) that will be used in the subscription - this only applies to HTTP/S subscriptions. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html) for more details.
"b
endpoint" REndpoint to send data to. The contents vary with the protocol. See details below.
"�
endpointAutoConfirmsB
 �Whether the endpoint is capable of [auto confirming subscription](http://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.html#SendMessageToHttp.prepare) (e.g., PagerDuty). Default is `false`.
"�
filterPolicyB" �JSON String with the filter policy that will be used in the subscription to filter messages seen by the target resource. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-filtering.html) for more details.
"p
filterPolicyScope" WWhether the `filter_policy` applies to `MessageAttributes` (default) or `MessageBody`.
";
ownerId" ,AWS account ID of the subscription's owner.
"L
pendingConfirmation
 1Whether the subscription has not been confirmed.
"�
protocol" �Protocol to use. Valid values are: `sqs`, `sms`, `lambda`, `firehose`, and `application`. Protocols `email`, `email-json`, `http` and `https` are also valid but partially supported. See details below.
"�
rawMessageDeliveryB
 �Whether to enable raw message delivery (the original message is directly passed, not wrapped in JSON with the original message in the message property). Default is `false`.
"�
redrivePolicyB" �JSON String with the redrive policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-dead-letter-queues.html#how-messages-moved-into-dead-letter-queue) for more details.
"�
replayPolicyB" �JSON String with the archived message replay policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-subscriber.html) for more details.
"�
subscriptionRoleArnB" �ARN of the IAM role to publish to Kinesis Data Firehose delivery stream. Refer to [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html).
"Z
topic" MARN of the SNS topic to subscribe to.

The following arguments are optional:
*��
!
sqsQueueaws:sqs/queue:Queue�## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const queue = new aws.sqs.Queue("queue", {
    name: "example-queue",
    delaySeconds: 90,
    maxMessageSize: 2048,
    messageRetentionSeconds: 86400,
    receiveWaitTimeSeconds: 10,
    redrivePolicy: JSON.stringify({
        deadLetterTargetArn: queueDeadletter.arn,
        maxReceiveCount: 4,
    }),
    tags: {
        Environment: "production",
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

queue = aws.sqs.Queue("queue",
    name="example-queue",
    delay_seconds=90,
    max_message_size=2048,
    message_retention_seconds=86400,
    receive_wait_time_seconds=10,
    redrive_policy=json.dumps({
        "deadLetterTargetArn": queue_deadletter["arn"],
        "maxReceiveCount": 4,
    }),
    tags={
        "Environment": "production",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var queue = new Aws.Sqs.Queue("queue", new()
    {
        Name = "example-queue",
        DelaySeconds = 90,
        MaxMessageSize = 2048,
        MessageRetentionSeconds = 86400,
        ReceiveWaitTimeSeconds = 10,
        RedrivePolicy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["deadLetterTargetArn"] = queueDeadletter.Arn,
            ["maxReceiveCount"] = 4,
        }),
        Tags = 
        {
            { "Environment", "production" },
        },
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"deadLetterTargetArn": queueDeadletter.Arn,
			"maxReceiveCount":     4,
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = sqs.NewQueue(ctx, "queue", &sqs.QueueArgs{
			Name:                    pulumi.String("example-queue"),
			DelaySeconds:            pulumi.Int(90),
			MaxMessageSize:          pulumi.Int(2048),
			MessageRetentionSeconds: pulumi.Int(86400),
			ReceiveWaitTimeSeconds:  pulumi.Int(10),
			RedrivePolicy:           pulumi.String(json0),
			Tags: pulumi.StringMap{
				"Environment": pulumi.String("production"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var queue = new Queue("queue", QueueArgs.builder()
            .name("example-queue")
            .delaySeconds(90)
            .maxMessageSize(2048)
            .messageRetentionSeconds(86400)
            .receiveWaitTimeSeconds(10)
            .redrivePolicy(serializeJson(
                jsonObject(
                    jsonProperty("deadLetterTargetArn", queueDeadletter.arn()),
                    jsonProperty("maxReceiveCount", 4)
                )))
            .tags(Map.of("Environment", "production"))
            .build());

    }
}
```
```yaml
resources:
  queue:
    type: aws:sqs:Queue
    properties:
      name: example-queue
      delaySeconds: 90
      maxMessageSize: 2048
      messageRetentionSeconds: 86400
      receiveWaitTimeSeconds: 10
      redrivePolicy:
        fn::toJSON:
          deadLetterTargetArn: ${queueDeadletter.arn}
          maxReceiveCount: 4
      tags:
        Environment: production
```
<!--End PulumiCodeChooser -->

## FIFO queue

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const queue = new aws.sqs.Queue("queue", {
    name: "example-queue.fifo",
    fifoQueue: true,
    contentBasedDeduplication: true,
});
```
```python
import pulumi
import pulumi_aws as aws

queue = aws.sqs.Queue("queue",
    name="example-queue.fifo",
    fifo_queue=True,
    content_based_deduplication=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var queue = new Aws.Sqs.Queue("queue", new()
    {
        Name = "example-queue.fifo",
        FifoQueue = true,
        ContentBasedDeduplication = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sqs.NewQueue(ctx, "queue", &sqs.QueueArgs{
			Name:                      pulumi.String("example-queue.fifo"),
			FifoQueue:                 pulumi.Bool(true),
			ContentBasedDeduplication: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var queue = new Queue("queue", QueueArgs.builder()
            .name("example-queue.fifo")
            .fifoQueue(true)
            .contentBasedDeduplication(true)
            .build());

    }
}
```
```yaml
resources:
  queue:
    type: aws:sqs:Queue
    properties:
      name: example-queue.fifo
      fifoQueue: true
      contentBasedDeduplication: true
```
<!--End PulumiCodeChooser -->

## High-throughput FIFO queue

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const queue = new aws.sqs.Queue("queue", {
    name: "pulumi-example-queue.fifo",
    fifoQueue: true,
    deduplicationScope: "messageGroup",
    fifoThroughputLimit: "perMessageGroupId",
});
```
```python
import pulumi
import pulumi_aws as aws

queue = aws.sqs.Queue("queue",
    name="pulumi-example-queue.fifo",
    fifo_queue=True,
    deduplication_scope="messageGroup",
    fifo_throughput_limit="perMessageGroupId")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var queue = new Aws.Sqs.Queue("queue", new()
    {
        Name = "pulumi-example-queue.fifo",
        FifoQueue = true,
        DeduplicationScope = "messageGroup",
        FifoThroughputLimit = "perMessageGroupId",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sqs.NewQueue(ctx, "queue", &sqs.QueueArgs{
			Name:                pulumi.String("pulumi-example-queue.fifo"),
			FifoQueue:           pulumi.Bool(true),
			DeduplicationScope:  pulumi.String("messageGroup"),
			FifoThroughputLimit: pulumi.String("perMessageGroupId"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var queue = new Queue("queue", QueueArgs.builder()
            .name("pulumi-example-queue.fifo")
            .fifoQueue(true)
            .deduplicationScope("messageGroup")
            .fifoThroughputLimit("perMessageGroupId")
            .build());

    }
}
```
```yaml
resources:
  queue:
    type: aws:sqs:Queue
    properties:
      name: pulumi-example-queue.fifo
      fifoQueue: true
      deduplicationScope: messageGroup
      fifoThroughputLimit: perMessageGroupId
```
<!--End PulumiCodeChooser -->

## Dead-letter queue

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const queue = new aws.sqs.Queue("queue", {
    name: "pulumi-example-queue",
    redrivePolicy: JSON.stringify({
        deadLetterTargetArn: queueDeadletter.arn,
        maxReceiveCount: 4,
    }),
});
const exampleQueueDeadletter = new aws.sqs.Queue("example_queue_deadletter", {name: "pulumi-example-deadletter-queue"});
const exampleQueueRedriveAllowPolicy = new aws.sqs.RedriveAllowPolicy("example_queue_redrive_allow_policy", {
    queueUrl: exampleQueueDeadletter.id,
    redriveAllowPolicy: JSON.stringify({
        redrivePermission: "byQueue",
        sourceQueueArns: [exampleQueue.arn],
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

queue = aws.sqs.Queue("queue",
    name="pulumi-example-queue",
    redrive_policy=json.dumps({
        "deadLetterTargetArn": queue_deadletter["arn"],
        "maxReceiveCount": 4,
    }))
example_queue_deadletter = aws.sqs.Queue("example_queue_deadletter", name="pulumi-example-deadletter-queue")
example_queue_redrive_allow_policy = aws.sqs.RedriveAllowPolicy("example_queue_redrive_allow_policy",
    queue_url=example_queue_deadletter.id,
    redrive_allow_policy=json.dumps({
        "redrivePermission": "byQueue",
        "sourceQueueArns": [example_queue["arn"]],
    }))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var queue = new Aws.Sqs.Queue("queue", new()
    {
        Name = "pulumi-example-queue",
        RedrivePolicy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["deadLetterTargetArn"] = queueDeadletter.Arn,
            ["maxReceiveCount"] = 4,
        }),
    });

    var exampleQueueDeadletter = new Aws.Sqs.Queue("example_queue_deadletter", new()
    {
        Name = "pulumi-example-deadletter-queue",
    });

    var exampleQueueRedriveAllowPolicy = new Aws.Sqs.RedriveAllowPolicy("example_queue_redrive_allow_policy", new()
    {
        QueueUrl = exampleQueueDeadletter.Id,
        RedriveAllowPolicyName = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["redrivePermission"] = "byQueue",
            ["sourceQueueArns"] = new[]
            {
                exampleQueue.Arn,
            },
        }),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"deadLetterTargetArn": queueDeadletter.Arn,
			"maxReceiveCount":     4,
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = sqs.NewQueue(ctx, "queue", &sqs.QueueArgs{
			Name:          pulumi.String("pulumi-example-queue"),
			RedrivePolicy: pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		exampleQueueDeadletter, err := sqs.NewQueue(ctx, "example_queue_deadletter", &sqs.QueueArgs{
			Name: pulumi.String("pulumi-example-deadletter-queue"),
		})
		if err != nil {
			return err
		}
		tmpJSON1, err := json.Marshal(map[string]interface{}{
			"redrivePermission": "byQueue",
			"sourceQueueArns": []interface{}{
				exampleQueue.Arn,
			},
		})
		if err != nil {
			return err
		}
		json1 := string(tmpJSON1)
		_, err = sqs.NewRedriveAllowPolicy(ctx, "example_queue_redrive_allow_policy", &sqs.RedriveAllowPolicyArgs{
			QueueUrl:           exampleQueueDeadletter.ID(),
			RedriveAllowPolicy: pulumi.String(json1),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import com.pulumi.aws.sqs.RedriveAllowPolicy;
import com.pulumi.aws.sqs.RedriveAllowPolicyArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var queue = new Queue("queue", QueueArgs.builder()
            .name("pulumi-example-queue")
            .redrivePolicy(serializeJson(
                jsonObject(
                    jsonProperty("deadLetterTargetArn", queueDeadletter.arn()),
                    jsonProperty("maxReceiveCount", 4)
                )))
            .build());

        var exampleQueueDeadletter = new Queue("exampleQueueDeadletter", QueueArgs.builder()
            .name("pulumi-example-deadletter-queue")
            .build());

        var exampleQueueRedriveAllowPolicy = new RedriveAllowPolicy("exampleQueueRedriveAllowPolicy", RedriveAllowPolicyArgs.builder()
            .queueUrl(exampleQueueDeadletter.id())
            .redriveAllowPolicy(serializeJson(
                jsonObject(
                    jsonProperty("redrivePermission", "byQueue"),
                    jsonProperty("sourceQueueArns", jsonArray(exampleQueue.arn()))
                )))
            .build());

    }
}
```
```yaml
resources:
  queue:
    type: aws:sqs:Queue
    properties:
      name: pulumi-example-queue
      redrivePolicy:
        fn::toJSON:
          deadLetterTargetArn: ${queueDeadletter.arn}
          maxReceiveCount: 4
  exampleQueueDeadletter:
    type: aws:sqs:Queue
    name: example_queue_deadletter
    properties:
      name: pulumi-example-deadletter-queue
  exampleQueueRedriveAllowPolicy:
    type: aws:sqs:RedriveAllowPolicy
    name: example_queue_redrive_allow_policy
    properties:
      queueUrl: ${exampleQueueDeadletter.id}
      redriveAllowPolicy:
        fn::toJSON:
          redrivePermission: byQueue
          sourceQueueArns:
            - ${exampleQueue.arn}
```
<!--End PulumiCodeChooser -->

## Server-side encryption (SSE)

Using [SSE-SQS](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const queue = new aws.sqs.Queue("queue", {
    name: "pulumi-example-queue",
    sqsManagedSseEnabled: true,
});
```
```python
import pulumi
import pulumi_aws as aws

queue = aws.sqs.Queue("queue",
    name="pulumi-example-queue",
    sqs_managed_sse_enabled=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var queue = new Aws.Sqs.Queue("queue", new()
    {
        Name = "pulumi-example-queue",
        SqsManagedSseEnabled = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sqs.NewQueue(ctx, "queue", &sqs.QueueArgs{
			Name:                 pulumi.String("pulumi-example-queue"),
			SqsManagedSseEnabled: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var queue = new Queue("queue", QueueArgs.builder()
            .name("pulumi-example-queue")
            .sqsManagedSseEnabled(true)
            .build());

    }
}
```
```yaml
resources:
  queue:
    type: aws:sqs:Queue
    properties:
      name: pulumi-example-queue
      sqsManagedSseEnabled: true
```
<!--End PulumiCodeChooser -->

Using [SSE-KMS](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const queue = new aws.sqs.Queue("queue", {
    name: "example-queue",
    kmsMasterKeyId: "alias/aws/sqs",
    kmsDataKeyReusePeriodSeconds: 300,
});
```
```python
import pulumi
import pulumi_aws as aws

queue = aws.sqs.Queue("queue",
    name="example-queue",
    kms_master_key_id="alias/aws/sqs",
    kms_data_key_reuse_period_seconds=300)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var queue = new Aws.Sqs.Queue("queue", new()
    {
        Name = "example-queue",
        KmsMasterKeyId = "alias/aws/sqs",
        KmsDataKeyReusePeriodSeconds = 300,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sqs.NewQueue(ctx, "queue", &sqs.QueueArgs{
			Name:                         pulumi.String("example-queue"),
			KmsMasterKeyId:               pulumi.String("alias/aws/sqs"),
			KmsDataKeyReusePeriodSeconds: pulumi.Int(300),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var queue = new Queue("queue", QueueArgs.builder()
            .name("example-queue")
            .kmsMasterKeyId("alias/aws/sqs")
            .kmsDataKeyReusePeriodSeconds(300)
            .build());

    }
}
```
```yaml
resources:
  queue:
    type: aws:sqs:Queue
    properties:
      name: example-queue
      kmsMasterKeyId: alias/aws/sqs
      kmsDataKeyReusePeriodSeconds: 300
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SQS Queues using the queue `url`. For example:

```sh
$ pulumi import aws:sqs/queue:Queue public_queue https://queue.amazonaws.com/80398EXAMPLE/MyQueue
```
�
contentBasedDeduplicationB
 �Enables content-based deduplication for FIFO queues. For more information, see the [related documentation](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing)
�
deduplicationScopeB" �Specifies whether message deduplication occurs at the message group or queue level. Valid values are `messageGroup` and `queue` (default).
�
delaySecondsB �The time in seconds that the delivery of all messages in the queue will be delayed. An integer from 0 to 900 (15 minutes). The default for this attribute is 0 seconds.
l
	fifoQueueB
 YBoolean designating a FIFO queue. If not set, it defaults to `false` making it standard.
�
fifoThroughputLimitB" �Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are `perQueue` (default) and `perMessageGroupId`.
�
kmsDataKeyReusePeriodSecondsB �The length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). The default is 300 (5 minutes).
�
kmsMasterKeyIdB" �The ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see [Key Terms](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms).
�
maxMessageSizeB �The limit of how many bytes a message can contain before Amazon SQS rejects it. An integer from 1024 bytes (1 KiB) up to 262144 bytes (256 KiB). The default for this attribute is 262144 (256 KiB).
�
messageRetentionSecondsB �The number of seconds Amazon SQS retains a message. Integer representing seconds, from 60 (1 minute) to 1209600 (14 days). The default for this attribute is 345600 (4 days).
�
nameB" �The name of the queue. Queue names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 80 characters long. For a FIFO (first-in-first-out) queue, the name must end with the `.fifo` suffix. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`
e

namePrefixB" QCreates a unique name beginning with the specified prefix. Conflicts with `name`
3
policyB" #The JSON policy for the SQS queue.
�
receiveWaitTimeSecondsB �The time for which a ReceiveMessage call will wait for a message to arrive (long polling) before returning. An integer from 0 to 20 (seconds). The default for this attribute is 0, meaning that the call will return immediately.
�
redriveAllowPolicyB" �The JSON policy to set up the Dead Letter Queue redrive permission, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html).
�
redrivePolicyB" �The JSON policy to set up the Dead Letter Queue, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html). **Note:** when specifying `maxReceiveCount`, you must specify it as an integer (`5`), and not a string (`"5"`).
�
sqsManagedSseEnabledB
 �Boolean to enable server-side encryption (SSE) of message content with SQS-owned encryption keys. See [Encryption at rest](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html). The provider will only perform drift detection of its value when present in a configuration.
�
tagsB2" �A map of tags to assign to the queue. If configured with a provider `default_tags` configuration block) present, tags with matching keys will overwrite those defined at the provider-level.
�
visibilityTimeoutSecondsB �The visibility timeout for the queue. An integer from 0 to 43200 (12 hours). The default for this attribute is 30. For more information about visibility timeout, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AboutVT.html).
"$
arn" The ARN of the SQS queue
"�
contentBasedDeduplicationB
 �Enables content-based deduplication for FIFO queues. For more information, see the [related documentation](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing)
"�
deduplicationScope" �Specifies whether message deduplication occurs at the message group or queue level. Valid values are `messageGroup` and `queue` (default).
"�
delaySecondsB �The time in seconds that the delivery of all messages in the queue will be delayed. An integer from 0 to 900 (15 minutes). The default for this attribute is 0 seconds.
"l
	fifoQueueB
 YBoolean designating a FIFO queue. If not set, it defaults to `false` making it standard.
"�
fifoThroughputLimit" �Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are `perQueue` (default) and `perMessageGroupId`.
"�
kmsDataKeyReusePeriodSeconds �The length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). The default is 300 (5 minutes).
"�
kmsMasterKeyIdB" �The ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see [Key Terms](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms).
"�
maxMessageSizeB �The limit of how many bytes a message can contain before Amazon SQS rejects it. An integer from 1024 bytes (1 KiB) up to 262144 bytes (256 KiB). The default for this attribute is 262144 (256 KiB).
"�
messageRetentionSecondsB �The number of seconds Amazon SQS retains a message. Integer representing seconds, from 60 (1 minute) to 1209600 (14 days). The default for this attribute is 345600 (4 days).
"�
name" �The name of the queue. Queue names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 80 characters long. For a FIFO (first-in-first-out) queue, the name must end with the `.fifo` suffix. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`
"c

namePrefix" QCreates a unique name beginning with the specified prefix. Conflicts with `name`
"1
policy" #The JSON policy for the SQS queue.
"�
receiveWaitTimeSecondsB �The time for which a ReceiveMessage call will wait for a message to arrive (long polling) before returning. An integer from 0 to 20 (seconds). The default for this attribute is 0, meaning that the call will return immediately.
"�
redriveAllowPolicy" �The JSON policy to set up the Dead Letter Queue redrive permission, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html).
"�
redrivePolicy" �The JSON policy to set up the Dead Letter Queue, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html). **Note:** when specifying `maxReceiveCount`, you must specify it as an integer (`5`), and not a string (`"5"`).
"�
sqsManagedSseEnabled
 �Boolean to enable server-side encryption (SSE) of message content with SQS-owned encryption keys. See [Encryption at rest](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html). The provider will only perform drift detection of its value when present in a configuration.
"�
tagsB2" �A map of tags to assign to the queue. If configured with a provider `default_tags` configuration block) present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"C
url" 8Same as `id`: The URL for the created Amazon SQS queue.
"�
visibilityTimeoutSecondsB �The visibility timeout for the queue. An integer from 0 to 43200 (12 hours). The default for this attribute is 30. For more information about visibility timeout, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AboutVT.html).
*�;
3
sqsQueuePolicyaws:sqs/queuePolicy:QueuePolicy�9Allows you to set a policy of an SQS Queue
while referencing ARN of the queue within the policy.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const q = new aws.sqs.Queue("q", {name: "examplequeue"});
const test = q.arn.apply(arn => aws.iam.getPolicyDocumentOutput({
    statements: [{
        sid: "First",
        effect: "Allow",
        principals: [{
            type: "*",
            identifiers: ["*"],
        }],
        actions: ["sqs:SendMessage"],
        resources: [arn],
        conditions: [{
            test: "ArnEquals",
            variable: "aws:SourceArn",
            values: [example.arn],
        }],
    }],
}));
const testQueuePolicy = new aws.sqs.QueuePolicy("test", {
    queueUrl: q.id,
    policy: test.apply(test => test.json),
});
```
```python
import pulumi
import pulumi_aws as aws

q = aws.sqs.Queue("q", name="examplequeue")
test = q.arn.apply(lambda arn: aws.iam.get_policy_document_output(statements=[{
    "sid": "First",
    "effect": "Allow",
    "principals": [{
        "type": "*",
        "identifiers": ["*"],
    }],
    "actions": ["sqs:SendMessage"],
    "resources": [arn],
    "conditions": [{
        "test": "ArnEquals",
        "variable": "aws:SourceArn",
        "values": [example["arn"]],
    }],
}]))
test_queue_policy = aws.sqs.QueuePolicy("test",
    queue_url=q.id,
    policy=test.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var q = new Aws.Sqs.Queue("q", new()
    {
        Name = "examplequeue",
    });

    var test = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "First",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "*",
                        Identifiers = new[]
                        {
                            "*",
                        },
                    },
                },
                Actions = new[]
                {
                    "sqs:SendMessage",
                },
                Resources = new[]
                {
                    q.Arn,
                },
                Conditions = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementConditionInputArgs
                    {
                        Test = "ArnEquals",
                        Variable = "aws:SourceArn",
                        Values = new[]
                        {
                            example.Arn,
                        },
                    },
                },
            },
        },
    });

    var testQueuePolicy = new Aws.Sqs.QueuePolicy("test", new()
    {
        QueueUrl = q.Id,
        Policy = test.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
q, err := sqs.NewQueue(ctx, "q", &sqs.QueueArgs{
Name: pulumi.String("examplequeue"),
})
if err != nil {
return err
}
test := q.Arn.ApplyT(func(arn string) (iam.GetPolicyDocumentResult, error) {
return iam.GetPolicyDocumentResult(interface{}(iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Sid: "First",
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "*",
Identifiers: []string{
"*",
},
},
},
Actions: []string{
"sqs:SendMessage",
},
Resources: []string{
arn,
},
Conditions: []iam.GetPolicyDocumentStatementCondition{
{
Test: "ArnEquals",
Variable: "aws:SourceArn",
Values: interface{}{
example.Arn,
},
},
},
},
},
}, nil))), nil
}).(iam.GetPolicyDocumentResultOutput)
_, err = sqs.NewQueuePolicy(ctx, "test", &sqs.QueuePolicyArgs{
QueueUrl: q.ID(),
Policy: pulumi.String(test.ApplyT(func(test iam.GetPolicyDocumentResult) (*string, error) {
return &test.Json, nil
}).(pulumi.StringPtrOutput)),
})
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.sqs.QueuePolicy;
import com.pulumi.aws.sqs.QueuePolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var q = new Queue("q", QueueArgs.builder()
            .name("examplequeue")
            .build());

        final var test = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("First")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("*")
                    .identifiers("*")
                    .build())
                .actions("sqs:SendMessage")
                .resources(q.arn())
                .conditions(GetPolicyDocumentStatementConditionArgs.builder()
                    .test("ArnEquals")
                    .variable("aws:SourceArn")
                    .values(example.arn())
                    .build())
                .build())
            .build());

        var testQueuePolicy = new QueuePolicy("testQueuePolicy", QueuePolicyArgs.builder()
            .queueUrl(q.id())
            .policy(test.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(test -> test.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  q:
    type: aws:sqs:Queue
    properties:
      name: examplequeue
  testQueuePolicy:
    type: aws:sqs:QueuePolicy
    name: test
    properties:
      queueUrl: ${q.id}
      policy: ${test.json}
variables:
  test:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: First
            effect: Allow
            principals:
              - type: '*'
                identifiers:
                  - '*'
            actions:
              - sqs:SendMessage
            resources:
              - ${q.arn}
            conditions:
              - test: ArnEquals
                variable: aws:SourceArn
                values:
                  - ${example.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SQS Queue Policies using the queue URL. For example:

```sh
$ pulumi import aws:sqs/queuePolicy:QueuePolicy test https://queue.amazonaws.com/123456789012/myqueue
```
1
policy" #The JSON policy for the SQS queue.
G
queueUrl" 7The URL of the SQS Queue to which to attach the policy
"1
policy" #The JSON policy for the SQS queue.
"G
queueUrl" 7The URL of the SQS Queue to which to attach the policy
*�4
H
sqsRedriveAllowPolicy-aws:sqs/redriveAllowPolicy:RedriveAllowPolicy�.Provides a SQS Queue Redrive Allow Policy resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sqs.Queue("example", {name: "examplequeue"});
const src = new aws.sqs.Queue("src", {
    name: "srcqueue",
    redrivePolicy: pulumi.jsonStringify({
        deadLetterTargetArn: example.arn,
        maxReceiveCount: 4,
    }),
});
const exampleRedriveAllowPolicy = new aws.sqs.RedriveAllowPolicy("example", {
    queueUrl: example.id,
    redriveAllowPolicy: pulumi.jsonStringify({
        redrivePermission: "byQueue",
        sourceQueueArns: [src.arn],
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.sqs.Queue("example", name="examplequeue")
src = aws.sqs.Queue("src",
    name="srcqueue",
    redrive_policy=pulumi.Output.json_dumps({
        "deadLetterTargetArn": example.arn,
        "maxReceiveCount": 4,
    }))
example_redrive_allow_policy = aws.sqs.RedriveAllowPolicy("example",
    queue_url=example.id,
    redrive_allow_policy=pulumi.Output.json_dumps({
        "redrivePermission": "byQueue",
        "sourceQueueArns": [src.arn],
    }))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Sqs.Queue("example", new()
    {
        Name = "examplequeue",
    });

    var src = new Aws.Sqs.Queue("src", new()
    {
        Name = "srcqueue",
        RedrivePolicy = Output.JsonSerialize(Output.Create(new Dictionary<string, object?>
        {
            ["deadLetterTargetArn"] = example.Arn,
            ["maxReceiveCount"] = 4,
        })),
    });

    var exampleRedriveAllowPolicy = new Aws.Sqs.RedriveAllowPolicy("example", new()
    {
        QueueUrl = example.Id,
        RedriveAllowPolicyName = Output.JsonSerialize(Output.Create(new Dictionary<string, object?>
        {
            ["redrivePermission"] = "byQueue",
            ["sourceQueueArns"] = new[]
            {
                src.Arn,
            },
        })),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sqs.NewQueue(ctx, "example", &sqs.QueueArgs{
			Name: pulumi.String("examplequeue"),
		})
		if err != nil {
			return err
		}
		src, err := sqs.NewQueue(ctx, "src", &sqs.QueueArgs{
			Name: pulumi.String("srcqueue"),
			RedrivePolicy: example.Arn.ApplyT(func(arn string) (pulumi.String, error) {
				var _zero pulumi.String
				tmpJSON0, err := json.Marshal(map[string]interface{}{
					"deadLetterTargetArn": arn,
					"maxReceiveCount":     4,
				})
				if err != nil {
					return _zero, err
				}
				json0 := string(tmpJSON0)
				return pulumi.String(json0), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		_, err = sqs.NewRedriveAllowPolicy(ctx, "example", &sqs.RedriveAllowPolicyArgs{
			QueueUrl: example.ID(),
			RedriveAllowPolicy: src.Arn.ApplyT(func(arn string) (pulumi.String, error) {
				var _zero pulumi.String
				tmpJSON1, err := json.Marshal(map[string]interface{}{
					"redrivePermission": "byQueue",
					"sourceQueueArns": []string{
						arn,
					},
				})
				if err != nil {
					return _zero, err
				}
				json1 := string(tmpJSON1)
				return pulumi.String(json1), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import com.pulumi.aws.sqs.RedriveAllowPolicy;
import com.pulumi.aws.sqs.RedriveAllowPolicyArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Queue("example", QueueArgs.builder()
            .name("examplequeue")
            .build());

        var src = new Queue("src", QueueArgs.builder()
            .name("srcqueue")
            .redrivePolicy(example.arn().applyValue(arn -> serializeJson(
                jsonObject(
                    jsonProperty("deadLetterTargetArn", arn),
                    jsonProperty("maxReceiveCount", 4)
                ))))
            .build());

        var exampleRedriveAllowPolicy = new RedriveAllowPolicy("exampleRedriveAllowPolicy", RedriveAllowPolicyArgs.builder()
            .queueUrl(example.id())
            .redriveAllowPolicy(src.arn().applyValue(arn -> serializeJson(
                jsonObject(
                    jsonProperty("redrivePermission", "byQueue"),
                    jsonProperty("sourceQueueArns", jsonArray(arn))
                ))))
            .build());

    }
}
```
```yaml
resources:
  src:
    type: aws:sqs:Queue
    properties:
      name: srcqueue
      redrivePolicy:
        fn::toJSON:
          deadLetterTargetArn: ${example.arn}
          maxReceiveCount: 4
  example:
    type: aws:sqs:Queue
    properties:
      name: examplequeue
  exampleRedriveAllowPolicy:
    type: aws:sqs:RedriveAllowPolicy
    name: example
    properties:
      queueUrl: ${example.id}
      redriveAllowPolicy:
        fn::toJSON:
          redrivePermission: byQueue
          sourceQueueArns:
            - ${src.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SQS Queue Redrive Allow Policies using the queue URL. For example:

```sh
$ pulumi import aws:sqs/redriveAllowPolicy:RedriveAllowPolicy test https://queue.amazonaws.com/123456789012/myqueue
```
G
queueUrl" 7The URL of the SQS Queue to which to attach the policy
�
redriveAllowPolicy" �The JSON redrive allow policy for the SQS queue. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
"G
queueUrl" 7The URL of the SQS Queue to which to attach the policy
"�
redriveAllowPolicy" �The JSON redrive allow policy for the SQS queue. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
*�5
9
sqsRedrivePolicy#aws:sqs/redrivePolicy:RedrivePolicy�.Allows you to set a redrive policy of an SQS Queue
while referencing ARN of the dead letter queue inside the redrive policy.

This is useful when you want to set a dedicated
dead letter queue for a standard or FIFO queue, but need
the dead letter queue to exist before setting the redrive policy.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const q = new aws.sqs.Queue("q", {name: "examplequeue"});
const ddl = new aws.sqs.Queue("ddl", {
    name: "examplequeue-ddl",
    redriveAllowPolicy: pulumi.jsonStringify({
        redrivePermission: "byQueue",
        sourceQueueArns: [q.arn],
    }),
});
const qRedrivePolicy = new aws.sqs.RedrivePolicy("q", {
    queueUrl: q.id,
    redrivePolicy: pulumi.jsonStringify({
        deadLetterTargetArn: ddl.arn,
        maxReceiveCount: 4,
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

q = aws.sqs.Queue("q", name="examplequeue")
ddl = aws.sqs.Queue("ddl",
    name="examplequeue-ddl",
    redrive_allow_policy=pulumi.Output.json_dumps({
        "redrivePermission": "byQueue",
        "sourceQueueArns": [q.arn],
    }))
q_redrive_policy = aws.sqs.RedrivePolicy("q",
    queue_url=q.id,
    redrive_policy=pulumi.Output.json_dumps({
        "deadLetterTargetArn": ddl.arn,
        "maxReceiveCount": 4,
    }))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var q = new Aws.Sqs.Queue("q", new()
    {
        Name = "examplequeue",
    });

    var ddl = new Aws.Sqs.Queue("ddl", new()
    {
        Name = "examplequeue-ddl",
        RedriveAllowPolicy = Output.JsonSerialize(Output.Create(new Dictionary<string, object?>
        {
            ["redrivePermission"] = "byQueue",
            ["sourceQueueArns"] = new[]
            {
                q.Arn,
            },
        })),
    });

    var qRedrivePolicy = new Aws.Sqs.RedrivePolicy("q", new()
    {
        QueueUrl = q.Id,
        RedrivePolicyName = Output.JsonSerialize(Output.Create(new Dictionary<string, object?>
        {
            ["deadLetterTargetArn"] = ddl.Arn,
            ["maxReceiveCount"] = 4,
        })),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		q, err := sqs.NewQueue(ctx, "q", &sqs.QueueArgs{
			Name: pulumi.String("examplequeue"),
		})
		if err != nil {
			return err
		}
		ddl, err := sqs.NewQueue(ctx, "ddl", &sqs.QueueArgs{
			Name: pulumi.String("examplequeue-ddl"),
			RedriveAllowPolicy: q.Arn.ApplyT(func(arn string) (pulumi.String, error) {
				var _zero pulumi.String
				tmpJSON0, err := json.Marshal(map[string]interface{}{
					"redrivePermission": "byQueue",
					"sourceQueueArns": []string{
						arn,
					},
				})
				if err != nil {
					return _zero, err
				}
				json0 := string(tmpJSON0)
				return pulumi.String(json0), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		_, err = sqs.NewRedrivePolicy(ctx, "q", &sqs.RedrivePolicyArgs{
			QueueUrl: q.ID(),
			RedrivePolicy: ddl.Arn.ApplyT(func(arn string) (pulumi.String, error) {
				var _zero pulumi.String
				tmpJSON1, err := json.Marshal(map[string]interface{}{
					"deadLetterTargetArn": arn,
					"maxReceiveCount":     4,
				})
				if err != nil {
					return _zero, err
				}
				json1 := string(tmpJSON1)
				return pulumi.String(json1), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.Queue;
import com.pulumi.aws.sqs.QueueArgs;
import com.pulumi.aws.sqs.RedrivePolicy;
import com.pulumi.aws.sqs.RedrivePolicyArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var q = new Queue("q", QueueArgs.builder()
            .name("examplequeue")
            .build());

        var ddl = new Queue("ddl", QueueArgs.builder()
            .name("examplequeue-ddl")
            .redriveAllowPolicy(q.arn().applyValue(arn -> serializeJson(
                jsonObject(
                    jsonProperty("redrivePermission", "byQueue"),
                    jsonProperty("sourceQueueArns", jsonArray(arn))
                ))))
            .build());

        var qRedrivePolicy = new RedrivePolicy("qRedrivePolicy", RedrivePolicyArgs.builder()
            .queueUrl(q.id())
            .redrivePolicy(ddl.arn().applyValue(arn -> serializeJson(
                jsonObject(
                    jsonProperty("deadLetterTargetArn", arn),
                    jsonProperty("maxReceiveCount", 4)
                ))))
            .build());

    }
}
```
```yaml
resources:
  q:
    type: aws:sqs:Queue
    properties:
      name: examplequeue
  ddl:
    type: aws:sqs:Queue
    properties:
      name: examplequeue-ddl
      redriveAllowPolicy:
        fn::toJSON:
          redrivePermission: byQueue
          sourceQueueArns:
            - ${q.arn}
  qRedrivePolicy:
    type: aws:sqs:RedrivePolicy
    name: q
    properties:
      queueUrl: ${q.id}
      redrivePolicy:
        fn::toJSON:
          deadLetterTargetArn: ${ddl.arn}
          maxReceiveCount: 4
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SQS Queue Redrive Policies using the queue URL. For example:

```sh
$ pulumi import aws:sqs/redrivePolicy:RedrivePolicy test https://queue.amazonaws.com/123456789012/myqueue
```
G
queueUrl" 7The URL of the SQS Queue to which to attach the policy
�
redrivePolicy" �The JSON redrive policy for the SQS queue. Accepts two key/val pairs: `deadLetterTargetArn` and `maxReceiveCount`. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
"G
queueUrl" 7The URL of the SQS Queue to which to attach the policy
"�
redrivePolicy" �The JSON redrive policy for the SQS queue. Accepts two key/val pairs: `deadLetterTargetArn` and `maxReceiveCount`. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
*�P
0
ssm
Activationaws:ssm/activation:Activation�?Registers an on-premises server or virtual machine with Amazon EC2 so that it can be managed using Run Command.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["ssm.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const testRole = new aws.iam.Role("test_role", {
    name: "test_role",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const testAttach = new aws.iam.RolePolicyAttachment("test_attach", {
    role: testRole.name,
    policyArn: "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore",
});
const foo = new aws.ssm.Activation("foo", {
    name: "test_ssm_activation",
    description: "Test",
    iamRole: testRole.id,
    registrationLimit: 5,
}, {
    dependsOn: [testAttach],
});
```
```python
import pulumi
import pulumi_aws as aws

assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["ssm.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
test_role = aws.iam.Role("test_role",
    name="test_role",
    assume_role_policy=assume_role.json)
test_attach = aws.iam.RolePolicyAttachment("test_attach",
    role=test_role.name,
    policy_arn="arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore")
foo = aws.ssm.Activation("foo",
    name="test_ssm_activation",
    description="Test",
    iam_role=test_role.id,
    registration_limit=5,
    opts = pulumi.ResourceOptions(depends_on=[test_attach]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var assumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "ssm.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
            },
        },
    });

    var testRole = new Aws.Iam.Role("test_role", new()
    {
        Name = "test_role",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var testAttach = new Aws.Iam.RolePolicyAttachment("test_attach", new()
    {
        Role = testRole.Name,
        PolicyArn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore",
    });

    var foo = new Aws.Ssm.Activation("foo", new()
    {
        Name = "test_ssm_activation",
        Description = "Test",
        IamRole = testRole.Id,
        RegistrationLimit = 5,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            testAttach,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"ssm.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"sts:AssumeRole",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		testRole, err := iam.NewRole(ctx, "test_role", &iam.RoleArgs{
			Name:             pulumi.String("test_role"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		testAttach, err := iam.NewRolePolicyAttachment(ctx, "test_attach", &iam.RolePolicyAttachmentArgs{
			Role:      testRole.Name,
			PolicyArn: pulumi.String("arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"),
		})
		if err != nil {
			return err
		}
		_, err = ssm.NewActivation(ctx, "foo", &ssm.ActivationArgs{
			Name:              pulumi.String("test_ssm_activation"),
			Description:       pulumi.String("Test"),
			IamRole:           testRole.ID(),
			RegistrationLimit: pulumi.Int(5),
		}, pulumi.DependsOn([]pulumi.Resource{
			testAttach,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.iam.RolePolicyAttachment;
import com.pulumi.aws.iam.RolePolicyAttachmentArgs;
import com.pulumi.aws.ssm.Activation;
import com.pulumi.aws.ssm.ActivationArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("ssm.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var testRole = new Role("testRole", RoleArgs.builder()
            .name("test_role")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var testAttach = new RolePolicyAttachment("testAttach", RolePolicyAttachmentArgs.builder()
            .role(testRole.name())
            .policyArn("arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore")
            .build());

        var foo = new Activation("foo", ActivationArgs.builder()
            .name("test_ssm_activation")
            .description("Test")
            .iamRole(testRole.id())
            .registrationLimit("5")
            .build(), CustomResourceOptions.builder()
                .dependsOn(testAttach)
                .build());

    }
}
```
```yaml
resources:
  testRole:
    type: aws:iam:Role
    name: test_role
    properties:
      name: test_role
      assumeRolePolicy: ${assumeRole.json}
  testAttach:
    type: aws:iam:RolePolicyAttachment
    name: test_attach
    properties:
      role: ${testRole.name}
      policyArn: arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore
  foo:
    type: aws:ssm:Activation
    properties:
      name: test_ssm_activation
      description: Test
      iamRole: ${testRole.id}
      registrationLimit: '5'
    options:
      dependsOn:
        - ${testAttach}
variables:
  assumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - ssm.amazonaws.com
            actions:
              - sts:AssumeRole
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AWS SSM Activation using the `id`. For example:

```sh
$ pulumi import aws:ssm/activation:Activation example e488f2f6-e686-4afb-8a04-ef6dfEXAMPLE
```
-> __Note:__ The `activation_code` attribute cannot be imported.

P
descriptionB" ;The description of the resource that you want to register.
�
expirationDateB" �UTC timestamp in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) by which this activation request should expire. The default value is 24 hours from resource creation time. This provider will only perform drift detection of its value when present in a configuration.
?
iamRole" 0The IAM Role to attach to the managed instance.
C
nameB" 5The default name of the registered managed instance.
z
registrationLimitB _The maximum number of managed instances you want to register. The default value is 1 instance.
�
tagsB2" �A map of tags to assign to the object. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"V
activationCode" @The code the system generates when it processes the activation.
"P
descriptionB" ;The description of the resource that you want to register.
"�
expirationDate" �UTC timestamp in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) by which this activation request should expire. The default value is 24 hours from resource creation time. This provider will only perform drift detection of its value when present in a configuration.
"6
expired
 'If the current activation has expired.
"?
iamRole" 0The IAM Role to attach to the managed instance.
"A
name" 5The default name of the registered managed instance.
"n
registrationCount UThe number of managed instances that are currently registered using this activation.
"z
registrationLimitB _The maximum number of managed instances you want to register. The default value is 1 instance.
"�
tagsB2" �A map of tags to assign to the object. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*��
3
ssmAssociationaws:ssm/association:Association�`Associates an SSM Document to an instance or EC2 tag.

## Example Usage

### Create an association for a specific instance

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.Association("example", {
    name: exampleAwsSsmDocument.name,
    targets: [{
        key: "InstanceIds",
        values: [exampleAwsInstance.id],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.Association("example",
    name=example_aws_ssm_document["name"],
    targets=[{
        "key": "InstanceIds",
        "values": [example_aws_instance["id"]],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.Association("example", new()
    {
        Name = exampleAwsSsmDocument.Name,
        Targets = new[]
        {
            new Aws.Ssm.Inputs.AssociationTargetArgs
            {
                Key = "InstanceIds",
                Values = new[]
                {
                    exampleAwsInstance.Id,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewAssociation(ctx, "example", &ssm.AssociationArgs{
			Name: pulumi.Any(exampleAwsSsmDocument.Name),
			Targets: ssm.AssociationTargetArray{
				&ssm.AssociationTargetArgs{
					Key: pulumi.String("InstanceIds"),
					Values: pulumi.StringArray{
						exampleAwsInstance.Id,
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.Association;
import com.pulumi.aws.ssm.AssociationArgs;
import com.pulumi.aws.ssm.inputs.AssociationTargetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Association("example", AssociationArgs.builder()
            .name(exampleAwsSsmDocument.name())
            .targets(AssociationTargetArgs.builder()
                .key("InstanceIds")
                .values(exampleAwsInstance.id())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:Association
    properties:
      name: ${exampleAwsSsmDocument.name}
      targets:
        - key: InstanceIds
          values:
            - ${exampleAwsInstance.id}
```
<!--End PulumiCodeChooser -->

### Create an association for all managed instances in an AWS account

To target all managed instances in an AWS account, set the `key` as `"InstanceIds"` with `values` set as `["*"]`. This example also illustrates how to use an Amazon owned SSM document named `AmazonCloudWatch-ManageAgent`.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.Association("example", {
    name: "AmazonCloudWatch-ManageAgent",
    targets: [{
        key: "InstanceIds",
        values: ["*"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.Association("example",
    name="AmazonCloudWatch-ManageAgent",
    targets=[{
        "key": "InstanceIds",
        "values": ["*"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.Association("example", new()
    {
        Name = "AmazonCloudWatch-ManageAgent",
        Targets = new[]
        {
            new Aws.Ssm.Inputs.AssociationTargetArgs
            {
                Key = "InstanceIds",
                Values = new[]
                {
                    "*",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewAssociation(ctx, "example", &ssm.AssociationArgs{
			Name: pulumi.String("AmazonCloudWatch-ManageAgent"),
			Targets: ssm.AssociationTargetArray{
				&ssm.AssociationTargetArgs{
					Key: pulumi.String("InstanceIds"),
					Values: pulumi.StringArray{
						pulumi.String("*"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.Association;
import com.pulumi.aws.ssm.AssociationArgs;
import com.pulumi.aws.ssm.inputs.AssociationTargetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Association("example", AssociationArgs.builder()
            .name("AmazonCloudWatch-ManageAgent")
            .targets(AssociationTargetArgs.builder()
                .key("InstanceIds")
                .values("*")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:Association
    properties:
      name: AmazonCloudWatch-ManageAgent
      targets:
        - key: InstanceIds
          values:
            - '*'
```
<!--End PulumiCodeChooser -->

### Create an association for a specific tag

This example shows how to target all managed instances that are assigned a tag key of `Environment` and value of `Development`.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.Association("example", {
    name: "AmazonCloudWatch-ManageAgent",
    targets: [{
        key: "tag:Environment",
        values: ["Development"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.Association("example",
    name="AmazonCloudWatch-ManageAgent",
    targets=[{
        "key": "tag:Environment",
        "values": ["Development"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.Association("example", new()
    {
        Name = "AmazonCloudWatch-ManageAgent",
        Targets = new[]
        {
            new Aws.Ssm.Inputs.AssociationTargetArgs
            {
                Key = "tag:Environment",
                Values = new[]
                {
                    "Development",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewAssociation(ctx, "example", &ssm.AssociationArgs{
			Name: pulumi.String("AmazonCloudWatch-ManageAgent"),
			Targets: ssm.AssociationTargetArray{
				&ssm.AssociationTargetArgs{
					Key: pulumi.String("tag:Environment"),
					Values: pulumi.StringArray{
						pulumi.String("Development"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.Association;
import com.pulumi.aws.ssm.AssociationArgs;
import com.pulumi.aws.ssm.inputs.AssociationTargetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Association("example", AssociationArgs.builder()
            .name("AmazonCloudWatch-ManageAgent")
            .targets(AssociationTargetArgs.builder()
                .key("tag:Environment")
                .values("Development")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:Association
    properties:
      name: AmazonCloudWatch-ManageAgent
      targets:
        - key: tag:Environment
          values:
            - Development
```
<!--End PulumiCodeChooser -->

### Create an association with a specific schedule

This example shows how to schedule an association in various ways.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.Association("example", {
    name: exampleAwsSsmDocument.name,
    scheduleExpression: "cron(0 2 ? * SUN *)",
    targets: [{
        key: "InstanceIds",
        values: [exampleAwsInstance.id],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.Association("example",
    name=example_aws_ssm_document["name"],
    schedule_expression="cron(0 2 ? * SUN *)",
    targets=[{
        "key": "InstanceIds",
        "values": [example_aws_instance["id"]],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.Association("example", new()
    {
        Name = exampleAwsSsmDocument.Name,
        ScheduleExpression = "cron(0 2 ? * SUN *)",
        Targets = new[]
        {
            new Aws.Ssm.Inputs.AssociationTargetArgs
            {
                Key = "InstanceIds",
                Values = new[]
                {
                    exampleAwsInstance.Id,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewAssociation(ctx, "example", &ssm.AssociationArgs{
			Name:               pulumi.Any(exampleAwsSsmDocument.Name),
			ScheduleExpression: pulumi.String("cron(0 2 ? * SUN *)"),
			Targets: ssm.AssociationTargetArray{
				&ssm.AssociationTargetArgs{
					Key: pulumi.String("InstanceIds"),
					Values: pulumi.StringArray{
						exampleAwsInstance.Id,
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.Association;
import com.pulumi.aws.ssm.AssociationArgs;
import com.pulumi.aws.ssm.inputs.AssociationTargetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Association("example", AssociationArgs.builder()
            .name(exampleAwsSsmDocument.name())
            .scheduleExpression("cron(0 2 ? * SUN *)")
            .targets(AssociationTargetArgs.builder()
                .key("InstanceIds")
                .values(exampleAwsInstance.id())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:Association
    properties:
      name: ${exampleAwsSsmDocument.name}
      scheduleExpression: cron(0 2 ? * SUN *)
      targets:
        - key: InstanceIds
          values:
            - ${exampleAwsInstance.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM associations using the `association_id`. For example:

```sh
$ pulumi import aws:ssm/association:Association test-association 10abcdef-0abc-1234-5678-90abcdef123456
```
�
applyOnlyAtCronIntervalB
 �By default, when you create a new or update associations, the system runs it immediately and then according to the schedule you specified. Enable this option if you do not want an association to run immediately after you create or update it. This parameter is not supported for rate expressions. Default: `false`.
C
associationNameB" *The descriptive name for the association.
�
automationTargetParameterNameB" �Specify the target for the association. This target is required for associations that use an `Automation` document and target resources by using rate controls. This should be set to the SSM document `parameter` that will define how your automation will branch out.
�
complianceSeverityB" The compliance severity for the association. Can be one of the following: `UNSPECIFIED`, `LOW`, `MEDIUM`, `HIGH` or `CRITICAL`
�
documentVersionB" qThe document version you want to associate with the target(s). Can be a specific version or the default version.
�

instanceIdB" �The instance ID to apply an SSM document to. Use `targets` with key `InstanceIds` for document schema versions 2.0 and above. Use the `targets` attribute instead.
�
maxConcurrencyB" �The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%.
�
	maxErrorsB" �The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify a number, for example 10, or a percentage of the target set, for example 10%. If you specify a threshold of 3, the stop command is sent when the fourth error is returned. If you specify a threshold of 10% for 50 associations, the stop command is sent when the sixth error is returned.
5
nameB" 'The name of the SSM document to apply.
�
outputLocationcBa:_
]
ssmAssociationOutputLocation;aws:ssm/AssociationOutputLocation:AssociationOutputLocation?An output location block. Output Location is documented below.
Z

parametersB2" DA block of arbitrary string parameters to pass to the SSM document.
�
scheduleExpressionB" �A [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html) that specifies when the association runs.
l
syncComplianceB" TThe mode for generating association compliance. You can specify `AUTO` or `MANUAL`.
�
tagsB2" �A map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
targetsMBK*I:G
E
ssmAssociationTarget+aws:ssm/AssociationTarget:AssociationTarget�A block containing the targets of the SSM association. Targets are documented below. AWS currently supports a maximum of 5 targets.
�
waitForSuccessTimeoutSecondsB �The number of seconds to wait for the association status to be `Success`. If `Success` status is not reached within the given time, create opration will fail.

Output Location (`output_location`) is an S3 bucket where you want to store the results of this association:
"�
applyOnlyAtCronIntervalB
 �By default, when you create a new or update associations, the system runs it immediately and then according to the schedule you specified. Enable this option if you do not want an association to run immediately after you create or update it. This parameter is not supported for rate expressions. Default: `false`.
"*
arn" The ARN of the SSM association
"4
associationId" The ID of the SSM association.
"C
associationNameB" *The descriptive name for the association.
"�
automationTargetParameterNameB" �Specify the target for the association. This target is required for associations that use an `Automation` document and target resources by using rate controls. This should be set to the SSM document `parameter` that will define how your automation will branch out.
"�
complianceSeverityB" The compliance severity for the association. Can be one of the following: `UNSPECIFIED`, `LOW`, `MEDIUM`, `HIGH` or `CRITICAL`
"�
documentVersion" qThe document version you want to associate with the target(s). Can be a specific version or the default version.
"�

instanceIdB" �The instance ID to apply an SSM document to. Use `targets` with key `InstanceIds` for document schema versions 2.0 and above. Use the `targets` attribute instead.
"�
maxConcurrencyB" �The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%.
"�
	maxErrorsB" �The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify a number, for example 10, or a percentage of the target set, for example 10%. If you specify a threshold of 3, the stop command is sent when the fourth error is returned. If you specify a threshold of 10% for 50 associations, the stop command is sent when the sixth error is returned.
"3
name" 'The name of the SSM document to apply.
"�
outputLocationcBa:_
]
ssmAssociationOutputLocation;aws:ssm/AssociationOutputLocation:AssociationOutputLocation?An output location block. Output Location is documented below.
"X

parameters2" DA block of arbitrary string parameters to pass to the SSM document.
"�
scheduleExpressionB" �A [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html) that specifies when the association runs.
"l
syncComplianceB" TThe mode for generating association compliance. You can specify `AUTO` or `MANUAL`.
"�
tagsB2" �A map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
targetsK*I:G
E
ssmAssociationTarget+aws:ssm/AssociationTarget:AssociationTarget�A block containing the targets of the SSM association. Targets are documented below. AWS currently supports a maximum of 5 targets.
"�
waitForSuccessTimeoutSecondsB �The number of seconds to wait for the association status to be `Success`. If `Success` status is not reached within the given time, create opration will fail.

Output Location (`output_location`) is an S3 bucket where you want to store the results of this association:
*��
B
ssmContactsRotation)aws:ssm/contactsRotation:ContactsRotation�## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.ContactsRotation("example", {
    contactIds: [exampleAwsSsmcontactsContact.arn],
    name: "rotation",
    recurrence: {
        numberOfOnCalls: 1,
        recurrenceMultiplier: 1,
        dailySettings: [{
            hourOfDay: 9,
            minuteOfHour: 0,
        }],
    },
    timeZoneId: "Australia/Sydney",
}, {
    dependsOn: [exampleAwsSsmincidentsReplicationSet],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.ContactsRotation("example",
    contact_ids=[example_aws_ssmcontacts_contact["arn"]],
    name="rotation",
    recurrence={
        "number_of_on_calls": 1,
        "recurrence_multiplier": 1,
        "daily_settings": [{
            "hour_of_day": 9,
            "minute_of_hour": 0,
        }],
    },
    time_zone_id="Australia/Sydney",
    opts = pulumi.ResourceOptions(depends_on=[example_aws_ssmincidents_replication_set]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.ContactsRotation("example", new()
    {
        ContactIds = new[]
        {
            exampleAwsSsmcontactsContact.Arn,
        },
        Name = "rotation",
        Recurrence = new Aws.Ssm.Inputs.ContactsRotationRecurrenceArgs
        {
            NumberOfOnCalls = 1,
            RecurrenceMultiplier = 1,
            DailySettings = new[]
            {
                new Aws.Ssm.Inputs.ContactsRotationRecurrenceDailySettingArgs
                {
                    HourOfDay = 9,
                    MinuteOfHour = 0,
                },
            },
        },
        TimeZoneId = "Australia/Sydney",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSsmincidentsReplicationSet,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewContactsRotation(ctx, "example", &ssm.ContactsRotationArgs{
			ContactIds: pulumi.StringArray{
				exampleAwsSsmcontactsContact.Arn,
			},
			Name: pulumi.String("rotation"),
			Recurrence: &ssm.ContactsRotationRecurrenceArgs{
				NumberOfOnCalls:      pulumi.Int(1),
				RecurrenceMultiplier: pulumi.Int(1),
				DailySettings: ssm.ContactsRotationRecurrenceDailySettingArray{
					&ssm.ContactsRotationRecurrenceDailySettingArgs{
						HourOfDay:    pulumi.Int(9),
						MinuteOfHour: pulumi.Int(0),
					},
				},
			},
			TimeZoneId: pulumi.String("Australia/Sydney"),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSsmincidentsReplicationSet,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.ContactsRotation;
import com.pulumi.aws.ssm.ContactsRotationArgs;
import com.pulumi.aws.ssm.inputs.ContactsRotationRecurrenceArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ContactsRotation("example", ContactsRotationArgs.builder()
            .contactIds(exampleAwsSsmcontactsContact.arn())
            .name("rotation")
            .recurrence(ContactsRotationRecurrenceArgs.builder()
                .numberOfOnCalls(1)
                .recurrenceMultiplier(1)
                .dailySettings(ContactsRotationRecurrenceDailySettingArgs.builder()
                    .hourOfDay(9)
                    .minuteOfHour(0)
                    .build())
                .build())
            .timeZoneId("Australia/Sydney")
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSsmincidentsReplicationSet)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:ContactsRotation
    properties:
      contactIds:
        - ${exampleAwsSsmcontactsContact.arn}
      name: rotation
      recurrence:
        numberOfOnCalls: 1
        recurrenceMultiplier: 1
        dailySettings:
          - hourOfDay: 9
            minuteOfHour: 0
      timeZoneId: Australia/Sydney
    options:
      dependsOn:
        - ${exampleAwsSsmincidentsReplicationSet}
```
<!--End PulumiCodeChooser -->

### Usage with Weekly Settings and Shift Coverages Fields

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.ContactsRotation("example", {
    contactIds: [exampleAwsSsmcontactsContact.arn],
    name: "rotation",
    recurrence: {
        numberOfOnCalls: 1,
        recurrenceMultiplier: 1,
        weeklySettings: [
            {
                dayOfWeek: "WED",
                handOffTime: {
                    hourOfDay: 4,
                    minuteOfHour: 25,
                },
            },
            {
                dayOfWeek: "FRI",
                handOffTime: {
                    hourOfDay: 15,
                    minuteOfHour: 57,
                },
            },
        ],
        shiftCoverages: [{
            mapBlockKey: "MON",
            coverageTimes: [{
                start: {
                    hourOfDay: 1,
                    minuteOfHour: 0,
                },
                end: {
                    hourOfDay: 23,
                    minuteOfHour: 0,
                },
            }],
        }],
    },
    startTime: "2023-07-20T02:21:49+00:00",
    timeZoneId: "Australia/Sydney",
    tags: {
        key1: "tag1",
        key2: "tag2",
    },
}, {
    dependsOn: [exampleAwsSsmincidentsReplicationSet],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.ContactsRotation("example",
    contact_ids=[example_aws_ssmcontacts_contact["arn"]],
    name="rotation",
    recurrence={
        "number_of_on_calls": 1,
        "recurrence_multiplier": 1,
        "weekly_settings": [
            {
                "day_of_week": "WED",
                "hand_off_time": {
                    "hour_of_day": 4,
                    "minute_of_hour": 25,
                },
            },
            {
                "day_of_week": "FRI",
                "hand_off_time": {
                    "hour_of_day": 15,
                    "minute_of_hour": 57,
                },
            },
        ],
        "shift_coverages": [{
            "map_block_key": "MON",
            "coverage_times": [{
                "start": {
                    "hour_of_day": 1,
                    "minute_of_hour": 0,
                },
                "end": {
                    "hour_of_day": 23,
                    "minute_of_hour": 0,
                },
            }],
        }],
    },
    start_time="2023-07-20T02:21:49+00:00",
    time_zone_id="Australia/Sydney",
    tags={
        "key1": "tag1",
        "key2": "tag2",
    },
    opts = pulumi.ResourceOptions(depends_on=[example_aws_ssmincidents_replication_set]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.ContactsRotation("example", new()
    {
        ContactIds = new[]
        {
            exampleAwsSsmcontactsContact.Arn,
        },
        Name = "rotation",
        Recurrence = new Aws.Ssm.Inputs.ContactsRotationRecurrenceArgs
        {
            NumberOfOnCalls = 1,
            RecurrenceMultiplier = 1,
            WeeklySettings = new[]
            {
                new Aws.Ssm.Inputs.ContactsRotationRecurrenceWeeklySettingArgs
                {
                    DayOfWeek = "WED",
                    HandOffTime = new Aws.Ssm.Inputs.ContactsRotationRecurrenceWeeklySettingHandOffTimeArgs
                    {
                        HourOfDay = 4,
                        MinuteOfHour = 25,
                    },
                },
                new Aws.Ssm.Inputs.ContactsRotationRecurrenceWeeklySettingArgs
                {
                    DayOfWeek = "FRI",
                    HandOffTime = new Aws.Ssm.Inputs.ContactsRotationRecurrenceWeeklySettingHandOffTimeArgs
                    {
                        HourOfDay = 15,
                        MinuteOfHour = 57,
                    },
                },
            },
            ShiftCoverages = new[]
            {
                new Aws.Ssm.Inputs.ContactsRotationRecurrenceShiftCoverageArgs
                {
                    MapBlockKey = "MON",
                    CoverageTimes = new[]
                    {
                        new Aws.Ssm.Inputs.ContactsRotationRecurrenceShiftCoverageCoverageTimeArgs
                        {
                            Start = new Aws.Ssm.Inputs.ContactsRotationRecurrenceShiftCoverageCoverageTimeStartArgs
                            {
                                HourOfDay = 1,
                                MinuteOfHour = 0,
                            },
                            End = new Aws.Ssm.Inputs.ContactsRotationRecurrenceShiftCoverageCoverageTimeEndArgs
                            {
                                HourOfDay = 23,
                                MinuteOfHour = 0,
                            },
                        },
                    },
                },
            },
        },
        StartTime = "2023-07-20T02:21:49+00:00",
        TimeZoneId = "Australia/Sydney",
        Tags = 
        {
            { "key1", "tag1" },
            { "key2", "tag2" },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSsmincidentsReplicationSet,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewContactsRotation(ctx, "example", &ssm.ContactsRotationArgs{
			ContactIds: pulumi.StringArray{
				exampleAwsSsmcontactsContact.Arn,
			},
			Name: pulumi.String("rotation"),
			Recurrence: &ssm.ContactsRotationRecurrenceArgs{
				NumberOfOnCalls:      pulumi.Int(1),
				RecurrenceMultiplier: pulumi.Int(1),
				WeeklySettings: ssm.ContactsRotationRecurrenceWeeklySettingArray{
					&ssm.ContactsRotationRecurrenceWeeklySettingArgs{
						DayOfWeek: pulumi.String("WED"),
						HandOffTime: &ssm.ContactsRotationRecurrenceWeeklySettingHandOffTimeArgs{
							HourOfDay:    pulumi.Int(4),
							MinuteOfHour: pulumi.Int(25),
						},
					},
					&ssm.ContactsRotationRecurrenceWeeklySettingArgs{
						DayOfWeek: pulumi.String("FRI"),
						HandOffTime: &ssm.ContactsRotationRecurrenceWeeklySettingHandOffTimeArgs{
							HourOfDay:    pulumi.Int(15),
							MinuteOfHour: pulumi.Int(57),
						},
					},
				},
				ShiftCoverages: ssm.ContactsRotationRecurrenceShiftCoverageArray{
					&ssm.ContactsRotationRecurrenceShiftCoverageArgs{
						MapBlockKey: pulumi.String("MON"),
						CoverageTimes: ssm.ContactsRotationRecurrenceShiftCoverageCoverageTimeArray{
							&ssm.ContactsRotationRecurrenceShiftCoverageCoverageTimeArgs{
								Start: &ssm.ContactsRotationRecurrenceShiftCoverageCoverageTimeStartArgs{
									HourOfDay:    pulumi.Int(1),
									MinuteOfHour: pulumi.Int(0),
								},
								End: &ssm.ContactsRotationRecurrenceShiftCoverageCoverageTimeEndArgs{
									HourOfDay:    pulumi.Int(23),
									MinuteOfHour: pulumi.Int(0),
								},
							},
						},
					},
				},
			},
			StartTime:  pulumi.String("2023-07-20T02:21:49+00:00"),
			TimeZoneId: pulumi.String("Australia/Sydney"),
			Tags: pulumi.StringMap{
				"key1": pulumi.String("tag1"),
				"key2": pulumi.String("tag2"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSsmincidentsReplicationSet,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.ContactsRotation;
import com.pulumi.aws.ssm.ContactsRotationArgs;
import com.pulumi.aws.ssm.inputs.ContactsRotationRecurrenceArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ContactsRotation("example", ContactsRotationArgs.builder()
            .contactIds(exampleAwsSsmcontactsContact.arn())
            .name("rotation")
            .recurrence(ContactsRotationRecurrenceArgs.builder()
                .numberOfOnCalls(1)
                .recurrenceMultiplier(1)
                .weeklySettings(                
                    ContactsRotationRecurrenceWeeklySettingArgs.builder()
                        .dayOfWeek("WED")
                        .handOffTime(ContactsRotationRecurrenceWeeklySettingHandOffTimeArgs.builder()
                            .hourOfDay(4)
                            .minuteOfHour(25)
                            .build())
                        .build(),
                    ContactsRotationRecurrenceWeeklySettingArgs.builder()
                        .dayOfWeek("FRI")
                        .handOffTime(ContactsRotationRecurrenceWeeklySettingHandOffTimeArgs.builder()
                            .hourOfDay(15)
                            .minuteOfHour(57)
                            .build())
                        .build())
                .shiftCoverages(ContactsRotationRecurrenceShiftCoverageArgs.builder()
                    .mapBlockKey("MON")
                    .coverageTimes(ContactsRotationRecurrenceShiftCoverageCoverageTimeArgs.builder()
                        .start(ContactsRotationRecurrenceShiftCoverageCoverageTimeStartArgs.builder()
                            .hourOfDay(1)
                            .minuteOfHour(0)
                            .build())
                        .end(ContactsRotationRecurrenceShiftCoverageCoverageTimeEndArgs.builder()
                            .hourOfDay(23)
                            .minuteOfHour(0)
                            .build())
                        .build())
                    .build())
                .build())
            .startTime("2023-07-20T02:21:49+00:00")
            .timeZoneId("Australia/Sydney")
            .tags(Map.ofEntries(
                Map.entry("key1", "tag1"),
                Map.entry("key2", "tag2")
            ))
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSsmincidentsReplicationSet)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:ContactsRotation
    properties:
      contactIds:
        - ${exampleAwsSsmcontactsContact.arn}
      name: rotation
      recurrence:
        numberOfOnCalls: 1
        recurrenceMultiplier: 1
        weeklySettings:
          - dayOfWeek: WED
            handOffTime:
              hourOfDay: 4
              minuteOfHour: 25
          - dayOfWeek: FRI
            handOffTime:
              hourOfDay: 15
              minuteOfHour: 57
        shiftCoverages:
          - mapBlockKey: MON
            coverageTimes:
              - start:
                  hourOfDay: 1
                  minuteOfHour: 0
                end:
                  hourOfDay: 23
                  minuteOfHour: 0
      startTime: 2023-07-20T02:21:49+00:00
      timeZoneId: Australia/Sydney
      tags:
        key1: tag1
        key2: tag2
    options:
      dependsOn:
        - ${exampleAwsSsmincidentsReplicationSet}
```
<!--End PulumiCodeChooser -->

### Usage with Monthly Settings Fields

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.ContactsRotation("example", {
    contactIds: [exampleAwsSsmcontactsContact.arn],
    name: "rotation",
    recurrence: {
        numberOfOnCalls: 1,
        recurrenceMultiplier: 1,
        monthlySettings: [
            {
                dayOfMonth: 20,
                handOffTime: {
                    hourOfDay: 8,
                    minuteOfHour: 0,
                },
            },
            {
                dayOfMonth: 13,
                handOffTime: {
                    hourOfDay: 12,
                    minuteOfHour: 34,
                },
            },
        ],
    },
    timeZoneId: "Australia/Sydney",
}, {
    dependsOn: [exampleAwsSsmincidentsReplicationSet],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.ContactsRotation("example",
    contact_ids=[example_aws_ssmcontacts_contact["arn"]],
    name="rotation",
    recurrence={
        "number_of_on_calls": 1,
        "recurrence_multiplier": 1,
        "monthly_settings": [
            {
                "day_of_month": 20,
                "hand_off_time": {
                    "hour_of_day": 8,
                    "minute_of_hour": 0,
                },
            },
            {
                "day_of_month": 13,
                "hand_off_time": {
                    "hour_of_day": 12,
                    "minute_of_hour": 34,
                },
            },
        ],
    },
    time_zone_id="Australia/Sydney",
    opts = pulumi.ResourceOptions(depends_on=[example_aws_ssmincidents_replication_set]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.ContactsRotation("example", new()
    {
        ContactIds = new[]
        {
            exampleAwsSsmcontactsContact.Arn,
        },
        Name = "rotation",
        Recurrence = new Aws.Ssm.Inputs.ContactsRotationRecurrenceArgs
        {
            NumberOfOnCalls = 1,
            RecurrenceMultiplier = 1,
            MonthlySettings = new[]
            {
                new Aws.Ssm.Inputs.ContactsRotationRecurrenceMonthlySettingArgs
                {
                    DayOfMonth = 20,
                    HandOffTime = new Aws.Ssm.Inputs.ContactsRotationRecurrenceMonthlySettingHandOffTimeArgs
                    {
                        HourOfDay = 8,
                        MinuteOfHour = 0,
                    },
                },
                new Aws.Ssm.Inputs.ContactsRotationRecurrenceMonthlySettingArgs
                {
                    DayOfMonth = 13,
                    HandOffTime = new Aws.Ssm.Inputs.ContactsRotationRecurrenceMonthlySettingHandOffTimeArgs
                    {
                        HourOfDay = 12,
                        MinuteOfHour = 34,
                    },
                },
            },
        },
        TimeZoneId = "Australia/Sydney",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSsmincidentsReplicationSet,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewContactsRotation(ctx, "example", &ssm.ContactsRotationArgs{
			ContactIds: pulumi.StringArray{
				exampleAwsSsmcontactsContact.Arn,
			},
			Name: pulumi.String("rotation"),
			Recurrence: &ssm.ContactsRotationRecurrenceArgs{
				NumberOfOnCalls:      pulumi.Int(1),
				RecurrenceMultiplier: pulumi.Int(1),
				MonthlySettings: ssm.ContactsRotationRecurrenceMonthlySettingArray{
					&ssm.ContactsRotationRecurrenceMonthlySettingArgs{
						DayOfMonth: pulumi.Int(20),
						HandOffTime: &ssm.ContactsRotationRecurrenceMonthlySettingHandOffTimeArgs{
							HourOfDay:    pulumi.Int(8),
							MinuteOfHour: pulumi.Int(0),
						},
					},
					&ssm.ContactsRotationRecurrenceMonthlySettingArgs{
						DayOfMonth: pulumi.Int(13),
						HandOffTime: &ssm.ContactsRotationRecurrenceMonthlySettingHandOffTimeArgs{
							HourOfDay:    pulumi.Int(12),
							MinuteOfHour: pulumi.Int(34),
						},
					},
				},
			},
			TimeZoneId: pulumi.String("Australia/Sydney"),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSsmincidentsReplicationSet,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.ContactsRotation;
import com.pulumi.aws.ssm.ContactsRotationArgs;
import com.pulumi.aws.ssm.inputs.ContactsRotationRecurrenceArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ContactsRotation("example", ContactsRotationArgs.builder()
            .contactIds(exampleAwsSsmcontactsContact.arn())
            .name("rotation")
            .recurrence(ContactsRotationRecurrenceArgs.builder()
                .numberOfOnCalls(1)
                .recurrenceMultiplier(1)
                .monthlySettings(                
                    ContactsRotationRecurrenceMonthlySettingArgs.builder()
                        .dayOfMonth(20)
                        .handOffTime(ContactsRotationRecurrenceMonthlySettingHandOffTimeArgs.builder()
                            .hourOfDay(8)
                            .minuteOfHour(0)
                            .build())
                        .build(),
                    ContactsRotationRecurrenceMonthlySettingArgs.builder()
                        .dayOfMonth(13)
                        .handOffTime(ContactsRotationRecurrenceMonthlySettingHandOffTimeArgs.builder()
                            .hourOfDay(12)
                            .minuteOfHour(34)
                            .build())
                        .build())
                .build())
            .timeZoneId("Australia/Sydney")
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSsmincidentsReplicationSet)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:ContactsRotation
    properties:
      contactIds:
        - ${exampleAwsSsmcontactsContact.arn}
      name: rotation
      recurrence:
        numberOfOnCalls: 1
        recurrenceMultiplier: 1
        monthlySettings:
          - dayOfMonth: 20
            handOffTime:
              hourOfDay: 8
              minuteOfHour: 0
          - dayOfMonth: 13
            handOffTime:
              hourOfDay: 12
              minuteOfHour: 34
      timeZoneId: Australia/Sydney
    options:
      dependsOn:
        - ${exampleAwsSsmincidentsReplicationSet}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CodeGuru Profiler Profiling Group using the `arn`. For example:

```sh
$ pulumi import aws:ssm/contactsRotation:ContactsRotation example arn:aws:ssm-contacts:us-east-1:012345678910:rotation/example
```
�

contactIds*" �Amazon Resource Names (ARNs) of the contacts to add to the rotation. The order in which you list the contacts is their shift order in the rotation schedule.
)
nameB" The name for the rotation.
�

recurrencefBd:b
`
ssmContactsRotationRecurrence=aws:ssm/ContactsRotationRecurrence:ContactsRotationRecurrence�Information about when an on-call rotation is in effect and how long the rotation period lasts. Exactly one of either `daily_settings`, `monthly_settings`, or `weekly_settings` must be populated. See Recurrence for more details.

The following arguments are optional:
^
	startTimeB" KThe date and time, in RFC 3339 format, that the rotation goes into effect.
�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
{

timeZoneId" iThe time zone to base the rotation’s activity on in Internet Assigned Numbers Authority (IANA) format.
";
arn" 0The Amazon Resource Name (ARN) of the rotation.
"�

contactIds*" �Amazon Resource Names (ARNs) of the contacts to add to the rotation. The order in which you list the contacts is their shift order in the rotation schedule.
"'
name" The name for the rotation.
"�

recurrencefBd:b
`
ssmContactsRotationRecurrence=aws:ssm/ContactsRotationRecurrence:ContactsRotationRecurrence�Information about when an on-call rotation is in effect and how long the rotation period lasts. Exactly one of either `daily_settings`, `monthly_settings`, or `weekly_settings` must be populated. See Recurrence for more details.

The following arguments are optional:
"^
	startTimeB" KThe date and time, in RFC 3339 format, that the rotation goes into effect.
"�
tagsB2" �A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"{

timeZoneId" iThe time zone to base the rotation’s activity on in Internet Assigned Numbers Authority (IANA) format.
*�(
N
ssmDefaultPatchBaseline1aws:ssm/defaultPatchBaseline:DefaultPatchBaseline� Resource for registering an AWS Systems Manager Default Patch Baseline.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const examplePatchBaseline = new aws.ssm.PatchBaseline("example", {
    name: "example",
    approvedPatches: ["KB123456"],
});
const example = new aws.ssm.DefaultPatchBaseline("example", {
    baselineId: examplePatchBaseline.id,
    operatingSystem: examplePatchBaseline.operatingSystem,
});
```
```python
import pulumi
import pulumi_aws as aws

example_patch_baseline = aws.ssm.PatchBaseline("example",
    name="example",
    approved_patches=["KB123456"])
example = aws.ssm.DefaultPatchBaseline("example",
    baseline_id=example_patch_baseline.id,
    operating_system=example_patch_baseline.operating_system)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var examplePatchBaseline = new Aws.Ssm.PatchBaseline("example", new()
    {
        Name = "example",
        ApprovedPatches = new[]
        {
            "KB123456",
        },
    });

    var example = new Aws.Ssm.DefaultPatchBaseline("example", new()
    {
        BaselineId = examplePatchBaseline.Id,
        OperatingSystem = examplePatchBaseline.OperatingSystem,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		examplePatchBaseline, err := ssm.NewPatchBaseline(ctx, "example", &ssm.PatchBaselineArgs{
			Name: pulumi.String("example"),
			ApprovedPatches: pulumi.StringArray{
				pulumi.String("KB123456"),
			},
		})
		if err != nil {
			return err
		}
		_, err = ssm.NewDefaultPatchBaseline(ctx, "example", &ssm.DefaultPatchBaselineArgs{
			BaselineId:      examplePatchBaseline.ID(),
			OperatingSystem: examplePatchBaseline.OperatingSystem,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.PatchBaseline;
import com.pulumi.aws.ssm.PatchBaselineArgs;
import com.pulumi.aws.ssm.DefaultPatchBaseline;
import com.pulumi.aws.ssm.DefaultPatchBaselineArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var examplePatchBaseline = new PatchBaseline("examplePatchBaseline", PatchBaselineArgs.builder()
            .name("example")
            .approvedPatches("KB123456")
            .build());

        var example = new DefaultPatchBaseline("example", DefaultPatchBaselineArgs.builder()
            .baselineId(examplePatchBaseline.id())
            .operatingSystem(examplePatchBaseline.operatingSystem())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:DefaultPatchBaseline
    properties:
      baselineId: ${examplePatchBaseline.id}
      operatingSystem: ${examplePatchBaseline.operatingSystem}
  examplePatchBaseline:
    type: aws:ssm:PatchBaseline
    name: example
    properties:
      name: example
      approvedPatches:
        - KB123456
```
<!--End PulumiCodeChooser -->

## Import

Using the patch baseline ARN:

Using the operating system value:

__Using `pulumi import` to import__ the Systems Manager Default Patch Baseline using the patch baseline ID, patch baseline ARN, or the operating system value. For example:

Using the patch baseline ID:

```sh
$ pulumi import aws:ssm/defaultPatchBaseline:DefaultPatchBaseline example pb-1234567890abcdef1
```
Using the patch baseline ARN:

```sh
$ pulumi import aws:ssm/defaultPatchBaseline:DefaultPatchBaseline example arn:aws:ssm:us-west-2:123456789012:patchbaseline/pb-1234567890abcdef1
```
Using the operating system value:

```sh
$ pulumi import aws:ssm/defaultPatchBaseline:DefaultPatchBaseline example CENTOS
```
�

baselineId" sID of the patch baseline.
Can be an ID or an ARN.
When specifying an AWS-provided patch baseline, must be the ARN.
�
operatingSystem" �The operating system the patch baseline applies to.
Valid values are
`AMAZON_LINUX`,
`AMAZON_LINUX_2`,
`AMAZON_LINUX_2022`,
`CENTOS`,
`DEBIAN`,
`MACOS`,
`ORACLE_LINUX`,
`RASPBIAN`,
`REDHAT_ENTERPRISE_LINUX`,
`ROCKY_LINUX`,
`SUSE`,
`UBUNTU`, and
`WINDOWS`.
"�

baselineId" sID of the patch baseline.
Can be an ID or an ARN.
When specifying an AWS-provided patch baseline, must be the ARN.
"�
operatingSystem" �The operating system the patch baseline applies to.
Valid values are
`AMAZON_LINUX`,
`AMAZON_LINUX_2`,
`AMAZON_LINUX_2022`,
`CENTOS`,
`DEBIAN`,
`MACOS`,
`ORACLE_LINUX`,
`RASPBIAN`,
`REDHAT_ENTERPRISE_LINUX`,
`ROCKY_LINUX`,
`SUSE`,
`UBUNTU`, and
`WINDOWS`.
*�n
*
ssmDocumentaws:ssm/document:Document�GProvides an SSM Document resource

> **NOTE on updating SSM documents:** Only documents with a schema version of 2.0
or greater can update their content once created, see [SSM Schema Features](http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-ssm-docs.html#document-schemas-features). To update a document with an older schema version you must recreate the resource. Not all document types support a schema version of 2.0 or greater. Refer to [SSM document schema features and examples](https://docs.aws.amazon.com/systems-manager/latest/userguide/document-schemas-features.html) for information about which schema versions are supported for the respective `document_type`.

## Example Usage

### Create an ssm document in JSON format

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = new aws.ssm.Document("foo", {
    name: "test_document",
    documentType: "Command",
    content: `  {
    "schemaVersion": "1.2",
    "description": "Check ip configuration of a Linux instance.",
    "parameters": {

    },
    "runtimeConfig": {
      "aws:runShellScript": {
        "properties": [
          {
            "id": "0.aws:runShellScript",
            "runCommand": ["ifconfig"]
          }
        ]
      }
    }
  }
`,
});
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.ssm.Document("foo",
    name="test_document",
    document_type="Command",
    content="""  {
    "schemaVersion": "1.2",
    "description": "Check ip configuration of a Linux instance.",
    "parameters": {

    },
    "runtimeConfig": {
      "aws:runShellScript": {
        "properties": [
          {
            "id": "0.aws:runShellScript",
            "runCommand": ["ifconfig"]
          }
        ]
      }
    }
  }
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var foo = new Aws.Ssm.Document("foo", new()
    {
        Name = "test_document",
        DocumentType = "Command",
        Content = @"  {
    ""schemaVersion"": ""1.2"",
    ""description"": ""Check ip configuration of a Linux instance."",
    ""parameters"": {

    },
    ""runtimeConfig"": {
      ""aws:runShellScript"": {
        ""properties"": [
          {
            ""id"": ""0.aws:runShellScript"",
            ""runCommand"": [""ifconfig""]
          }
        ]
      }
    }
  }
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewDocument(ctx, "foo", &ssm.DocumentArgs{
			Name:         pulumi.String("test_document"),
			DocumentType: pulumi.String("Command"),
			Content: pulumi.String(`  {
    "schemaVersion": "1.2",
    "description": "Check ip configuration of a Linux instance.",
    "parameters": {

    },
    "runtimeConfig": {
      "aws:runShellScript": {
        "properties": [
          {
            "id": "0.aws:runShellScript",
            "runCommand": ["ifconfig"]
          }
        ]
      }
    }
  }
`),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.Document;
import com.pulumi.aws.ssm.DocumentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var foo = new Document("foo", DocumentArgs.builder()
            .name("test_document")
            .documentType("Command")
            .content("""
  {
    "schemaVersion": "1.2",
    "description": "Check ip configuration of a Linux instance.",
    "parameters": {

    },
    "runtimeConfig": {
      "aws:runShellScript": {
        "properties": [
          {
            "id": "0.aws:runShellScript",
            "runCommand": ["ifconfig"]
          }
        ]
      }
    }
  }
            """)
            .build());

    }
}
```
```yaml
resources:
  foo:
    type: aws:ssm:Document
    properties:
      name: test_document
      documentType: Command
      content: |2
          {
            "schemaVersion": "1.2",
            "description": "Check ip configuration of a Linux instance.",
            "parameters": {

            },
            "runtimeConfig": {
              "aws:runShellScript": {
                "properties": [
                  {
                    "id": "0.aws:runShellScript",
                    "runCommand": ["ifconfig"]
                  }
                ]
              }
            }
          }
```
<!--End PulumiCodeChooser -->

### Create an ssm document in YAML format

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = new aws.ssm.Document("foo", {
    name: "test_document",
    documentFormat: "YAML",
    documentType: "Command",
    content: `schemaVersion: '1.2'
description: Check ip configuration of a Linux instance.
parameters: {}
runtimeConfig:
  'aws:runShellScript':
    properties:
      - id: '0.aws:runShellScript'
        runCommand:
          - ifconfig
`,
});
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.ssm.Document("foo",
    name="test_document",
    document_format="YAML",
    document_type="Command",
    content="""schemaVersion: '1.2'
description: Check ip configuration of a Linux instance.
parameters: {}
runtimeConfig:
  'aws:runShellScript':
    properties:
      - id: '0.aws:runShellScript'
        runCommand:
          - ifconfig
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var foo = new Aws.Ssm.Document("foo", new()
    {
        Name = "test_document",
        DocumentFormat = "YAML",
        DocumentType = "Command",
        Content = @"schemaVersion: '1.2'
description: Check ip configuration of a Linux instance.
parameters: {}
runtimeConfig:
  'aws:runShellScript':
    properties:
      - id: '0.aws:runShellScript'
        runCommand:
          - ifconfig
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewDocument(ctx, "foo", &ssm.DocumentArgs{
			Name:           pulumi.String("test_document"),
			DocumentFormat: pulumi.String("YAML"),
			DocumentType:   pulumi.String("Command"),
			Content: pulumi.String(`schemaVersion: '1.2'
description: Check ip configuration of a Linux instance.
parameters: {}
runtimeConfig:
  'aws:runShellScript':
    properties:
      - id: '0.aws:runShellScript'
        runCommand:
          - ifconfig
`),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.Document;
import com.pulumi.aws.ssm.DocumentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var foo = new Document("foo", DocumentArgs.builder()
            .name("test_document")
            .documentFormat("YAML")
            .documentType("Command")
            .content("""
schemaVersion: '1.2'
description: Check ip configuration of a Linux instance.
parameters: {}
runtimeConfig:
  'aws:runShellScript':
    properties:
      - id: '0.aws:runShellScript'
        runCommand:
          - ifconfig
            """)
            .build());

    }
}
```
```yaml
resources:
  foo:
    type: aws:ssm:Document
    properties:
      name: test_document
      documentFormat: YAML
      documentType: Command
      content: |
        schemaVersion: '1.2'
        description: Check ip configuration of a Linux instance.
        parameters: {}
        runtimeConfig:
          'aws:runShellScript':
            properties:
              - id: '0.aws:runShellScript'
                runCommand:
                  - ifconfig
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM Documents using the name. For example:

```sh
$ pulumi import aws:ssm/document:Document example example
```
The `attachments_source` argument does not have an SSM API method for reading the attachment information detail after creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:

�
attachmentsSourceseBc*a:_
]
ssmDocumentAttachmentsSource;aws:ssm/DocumentAttachmentsSource:DocumentAttachmentsSource�One or more configuration blocks describing attachments sources to a version of a document. See `attachments_source` block below for details.
�
content" �The content for the SSM document in JSON or YAML format. The content of the document must not exceed 64KB. This quota also includes the content specified for input parameters at runtime. We recommend storing the contents for your new document in an external JSON or YAML file and referencing the file in a command.
Z
documentFormatB" BThe format of the document. Valid values: `JSON`, `TEXT`, `YAML`.
�
documentType" �The type of the document. For a list of valid values, see the [API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_CreateDocument.html#systemsmanager-CreateDocument-request-DocumentType).
(
nameB" The name of the document.
l
permissionsB2" UAdditional permissions to attach to the document. See Permissions below for details.
�
tagsB2" �A map of tags to assign to the object. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�

targetTypeB" �The target type which defines the kinds of resources the document can run on. For example, `/AWS::EC2::Instance`. For a list of valid resource types, see [AWS resource and property types reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html).
�
versionNameB" �The version of the artifact associated with the document. For example, `12.6`. This value is unique across all versions of a document, and can't be changed.
";
arn" 0The Amazon Resource Name (ARN) of the document.
"�
attachmentsSourceseBc*a:_
]
ssmDocumentAttachmentsSource;aws:ssm/DocumentAttachmentsSource:DocumentAttachmentsSource�One or more configuration blocks describing attachments sources to a version of a document. See `attachments_source` block below for details.
"�
content" �The content for the SSM document in JSON or YAML format. The content of the document must not exceed 64KB. This quota also includes the content specified for input parameters at runtime. We recommend storing the contents for your new document in an external JSON or YAML file and referencing the file in a command.
"6
createdDate" #The date the document was created.
";
defaultVersion" %The default version of the document.
"�
description" zA description of what the parameter does, how to use it, the default value, and whether or not the parameter is optional.
"Z
documentFormatB" BThe format of the document. Valid values: `JSON`, `TEXT`, `YAML`.
"�
documentType" �The type of the document. For a list of valid values, see the [API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_CreateDocument.html#systemsmanager-CreateDocument-request-DocumentType).
"-
documentVersion" The document version.
"Y
hash" MThe Sha256 or Sha1 hash created by the system when the document was created.
"O
hashType" ?The hash type of the document. Valid values: `Sha256`, `Sha1`.
"9
latestVersion" $The latest version of the document.
"&
name" The name of the document.
"E
owner" 8The Amazon Web Services user that created the document.
"�

parametersK*I:G
E
ssmDocumentParameter+aws:ssm/DocumentParameter:DocumentParametervOne or more configuration blocks describing the parameters for the document. See `parameter` block below for details.
"l
permissionsB2" UAdditional permissions to attach to the document. See Permissions below for details.
"�
platformTypes*" zThe list of operating system (OS) platforms compatible with this SSM document. Valid values: `Windows`, `Linux`, `MacOS`.
"9
schemaVersion" $The schema version of the document.
"t
status" fThe status of the SSM document. Valid values: `Creating`, `Active`, `Updating`, `Deleting`, `Failed`.
"�
tagsB2" �A map of tags to assign to the object. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�

targetTypeB" �The target type which defines the kinds of resources the document can run on. For example, `/AWS::EC2::Instance`. For a list of valid resource types, see [AWS resource and property types reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html).
"�
versionNameB" �The version of the artifact associated with the document. For example, `12.6`. This value is unique across all versions of a document, and can't be changed.
*�0
E
ssmMaintenanceWindow+aws:ssm/maintenanceWindow:MaintenanceWindow�Provides an SSM Maintenance Window resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const production = new aws.ssm.MaintenanceWindow("production", {
    name: "maintenance-window-application",
    schedule: "cron(0 16 ? * TUE *)",
    duration: 3,
    cutoff: 1,
});
```
```python
import pulumi
import pulumi_aws as aws

production = aws.ssm.MaintenanceWindow("production",
    name="maintenance-window-application",
    schedule="cron(0 16 ? * TUE *)",
    duration=3,
    cutoff=1)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var production = new Aws.Ssm.MaintenanceWindow("production", new()
    {
        Name = "maintenance-window-application",
        Schedule = "cron(0 16 ? * TUE *)",
        Duration = 3,
        Cutoff = 1,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewMaintenanceWindow(ctx, "production", &ssm.MaintenanceWindowArgs{
			Name:     pulumi.String("maintenance-window-application"),
			Schedule: pulumi.String("cron(0 16 ? * TUE *)"),
			Duration: pulumi.Int(3),
			Cutoff:   pulumi.Int(1),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.MaintenanceWindow;
import com.pulumi.aws.ssm.MaintenanceWindowArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var production = new MaintenanceWindow("production", MaintenanceWindowArgs.builder()
            .name("maintenance-window-application")
            .schedule("cron(0 16 ? * TUE *)")
            .duration(3)
            .cutoff(1)
            .build());

    }
}
```
```yaml
resources:
  production:
    type: aws:ssm:MaintenanceWindow
    properties:
      name: maintenance-window-application
      schedule: cron(0 16 ? * TUE *)
      duration: 3
      cutoff: 1
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM  Maintenance Windows using the maintenance window `id`. For example:

```sh
$ pulumi import aws:ssm/maintenanceWindow:MaintenanceWindow imported-window mw-0123456789
```
�
allowUnassociatedTargetsB
 nWhether targets must be registered with the Maintenance Window before tasks can be defined for those targets.
�
cutoff |The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.
?
descriptionB" *A description for the maintenance window.
A
duration 1The duration of the Maintenance Window in hours.
M
enabledB
 <Whether the maintenance window is enabled. Default: `true`.
�
endDateB" �Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to no longer run the maintenance window.
2
nameB" $The name of the maintenance window.
�
schedule" �The schedule of the Maintenance Window in the form of a [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html).
�
scheduleOffsetB yThe number of days to wait after the date and time specified by a CRON expression before running the maintenance window.
�
scheduleTimezoneB" �Timezone for schedule in [Internet Assigned Numbers Authority (IANA) Time Zone Database format](https://www.iana.org/time-zones). For example: `America/Los_Angeles`, `etc/UTC`, or `Asia/Seoul`.
�
	startDateB" �Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to begin the maintenance window.
�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
allowUnassociatedTargetsB
 nWhether targets must be registered with the Maintenance Window before tasks can be defined for those targets.
"�
cutoff |The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.
"?
descriptionB" *A description for the maintenance window.
"A
duration 1The duration of the Maintenance Window in hours.
"M
enabledB
 <Whether the maintenance window is enabled. Default: `true`.
"�
endDateB" �Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to no longer run the maintenance window.
"0
name" $The name of the maintenance window.
"�
schedule" �The schedule of the Maintenance Window in the form of a [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html).
"�
scheduleOffsetB yThe number of days to wait after the date and time specified by a CRON expression before running the maintenance window.
"�
scheduleTimezoneB" �Timezone for schedule in [Internet Assigned Numbers Authority (IANA) Time Zone Database format](https://www.iana.org/time-zones). For example: `America/Los_Angeles`, `etc/UTC`, or `Asia/Seoul`.
"�
	startDateB" �Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to begin the maintenance window.
"�
tagsB2" �A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�d
W
ssmMaintenanceWindowTarget7aws:ssm/maintenanceWindowTarget:MaintenanceWindowTarget�SProvides an SSM Maintenance Window Target resource

## Example Usage

### Instance Target

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const window = new aws.ssm.MaintenanceWindow("window", {
    name: "maintenance-window-webapp",
    schedule: "cron(0 16 ? * TUE *)",
    duration: 3,
    cutoff: 1,
});
const target1 = new aws.ssm.MaintenanceWindowTarget("target1", {
    windowId: window.id,
    name: "maintenance-window-target",
    description: "This is a maintenance window target",
    resourceType: "INSTANCE",
    targets: [{
        key: "tag:Name",
        values: ["acceptance_test"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

window = aws.ssm.MaintenanceWindow("window",
    name="maintenance-window-webapp",
    schedule="cron(0 16 ? * TUE *)",
    duration=3,
    cutoff=1)
target1 = aws.ssm.MaintenanceWindowTarget("target1",
    window_id=window.id,
    name="maintenance-window-target",
    description="This is a maintenance window target",
    resource_type="INSTANCE",
    targets=[{
        "key": "tag:Name",
        "values": ["acceptance_test"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var window = new Aws.Ssm.MaintenanceWindow("window", new()
    {
        Name = "maintenance-window-webapp",
        Schedule = "cron(0 16 ? * TUE *)",
        Duration = 3,
        Cutoff = 1,
    });

    var target1 = new Aws.Ssm.MaintenanceWindowTarget("target1", new()
    {
        WindowId = window.Id,
        Name = "maintenance-window-target",
        Description = "This is a maintenance window target",
        ResourceType = "INSTANCE",
        Targets = new[]
        {
            new Aws.Ssm.Inputs.MaintenanceWindowTargetTargetArgs
            {
                Key = "tag:Name",
                Values = new[]
                {
                    "acceptance_test",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		window, err := ssm.NewMaintenanceWindow(ctx, "window", &ssm.MaintenanceWindowArgs{
			Name:     pulumi.String("maintenance-window-webapp"),
			Schedule: pulumi.String("cron(0 16 ? * TUE *)"),
			Duration: pulumi.Int(3),
			Cutoff:   pulumi.Int(1),
		})
		if err != nil {
			return err
		}
		_, err = ssm.NewMaintenanceWindowTarget(ctx, "target1", &ssm.MaintenanceWindowTargetArgs{
			WindowId:     window.ID(),
			Name:         pulumi.String("maintenance-window-target"),
			Description:  pulumi.String("This is a maintenance window target"),
			ResourceType: pulumi.String("INSTANCE"),
			Targets: ssm.MaintenanceWindowTargetTargetArray{
				&ssm.MaintenanceWindowTargetTargetArgs{
					Key: pulumi.String("tag:Name"),
					Values: pulumi.StringArray{
						pulumi.String("acceptance_test"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.MaintenanceWindow;
import com.pulumi.aws.ssm.MaintenanceWindowArgs;
import com.pulumi.aws.ssm.MaintenanceWindowTarget;
import com.pulumi.aws.ssm.MaintenanceWindowTargetArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTargetTargetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var window = new MaintenanceWindow("window", MaintenanceWindowArgs.builder()
            .name("maintenance-window-webapp")
            .schedule("cron(0 16 ? * TUE *)")
            .duration(3)
            .cutoff(1)
            .build());

        var target1 = new MaintenanceWindowTarget("target1", MaintenanceWindowTargetArgs.builder()
            .windowId(window.id())
            .name("maintenance-window-target")
            .description("This is a maintenance window target")
            .resourceType("INSTANCE")
            .targets(MaintenanceWindowTargetTargetArgs.builder()
                .key("tag:Name")
                .values("acceptance_test")
                .build())
            .build());

    }
}
```
```yaml
resources:
  window:
    type: aws:ssm:MaintenanceWindow
    properties:
      name: maintenance-window-webapp
      schedule: cron(0 16 ? * TUE *)
      duration: 3
      cutoff: 1
  target1:
    type: aws:ssm:MaintenanceWindowTarget
    properties:
      windowId: ${window.id}
      name: maintenance-window-target
      description: This is a maintenance window target
      resourceType: INSTANCE
      targets:
        - key: tag:Name
          values:
            - acceptance_test
```
<!--End PulumiCodeChooser -->

### Resource Group Target

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const window = new aws.ssm.MaintenanceWindow("window", {
    name: "maintenance-window-webapp",
    schedule: "cron(0 16 ? * TUE *)",
    duration: 3,
    cutoff: 1,
});
const target1 = new aws.ssm.MaintenanceWindowTarget("target1", {
    windowId: window.id,
    name: "maintenance-window-target",
    description: "This is a maintenance window target",
    resourceType: "RESOURCE_GROUP",
    targets: [{
        key: "resource-groups:ResourceTypeFilters",
        values: ["AWS::EC2::Instance"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

window = aws.ssm.MaintenanceWindow("window",
    name="maintenance-window-webapp",
    schedule="cron(0 16 ? * TUE *)",
    duration=3,
    cutoff=1)
target1 = aws.ssm.MaintenanceWindowTarget("target1",
    window_id=window.id,
    name="maintenance-window-target",
    description="This is a maintenance window target",
    resource_type="RESOURCE_GROUP",
    targets=[{
        "key": "resource-groups:ResourceTypeFilters",
        "values": ["AWS::EC2::Instance"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var window = new Aws.Ssm.MaintenanceWindow("window", new()
    {
        Name = "maintenance-window-webapp",
        Schedule = "cron(0 16 ? * TUE *)",
        Duration = 3,
        Cutoff = 1,
    });

    var target1 = new Aws.Ssm.MaintenanceWindowTarget("target1", new()
    {
        WindowId = window.Id,
        Name = "maintenance-window-target",
        Description = "This is a maintenance window target",
        ResourceType = "RESOURCE_GROUP",
        Targets = new[]
        {
            new Aws.Ssm.Inputs.MaintenanceWindowTargetTargetArgs
            {
                Key = "resource-groups:ResourceTypeFilters",
                Values = new[]
                {
                    "AWS::EC2::Instance",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		window, err := ssm.NewMaintenanceWindow(ctx, "window", &ssm.MaintenanceWindowArgs{
			Name:     pulumi.String("maintenance-window-webapp"),
			Schedule: pulumi.String("cron(0 16 ? * TUE *)"),
			Duration: pulumi.Int(3),
			Cutoff:   pulumi.Int(1),
		})
		if err != nil {
			return err
		}
		_, err = ssm.NewMaintenanceWindowTarget(ctx, "target1", &ssm.MaintenanceWindowTargetArgs{
			WindowId:     window.ID(),
			Name:         pulumi.String("maintenance-window-target"),
			Description:  pulumi.String("This is a maintenance window target"),
			ResourceType: pulumi.String("RESOURCE_GROUP"),
			Targets: ssm.MaintenanceWindowTargetTargetArray{
				&ssm.MaintenanceWindowTargetTargetArgs{
					Key: pulumi.String("resource-groups:ResourceTypeFilters"),
					Values: pulumi.StringArray{
						pulumi.String("AWS::EC2::Instance"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.MaintenanceWindow;
import com.pulumi.aws.ssm.MaintenanceWindowArgs;
import com.pulumi.aws.ssm.MaintenanceWindowTarget;
import com.pulumi.aws.ssm.MaintenanceWindowTargetArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTargetTargetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var window = new MaintenanceWindow("window", MaintenanceWindowArgs.builder()
            .name("maintenance-window-webapp")
            .schedule("cron(0 16 ? * TUE *)")
            .duration(3)
            .cutoff(1)
            .build());

        var target1 = new MaintenanceWindowTarget("target1", MaintenanceWindowTargetArgs.builder()
            .windowId(window.id())
            .name("maintenance-window-target")
            .description("This is a maintenance window target")
            .resourceType("RESOURCE_GROUP")
            .targets(MaintenanceWindowTargetTargetArgs.builder()
                .key("resource-groups:ResourceTypeFilters")
                .values("AWS::EC2::Instance")
                .build())
            .build());

    }
}
```
```yaml
resources:
  window:
    type: aws:ssm:MaintenanceWindow
    properties:
      name: maintenance-window-webapp
      schedule: cron(0 16 ? * TUE *)
      duration: 3
      cutoff: 1
  target1:
    type: aws:ssm:MaintenanceWindowTarget
    properties:
      windowId: ${window.id}
      name: maintenance-window-target
      description: This is a maintenance window target
      resourceType: RESOURCE_GROUP
      targets:
        - key: resource-groups:ResourceTypeFilters
          values:
            - AWS::EC2::Instance
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM Maintenance Window targets using `WINDOW_ID/WINDOW_TARGET_ID`. For example:

```sh
$ pulumi import aws:ssm/maintenanceWindowTarget:MaintenanceWindowTarget example mw-0c50858d01EXAMPLE/23639a0b-ddbc-4bca-9e72-78d96EXAMPLE
```
G
descriptionB" 2The description of the maintenance window target.
9
nameB" +The name of the maintenance window target.
�
ownerInformationB" �User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.
�
resourceType" vThe type of target being registered with the Maintenance Window. Possible values are `INSTANCE` and `RESOURCE_GROUP`.
�
targetso*m:k
i
ssmMaintenanceWindowTargetTargetCaws:ssm/MaintenanceWindowTargetTarget:MaintenanceWindowTargetTarget�The targets to register with the maintenance window. In other words, the instances to run commands on when the maintenance window runs. You can specify targets using instance IDs, resource group names, or tags that have been applied to instances. For more information about these examples formats see
(https://docs.aws.amazon.com/systems-manager/latest/userguide/mw-cli-tutorial-targets-examples.html)
N
windowId" >The Id of the maintenance window to register the target with.
"G
descriptionB" 2The description of the maintenance window target.
"7
name" +The name of the maintenance window target.
"�
ownerInformationB" �User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.
"�
resourceType" vThe type of target being registered with the Maintenance Window. Possible values are `INSTANCE` and `RESOURCE_GROUP`.
"�
targetso*m:k
i
ssmMaintenanceWindowTargetTargetCaws:ssm/MaintenanceWindowTargetTarget:MaintenanceWindowTargetTarget�The targets to register with the maintenance window. In other words, the instances to run commands on when the maintenance window runs. You can specify targets using instance IDs, resource group names, or tags that have been applied to instances. For more information about these examples formats see
(https://docs.aws.amazon.com/systems-manager/latest/userguide/mw-cli-tutorial-targets-examples.html)
"N
windowId" >The Id of the maintenance window to register the target with.
*��
Q
ssmMaintenanceWindowTask3aws:ssm/maintenanceWindowTask:MaintenanceWindowTask��Provides an SSM Maintenance Window Task resource

## Example Usage

### Automation Tasks

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.MaintenanceWindowTask("example", {
    maxConcurrency: "2",
    maxErrors: "1",
    priority: 1,
    taskArn: "AWS-RestartEC2Instance",
    taskType: "AUTOMATION",
    windowId: exampleAwsSsmMaintenanceWindow.id,
    targets: [{
        key: "InstanceIds",
        values: [exampleAwsInstance.id],
    }],
    taskInvocationParameters: {
        automationParameters: {
            documentVersion: "$LATEST",
            parameters: [{
                name: "InstanceId",
                values: [exampleAwsInstance.id],
            }],
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.MaintenanceWindowTask("example",
    max_concurrency="2",
    max_errors="1",
    priority=1,
    task_arn="AWS-RestartEC2Instance",
    task_type="AUTOMATION",
    window_id=example_aws_ssm_maintenance_window["id"],
    targets=[{
        "key": "InstanceIds",
        "values": [example_aws_instance["id"]],
    }],
    task_invocation_parameters={
        "automation_parameters": {
            "document_version": "$LATEST",
            "parameters": [{
                "name": "InstanceId",
                "values": [example_aws_instance["id"]],
            }],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.MaintenanceWindowTask("example", new()
    {
        MaxConcurrency = "2",
        MaxErrors = "1",
        Priority = 1,
        TaskArn = "AWS-RestartEC2Instance",
        TaskType = "AUTOMATION",
        WindowId = exampleAwsSsmMaintenanceWindow.Id,
        Targets = new[]
        {
            new Aws.Ssm.Inputs.MaintenanceWindowTaskTargetArgs
            {
                Key = "InstanceIds",
                Values = new[]
                {
                    exampleAwsInstance.Id,
                },
            },
        },
        TaskInvocationParameters = new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersArgs
        {
            AutomationParameters = new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersAutomationParametersArgs
            {
                DocumentVersion = "$LATEST",
                Parameters = new[]
                {
                    new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameterArgs
                    {
                        Name = "InstanceId",
                        Values = new[]
                        {
                            exampleAwsInstance.Id,
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewMaintenanceWindowTask(ctx, "example", &ssm.MaintenanceWindowTaskArgs{
			MaxConcurrency: pulumi.String("2"),
			MaxErrors:      pulumi.String("1"),
			Priority:       pulumi.Int(1),
			TaskArn:        pulumi.String("AWS-RestartEC2Instance"),
			TaskType:       pulumi.String("AUTOMATION"),
			WindowId:       pulumi.Any(exampleAwsSsmMaintenanceWindow.Id),
			Targets: ssm.MaintenanceWindowTaskTargetArray{
				&ssm.MaintenanceWindowTaskTargetArgs{
					Key: pulumi.String("InstanceIds"),
					Values: pulumi.StringArray{
						exampleAwsInstance.Id,
					},
				},
			},
			TaskInvocationParameters: &ssm.MaintenanceWindowTaskTaskInvocationParametersArgs{
				AutomationParameters: &ssm.MaintenanceWindowTaskTaskInvocationParametersAutomationParametersArgs{
					DocumentVersion: pulumi.String("$LATEST"),
					Parameters: ssm.MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameterArray{
						&ssm.MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameterArgs{
							Name: pulumi.String("InstanceId"),
							Values: pulumi.StringArray{
								exampleAwsInstance.Id,
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.MaintenanceWindowTask;
import com.pulumi.aws.ssm.MaintenanceWindowTaskArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTargetArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTaskInvocationParametersArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTaskInvocationParametersAutomationParametersArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new MaintenanceWindowTask("example", MaintenanceWindowTaskArgs.builder()
            .maxConcurrency(2)
            .maxErrors(1)
            .priority(1)
            .taskArn("AWS-RestartEC2Instance")
            .taskType("AUTOMATION")
            .windowId(exampleAwsSsmMaintenanceWindow.id())
            .targets(MaintenanceWindowTaskTargetArgs.builder()
                .key("InstanceIds")
                .values(exampleAwsInstance.id())
                .build())
            .taskInvocationParameters(MaintenanceWindowTaskTaskInvocationParametersArgs.builder()
                .automationParameters(MaintenanceWindowTaskTaskInvocationParametersAutomationParametersArgs.builder()
                    .documentVersion("$LATEST")
                    .parameters(MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameterArgs.builder()
                        .name("InstanceId")
                        .values(exampleAwsInstance.id())
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:MaintenanceWindowTask
    properties:
      maxConcurrency: 2
      maxErrors: 1
      priority: 1
      taskArn: AWS-RestartEC2Instance
      taskType: AUTOMATION
      windowId: ${exampleAwsSsmMaintenanceWindow.id}
      targets:
        - key: InstanceIds
          values:
            - ${exampleAwsInstance.id}
      taskInvocationParameters:
        automationParameters:
          documentVersion: $LATEST
          parameters:
            - name: InstanceId
              values:
                - ${exampleAwsInstance.id}
```
<!--End PulumiCodeChooser -->

### Lambda Tasks

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const example = new aws.ssm.MaintenanceWindowTask("example", {
    maxConcurrency: "2",
    maxErrors: "1",
    priority: 1,
    taskArn: exampleAwsLambdaFunction.arn,
    taskType: "LAMBDA",
    windowId: exampleAwsSsmMaintenanceWindow.id,
    targets: [{
        key: "InstanceIds",
        values: [exampleAwsInstance.id],
    }],
    taskInvocationParameters: {
        lambdaParameters: {
            clientContext: std.base64encode({
                input: "{\"key1\":\"value1\"}",
            }).then(invoke => invoke.result),
            payload: "{\"key1\":\"value1\"}",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

example = aws.ssm.MaintenanceWindowTask("example",
    max_concurrency="2",
    max_errors="1",
    priority=1,
    task_arn=example_aws_lambda_function["arn"],
    task_type="LAMBDA",
    window_id=example_aws_ssm_maintenance_window["id"],
    targets=[{
        "key": "InstanceIds",
        "values": [example_aws_instance["id"]],
    }],
    task_invocation_parameters={
        "lambda_parameters": {
            "client_context": std.base64encode(input="{\"key1\":\"value1\"}").result,
            "payload": "{\"key1\":\"value1\"}",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.MaintenanceWindowTask("example", new()
    {
        MaxConcurrency = "2",
        MaxErrors = "1",
        Priority = 1,
        TaskArn = exampleAwsLambdaFunction.Arn,
        TaskType = "LAMBDA",
        WindowId = exampleAwsSsmMaintenanceWindow.Id,
        Targets = new[]
        {
            new Aws.Ssm.Inputs.MaintenanceWindowTaskTargetArgs
            {
                Key = "InstanceIds",
                Values = new[]
                {
                    exampleAwsInstance.Id,
                },
            },
        },
        TaskInvocationParameters = new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersArgs
        {
            LambdaParameters = new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersLambdaParametersArgs
            {
                ClientContext = Std.Base64encode.Invoke(new()
                {
                    Input = "{\"key1\":\"value1\"}",
                }).Apply(invoke => invoke.Result),
                Payload = "{\"key1\":\"value1\"}",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		invokeBase64encode, err := std.Base64encode(ctx, &std.Base64encodeArgs{
			Input: "{\"key1\":\"value1\"}",
		}, nil)
		if err != nil {
			return err
		}
		_, err = ssm.NewMaintenanceWindowTask(ctx, "example", &ssm.MaintenanceWindowTaskArgs{
			MaxConcurrency: pulumi.String("2"),
			MaxErrors:      pulumi.String("1"),
			Priority:       pulumi.Int(1),
			TaskArn:        pulumi.Any(exampleAwsLambdaFunction.Arn),
			TaskType:       pulumi.String("LAMBDA"),
			WindowId:       pulumi.Any(exampleAwsSsmMaintenanceWindow.Id),
			Targets: ssm.MaintenanceWindowTaskTargetArray{
				&ssm.MaintenanceWindowTaskTargetArgs{
					Key: pulumi.String("InstanceIds"),
					Values: pulumi.StringArray{
						exampleAwsInstance.Id,
					},
				},
			},
			TaskInvocationParameters: &ssm.MaintenanceWindowTaskTaskInvocationParametersArgs{
				LambdaParameters: &ssm.MaintenanceWindowTaskTaskInvocationParametersLambdaParametersArgs{
					ClientContext: pulumi.String(invokeBase64encode.Result),
					Payload:       pulumi.String("{\"key1\":\"value1\"}"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.MaintenanceWindowTask;
import com.pulumi.aws.ssm.MaintenanceWindowTaskArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTargetArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTaskInvocationParametersArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTaskInvocationParametersLambdaParametersArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new MaintenanceWindowTask("example", MaintenanceWindowTaskArgs.builder()
            .maxConcurrency(2)
            .maxErrors(1)
            .priority(1)
            .taskArn(exampleAwsLambdaFunction.arn())
            .taskType("LAMBDA")
            .windowId(exampleAwsSsmMaintenanceWindow.id())
            .targets(MaintenanceWindowTaskTargetArgs.builder()
                .key("InstanceIds")
                .values(exampleAwsInstance.id())
                .build())
            .taskInvocationParameters(MaintenanceWindowTaskTaskInvocationParametersArgs.builder()
                .lambdaParameters(MaintenanceWindowTaskTaskInvocationParametersLambdaParametersArgs.builder()
                    .clientContext(StdFunctions.base64encode(Base64encodeArgs.builder()
                        .input("{\"key1\":\"value1\"}")
                        .build()).result())
                    .payload("{\"key1\":\"value1\"}")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:MaintenanceWindowTask
    properties:
      maxConcurrency: 2
      maxErrors: 1
      priority: 1
      taskArn: ${exampleAwsLambdaFunction.arn}
      taskType: LAMBDA
      windowId: ${exampleAwsSsmMaintenanceWindow.id}
      targets:
        - key: InstanceIds
          values:
            - ${exampleAwsInstance.id}
      taskInvocationParameters:
        lambdaParameters:
          clientContext:
            fn::invoke:
              function: std:base64encode
              arguments:
                input: '{"key1":"value1"}'
              return: result
          payload: '{"key1":"value1"}'
```
<!--End PulumiCodeChooser -->

### Run Command Tasks

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.MaintenanceWindowTask("example", {
    maxConcurrency: "2",
    maxErrors: "1",
    priority: 1,
    taskArn: "AWS-RunShellScript",
    taskType: "RUN_COMMAND",
    windowId: exampleAwsSsmMaintenanceWindow.id,
    targets: [{
        key: "InstanceIds",
        values: [exampleAwsInstance.id],
    }],
    taskInvocationParameters: {
        runCommandParameters: {
            outputS3Bucket: exampleAwsS3Bucket.id,
            outputS3KeyPrefix: "output",
            serviceRoleArn: exampleAwsIamRole.arn,
            timeoutSeconds: 600,
            notificationConfig: {
                notificationArn: exampleAwsSnsTopic.arn,
                notificationEvents: ["All"],
                notificationType: "Command",
            },
            parameters: [{
                name: "commands",
                values: ["date"],
            }],
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.MaintenanceWindowTask("example",
    max_concurrency="2",
    max_errors="1",
    priority=1,
    task_arn="AWS-RunShellScript",
    task_type="RUN_COMMAND",
    window_id=example_aws_ssm_maintenance_window["id"],
    targets=[{
        "key": "InstanceIds",
        "values": [example_aws_instance["id"]],
    }],
    task_invocation_parameters={
        "run_command_parameters": {
            "output_s3_bucket": example_aws_s3_bucket["id"],
            "output_s3_key_prefix": "output",
            "service_role_arn": example_aws_iam_role["arn"],
            "timeout_seconds": 600,
            "notification_config": {
                "notification_arn": example_aws_sns_topic["arn"],
                "notification_events": ["All"],
                "notification_type": "Command",
            },
            "parameters": [{
                "name": "commands",
                "values": ["date"],
            }],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.MaintenanceWindowTask("example", new()
    {
        MaxConcurrency = "2",
        MaxErrors = "1",
        Priority = 1,
        TaskArn = "AWS-RunShellScript",
        TaskType = "RUN_COMMAND",
        WindowId = exampleAwsSsmMaintenanceWindow.Id,
        Targets = new[]
        {
            new Aws.Ssm.Inputs.MaintenanceWindowTaskTargetArgs
            {
                Key = "InstanceIds",
                Values = new[]
                {
                    exampleAwsInstance.Id,
                },
            },
        },
        TaskInvocationParameters = new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersArgs
        {
            RunCommandParameters = new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersArgs
            {
                OutputS3Bucket = exampleAwsS3Bucket.Id,
                OutputS3KeyPrefix = "output",
                ServiceRoleArn = exampleAwsIamRole.Arn,
                TimeoutSeconds = 600,
                NotificationConfig = new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfigArgs
                {
                    NotificationArn = exampleAwsSnsTopic.Arn,
                    NotificationEvents = new[]
                    {
                        "All",
                    },
                    NotificationType = "Command",
                },
                Parameters = new[]
                {
                    new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameterArgs
                    {
                        Name = "commands",
                        Values = new[]
                        {
                            "date",
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewMaintenanceWindowTask(ctx, "example", &ssm.MaintenanceWindowTaskArgs{
			MaxConcurrency: pulumi.String("2"),
			MaxErrors:      pulumi.String("1"),
			Priority:       pulumi.Int(1),
			TaskArn:        pulumi.String("AWS-RunShellScript"),
			TaskType:       pulumi.String("RUN_COMMAND"),
			WindowId:       pulumi.Any(exampleAwsSsmMaintenanceWindow.Id),
			Targets: ssm.MaintenanceWindowTaskTargetArray{
				&ssm.MaintenanceWindowTaskTargetArgs{
					Key: pulumi.String("InstanceIds"),
					Values: pulumi.StringArray{
						exampleAwsInstance.Id,
					},
				},
			},
			TaskInvocationParameters: &ssm.MaintenanceWindowTaskTaskInvocationParametersArgs{
				RunCommandParameters: &ssm.MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersArgs{
					OutputS3Bucket:    pulumi.Any(exampleAwsS3Bucket.Id),
					OutputS3KeyPrefix: pulumi.String("output"),
					ServiceRoleArn:    pulumi.Any(exampleAwsIamRole.Arn),
					TimeoutSeconds:    pulumi.Int(600),
					NotificationConfig: &ssm.MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfigArgs{
						NotificationArn: pulumi.Any(exampleAwsSnsTopic.Arn),
						NotificationEvents: pulumi.StringArray{
							pulumi.String("All"),
						},
						NotificationType: pulumi.String("Command"),
					},
					Parameters: ssm.MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameterArray{
						&ssm.MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameterArgs{
							Name: pulumi.String("commands"),
							Values: pulumi.StringArray{
								pulumi.String("date"),
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.MaintenanceWindowTask;
import com.pulumi.aws.ssm.MaintenanceWindowTaskArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTargetArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTaskInvocationParametersArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new MaintenanceWindowTask("example", MaintenanceWindowTaskArgs.builder()
            .maxConcurrency(2)
            .maxErrors(1)
            .priority(1)
            .taskArn("AWS-RunShellScript")
            .taskType("RUN_COMMAND")
            .windowId(exampleAwsSsmMaintenanceWindow.id())
            .targets(MaintenanceWindowTaskTargetArgs.builder()
                .key("InstanceIds")
                .values(exampleAwsInstance.id())
                .build())
            .taskInvocationParameters(MaintenanceWindowTaskTaskInvocationParametersArgs.builder()
                .runCommandParameters(MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersArgs.builder()
                    .outputS3Bucket(exampleAwsS3Bucket.id())
                    .outputS3KeyPrefix("output")
                    .serviceRoleArn(exampleAwsIamRole.arn())
                    .timeoutSeconds(600)
                    .notificationConfig(MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfigArgs.builder()
                        .notificationArn(exampleAwsSnsTopic.arn())
                        .notificationEvents("All")
                        .notificationType("Command")
                        .build())
                    .parameters(MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameterArgs.builder()
                        .name("commands")
                        .values("date")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:MaintenanceWindowTask
    properties:
      maxConcurrency: 2
      maxErrors: 1
      priority: 1
      taskArn: AWS-RunShellScript
      taskType: RUN_COMMAND
      windowId: ${exampleAwsSsmMaintenanceWindow.id}
      targets:
        - key: InstanceIds
          values:
            - ${exampleAwsInstance.id}
      taskInvocationParameters:
        runCommandParameters:
          outputS3Bucket: ${exampleAwsS3Bucket.id}
          outputS3KeyPrefix: output
          serviceRoleArn: ${exampleAwsIamRole.arn}
          timeoutSeconds: 600
          notificationConfig:
            notificationArn: ${exampleAwsSnsTopic.arn}
            notificationEvents:
              - All
            notificationType: Command
          parameters:
            - name: commands
              values:
                - date
```
<!--End PulumiCodeChooser -->

### Step Function Tasks

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssm.MaintenanceWindowTask("example", {
    maxConcurrency: "2",
    maxErrors: "1",
    priority: 1,
    taskArn: exampleAwsSfnActivity.id,
    taskType: "STEP_FUNCTIONS",
    windowId: exampleAwsSsmMaintenanceWindow.id,
    targets: [{
        key: "InstanceIds",
        values: [exampleAwsInstance.id],
    }],
    taskInvocationParameters: {
        stepFunctionsParameters: {
            input: "{\"key1\":\"value1\"}",
            name: "example",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.MaintenanceWindowTask("example",
    max_concurrency="2",
    max_errors="1",
    priority=1,
    task_arn=example_aws_sfn_activity["id"],
    task_type="STEP_FUNCTIONS",
    window_id=example_aws_ssm_maintenance_window["id"],
    targets=[{
        "key": "InstanceIds",
        "values": [example_aws_instance["id"]],
    }],
    task_invocation_parameters={
        "step_functions_parameters": {
            "input": "{\"key1\":\"value1\"}",
            "name": "example",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ssm.MaintenanceWindowTask("example", new()
    {
        MaxConcurrency = "2",
        MaxErrors = "1",
        Priority = 1,
        TaskArn = exampleAwsSfnActivity.Id,
        TaskType = "STEP_FUNCTIONS",
        WindowId = exampleAwsSsmMaintenanceWindow.Id,
        Targets = new[]
        {
            new Aws.Ssm.Inputs.MaintenanceWindowTaskTargetArgs
            {
                Key = "InstanceIds",
                Values = new[]
                {
                    exampleAwsInstance.Id,
                },
            },
        },
        TaskInvocationParameters = new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersArgs
        {
            StepFunctionsParameters = new Aws.Ssm.Inputs.MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParametersArgs
            {
                Input = "{\"key1\":\"value1\"}",
                Name = "example",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewMaintenanceWindowTask(ctx, "example", &ssm.MaintenanceWindowTaskArgs{
			MaxConcurrency: pulumi.String("2"),
			MaxErrors:      pulumi.String("1"),
			Priority:       pulumi.Int(1),
			TaskArn:        pulumi.Any(exampleAwsSfnActivity.Id),
			TaskType:       pulumi.String("STEP_FUNCTIONS"),
			WindowId:       pulumi.Any(exampleAwsSsmMaintenanceWindow.Id),
			Targets: ssm.MaintenanceWindowTaskTargetArray{
				&ssm.MaintenanceWindowTaskTargetArgs{
					Key: pulumi.String("InstanceIds"),
					Values: pulumi.StringArray{
						exampleAwsInstance.Id,
					},
				},
			},
			TaskInvocationParameters: &ssm.MaintenanceWindowTaskTaskInvocationParametersArgs{
				StepFunctionsParameters: &ssm.MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParametersArgs{
					Input: pulumi.String("{\"key1\":\"value1\"}"),
					Name:  pulumi.String("example"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.MaintenanceWindowTask;
import com.pulumi.aws.ssm.MaintenanceWindowTaskArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTargetArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTaskInvocationParametersArgs;
import com.pulumi.aws.ssm.inputs.MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParametersArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new MaintenanceWindowTask("example", MaintenanceWindowTaskArgs.builder()
            .maxConcurrency(2)
            .maxErrors(1)
            .priority(1)
            .taskArn(exampleAwsSfnActivity.id())
            .taskType("STEP_FUNCTIONS")
            .windowId(exampleAwsSsmMaintenanceWindow.id())
            .targets(MaintenanceWindowTaskTargetArgs.builder()
                .key("InstanceIds")
                .values(exampleAwsInstance.id())
                .build())
            .taskInvocationParameters(MaintenanceWindowTaskTaskInvocationParametersArgs.builder()
                .stepFunctionsParameters(MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParametersArgs.builder()
                    .input("{\"key1\":\"value1\"}")
                    .name("example")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssm:MaintenanceWindowTask
    properties:
      maxConcurrency: 2
      maxErrors: 1
      priority: 1
      taskArn: ${exampleAwsSfnActivity.id}
      taskType: STEP_FUNCTIONS
      windowId: ${exampleAwsSsmMaintenanceWindow.id}
      targets:
        - key: InstanceIds
          values:
            - ${exampleAwsInstance.id}
      taskInvocationParameters:
        stepFunctionsParameters:
          input: '{"key1":"value1"}'
          name: example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AWS Maintenance Window Task using the `window_id` and `window_task_id` separated by `/`. For example:

```sh
$ pulumi import aws:ssm/maintenanceWindowTask:MaintenanceWindowTask task <window_id>/<window_task_id>
```
�
cutoffBehaviorB" �Indicates whether tasks should continue to run after the cutoff time specified in the maintenance windows is reached. Valid values are `CONTINUE_TASK` and `CANCEL_TASK`.
E
descriptionB" 0The description of the maintenance window task.
\
maxConcurrencyB" DThe maximum number of targets this task can be run for in parallel.
`
	maxErrorsB" MThe maximum number of errors allowed before this task stops being scheduled.
7
nameB" )The name of the maintenance window task.
�
priorityB �The priority of the task in the Maintenance Window, the lower the number the higher the priority. Tasks in a Maintenance Window are scheduled in priority order with tasks that have the same priority scheduled in parallel.
�
serviceRoleArnB" �The role that should be assumed when executing the task. If a role is not provided, Systems Manager uses your account's service-linked role. If no service-linked role for Systems Manager exists in your account, it is created for you.
�
targetskBi*g:e
c
ssmMaintenanceWindowTaskTarget?aws:ssm/MaintenanceWindowTaskTarget:MaintenanceWindowTaskTarget�The targets (either instances or window target ids). Instances are specified using Key=InstanceIds,Values=instanceid1,instanceid2. Window target ids are specified using Key=WindowTargetIds,Values=window target id1, window target id2.
/
taskArn"  The ARN of the task to execute.
�
taskInvocationParameters�B�:�
�
ssm-MaintenanceWindowTaskTaskInvocationParameterscaws:ssm/MaintenanceWindowTaskTaskInvocationParameters:MaintenanceWindowTaskTaskInvocationParameters8Configuration block with parameters for task execution.
|
taskType" lThe type of task being registered. Valid values: `AUTOMATION`, `LAMBDA`, `RUN_COMMAND` or `STEP_FUNCTIONS`.
L
windowId" <The Id of the maintenance window to register the task with.
"3
arn" (The ARN of the maintenance window task.
"�
cutoffBehaviorB" �Indicates whether tasks should continue to run after the cutoff time specified in the maintenance windows is reached. Valid values are `CONTINUE_TASK` and `CANCEL_TASK`.
"E
descriptionB" 0The description of the maintenance window task.
"Z
maxConcurrency" DThe maximum number of targets this task can be run for in parallel.
"^
	maxErrors" MThe maximum number of errors allowed before this task stops being scheduled.
"5
name" )The name of the maintenance window task.
"�
priorityB �The priority of the task in the Maintenance Window, the lower the number the higher the priority. Tasks in a Maintenance Window are scheduled in priority order with tasks that have the same priority scheduled in parallel.
"�
serviceRoleArn" �The role that should be assumed when executing the task. If a role is not provided, Systems Manager uses your account's service-linked role. If no service-linked role for Systems Manager exists in your account, it is created for you.
"�
targetskBi*g:e
c
ssmMaintenanceWindowTaskTarget?aws:ssm/MaintenanceWindowTaskTarget:MaintenanceWindowTaskTarget�The targets (either instances or window target ids). Instances are specified using Key=InstanceIds,Values=instanceid1,instanceid2. Window target ids are specified using Key=WindowTargetIds,Values=window target id1, window target id2.
"/
taskArn"  The ARN of the task to execute.
"�
taskInvocationParameters�B�:�
�
ssm-MaintenanceWindowTaskTaskInvocationParameterscaws:ssm/MaintenanceWindowTaskTaskInvocationParameters:MaintenanceWindowTaskTaskInvocationParameters8Configuration block with parameters for task execution.
"|
taskType" lThe type of task being registered. Valid values: `AUTOMATION`, `LAMBDA`, `RUN_COMMAND` or `STEP_FUNCTIONS`.
"L
windowId" <The Id of the maintenance window to register the task with.
";
windowTaskId" 'The ID of the maintenance window task.
*�m
-
ssm	Parameteraws:ssm/parameter:Parameter�BProvides an SSM Parameter resource.

> **Note:** `overwrite` also makes it possible to overwrite an existing SSM Parameter that's not created by the provider before. This argument has been deprecated and will be removed in v6.0.0 of the provider. For more information on how this affects the behavior of this resource, see this issue comment.

## Example Usage

### Basic example

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = new aws.ssm.Parameter("foo", {
    name: "foo",
    type: aws.ssm.ParameterType.String,
    value: "bar",
});
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.ssm.Parameter("foo",
    name="foo",
    type=aws.ssm.ParameterType.STRING,
    value="bar")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var foo = new Aws.Ssm.Parameter("foo", new()
    {
        Name = "foo",
        Type = Aws.Ssm.ParameterType.String,
        Value = "bar",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewParameter(ctx, "foo", &ssm.ParameterArgs{
			Name:  pulumi.String("foo"),
			Type:  pulumi.String(ssm.ParameterTypeString),
			Value: pulumi.String("bar"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.Parameter;
import com.pulumi.aws.ssm.ParameterArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var foo = new Parameter("foo", ParameterArgs.builder()
            .name("foo")
            .type("String")
            .value("bar")
            .build());

    }
}
```
```yaml
resources:
  foo:
    type: aws:ssm:Parameter
    properties:
      name: foo
      type: String
      value: bar
```
<!--End PulumiCodeChooser -->

### Encrypted string using default SSM KMS key

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const _default = new aws.rds.Instance("default", {
    allocatedStorage: 10,
    storageType: aws.rds.StorageType.GP2,
    engine: "mysql",
    engineVersion: "5.7.16",
    instanceClass: aws.rds.InstanceType.T2_Micro,
    dbName: "mydb",
    username: "foo",
    password: databaseMasterPassword,
    dbSubnetGroupName: "my_database_subnet_group",
    parameterGroupName: "default.mysql5.7",
});
const secret = new aws.ssm.Parameter("secret", {
    name: "/production/database/password/master",
    description: "The parameter description",
    type: aws.ssm.ParameterType.SecureString,
    value: databaseMasterPassword,
    tags: {
        environment: "production",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

default = aws.rds.Instance("default",
    allocated_storage=10,
    storage_type=aws.rds.StorageType.GP2,
    engine="mysql",
    engine_version="5.7.16",
    instance_class=aws.rds.InstanceType.T2_MICRO,
    db_name="mydb",
    username="foo",
    password=database_master_password,
    db_subnet_group_name="my_database_subnet_group",
    parameter_group_name="default.mysql5.7")
secret = aws.ssm.Parameter("secret",
    name="/production/database/password/master",
    description="The parameter description",
    type=aws.ssm.ParameterType.SECURE_STRING,
    value=database_master_password,
    tags={
        "environment": "production",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var @default = new Aws.Rds.Instance("default", new()
    {
        AllocatedStorage = 10,
        StorageType = Aws.Rds.StorageType.GP2,
        Engine = "mysql",
        EngineVersion = "5.7.16",
        InstanceClass = Aws.Rds.InstanceType.T2_Micro,
        DbName = "mydb",
        Username = "foo",
        Password = databaseMasterPassword,
        DbSubnetGroupName = "my_database_subnet_group",
        ParameterGroupName = "default.mysql5.7",
    });

    var secret = new Aws.Ssm.Parameter("secret", new()
    {
        Name = "/production/database/password/master",
        Description = "The parameter description",
        Type = Aws.Ssm.ParameterType.SecureString,
        Value = databaseMasterPassword,
        Tags = 
        {
            { "environment", "production" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/rds"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := rds.NewInstance(ctx, "default", &rds.InstanceArgs{
			AllocatedStorage:   pulumi.Int(10),
			StorageType:        pulumi.String(rds.StorageTypeGP2),
			Engine:             pulumi.String("mysql"),
			EngineVersion:      pulumi.String("5.7.16"),
			InstanceClass:      pulumi.String(rds.InstanceType_T2_Micro),
			DbName:             pulumi.String("mydb"),
			Username:           pulumi.String("foo"),
			Password:           pulumi.Any(databaseMasterPassword),
			DbSubnetGroupName:  pulumi.String("my_database_subnet_group"),
			ParameterGroupName: pulumi.String("default.mysql5.7"),
		})
		if err != nil {
			return err
		}
		_, err = ssm.NewParameter(ctx, "secret", &ssm.ParameterArgs{
			Name:        pulumi.String("/production/database/password/master"),
			Description: pulumi.String("The parameter description"),
			Type:        pulumi.String(ssm.ParameterTypeSecureString),
			Value:       pulumi.Any(databaseMasterPassword),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("production"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.rds.Instance;
import com.pulumi.aws.rds.InstanceArgs;
import com.pulumi.aws.ssm.Parameter;
import com.pulumi.aws.ssm.ParameterArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Instance("default", InstanceArgs.builder()
            .allocatedStorage(10)
            .storageType("gp2")
            .engine("mysql")
            .engineVersion("5.7.16")
            .instanceClass("db.t2.micro")
            .dbName("mydb")
            .username("foo")
            .password(databaseMasterPassword)
            .dbSubnetGroupName("my_database_subnet_group")
            .parameterGroupName("default.mysql5.7")
            .build());

        var secret = new Parameter("secret", ParameterArgs.builder()
            .name("/production/database/password/master")
            .description("The parameter description")
            .type("SecureString")
            .value(databaseMasterPassword)
            .tags(Map.of("environment", "production"))
            .build());

    }
}
```
```yaml
resources:
  default:
    type: aws:rds:Instance
    properties:
      allocatedStorage: 10
      storageType: gp2
      engine: mysql
      engineVersion: 5.7.16
      instanceClass: db.t2.micro
      dbName: mydb
      username: foo
      password: ${databaseMasterPassword}
      dbSubnetGroupName: my_database_subnet_group
      parameterGroupName: default.mysql5.7
  secret:
    type: aws:ssm:Parameter
    properties:
      name: /production/database/password/master
      description: The parameter description
      type: SecureString
      value: ${databaseMasterPassword}
      tags:
        environment: production
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM Parameters using the parameter store `name`. For example:

```sh
$ pulumi import aws:ssm/parameter:Parameter my_param /my_path/my_paramname
```
Q
allowedPatternB" 9Regular expression used to validate the parameter value.
#
arnB" ARN of the parameter.
�
dataTypeB" �Data type of the parameter. Valid values: `text`, `aws:ssm:integration` and `aws:ec2:image` for AMI format, see the [Native parameter support for Amazon Machine Image IDs](https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-ec2-aliases.html).
3
descriptionB" Description of the parameter.
�
insecureValueB" �Value of the parameter. **Use caution:** This value is _never_ marked as sensitive in the pulumi preview output. This argument is not valid with a `type` of `SecureString`.
@
keyIdB" 1KMS key ID or ARN for encrypting a SecureString.
�
nameB" �Name of the parameter. If the name contains a path (e.g., any forward slashes (`/`)), it must be fully qualified with a leading forward slash (`/`). For additional requirements and constraints, see the [AWS SSM User Guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-parameter-name-constraints.html).
�
	overwriteB
 �Overwrite an existing parameter. If not specified, defaults to `false` if the resource has not been created by Pulumi to avoid overwrite of existing resource, and will default to `true` otherwise (Pulumi lifecycle rules should then be used to manage the update behavior).
�
tagsB2" �Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
tierB" �Parameter tier to assign to the parameter. If not specified, will use the default parameter tier for the region. Valid tiers are `Standard`, `Advanced`, and `Intelligent-Tiering`. Downgrading an `Advanced` tier parameter to `Standard` will recreate the resource. For more information on parameter tiers, see the [AWS SSM Parameter tier comparison and guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-advanced-parameters.html).
�
type" yType of the parameter. Valid types are `String`, `StringList` and `SecureString`.

The following arguments are optional:
�
valueB" �Value of the parameter. This value is always marked as sensitive in the pulumi preview output, regardless of `type`.

> **NOTE:** `aws:ssm:integration` data_type parameters must be of the type `SecureString` and the name must start with the prefix `/d9d01087-4a3f-49e0-b0b4-d568d7826553/ssm/integrations/webhook/`. See [here](https://docs.aws.amazon.com/systems-manager/latest/userguide/creating-integrations.html) for information on the usage of `aws:ssm:integration` parameters.
"Q
allowedPatternB" 9Regular expression used to validate the parameter value.
"!
arn" ARN of the parameter.
"�
dataType" �Data type of the parameter. Valid values: `text`, `aws:ssm:integration` and `aws:ec2:image` for AMI format, see the [Native parameter support for Amazon Machine Image IDs](https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-ec2-aliases.html).
"3
descriptionB" Description of the parameter.
"�
insecureValue" �Value of the parameter. **Use caution:** This value is _never_ marked as sensitive in the pulumi preview output. This argument is not valid with a `type` of `SecureString`.
">
keyId" 1KMS key ID or ARN for encrypting a SecureString.
"�
name" �Name of the parameter. If the name contains a path (e.g., any forward slashes (`/`)), it must be fully qualified with a leading forward slash (`/`). For additional requirements and constraints, see the [AWS SSM User Guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-parameter-name-constraints.html).
"�
	overwriteB
 �Overwrite an existing parameter. If not specified, defaults to `false` if the resource has not been created by Pulumi to avoid overwrite of existing resource, and will default to `true` otherwise (Pulumi lifecycle rules should then be used to manage the update behavior).
"�
tagsB2" �Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
tierB" �Parameter tier to assign to the parameter. If not specified, will use the default parameter tier for the region. Valid tiers are `Standard`, `Advanced`, and `Intelligent-Tiering`. Downgrading an `Advanced` tier parameter to `Standard` will recreate the resource. For more information on parameter tiers, see the [AWS SSM Parameter tier comparison and guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-advanced-parameters.html).
"�
type" yType of the parameter. Valid types are `String`, `StringList` and `SecureString`.

The following arguments are optional:
"�
value" �Value of the parameter. This value is always marked as sensitive in the pulumi preview output, regardless of `type`.

> **NOTE:** `aws:ssm:integration` data_type parameters must be of the type `SecureString` and the name must start with the prefix `/d9d01087-4a3f-49e0-b0b4-d568d7826553/ssm/integrations/webhook/`. See [here](https://docs.aws.amazon.com/systems-manager/latest/userguide/creating-integrations.html) for information on the usage of `aws:ssm:integration` parameters.
")
version Version of the parameter.
*��
9
ssmPatchBaseline#aws:ssm/patchBaseline:PatchBaseline��Provides an SSM Patch Baseline resource.

> **NOTE on Patch Baselines:** The `approved_patches` and `approval_rule` are
both marked as optional fields, but the Patch Baseline requires that at least one
of them is specified.

## Example Usage

### Basic Usage

Using `approved_patches` only.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const production = new aws.ssm.PatchBaseline("production", {
    name: "patch-baseline",
    approvedPatches: ["KB123456"],
});
```
```python
import pulumi
import pulumi_aws as aws

production = aws.ssm.PatchBaseline("production",
    name="patch-baseline",
    approved_patches=["KB123456"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var production = new Aws.Ssm.PatchBaseline("production", new()
    {
        Name = "patch-baseline",
        ApprovedPatches = new[]
        {
            "KB123456",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewPatchBaseline(ctx, "production", &ssm.PatchBaselineArgs{
			Name: pulumi.String("patch-baseline"),
			ApprovedPatches: pulumi.StringArray{
				pulumi.String("KB123456"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.PatchBaseline;
import com.pulumi.aws.ssm.PatchBaselineArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var production = new PatchBaseline("production", PatchBaselineArgs.builder()
            .name("patch-baseline")
            .approvedPatches("KB123456")
            .build());

    }
}
```
```yaml
resources:
  production:
    type: aws:ssm:PatchBaseline
    properties:
      name: patch-baseline
      approvedPatches:
        - KB123456
```
<!--End PulumiCodeChooser -->

### Advanced Usage, specifying patch filters

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const production = new aws.ssm.PatchBaseline("production", {
    name: "patch-baseline",
    description: "Patch Baseline Description",
    approvedPatches: [
        "KB123456",
        "KB456789",
    ],
    rejectedPatches: ["KB987654"],
    globalFilters: [
        {
            key: "PRODUCT",
            values: ["WindowsServer2008"],
        },
        {
            key: "CLASSIFICATION",
            values: ["ServicePacks"],
        },
        {
            key: "MSRC_SEVERITY",
            values: ["Low"],
        },
    ],
    approvalRules: [
        {
            approveAfterDays: 7,
            complianceLevel: "HIGH",
            patchFilters: [
                {
                    key: "PRODUCT",
                    values: ["WindowsServer2016"],
                },
                {
                    key: "CLASSIFICATION",
                    values: [
                        "CriticalUpdates",
                        "SecurityUpdates",
                        "Updates",
                    ],
                },
                {
                    key: "MSRC_SEVERITY",
                    values: [
                        "Critical",
                        "Important",
                        "Moderate",
                    ],
                },
            ],
        },
        {
            approveAfterDays: 7,
            patchFilters: [{
                key: "PRODUCT",
                values: ["WindowsServer2012"],
            }],
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

production = aws.ssm.PatchBaseline("production",
    name="patch-baseline",
    description="Patch Baseline Description",
    approved_patches=[
        "KB123456",
        "KB456789",
    ],
    rejected_patches=["KB987654"],
    global_filters=[
        {
            "key": "PRODUCT",
            "values": ["WindowsServer2008"],
        },
        {
            "key": "CLASSIFICATION",
            "values": ["ServicePacks"],
        },
        {
            "key": "MSRC_SEVERITY",
            "values": ["Low"],
        },
    ],
    approval_rules=[
        {
            "approve_after_days": 7,
            "compliance_level": "HIGH",
            "patch_filters": [
                {
                    "key": "PRODUCT",
                    "values": ["WindowsServer2016"],
                },
                {
                    "key": "CLASSIFICATION",
                    "values": [
                        "CriticalUpdates",
                        "SecurityUpdates",
                        "Updates",
                    ],
                },
                {
                    "key": "MSRC_SEVERITY",
                    "values": [
                        "Critical",
                        "Important",
                        "Moderate",
                    ],
                },
            ],
        },
        {
            "approve_after_days": 7,
            "patch_filters": [{
                "key": "PRODUCT",
                "values": ["WindowsServer2012"],
            }],
        },
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var production = new Aws.Ssm.PatchBaseline("production", new()
    {
        Name = "patch-baseline",
        Description = "Patch Baseline Description",
        ApprovedPatches = new[]
        {
            "KB123456",
            "KB456789",
        },
        RejectedPatches = new[]
        {
            "KB987654",
        },
        GlobalFilters = new[]
        {
            new Aws.Ssm.Inputs.PatchBaselineGlobalFilterArgs
            {
                Key = "PRODUCT",
                Values = new[]
                {
                    "WindowsServer2008",
                },
            },
            new Aws.Ssm.Inputs.PatchBaselineGlobalFilterArgs
            {
                Key = "CLASSIFICATION",
                Values = new[]
                {
                    "ServicePacks",
                },
            },
            new Aws.Ssm.Inputs.PatchBaselineGlobalFilterArgs
            {
                Key = "MSRC_SEVERITY",
                Values = new[]
                {
                    "Low",
                },
            },
        },
        ApprovalRules = new[]
        {
            new Aws.Ssm.Inputs.PatchBaselineApprovalRuleArgs
            {
                ApproveAfterDays = 7,
                ComplianceLevel = "HIGH",
                PatchFilters = new[]
                {
                    new Aws.Ssm.Inputs.PatchBaselineApprovalRulePatchFilterArgs
                    {
                        Key = "PRODUCT",
                        Values = new[]
                        {
                            "WindowsServer2016",
                        },
                    },
                    new Aws.Ssm.Inputs.PatchBaselineApprovalRulePatchFilterArgs
                    {
                        Key = "CLASSIFICATION",
                        Values = new[]
                        {
                            "CriticalUpdates",
                            "SecurityUpdates",
                            "Updates",
                        },
                    },
                    new Aws.Ssm.Inputs.PatchBaselineApprovalRulePatchFilterArgs
                    {
                        Key = "MSRC_SEVERITY",
                        Values = new[]
                        {
                            "Critical",
                            "Important",
                            "Moderate",
                        },
                    },
                },
            },
            new Aws.Ssm.Inputs.PatchBaselineApprovalRuleArgs
            {
                ApproveAfterDays = 7,
                PatchFilters = new[]
                {
                    new Aws.Ssm.Inputs.PatchBaselineApprovalRulePatchFilterArgs
                    {
                        Key = "PRODUCT",
                        Values = new[]
                        {
                            "WindowsServer2012",
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewPatchBaseline(ctx, "production", &ssm.PatchBaselineArgs{
			Name:        pulumi.String("patch-baseline"),
			Description: pulumi.String("Patch Baseline Description"),
			ApprovedPatches: pulumi.StringArray{
				pulumi.String("KB123456"),
				pulumi.String("KB456789"),
			},
			RejectedPatches: pulumi.StringArray{
				pulumi.String("KB987654"),
			},
			GlobalFilters: ssm.PatchBaselineGlobalFilterArray{
				&ssm.PatchBaselineGlobalFilterArgs{
					Key: pulumi.String("PRODUCT"),
					Values: pulumi.StringArray{
						pulumi.String("WindowsServer2008"),
					},
				},
				&ssm.PatchBaselineGlobalFilterArgs{
					Key: pulumi.String("CLASSIFICATION"),
					Values: pulumi.StringArray{
						pulumi.String("ServicePacks"),
					},
				},
				&ssm.PatchBaselineGlobalFilterArgs{
					Key: pulumi.String("MSRC_SEVERITY"),
					Values: pulumi.StringArray{
						pulumi.String("Low"),
					},
				},
			},
			ApprovalRules: ssm.PatchBaselineApprovalRuleArray{
				&ssm.PatchBaselineApprovalRuleArgs{
					ApproveAfterDays: pulumi.Int(7),
					ComplianceLevel:  pulumi.String("HIGH"),
					PatchFilters: ssm.PatchBaselineApprovalRulePatchFilterArray{
						&ssm.PatchBaselineApprovalRulePatchFilterArgs{
							Key: pulumi.String("PRODUCT"),
							Values: pulumi.StringArray{
								pulumi.String("WindowsServer2016"),
							},
						},
						&ssm.PatchBaselineApprovalRulePatchFilterArgs{
							Key: pulumi.String("CLASSIFICATION"),
							Values: pulumi.StringArray{
								pulumi.String("CriticalUpdates"),
								pulumi.String("SecurityUpdates"),
								pulumi.String("Updates"),
							},
						},
						&ssm.PatchBaselineApprovalRulePatchFilterArgs{
							Key: pulumi.String("MSRC_SEVERITY"),
							Values: pulumi.StringArray{
								pulumi.String("Critical"),
								pulumi.String("Important"),
								pulumi.String("Moderate"),
							},
						},
					},
				},
				&ssm.PatchBaselineApprovalRuleArgs{
					ApproveAfterDays: pulumi.Int(7),
					PatchFilters: ssm.PatchBaselineApprovalRulePatchFilterArray{
						&ssm.PatchBaselineApprovalRulePatchFilterArgs{
							Key: pulumi.String("PRODUCT"),
							Values: pulumi.StringArray{
								pulumi.String("WindowsServer2012"),
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.PatchBaseline;
import com.pulumi.aws.ssm.PatchBaselineArgs;
import com.pulumi.aws.ssm.inputs.PatchBaselineGlobalFilterArgs;
import com.pulumi.aws.ssm.inputs.PatchBaselineApprovalRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var production = new PatchBaseline("production", PatchBaselineArgs.builder()
            .name("patch-baseline")
            .description("Patch Baseline Description")
            .approvedPatches(            
                "KB123456",
                "KB456789")
            .rejectedPatches("KB987654")
            .globalFilters(            
                PatchBaselineGlobalFilterArgs.builder()
                    .key("PRODUCT")
                    .values("WindowsServer2008")
                    .build(),
                PatchBaselineGlobalFilterArgs.builder()
                    .key("CLASSIFICATION")
                    .values("ServicePacks")
                    .build(),
                PatchBaselineGlobalFilterArgs.builder()
                    .key("MSRC_SEVERITY")
                    .values("Low")
                    .build())
            .approvalRules(            
                PatchBaselineApprovalRuleArgs.builder()
                    .approveAfterDays(7)
                    .complianceLevel("HIGH")
                    .patchFilters(                    
                        PatchBaselineApprovalRulePatchFilterArgs.builder()
                            .key("PRODUCT")
                            .values("WindowsServer2016")
                            .build(),
                        PatchBaselineApprovalRulePatchFilterArgs.builder()
                            .key("CLASSIFICATION")
                            .values(                            
                                "CriticalUpdates",
                                "SecurityUpdates",
                                "Updates")
                            .build(),
                        PatchBaselineApprovalRulePatchFilterArgs.builder()
                            .key("MSRC_SEVERITY")
                            .values(                            
                                "Critical",
                                "Important",
                                "Moderate")
                            .build())
                    .build(),
                PatchBaselineApprovalRuleArgs.builder()
                    .approveAfterDays(7)
                    .patchFilters(PatchBaselineApprovalRulePatchFilterArgs.builder()
                        .key("PRODUCT")
                        .values("WindowsServer2012")
                        .build())
                    .build())
            .build());

    }
}
```
```yaml
resources:
  production:
    type: aws:ssm:PatchBaseline
    properties:
      name: patch-baseline
      description: Patch Baseline Description
      approvedPatches:
        - KB123456
        - KB456789
      rejectedPatches:
        - KB987654
      globalFilters:
        - key: PRODUCT
          values:
            - WindowsServer2008
        - key: CLASSIFICATION
          values:
            - ServicePacks
        - key: MSRC_SEVERITY
          values:
            - Low
      approvalRules:
        - approveAfterDays: 7
          complianceLevel: HIGH
          patchFilters:
            - key: PRODUCT
              values:
                - WindowsServer2016
            - key: CLASSIFICATION
              values:
                - CriticalUpdates
                - SecurityUpdates
                - Updates
            - key: MSRC_SEVERITY
              values:
                - Critical
                - Important
                - Moderate
        - approveAfterDays: 7
          patchFilters:
            - key: PRODUCT
              values:
                - WindowsServer2012
```
<!--End PulumiCodeChooser -->

### Advanced usage, specifying Microsoft application and Windows patch rules

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const windowsOsApps = new aws.ssm.PatchBaseline("windows_os_apps", {
    name: "WindowsOSAndMicrosoftApps",
    description: "Patch both Windows and Microsoft apps",
    operatingSystem: "WINDOWS",
    approvalRules: [
        {
            approveAfterDays: 7,
            patchFilters: [
                {
                    key: "CLASSIFICATION",
                    values: [
                        "CriticalUpdates",
                        "SecurityUpdates",
                    ],
                },
                {
                    key: "MSRC_SEVERITY",
                    values: [
                        "Critical",
                        "Important",
                    ],
                },
            ],
        },
        {
            approveAfterDays: 7,
            patchFilters: [
                {
                    key: "PATCH_SET",
                    values: ["APPLICATION"],
                },
                {
                    key: "PRODUCT",
                    values: [
                        "Office 2013",
                        "Office 2016",
                    ],
                },
            ],
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

windows_os_apps = aws.ssm.PatchBaseline("windows_os_apps",
    name="WindowsOSAndMicrosoftApps",
    description="Patch both Windows and Microsoft apps",
    operating_system="WINDOWS",
    approval_rules=[
        {
            "approve_after_days": 7,
            "patch_filters": [
                {
                    "key": "CLASSIFICATION",
                    "values": [
                        "CriticalUpdates",
                        "SecurityUpdates",
                    ],
                },
                {
                    "key": "MSRC_SEVERITY",
                    "values": [
                        "Critical",
                        "Important",
                    ],
                },
            ],
        },
        {
            "approve_after_days": 7,
            "patch_filters": [
                {
                    "key": "PATCH_SET",
                    "values": ["APPLICATION"],
                },
                {
                    "key": "PRODUCT",
                    "values": [
                        "Office 2013",
                        "Office 2016",
                    ],
                },
            ],
        },
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var windowsOsApps = new Aws.Ssm.PatchBaseline("windows_os_apps", new()
    {
        Name = "WindowsOSAndMicrosoftApps",
        Description = "Patch both Windows and Microsoft apps",
        OperatingSystem = "WINDOWS",
        ApprovalRules = new[]
        {
            new Aws.Ssm.Inputs.PatchBaselineApprovalRuleArgs
            {
                ApproveAfterDays = 7,
                PatchFilters = new[]
                {
                    new Aws.Ssm.Inputs.PatchBaselineApprovalRulePatchFilterArgs
                    {
                        Key = "CLASSIFICATION",
                        Values = new[]
                        {
                            "CriticalUpdates",
                            "SecurityUpdates",
                        },
                    },
                    new Aws.Ssm.Inputs.PatchBaselineApprovalRulePatchFilterArgs
                    {
                        Key = "MSRC_SEVERITY",
                        Values = new[]
                        {
                            "Critical",
                            "Important",
                        },
                    },
                },
            },
            new Aws.Ssm.Inputs.PatchBaselineApprovalRuleArgs
            {
                ApproveAfterDays = 7,
                PatchFilters = new[]
                {
                    new Aws.Ssm.Inputs.PatchBaselineApprovalRulePatchFilterArgs
                    {
                        Key = "PATCH_SET",
                        Values = new[]
                        {
                            "APPLICATION",
                        },
                    },
                    new Aws.Ssm.Inputs.PatchBaselineApprovalRulePatchFilterArgs
                    {
                        Key = "PRODUCT",
                        Values = new[]
                        {
                            "Office 2013",
                            "Office 2016",
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewPatchBaseline(ctx, "windows_os_apps", &ssm.PatchBaselineArgs{
			Name:            pulumi.String("WindowsOSAndMicrosoftApps"),
			Description:     pulumi.String("Patch both Windows and Microsoft apps"),
			OperatingSystem: pulumi.String("WINDOWS"),
			ApprovalRules: ssm.PatchBaselineApprovalRuleArray{
				&ssm.PatchBaselineApprovalRuleArgs{
					ApproveAfterDays: pulumi.Int(7),
					PatchFilters: ssm.PatchBaselineApprovalRulePatchFilterArray{
						&ssm.PatchBaselineApprovalRulePatchFilterArgs{
							Key: pulumi.String("CLASSIFICATION"),
							Values: pulumi.StringArray{
								pulumi.String("CriticalUpdates"),
								pulumi.String("SecurityUpdates"),
							},
						},
						&ssm.PatchBaselineApprovalRulePatchFilterArgs{
							Key: pulumi.String("MSRC_SEVERITY"),
							Values: pulumi.StringArray{
								pulumi.String("Critical"),
								pulumi.String("Important"),
							},
						},
					},
				},
				&ssm.PatchBaselineApprovalRuleArgs{
					ApproveAfterDays: pulumi.Int(7),
					PatchFilters: ssm.PatchBaselineApprovalRulePatchFilterArray{
						&ssm.PatchBaselineApprovalRulePatchFilterArgs{
							Key: pulumi.String("PATCH_SET"),
							Values: pulumi.StringArray{
								pulumi.String("APPLICATION"),
							},
						},
						&ssm.PatchBaselineApprovalRulePatchFilterArgs{
							Key: pulumi.String("PRODUCT"),
							Values: pulumi.StringArray{
								pulumi.String("Office 2013"),
								pulumi.String("Office 2016"),
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.PatchBaseline;
import com.pulumi.aws.ssm.PatchBaselineArgs;
import com.pulumi.aws.ssm.inputs.PatchBaselineApprovalRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var windowsOsApps = new PatchBaseline("windowsOsApps", PatchBaselineArgs.builder()
            .name("WindowsOSAndMicrosoftApps")
            .description("Patch both Windows and Microsoft apps")
            .operatingSystem("WINDOWS")
            .approvalRules(            
                PatchBaselineApprovalRuleArgs.builder()
                    .approveAfterDays(7)
                    .patchFilters(                    
                        PatchBaselineApprovalRulePatchFilterArgs.builder()
                            .key("CLASSIFICATION")
                            .values(                            
                                "CriticalUpdates",
                                "SecurityUpdates")
                            .build(),
                        PatchBaselineApprovalRulePatchFilterArgs.builder()
                            .key("MSRC_SEVERITY")
                            .values(                            
                                "Critical",
                                "Important")
                            .build())
                    .build(),
                PatchBaselineApprovalRuleArgs.builder()
                    .approveAfterDays(7)
                    .patchFilters(                    
                        PatchBaselineApprovalRulePatchFilterArgs.builder()
                            .key("PATCH_SET")
                            .values("APPLICATION")
                            .build(),
                        PatchBaselineApprovalRulePatchFilterArgs.builder()
                            .key("PRODUCT")
                            .values(                            
                                "Office 2013",
                                "Office 2016")
                            .build())
                    .build())
            .build());

    }
}
```
```yaml
resources:
  windowsOsApps:
    type: aws:ssm:PatchBaseline
    name: windows_os_apps
    properties:
      name: WindowsOSAndMicrosoftApps
      description: Patch both Windows and Microsoft apps
      operatingSystem: WINDOWS
      approvalRules:
        - approveAfterDays: 7
          patchFilters:
            - key: CLASSIFICATION
              values:
                - CriticalUpdates
                - SecurityUpdates
            - key: MSRC_SEVERITY
              values:
                - Critical
                - Important
        - approveAfterDays: 7
          patchFilters:
            - key: PATCH_SET
              values:
                - APPLICATION
            - key: PRODUCT
              values:
                - Office 2013
                - Office 2016
```
<!--End PulumiCodeChooser -->

### Advanced usage, specifying alternate patch source repository

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const al201709 = new aws.ssm.PatchBaseline("al_2017_09", {
    approvalRules: [{}],
    name: "Amazon-Linux-2017.09",
    description: "My patch repository for Amazon Linux 2017.09",
    operatingSystem: "AMAZON_LINUX",
    sources: [{
        name: "My-AL2017.09",
        products: ["AmazonLinux2017.09"],
        configuration: `[amzn-main]
name=amzn-main-Base
mirrorlist=http://repo./awsregion./awsdomain//releasever/main/mirror.list
mirrorlist_expire=300
metadata_expire=300
priority=10
failovermethod=priority
fastestmirror_enabled=0
gpgcheck=1
gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-amazon-ga
enabled=1
retries=3
timeout=5
report_instanceid=yes
`,
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

al201709 = aws.ssm.PatchBaseline("al_2017_09",
    approval_rules=[{}],
    name="Amazon-Linux-2017.09",
    description="My patch repository for Amazon Linux 2017.09",
    operating_system="AMAZON_LINUX",
    sources=[{
        "name": "My-AL2017.09",
        "products": ["AmazonLinux2017.09"],
        "configuration": """[amzn-main]
name=amzn-main-Base
mirrorlist=http://repo./$awsregion./$awsdomain//$releasever/main/mirror.list
mirrorlist_expire=300
metadata_expire=300
priority=10
failovermethod=priority
fastestmirror_enabled=0
gpgcheck=1
gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-amazon-ga
enabled=1
retries=3
timeout=5
report_instanceid=yes
""",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var al201709 = new Aws.Ssm.PatchBaseline("al_2017_09", new()
    {
        ApprovalRules = new[]
        {
            null,
        },
        Name = "Amazon-Linux-2017.09",
        Description = "My patch repository for Amazon Linux 2017.09",
        OperatingSystem = "AMAZON_LINUX",
        Sources = new[]
        {
            new Aws.Ssm.Inputs.PatchBaselineSourceArgs
            {
                Name = "My-AL2017.09",
                Products = new[]
                {
                    "AmazonLinux2017.09",
                },
                Configuration = @"[amzn-main]
name=amzn-main-Base
mirrorlist=http://repo./$awsregion./$awsdomain//$releasever/main/mirror.list
mirrorlist_expire=300
metadata_expire=300
priority=10
failovermethod=priority
fastestmirror_enabled=0
gpgcheck=1
gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-amazon-ga
enabled=1
retries=3
timeout=5
report_instanceid=yes
",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewPatchBaseline(ctx, "al_2017_09", &ssm.PatchBaselineArgs{
			ApprovalRules: ssm.PatchBaselineApprovalRuleArray{
				&ssm.PatchBaselineApprovalRuleArgs{},
			},
			Name:            pulumi.String("Amazon-Linux-2017.09"),
			Description:     pulumi.String("My patch repository for Amazon Linux 2017.09"),
			OperatingSystem: pulumi.String("AMAZON_LINUX"),
			Sources: ssm.PatchBaselineSourceArray{
				&ssm.PatchBaselineSourceArgs{
					Name: pulumi.String("My-AL2017.09"),
					Products: pulumi.StringArray{
						pulumi.String("AmazonLinux2017.09"),
					},
					Configuration: pulumi.String(`[amzn-main]
name=amzn-main-Base
mirrorlist=http://repo./$awsregion./$awsdomain//$releasever/main/mirror.list
mirrorlist_expire=300
metadata_expire=300
priority=10
failovermethod=priority
fastestmirror_enabled=0
gpgcheck=1
gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-amazon-ga
enabled=1
retries=3
timeout=5
report_instanceid=yes
`),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.PatchBaseline;
import com.pulumi.aws.ssm.PatchBaselineArgs;
import com.pulumi.aws.ssm.inputs.PatchBaselineApprovalRuleArgs;
import com.pulumi.aws.ssm.inputs.PatchBaselineSourceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var al201709 = new PatchBaseline("al201709", PatchBaselineArgs.builder()
            .approvalRules()
            .name("Amazon-Linux-2017.09")
            .description("My patch repository for Amazon Linux 2017.09")
            .operatingSystem("AMAZON_LINUX")
            .sources(PatchBaselineSourceArgs.builder()
                .name("My-AL2017.09")
                .products("AmazonLinux2017.09")
                .configuration("""
[amzn-main]
name=amzn-main-Base
mirrorlist=http://repo./$awsregion./$awsdomain//$releasever/main/mirror.list
mirrorlist_expire=300
metadata_expire=300
priority=10
failovermethod=priority
fastestmirror_enabled=0
gpgcheck=1
gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-amazon-ga
enabled=1
retries=3
timeout=5
report_instanceid=yes
                """)
                .build())
            .build());

    }
}
```
```yaml
resources:
  al201709:
    type: aws:ssm:PatchBaseline
    name: al_2017_09
    properties:
      approvalRules:
        - {}
      name: Amazon-Linux-2017.09
      description: My patch repository for Amazon Linux 2017.09
      operatingSystem: AMAZON_LINUX
      sources:
        - name: My-AL2017.09
          products:
            - AmazonLinux2017.09
          configuration: |
            [amzn-main]
            name=amzn-main-Base
            mirrorlist=http://repo./$awsregion./$awsdomain//$releasever/main/mirror.list
            mirrorlist_expire=300
            metadata_expire=300
            priority=10
            failovermethod=priority
            fastestmirror_enabled=0
            gpgcheck=1
            gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-amazon-ga
            enabled=1
            retries=3
            timeout=5
            report_instanceid=yes
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM Patch Baselines using their baseline ID. For example:

```sh
$ pulumi import aws:ssm/patchBaseline:PatchBaseline example pb-12345678
```
�
approvalRuleseBc*a:_
]
ssmPatchBaselineApprovalRule;aws:ssm/PatchBaselineApprovalRule:PatchBaselineApprovalRule{Set of rules used to include patches in the baseline. Up to 10 approval rules can be specified. See `approval_rule` below.
{
approvedPatchesB*" `List of explicitly approved patches for the baseline. Cannot be specified with `approval_rule`.
�
approvedPatchesComplianceLevelB" �Compliance level for approved patches. This means that if an approved patch is reported as missing, this is the severity of the compliance violation. Valid values are `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`, `INFORMATIONAL`, `UNSPECIFIED`. The default value is `UNSPECIFIED`.
�
 approvedPatchesEnableNonSecurityB
 �Whether the list of approved patches includes non-security updates that should be applied to the instances. Applies to Linux instances only.
8
descriptionB" #Description of the patch baseline.
�
globalFilterseBc*a:_
]
ssmPatchBaselineGlobalFilter;aws:ssm/PatchBaselineGlobalFilter:PatchBaselineGlobalFilter�Set of global filters used to exclude patches from the baseline. Up to 4 global filters can be specified using Key/Value pairs. Valid Keys are `PRODUCT`, `CLASSIFICATION`, `MSRC_SEVERITY`, and `PATCH_ID`.
Q
nameB" CName of the patch baseline.

The following arguments are optional:
�
operatingSystemB" �Operating system the patch baseline applies to. Valid values are `ALMA_LINUX`, `AMAZON_LINUX`, `AMAZON_LINUX_2`, `AMAZON_LINUX_2022`, `AMAZON_LINUX_2023`, `CENTOS`, `DEBIAN`, `MACOS`, `ORACLE_LINUX`, `RASPBIAN`, `REDHAT_ENTERPRISE_LINUX`, `ROCKY_LINUX`, `SUSE`, `UBUNTU`, and `WINDOWS`. The default value is `WINDOWS`.
5
rejectedPatchesB*" List of rejected patches.
�
rejectedPatchesActionB" �Action for Patch Manager to take on patches included in the `rejected_patches` list. Valid values are `ALLOW_AS_DEPENDENCY` and `BLOCK`.
�
sourcesSBQ*O:M
K
ssmPatchBaselineSource/aws:ssm/PatchBaselineSource:PatchBaselineSourcemConfiguration block with alternate sources for patches. Applies to Linux instances only. See `source` below.
�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
approvalRuleseBc*a:_
]
ssmPatchBaselineApprovalRule;aws:ssm/PatchBaselineApprovalRule:PatchBaselineApprovalRule{Set of rules used to include patches in the baseline. Up to 10 approval rules can be specified. See `approval_rule` below.
"{
approvedPatchesB*" `List of explicitly approved patches for the baseline. Cannot be specified with `approval_rule`.
"�
approvedPatchesComplianceLevelB" �Compliance level for approved patches. This means that if an approved patch is reported as missing, this is the severity of the compliance violation. Valid values are `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`, `INFORMATIONAL`, `UNSPECIFIED`. The default value is `UNSPECIFIED`.
"�
 approvedPatchesEnableNonSecurityB
 �Whether the list of approved patches includes non-security updates that should be applied to the instances. Applies to Linux instances only.
" 
arn" ARN of the baseline.
"8
descriptionB" #Description of the patch baseline.
"�
globalFilterseBc*a:_
]
ssmPatchBaselineGlobalFilter;aws:ssm/PatchBaselineGlobalFilter:PatchBaselineGlobalFilter�Set of global filters used to exclude patches from the baseline. Up to 4 global filters can be specified using Key/Value pairs. Valid Keys are `PRODUCT`, `CLASSIFICATION`, `MSRC_SEVERITY`, and `PATCH_ID`.
"-
json" !JSON definition of the baseline.
"O
name" CName of the patch baseline.

The following arguments are optional:
"�
operatingSystemB" �Operating system the patch baseline applies to. Valid values are `ALMA_LINUX`, `AMAZON_LINUX`, `AMAZON_LINUX_2`, `AMAZON_LINUX_2022`, `AMAZON_LINUX_2023`, `CENTOS`, `DEBIAN`, `MACOS`, `ORACLE_LINUX`, `RASPBIAN`, `REDHAT_ENTERPRISE_LINUX`, `ROCKY_LINUX`, `SUSE`, `UBUNTU`, and `WINDOWS`. The default value is `WINDOWS`.
"5
rejectedPatchesB*" List of rejected patches.
"�
rejectedPatchesAction" �Action for Patch Manager to take on patches included in the `rejected_patches` list. Valid values are `ALLOW_AS_DEPENDENCY` and `BLOCK`.
"�
sourcesSBQ*O:M
K
ssmPatchBaselineSource/aws:ssm/PatchBaselineSource:PatchBaselineSourcemConfiguration block with alternate sources for patches. Applies to Linux instances only. See `source` below.
"�
tagsB2" �Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*�
0
ssm
PatchGroupaws:ssm/patchGroup:PatchGroup�Provides an SSM Patch Group resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const production = new aws.ssm.PatchBaseline("production", {
    name: "patch-baseline",
    approvedPatches: ["KB123456"],
});
const patchgroup = new aws.ssm.PatchGroup("patchgroup", {
    baselineId: production.id,
    patchGroup: "patch-group-name",
});
```
```python
import pulumi
import pulumi_aws as aws

production = aws.ssm.PatchBaseline("production",
    name="patch-baseline",
    approved_patches=["KB123456"])
patchgroup = aws.ssm.PatchGroup("patchgroup",
    baseline_id=production.id,
    patch_group="patch-group-name")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var production = new Aws.Ssm.PatchBaseline("production", new()
    {
        Name = "patch-baseline",
        ApprovedPatches = new[]
        {
            "KB123456",
        },
    });

    var patchgroup = new Aws.Ssm.PatchGroup("patchgroup", new()
    {
        BaselineId = production.Id,
        PatchGroupName = "patch-group-name",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		production, err := ssm.NewPatchBaseline(ctx, "production", &ssm.PatchBaselineArgs{
			Name: pulumi.String("patch-baseline"),
			ApprovedPatches: pulumi.StringArray{
				pulumi.String("KB123456"),
			},
		})
		if err != nil {
			return err
		}
		_, err = ssm.NewPatchGroup(ctx, "patchgroup", &ssm.PatchGroupArgs{
			BaselineId: production.ID(),
			PatchGroup: pulumi.String("patch-group-name"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.PatchBaseline;
import com.pulumi.aws.ssm.PatchBaselineArgs;
import com.pulumi.aws.ssm.PatchGroup;
import com.pulumi.aws.ssm.PatchGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var production = new PatchBaseline("production", PatchBaselineArgs.builder()
            .name("patch-baseline")
            .approvedPatches("KB123456")
            .build());

        var patchgroup = new PatchGroup("patchgroup", PatchGroupArgs.builder()
            .baselineId(production.id())
            .patchGroup("patch-group-name")
            .build());

    }
}
```
```yaml
resources:
  production:
    type: aws:ssm:PatchBaseline
    properties:
      name: patch-baseline
      approvedPatches:
        - KB123456
  patchgroup:
    type: aws:ssm:PatchGroup
    properties:
      baselineId: ${production.id}
      patchGroup: patch-group-name
```
<!--End PulumiCodeChooser -->
Q

baselineId" ?The ID of the patch baseline to register the patch group with.
a

patchGroup" OThe name of the patch group that should be registered with the patch baseline.
"Q

baselineId" ?The ID of the patch baseline to register the patch group with.
"a

patchGroup" OThe name of the patch group that should be registered with the patch baseline.
*�M
l
ssmQuicksetupConfigurationManagerEaws:ssm/quicksetupConfigurationManager:QuicksetupConfigurationManager�:Resource for managing an AWS SSM Quick Setup Configuration Manager.

## Example Usage

### Patch Policy Configuration Type

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getCallerIdentity({});
const currentGetPartition = aws.getPartition({});
const currentGetRegion = aws.getRegion({});
const example = aws.ssm.getPatchBaselines({
    defaultBaselines: true,
});
// transform the output of the aws_ssm_patch_baselines data source
// into the format expected by the SelectedPatchBaselines parameter
const selectedPatchBaselines = JSON.stringify(example.then(example => .reduce((__obj, baseline) => ({ ...__obj, [baseline.operatingSystem]: {
    value: baseline.baselineId,
    label: baseline.baselineName,
    description: baseline.baselineDescription,
    disabled: !baseline.defaultBaseline,
} }))));
const exampleQuicksetupConfigurationManager = new aws.ssm.QuicksetupConfigurationManager("example", {
    name: "example",
    configurationDefinition: {
        localDeploymentAdministrationRoleArn: Promise.all([currentGetPartition, current]).then(([currentGetPartition, current]) => `arn:${currentGetPartition.partition}:iam::${current.accountId}:role/AWS-QuickSetup-PatchPolicy-LocalAdministrationRole`),
        localDeploymentExecutionRoleName: "AWS-QuickSetup-PatchPolicy-LocalExecutionRole",
        type: "AWSQuickSetupType-PatchPolicy",
        parameters: {
            ConfigurationOptionsPatchOperation: "Scan",
            ConfigurationOptionsScanValue: "cron(0 1 * * ? *)",
            ConfigurationOptionsScanNextInterval: "false",
            PatchBaselineRegion: currentGetRegion.then(currentGetRegion => currentGetRegion.name),
            PatchBaselineUseDefault: "default",
            PatchPolicyName: "example",
            SelectedPatchBaselines: selectedPatchBaselines,
            OutputLogEnableS3: "false",
            RateControlConcurrency: "10%",
            RateControlErrorThreshold: "2%",
            IsPolicyAttachAllowed: "false",
            TargetAccounts: current.then(current => current.accountId),
            TargetRegions: currentGetRegion.then(currentGetRegion => currentGetRegion.name),
            TargetType: "*",
        },
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

current = aws.get_caller_identity()
current_get_partition = aws.get_partition()
current_get_region = aws.get_region()
example = aws.ssm.get_patch_baselines(default_baselines=True)
# transform the output of the aws_ssm_patch_baselines data source
# into the format expected by the SelectedPatchBaselines parameter
selected_patch_baselines = json.dumps({baseline.operating_system: {
    "value": baseline.baseline_id,
    "label": baseline.baseline_name,
    "description": baseline.baseline_description,
    "disabled": not baseline.default_baseline,
} for baseline in example.baseline_identities})
example_quicksetup_configuration_manager = aws.ssm.QuicksetupConfigurationManager("example",
    name="example",
    configuration_definition={
        "local_deployment_administration_role_arn": f"arn:{current_get_partition.partition}:iam::{current.account_id}:role/AWS-QuickSetup-PatchPolicy-LocalAdministrationRole",
        "local_deployment_execution_role_name": "AWS-QuickSetup-PatchPolicy-LocalExecutionRole",
        "type": "AWSQuickSetupType-PatchPolicy",
        "parameters": {
            "ConfigurationOptionsPatchOperation": "Scan",
            "ConfigurationOptionsScanValue": "cron(0 1 * * ? *)",
            "ConfigurationOptionsScanNextInterval": "false",
            "PatchBaselineRegion": current_get_region.name,
            "PatchBaselineUseDefault": "default",
            "PatchPolicyName": "example",
            "SelectedPatchBaselines": selected_patch_baselines,
            "OutputLogEnableS3": "false",
            "RateControlConcurrency": "10%",
            "RateControlErrorThreshold": "2%",
            "IsPolicyAttachAllowed": "false",
            "TargetAccounts": current.account_id,
            "TargetRegions": current_get_region.name,
            "TargetType": "*",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetCallerIdentity.Invoke();

    var currentGetPartition = Aws.GetPartition.Invoke();

    var currentGetRegion = Aws.GetRegion.Invoke();

    var example = Aws.Ssm.GetPatchBaselines.Invoke(new()
    {
        DefaultBaselines = true,
    });

    // transform the output of the aws_ssm_patch_baselines data source
    // into the format expected by the SelectedPatchBaselines parameter
    var selectedPatchBaselines = JsonSerializer.Serialize(.ToDictionary(item => {
        var baseline = item.Value;
        return baseline.OperatingSystem;
    }, item => {
        var baseline = item.Value;
        return 
        {
            { "value", baseline.BaselineId },
            { "label", baseline.BaselineName },
            { "description", baseline.BaselineDescription },
            { "disabled", !baseline.DefaultBaseline },
        };
    }));

    var exampleQuicksetupConfigurationManager = new Aws.Ssm.QuicksetupConfigurationManager("example", new()
    {
        Name = "example",
        ConfigurationDefinition = new Aws.Ssm.Inputs.QuicksetupConfigurationManagerConfigurationDefinitionArgs
        {
            LocalDeploymentAdministrationRoleArn = Output.Tuple(currentGetPartition, current).Apply(values =>
            {
                var currentGetPartition = values.Item1;
                var current = values.Item2;
                return $"arn:{currentGetPartition.Apply(getPartitionResult => getPartitionResult.Partition)}:iam::{current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId)}:role/AWS-QuickSetup-PatchPolicy-LocalAdministrationRole";
            }),
            LocalDeploymentExecutionRoleName = "AWS-QuickSetup-PatchPolicy-LocalExecutionRole",
            Type = "AWSQuickSetupType-PatchPolicy",
            Parameters = 
            {
                { "ConfigurationOptionsPatchOperation", "Scan" },
                { "ConfigurationOptionsScanValue", "cron(0 1 * * ? *)" },
                { "ConfigurationOptionsScanNextInterval", "false" },
                { "PatchBaselineRegion", currentGetRegion.Apply(getRegionResult => getRegionResult.Name) },
                { "PatchBaselineUseDefault", "default" },
                { "PatchPolicyName", "example" },
                { "SelectedPatchBaselines", selectedPatchBaselines },
                { "OutputLogEnableS3", "false" },
                { "RateControlConcurrency", "10%" },
                { "RateControlErrorThreshold", "2%" },
                { "IsPolicyAttachAllowed", "false" },
                { "TargetAccounts", current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId) },
                { "TargetRegions", currentGetRegion.Apply(getRegionResult => getRegionResult.Name) },
                { "TargetType", "*" },
            },
        },
    });

});
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM Quick Setup Configuration Manager using the `manager_arn`. For example:

```sh
$ pulumi import aws:ssm/quicksetupConfigurationManager:QuicksetupConfigurationManager example arn:aws:ssm-quicksetup:us-east-1:012345678901:configuration-manager/abcd-1234
```
�
configurationDefinition�B�:�
�
ssm5QuicksetupConfigurationManagerConfigurationDefinitionsaws:ssm/QuicksetupConfigurationManagerConfigurationDefinition:QuicksetupConfigurationManagerConfigurationDefinitionzDefinition of the Quick Setup configuration that the configuration manager deploys. See `configuration_definition` below.
?
descriptionB" *Description of the configuration manager.
Q
nameB" CConfiguration manager name.

The following arguments are optional:
�
tagsB2" �Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
�
timeouts�B�:�
�
ssm&QuicksetupConfigurationManagerTimeoutsUaws:ssm/QuicksetupConfigurationManagerTimeouts:QuicksetupConfigurationManagerTimeouts"�
configurationDefinition�B�:�
�
ssm5QuicksetupConfigurationManagerConfigurationDefinitionsaws:ssm/QuicksetupConfigurationManagerConfigurationDefinition:QuicksetupConfigurationManagerConfigurationDefinitionzDefinition of the Quick Setup configuration that the configuration manager deploys. See `configuration_definition` below.
"=
description" *Description of the configuration manager.
"4

managerArn" "ARN of the Configuration Manager.
"O
name" CConfiguration manager name.

The following arguments are optional:
"�
statusSummaries�*�:�
�
ssm+QuicksetupConfigurationManagerStatusSummary_aws:ssm/QuicksetupConfigurationManagerStatusSummary:QuicksetupConfigurationManagerStatusSummary�A summary of the state of the configuration manager. This includes deployment statuses, association statuses, drift statuses, health checks, and more. See `status_summaries` below.
"�
tagsB2" �Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
timeouts�B�:�
�
ssm&QuicksetupConfigurationManagerTimeoutsUaws:ssm/QuicksetupConfigurationManagerTimeouts:QuicksetupConfigurationManagerTimeouts*�d
B
ssmResourceDataSync)aws:ssm/resourceDataSync:ResourceDataSync�`Provides a SSM resource data sync.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const hogeBucketV2 = new aws.s3.BucketV2("hoge", {bucket: "tf-test-bucket-1234"});
const hoge = aws.iam.getPolicyDocument({
    statements: [
        {
            sid: "SSMBucketPermissionsCheck",
            effect: "Allow",
            principals: [{
                type: "Service",
                identifiers: ["ssm.amazonaws.com"],
            }],
            actions: ["s3:GetBucketAcl"],
            resources: ["arn:aws:s3:::tf-test-bucket-1234"],
        },
        {
            sid: "SSMBucketDelivery",
            effect: "Allow",
            principals: [{
                type: "Service",
                identifiers: ["ssm.amazonaws.com"],
            }],
            actions: ["s3:PutObject"],
            resources: ["arn:aws:s3:::tf-test-bucket-1234/*"],
            conditions: [{
                test: "StringEquals",
                variable: "s3:x-amz-acl",
                values: ["bucket-owner-full-control"],
            }],
        },
    ],
});
const hogeBucketPolicy = new aws.s3.BucketPolicy("hoge", {
    bucket: hogeBucketV2.id,
    policy: hoge.then(hoge => hoge.json),
});
const foo = new aws.ssm.ResourceDataSync("foo", {
    name: "foo",
    s3Destination: {
        bucketName: hogeBucketV2.bucket,
        region: hogeBucketV2.region,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

hoge_bucket_v2 = aws.s3.BucketV2("hoge", bucket="tf-test-bucket-1234")
hoge = aws.iam.get_policy_document(statements=[
    {
        "sid": "SSMBucketPermissionsCheck",
        "effect": "Allow",
        "principals": [{
            "type": "Service",
            "identifiers": ["ssm.amazonaws.com"],
        }],
        "actions": ["s3:GetBucketAcl"],
        "resources": ["arn:aws:s3:::tf-test-bucket-1234"],
    },
    {
        "sid": "SSMBucketDelivery",
        "effect": "Allow",
        "principals": [{
            "type": "Service",
            "identifiers": ["ssm.amazonaws.com"],
        }],
        "actions": ["s3:PutObject"],
        "resources": ["arn:aws:s3:::tf-test-bucket-1234/*"],
        "conditions": [{
            "test": "StringEquals",
            "variable": "s3:x-amz-acl",
            "values": ["bucket-owner-full-control"],
        }],
    },
])
hoge_bucket_policy = aws.s3.BucketPolicy("hoge",
    bucket=hoge_bucket_v2.id,
    policy=hoge.json)
foo = aws.ssm.ResourceDataSync("foo",
    name="foo",
    s3_destination={
        "bucket_name": hoge_bucket_v2.bucket,
        "region": hoge_bucket_v2.region,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var hogeBucketV2 = new Aws.S3.BucketV2("hoge", new()
    {
        Bucket = "tf-test-bucket-1234",
    });

    var hoge = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "SSMBucketPermissionsCheck",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "ssm.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:GetBucketAcl",
                },
                Resources = new[]
                {
                    "arn:aws:s3:::tf-test-bucket-1234",
                },
            },
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "SSMBucketDelivery",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "ssm.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:PutObject",
                },
                Resources = new[]
                {
                    "arn:aws:s3:::tf-test-bucket-1234/*",
                },
                Conditions = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementConditionInputArgs
                    {
                        Test = "StringEquals",
                        Variable = "s3:x-amz-acl",
                        Values = new[]
                        {
                            "bucket-owner-full-control",
                        },
                    },
                },
            },
        },
    });

    var hogeBucketPolicy = new Aws.S3.BucketPolicy("hoge", new()
    {
        Bucket = hogeBucketV2.Id,
        Policy = hoge.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var foo = new Aws.Ssm.ResourceDataSync("foo", new()
    {
        Name = "foo",
        S3Destination = new Aws.Ssm.Inputs.ResourceDataSyncS3DestinationArgs
        {
            BucketName = hogeBucketV2.Bucket,
            Region = hogeBucketV2.Region,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		hogeBucketV2, err := s3.NewBucketV2(ctx, "hoge", &s3.BucketV2Args{
			Bucket: pulumi.String("tf-test-bucket-1234"),
		})
		if err != nil {
			return err
		}
		hoge, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid:    pulumi.StringRef("SSMBucketPermissionsCheck"),
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"ssm.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"s3:GetBucketAcl",
					},
					Resources: []string{
						"arn:aws:s3:::tf-test-bucket-1234",
					},
				},
				{
					Sid:    pulumi.StringRef("SSMBucketDelivery"),
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"ssm.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"s3:PutObject",
					},
					Resources: []string{
						"arn:aws:s3:::tf-test-bucket-1234/*",
					},
					Conditions: []iam.GetPolicyDocumentStatementCondition{
						{
							Test:     "StringEquals",
							Variable: "s3:x-amz-acl",
							Values: []string{
								"bucket-owner-full-control",
							},
						},
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = s3.NewBucketPolicy(ctx, "hoge", &s3.BucketPolicyArgs{
			Bucket: hogeBucketV2.ID(),
			Policy: pulumi.String(hoge.Json),
		})
		if err != nil {
			return err
		}
		_, err = ssm.NewResourceDataSync(ctx, "foo", &ssm.ResourceDataSyncArgs{
			Name: pulumi.String("foo"),
			S3Destination: &ssm.ResourceDataSyncS3DestinationArgs{
				BucketName: hogeBucketV2.Bucket,
				Region:     hogeBucketV2.Region,
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.s3.BucketPolicy;
import com.pulumi.aws.s3.BucketPolicyArgs;
import com.pulumi.aws.ssm.ResourceDataSync;
import com.pulumi.aws.ssm.ResourceDataSyncArgs;
import com.pulumi.aws.ssm.inputs.ResourceDataSyncS3DestinationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var hogeBucketV2 = new BucketV2("hogeBucketV2", BucketV2Args.builder()
            .bucket("tf-test-bucket-1234")
            .build());

        final var hoge = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(            
                GetPolicyDocumentStatementArgs.builder()
                    .sid("SSMBucketPermissionsCheck")
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("Service")
                        .identifiers("ssm.amazonaws.com")
                        .build())
                    .actions("s3:GetBucketAcl")
                    .resources("arn:aws:s3:::tf-test-bucket-1234")
                    .build(),
                GetPolicyDocumentStatementArgs.builder()
                    .sid("SSMBucketDelivery")
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("Service")
                        .identifiers("ssm.amazonaws.com")
                        .build())
                    .actions("s3:PutObject")
                    .resources("arn:aws:s3:::tf-test-bucket-1234/*")
                    .conditions(GetPolicyDocumentStatementConditionArgs.builder()
                        .test("StringEquals")
                        .variable("s3:x-amz-acl")
                        .values("bucket-owner-full-control")
                        .build())
                    .build())
            .build());

        var hogeBucketPolicy = new BucketPolicy("hogeBucketPolicy", BucketPolicyArgs.builder()
            .bucket(hogeBucketV2.id())
            .policy(hoge.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var foo = new ResourceDataSync("foo", ResourceDataSyncArgs.builder()
            .name("foo")
            .s3Destination(ResourceDataSyncS3DestinationArgs.builder()
                .bucketName(hogeBucketV2.bucket())
                .region(hogeBucketV2.region())
                .build())
            .build());

    }
}
```
```yaml
resources:
  hogeBucketV2:
    type: aws:s3:BucketV2
    name: hoge
    properties:
      bucket: tf-test-bucket-1234
  hogeBucketPolicy:
    type: aws:s3:BucketPolicy
    name: hoge
    properties:
      bucket: ${hogeBucketV2.id}
      policy: ${hoge.json}
  foo:
    type: aws:ssm:ResourceDataSync
    properties:
      name: foo
      s3Destination:
        bucketName: ${hogeBucketV2.bucket}
        region: ${hogeBucketV2.region}
variables:
  hoge:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: SSMBucketPermissionsCheck
            effect: Allow
            principals:
              - type: Service
                identifiers:
                  - ssm.amazonaws.com
            actions:
              - s3:GetBucketAcl
            resources:
              - arn:aws:s3:::tf-test-bucket-1234
          - sid: SSMBucketDelivery
            effect: Allow
            principals:
              - type: Service
                identifiers:
                  - ssm.amazonaws.com
            actions:
              - s3:PutObject
            resources:
              - arn:aws:s3:::tf-test-bucket-1234/*
            conditions:
              - test: StringEquals
                variable: s3:x-amz-acl
                values:
                  - bucket-owner-full-control
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM resource data sync using the `name`. For example:

```sh
$ pulumi import aws:ssm/resourceDataSync:ResourceDataSync example example-name
```
*
nameB" Name for the configuration.
�
s3Destinationm:k
i
ssmResourceDataSyncS3DestinationCaws:ssm/ResourceDataSyncS3Destination:ResourceDataSyncS3Destination.Amazon S3 configuration details for the sync.
"(
name" Name for the configuration.
"�
s3Destinationm:k
i
ssmResourceDataSyncS3DestinationCaws:ssm/ResourceDataSyncS3Destination:ResourceDataSyncS3Destination.Amazon S3 configuration details for the sync.
*�
<
ssmServiceSetting%aws:ssm/serviceSetting:ServiceSetting�This setting defines how a user interacts with or uses a service or a feature of a service.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testSetting = new aws.ssm.ServiceSetting("test_setting", {
    settingId: "arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled",
    settingValue: "true",
});
```
```python
import pulumi
import pulumi_aws as aws

test_setting = aws.ssm.ServiceSetting("test_setting",
    setting_id="arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled",
    setting_value="true")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testSetting = new Aws.Ssm.ServiceSetting("test_setting", new()
    {
        SettingId = "arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled",
        SettingValue = "true",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.NewServiceSetting(ctx, "test_setting", &ssm.ServiceSettingArgs{
			SettingId:    pulumi.String("arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled"),
			SettingValue: pulumi.String("true"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.ServiceSetting;
import com.pulumi.aws.ssm.ServiceSettingArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var testSetting = new ServiceSetting("testSetting", ServiceSettingArgs.builder()
            .settingId("arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled")
            .settingValue("true")
            .build());

    }
}
```
```yaml
resources:
  testSetting:
    type: aws:ssm:ServiceSetting
    name: test_setting
    properties:
      settingId: arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled
      settingValue: 'true'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AWS SSM Service Setting using the `setting_id`. For example:

```sh
$ pulumi import aws:ssm/serviceSetting:ServiceSetting example arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled
```
,
	settingId" ID of the service setting.
2
settingValue" Value of the service setting.
"'
arn" ARN of the service setting.
",
	settingId" ID of the service setting.
"2
settingValue" Value of the service setting.
"f
status" XStatus of the service setting. Value can be `Default`, `Customized` or `PendingUpdate`.
*�9
7
ssmcontactsContactaws:ssmcontacts/contact:Contact�-Resource for managing an AWS SSM Contact.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssmcontacts.Contact("example", {
    alias: "alias",
    type: "PERSONAL",
}, {
    dependsOn: [exampleAwsSsmincidentsReplicationSet],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssmcontacts.Contact("example",
    alias="alias",
    type="PERSONAL",
    opts = pulumi.ResourceOptions(depends_on=[example_aws_ssmincidents_replication_set]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsmContacts.Contact("example", new()
    {
        Alias = "alias",
        Type = "PERSONAL",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSsmincidentsReplicationSet,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmcontacts.NewContact(ctx, "example", &ssmcontacts.ContactArgs{
			Alias: pulumi.String("alias"),
			Type:  pulumi.String("PERSONAL"),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSsmincidentsReplicationSet,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.Contact;
import com.pulumi.aws.ssmcontacts.ContactArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Contact("example", ContactArgs.builder()
            .alias("alias")
            .type("PERSONAL")
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSsmincidentsReplicationSet)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssmcontacts:Contact
    properties:
      alias: alias
      type: PERSONAL
    options:
      dependsOn:
        - ${exampleAwsSsmincidentsReplicationSet}
```
<!--End PulumiCodeChooser -->

### Usage With All Fields

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssmcontacts.Contact("example", {
    alias: "alias",
    displayName: "displayName",
    type: "ESCALATION",
    tags: {
        key: "value",
    },
}, {
    dependsOn: [exampleAwsSsmincidentsReplicationSet],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssmcontacts.Contact("example",
    alias="alias",
    display_name="displayName",
    type="ESCALATION",
    tags={
        "key": "value",
    },
    opts = pulumi.ResourceOptions(depends_on=[example_aws_ssmincidents_replication_set]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsmContacts.Contact("example", new()
    {
        Alias = "alias",
        DisplayName = "displayName",
        Type = "ESCALATION",
        Tags = 
        {
            { "key", "value" },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSsmincidentsReplicationSet,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmcontacts.NewContact(ctx, "example", &ssmcontacts.ContactArgs{
			Alias:       pulumi.String("alias"),
			DisplayName: pulumi.String("displayName"),
			Type:        pulumi.String("ESCALATION"),
			Tags: pulumi.StringMap{
				"key": pulumi.String("value"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSsmincidentsReplicationSet,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.Contact;
import com.pulumi.aws.ssmcontacts.ContactArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Contact("example", ContactArgs.builder()
            .alias("alias")
            .displayName("displayName")
            .type("ESCALATION")
            .tags(Map.of("key", "value"))
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSsmincidentsReplicationSet)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssmcontacts:Contact
    properties:
      alias: alias
      displayName: displayName
      type: ESCALATION
      tags:
        key: value
    options:
      dependsOn:
        - ${exampleAwsSsmincidentsReplicationSet}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM Contact using the `ARN`. For example:

```sh
$ pulumi import aws:ssmcontacts/contact:Contact example {ARNValue}
```
�
alias" �A unique and identifiable alias for the contact or escalation plan. Must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), and hyphens (`-`).
�
displayNameB" �Full friendly name of the contact or escalation plan. If set, must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), hyphens (`-`), periods (`.`), and spaces.
7
tagsB2" 'Map of tags to assign to the resource.
�
type" �The type of contact engaged. A single contact is type PERSONAL and an escalation
plan is type ESCALATION.

The following arguments are optional:
"�
alias" �A unique and identifiable alias for the contact or escalation plan. Must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), and hyphens (`-`).
"M
arn" BThe Amazon Resource Name (ARN) of the contact or escalation plan.
"�
displayNameB" �Full friendly name of the contact or escalation plan. If set, must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), hyphens (`-`), periods (`.`), and spaces.
"7
tagsB2" 'Map of tags to assign to the resource.
"�
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"�
type" �The type of contact engaged. A single contact is type PERSONAL and an escalation
plan is type ESCALATION.

The following arguments are optional:
*�G
L
ssmcontactsContactChannel-aws:ssmcontacts/contactChannel:ContactChannel�;Resource for managing an AWS SSM Contacts Contact Channel.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssmcontacts.ContactChannel("example", {
    contactId: "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
    deliveryAddress: {
        simpleAddress: "email@example.com",
    },
    name: "Example contact channel",
    type: "EMAIL",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssmcontacts.ContactChannel("example",
    contact_id="arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
    delivery_address={
        "simple_address": "email@example.com",
    },
    name="Example contact channel",
    type="EMAIL")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsmContacts.ContactChannel("example", new()
    {
        ContactId = "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
        DeliveryAddress = new Aws.SsmContacts.Inputs.ContactChannelDeliveryAddressArgs
        {
            SimpleAddress = "email@example.com",
        },
        Name = "Example contact channel",
        Type = "EMAIL",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmcontacts.NewContactChannel(ctx, "example", &ssmcontacts.ContactChannelArgs{
			ContactId: pulumi.String("arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias"),
			DeliveryAddress: &ssmcontacts.ContactChannelDeliveryAddressArgs{
				SimpleAddress: pulumi.String("email@example.com"),
			},
			Name: pulumi.String("Example contact channel"),
			Type: pulumi.String("EMAIL"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.ContactChannel;
import com.pulumi.aws.ssmcontacts.ContactChannelArgs;
import com.pulumi.aws.ssmcontacts.inputs.ContactChannelDeliveryAddressArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ContactChannel("example", ContactChannelArgs.builder()
            .contactId("arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias")
            .deliveryAddress(ContactChannelDeliveryAddressArgs.builder()
                .simpleAddress("email@example.com")
                .build())
            .name("Example contact channel")
            .type("EMAIL")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssmcontacts:ContactChannel
    properties:
      contactId: arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias
      deliveryAddress:
        simpleAddress: email@example.com
      name: Example contact channel
      type: EMAIL
```
<!--End PulumiCodeChooser -->

### Usage with SSM Contact

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleContact = new aws.ssmcontacts.Contact("example_contact", {
    alias: "example_contact",
    type: "PERSONAL",
});
const example = new aws.ssmcontacts.ContactChannel("example", {
    contactId: exampleContact.arn,
    deliveryAddress: {
        simpleAddress: "email@example.com",
    },
    name: "Example contact channel",
    type: "EMAIL",
});
```
```python
import pulumi
import pulumi_aws as aws

example_contact = aws.ssmcontacts.Contact("example_contact",
    alias="example_contact",
    type="PERSONAL")
example = aws.ssmcontacts.ContactChannel("example",
    contact_id=example_contact.arn,
    delivery_address={
        "simple_address": "email@example.com",
    },
    name="Example contact channel",
    type="EMAIL")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleContact = new Aws.SsmContacts.Contact("example_contact", new()
    {
        Alias = "example_contact",
        Type = "PERSONAL",
    });

    var example = new Aws.SsmContacts.ContactChannel("example", new()
    {
        ContactId = exampleContact.Arn,
        DeliveryAddress = new Aws.SsmContacts.Inputs.ContactChannelDeliveryAddressArgs
        {
            SimpleAddress = "email@example.com",
        },
        Name = "Example contact channel",
        Type = "EMAIL",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleContact, err := ssmcontacts.NewContact(ctx, "example_contact", &ssmcontacts.ContactArgs{
			Alias: pulumi.String("example_contact"),
			Type:  pulumi.String("PERSONAL"),
		})
		if err != nil {
			return err
		}
		_, err = ssmcontacts.NewContactChannel(ctx, "example", &ssmcontacts.ContactChannelArgs{
			ContactId: exampleContact.Arn,
			DeliveryAddress: &ssmcontacts.ContactChannelDeliveryAddressArgs{
				SimpleAddress: pulumi.String("email@example.com"),
			},
			Name: pulumi.String("Example contact channel"),
			Type: pulumi.String("EMAIL"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.Contact;
import com.pulumi.aws.ssmcontacts.ContactArgs;
import com.pulumi.aws.ssmcontacts.ContactChannel;
import com.pulumi.aws.ssmcontacts.ContactChannelArgs;
import com.pulumi.aws.ssmcontacts.inputs.ContactChannelDeliveryAddressArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var exampleContact = new Contact("exampleContact", ContactArgs.builder()
            .alias("example_contact")
            .type("PERSONAL")
            .build());

        var example = new ContactChannel("example", ContactChannelArgs.builder()
            .contactId(exampleContact.arn())
            .deliveryAddress(ContactChannelDeliveryAddressArgs.builder()
                .simpleAddress("email@example.com")
                .build())
            .name("Example contact channel")
            .type("EMAIL")
            .build());

    }
}
```
```yaml
resources:
  exampleContact:
    type: aws:ssmcontacts:Contact
    name: example_contact
    properties:
      alias: example_contact
      type: PERSONAL
  example:
    type: aws:ssmcontacts:ContactChannel
    properties:
      contactId: ${exampleContact.arn}
      deliveryAddress:
        simpleAddress: email@example.com
      name: Example contact channel
      type: EMAIL
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM Contact Channel using the `ARN`. For example:

```sh
$ pulumi import aws:ssmcontacts/contactChannel:ContactChannel example arn:aws:ssm-contacts:us-west-2:123456789012:contact-channel/example
```
h
	contactId" WAmazon Resource Name (ARN) of the AWS SSM Contact that the contact channel belongs to.
�
deliveryAddress}:{
y
ssmcontactsContactChannelDeliveryAddressKaws:ssmcontacts/ContactChannelDeliveryAddress:ContactChannelDeliveryAddressCBlock that contains contact engagement details. See details below.
�
nameB" �Name of the contact channel. Must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), hyphens (`-`), periods (`.`), and spaces.
K
type" ?Type of the contact channel. One of `SMS`, `VOICE` or `EMAIL`.
"�
activationStatus" �Whether the contact channel is activated. The contact channel must be activated to use it to engage the contact. One of `ACTIVATED` or `NOT_ACTIVATED`.
">
arn" 3Amazon Resource Name (ARN) of the contact channel.
"h
	contactId" WAmazon Resource Name (ARN) of the AWS SSM Contact that the contact channel belongs to.
"�
deliveryAddress}:{
y
ssmcontactsContactChannelDeliveryAddressKaws:ssmcontacts/ContactChannelDeliveryAddress:ContactChannelDeliveryAddressCBlock that contains contact engagement details. See details below.
"�
name" �Name of the contact channel. Must be between 1 and 255 characters, and may contain alphanumerics, underscores (`_`), hyphens (`-`), periods (`.`), and spaces.
"K
type" ?Type of the contact channel. One of `SMS`, `VOICE` or `EMAIL`.
*��
.
ssmcontactsPlanaws:ssmcontacts/plan:Plan�zResource for managing an AWS SSM Contact Plan.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssmcontacts.Plan("example", {
    contactId: "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
    stages: [{
        durationInMinutes: 1,
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssmcontacts.Plan("example",
    contact_id="arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
    stages=[{
        "duration_in_minutes": 1,
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsmContacts.Plan("example", new()
    {
        ContactId = "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
        Stages = new[]
        {
            new Aws.SsmContacts.Inputs.PlanStageArgs
            {
                DurationInMinutes = 1,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmcontacts.NewPlan(ctx, "example", &ssmcontacts.PlanArgs{
			ContactId: pulumi.String("arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias"),
			Stages: ssmcontacts.PlanStageArray{
				&ssmcontacts.PlanStageArgs{
					DurationInMinutes: pulumi.Int(1),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.Plan;
import com.pulumi.aws.ssmcontacts.PlanArgs;
import com.pulumi.aws.ssmcontacts.inputs.PlanStageArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Plan("example", PlanArgs.builder()
            .contactId("arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias")
            .stages(PlanStageArgs.builder()
                .durationInMinutes(1)
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssmcontacts:Plan
    properties:
      contactId: arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias
      stages:
        - durationInMinutes: 1
```
<!--End PulumiCodeChooser -->

### Usage with SSM Contact

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const contact = new aws.ssmcontacts.Contact("contact", {
    alias: "alias",
    type: "PERSONAL",
});
const plan = new aws.ssmcontacts.Plan("plan", {
    contactId: contact.arn,
    stages: [{
        durationInMinutes: 1,
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

contact = aws.ssmcontacts.Contact("contact",
    alias="alias",
    type="PERSONAL")
plan = aws.ssmcontacts.Plan("plan",
    contact_id=contact.arn,
    stages=[{
        "duration_in_minutes": 1,
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var contact = new Aws.SsmContacts.Contact("contact", new()
    {
        Alias = "alias",
        Type = "PERSONAL",
    });

    var plan = new Aws.SsmContacts.Plan("plan", new()
    {
        ContactId = contact.Arn,
        Stages = new[]
        {
            new Aws.SsmContacts.Inputs.PlanStageArgs
            {
                DurationInMinutes = 1,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		contact, err := ssmcontacts.NewContact(ctx, "contact", &ssmcontacts.ContactArgs{
			Alias: pulumi.String("alias"),
			Type:  pulumi.String("PERSONAL"),
		})
		if err != nil {
			return err
		}
		_, err = ssmcontacts.NewPlan(ctx, "plan", &ssmcontacts.PlanArgs{
			ContactId: contact.Arn,
			Stages: ssmcontacts.PlanStageArray{
				&ssmcontacts.PlanStageArgs{
					DurationInMinutes: pulumi.Int(1),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.Contact;
import com.pulumi.aws.ssmcontacts.ContactArgs;
import com.pulumi.aws.ssmcontacts.Plan;
import com.pulumi.aws.ssmcontacts.PlanArgs;
import com.pulumi.aws.ssmcontacts.inputs.PlanStageArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var contact = new Contact("contact", ContactArgs.builder()
            .alias("alias")
            .type("PERSONAL")
            .build());

        var plan = new Plan("plan", PlanArgs.builder()
            .contactId(contact.arn())
            .stages(PlanStageArgs.builder()
                .durationInMinutes(1)
                .build())
            .build());

    }
}
```
```yaml
resources:
  contact:
    type: aws:ssmcontacts:Contact
    properties:
      alias: alias
      type: PERSONAL
  plan:
    type: aws:ssmcontacts:Plan
    properties:
      contactId: ${contact.arn}
      stages:
        - durationInMinutes: 1
```
<!--End PulumiCodeChooser -->

### Usage With All Fields

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const escalationPlan = new aws.ssmcontacts.Contact("escalation_plan", {
    alias: "escalation-plan-alias",
    type: "ESCALATION",
});
const contactOne = new aws.ssmcontacts.Contact("contact_one", {
    alias: "alias",
    type: "PERSONAL",
});
const contactTwo = new aws.ssmcontacts.Contact("contact_two", {
    alias: "alias",
    type: "PERSONAL",
});
const test = new aws.ssmcontacts.Plan("test", {
    contactId: escalationPlan.arn,
    stages: [{
        durationInMinutes: 0,
        targets: [
            {
                contactTargetInfo: {
                    isEssential: false,
                    contactId: contactOne.arn,
                },
            },
            {
                contactTargetInfo: {
                    isEssential: true,
                    contactId: contactTwo.arn,
                },
            },
            {
                channelTargetInfo: {
                    retryIntervalInMinutes: 2,
                    contactChannelId: channel.arn,
                },
            },
        ],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

escalation_plan = aws.ssmcontacts.Contact("escalation_plan",
    alias="escalation-plan-alias",
    type="ESCALATION")
contact_one = aws.ssmcontacts.Contact("contact_one",
    alias="alias",
    type="PERSONAL")
contact_two = aws.ssmcontacts.Contact("contact_two",
    alias="alias",
    type="PERSONAL")
test = aws.ssmcontacts.Plan("test",
    contact_id=escalation_plan.arn,
    stages=[{
        "duration_in_minutes": 0,
        "targets": [
            {
                "contact_target_info": {
                    "is_essential": False,
                    "contact_id": contact_one.arn,
                },
            },
            {
                "contact_target_info": {
                    "is_essential": True,
                    "contact_id": contact_two.arn,
                },
            },
            {
                "channel_target_info": {
                    "retry_interval_in_minutes": 2,
                    "contact_channel_id": channel["arn"],
                },
            },
        ],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var escalationPlan = new Aws.SsmContacts.Contact("escalation_plan", new()
    {
        Alias = "escalation-plan-alias",
        Type = "ESCALATION",
    });

    var contactOne = new Aws.SsmContacts.Contact("contact_one", new()
    {
        Alias = "alias",
        Type = "PERSONAL",
    });

    var contactTwo = new Aws.SsmContacts.Contact("contact_two", new()
    {
        Alias = "alias",
        Type = "PERSONAL",
    });

    var test = new Aws.SsmContacts.Plan("test", new()
    {
        ContactId = escalationPlan.Arn,
        Stages = new[]
        {
            new Aws.SsmContacts.Inputs.PlanStageArgs
            {
                DurationInMinutes = 0,
                Targets = new[]
                {
                    new Aws.SsmContacts.Inputs.PlanStageTargetArgs
                    {
                        ContactTargetInfo = new Aws.SsmContacts.Inputs.PlanStageTargetContactTargetInfoArgs
                        {
                            IsEssential = false,
                            ContactId = contactOne.Arn,
                        },
                    },
                    new Aws.SsmContacts.Inputs.PlanStageTargetArgs
                    {
                        ContactTargetInfo = new Aws.SsmContacts.Inputs.PlanStageTargetContactTargetInfoArgs
                        {
                            IsEssential = true,
                            ContactId = contactTwo.Arn,
                        },
                    },
                    new Aws.SsmContacts.Inputs.PlanStageTargetArgs
                    {
                        ChannelTargetInfo = new Aws.SsmContacts.Inputs.PlanStageTargetChannelTargetInfoArgs
                        {
                            RetryIntervalInMinutes = 2,
                            ContactChannelId = channel.Arn,
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		escalationPlan, err := ssmcontacts.NewContact(ctx, "escalation_plan", &ssmcontacts.ContactArgs{
			Alias: pulumi.String("escalation-plan-alias"),
			Type:  pulumi.String("ESCALATION"),
		})
		if err != nil {
			return err
		}
		contactOne, err := ssmcontacts.NewContact(ctx, "contact_one", &ssmcontacts.ContactArgs{
			Alias: pulumi.String("alias"),
			Type:  pulumi.String("PERSONAL"),
		})
		if err != nil {
			return err
		}
		contactTwo, err := ssmcontacts.NewContact(ctx, "contact_two", &ssmcontacts.ContactArgs{
			Alias: pulumi.String("alias"),
			Type:  pulumi.String("PERSONAL"),
		})
		if err != nil {
			return err
		}
		_, err = ssmcontacts.NewPlan(ctx, "test", &ssmcontacts.PlanArgs{
			ContactId: escalationPlan.Arn,
			Stages: ssmcontacts.PlanStageArray{
				&ssmcontacts.PlanStageArgs{
					DurationInMinutes: pulumi.Int(0),
					Targets: ssmcontacts.PlanStageTargetArray{
						&ssmcontacts.PlanStageTargetArgs{
							ContactTargetInfo: &ssmcontacts.PlanStageTargetContactTargetInfoArgs{
								IsEssential: pulumi.Bool(false),
								ContactId:   contactOne.Arn,
							},
						},
						&ssmcontacts.PlanStageTargetArgs{
							ContactTargetInfo: &ssmcontacts.PlanStageTargetContactTargetInfoArgs{
								IsEssential: pulumi.Bool(true),
								ContactId:   contactTwo.Arn,
							},
						},
						&ssmcontacts.PlanStageTargetArgs{
							ChannelTargetInfo: &ssmcontacts.PlanStageTargetChannelTargetInfoArgs{
								RetryIntervalInMinutes: pulumi.Int(2),
								ContactChannelId:       pulumi.Any(channel.Arn),
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.Contact;
import com.pulumi.aws.ssmcontacts.ContactArgs;
import com.pulumi.aws.ssmcontacts.Plan;
import com.pulumi.aws.ssmcontacts.PlanArgs;
import com.pulumi.aws.ssmcontacts.inputs.PlanStageArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var escalationPlan = new Contact("escalationPlan", ContactArgs.builder()
            .alias("escalation-plan-alias")
            .type("ESCALATION")
            .build());

        var contactOne = new Contact("contactOne", ContactArgs.builder()
            .alias("alias")
            .type("PERSONAL")
            .build());

        var contactTwo = new Contact("contactTwo", ContactArgs.builder()
            .alias("alias")
            .type("PERSONAL")
            .build());

        var test = new Plan("test", PlanArgs.builder()
            .contactId(escalationPlan.arn())
            .stages(PlanStageArgs.builder()
                .durationInMinutes(0)
                .targets(                
                    PlanStageTargetArgs.builder()
                        .contactTargetInfo(PlanStageTargetContactTargetInfoArgs.builder()
                            .isEssential(false)
                            .contactId(contactOne.arn())
                            .build())
                        .build(),
                    PlanStageTargetArgs.builder()
                        .contactTargetInfo(PlanStageTargetContactTargetInfoArgs.builder()
                            .isEssential(true)
                            .contactId(contactTwo.arn())
                            .build())
                        .build(),
                    PlanStageTargetArgs.builder()
                        .channelTargetInfo(PlanStageTargetChannelTargetInfoArgs.builder()
                            .retryIntervalInMinutes(2)
                            .contactChannelId(channel.arn())
                            .build())
                        .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  escalationPlan:
    type: aws:ssmcontacts:Contact
    name: escalation_plan
    properties:
      alias: escalation-plan-alias
      type: ESCALATION
  contactOne:
    type: aws:ssmcontacts:Contact
    name: contact_one
    properties:
      alias: alias
      type: PERSONAL
  contactTwo:
    type: aws:ssmcontacts:Contact
    name: contact_two
    properties:
      alias: alias
      type: PERSONAL
  test:
    type: aws:ssmcontacts:Plan
    properties:
      contactId: ${escalationPlan.arn}
      stages:
        - durationInMinutes: 0
          targets:
            - contactTargetInfo:
                isEssential: false
                contactId: ${contactOne.arn}
            - contactTargetInfo:
                isEssential: true
                contactId: ${contactTwo.arn}
            - channelTargetInfo:
                retryIntervalInMinutes: 2
                contactChannelId: ${channel.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSM Contact Plan using the Contact ARN. For example:

```sh
$ pulumi import aws:ssmcontacts/plan:Plan example {ARNValue}
```
S
	contactId" BThe Amazon Resource Name (ARN) of the contact or escalation plan.
�
stagesC*A:?
=
ssmcontacts	PlanStage#aws:ssmcontacts/PlanStage:PlanStage�One or more configuration blocks for specifying a list of stages that the escalation plan or engagement plan uses to engage contacts and contact methods. See Stage below for more details.
"S
	contactId" BThe Amazon Resource Name (ARN) of the contact or escalation plan.
"�
stagesC*A:?
=
ssmcontacts	PlanStage#aws:ssmcontacts/PlanStage:PlanStage�One or more configuration blocks for specifying a list of stages that the escalation plan or engagement plan uses to engage contacts and contact methods. See Stage below for more details.
*�h
N
ssmincidentsReplicationSet.aws:ssmincidents/replicationSet:ReplicationSet�aProvides a resource for managing a replication set in AWS Systems Manager Incident Manager.

> **NOTE:** Deleting a replication set also deletes all Incident Manager related data including response plans, incident records, contacts and escalation plans.

## Example Usage

### Basic Usage

Create a replication set.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const replicationSetName = new aws.ssmincidents.ReplicationSet("replicationSetName", {
    regions: [{
        name: "us-west-2",
    }],
    tags: {
        exampleTag: "exampleValue",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

replication_set_name = aws.ssmincidents.ReplicationSet("replicationSetName",
    regions=[{
        "name": "us-west-2",
    }],
    tags={
        "exampleTag": "exampleValue",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var replicationSetName = new Aws.SsmIncidents.ReplicationSet("replicationSetName", new()
    {
        Regions = new[]
        {
            new Aws.SsmIncidents.Inputs.ReplicationSetRegionArgs
            {
                Name = "us-west-2",
            },
        },
        Tags = 
        {
            { "exampleTag", "exampleValue" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmincidents"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmincidents.NewReplicationSet(ctx, "replicationSetName", &ssmincidents.ReplicationSetArgs{
			Regions: ssmincidents.ReplicationSetRegionArray{
				&ssmincidents.ReplicationSetRegionArgs{
					Name: pulumi.String("us-west-2"),
				},
			},
			Tags: pulumi.StringMap{
				"exampleTag": pulumi.String("exampleValue"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmincidents.ReplicationSet;
import com.pulumi.aws.ssmincidents.ReplicationSetArgs;
import com.pulumi.aws.ssmincidents.inputs.ReplicationSetRegionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var replicationSetName = new ReplicationSet("replicationSetName", ReplicationSetArgs.builder()
            .regions(ReplicationSetRegionArgs.builder()
                .name("us-west-2")
                .build())
            .tags(Map.of("exampleTag", "exampleValue"))
            .build());

    }
}
```
```yaml
resources:
  replicationSetName:
    type: aws:ssmincidents:ReplicationSet
    properties:
      regions:
        - name: us-west-2
      tags:
        exampleTag: exampleValue
```
<!--End PulumiCodeChooser -->

Add a Region to a replication set. (You can add only one Region at a time.)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const replicationSetName = new aws.ssmincidents.ReplicationSet("replicationSetName", {regions: [
    {
        name: "us-west-2",
    },
    {
        name: "ap-southeast-2",
    },
]});
```
```python
import pulumi
import pulumi_aws as aws

replication_set_name = aws.ssmincidents.ReplicationSet("replicationSetName", regions=[
    {
        "name": "us-west-2",
    },
    {
        "name": "ap-southeast-2",
    },
])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var replicationSetName = new Aws.SsmIncidents.ReplicationSet("replicationSetName", new()
    {
        Regions = new[]
        {
            new Aws.SsmIncidents.Inputs.ReplicationSetRegionArgs
            {
                Name = "us-west-2",
            },
            new Aws.SsmIncidents.Inputs.ReplicationSetRegionArgs
            {
                Name = "ap-southeast-2",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmincidents"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmincidents.NewReplicationSet(ctx, "replicationSetName", &ssmincidents.ReplicationSetArgs{
			Regions: ssmincidents.ReplicationSetRegionArray{
				&ssmincidents.ReplicationSetRegionArgs{
					Name: pulumi.String("us-west-2"),
				},
				&ssmincidents.ReplicationSetRegionArgs{
					Name: pulumi.String("ap-southeast-2"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmincidents.ReplicationSet;
import com.pulumi.aws.ssmincidents.ReplicationSetArgs;
import com.pulumi.aws.ssmincidents.inputs.ReplicationSetRegionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var replicationSetName = new ReplicationSet("replicationSetName", ReplicationSetArgs.builder()
            .regions(            
                ReplicationSetRegionArgs.builder()
                    .name("us-west-2")
                    .build(),
                ReplicationSetRegionArgs.builder()
                    .name("ap-southeast-2")
                    .build())
            .build());

    }
}
```
```yaml
resources:
  replicationSetName:
    type: aws:ssmincidents:ReplicationSet
    properties:
      regions:
        - name: us-west-2
        - name: ap-southeast-2
```
<!--End PulumiCodeChooser -->

Delete a Region from a replication set. (You can delete only one Region at a time.)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const replicationSetName = new aws.ssmincidents.ReplicationSet("replicationSetName", {regions: [{
    name: "us-west-2",
}]});
```
```python
import pulumi
import pulumi_aws as aws

replication_set_name = aws.ssmincidents.ReplicationSet("replicationSetName", regions=[{
    "name": "us-west-2",
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var replicationSetName = new Aws.SsmIncidents.ReplicationSet("replicationSetName", new()
    {
        Regions = new[]
        {
            new Aws.SsmIncidents.Inputs.ReplicationSetRegionArgs
            {
                Name = "us-west-2",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmincidents"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmincidents.NewReplicationSet(ctx, "replicationSetName", &ssmincidents.ReplicationSetArgs{
			Regions: ssmincidents.ReplicationSetRegionArray{
				&ssmincidents.ReplicationSetRegionArgs{
					Name: pulumi.String("us-west-2"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmincidents.ReplicationSet;
import com.pulumi.aws.ssmincidents.ReplicationSetArgs;
import com.pulumi.aws.ssmincidents.inputs.ReplicationSetRegionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var replicationSetName = new ReplicationSet("replicationSetName", ReplicationSetArgs.builder()
            .regions(ReplicationSetRegionArgs.builder()
                .name("us-west-2")
                .build())
            .build());

    }
}
```
```yaml
resources:
  replicationSetName:
    type: aws:ssmincidents:ReplicationSet
    properties:
      regions:
        - name: us-west-2
```
<!--End PulumiCodeChooser -->

## Basic Usage with an AWS Customer Managed Key

Create a replication set with an AWS Key Management Service (AWS KMS) customer manager key:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleKey = new aws.kms.Key("example_key", {});
const replicationSetName = new aws.ssmincidents.ReplicationSet("replicationSetName", {
    regions: [{
        name: "us-west-2",
        kmsKeyArn: exampleKey.arn,
    }],
    tags: {
        exampleTag: "exampleValue",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example_key = aws.kms.Key("example_key")
replication_set_name = aws.ssmincidents.ReplicationSet("replicationSetName",
    regions=[{
        "name": "us-west-2",
        "kms_key_arn": example_key.arn,
    }],
    tags={
        "exampleTag": "exampleValue",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleKey = new Aws.Kms.Key("example_key");

    var replicationSetName = new Aws.SsmIncidents.ReplicationSet("replicationSetName", new()
    {
        Regions = new[]
        {
            new Aws.SsmIncidents.Inputs.ReplicationSetRegionArgs
            {
                Name = "us-west-2",
                KmsKeyArn = exampleKey.Arn,
            },
        },
        Tags = 
        {
            { "exampleTag", "exampleValue" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kms"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmincidents"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleKey, err := kms.NewKey(ctx, "example_key", nil)
		if err != nil {
			return err
		}
		_, err = ssmincidents.NewReplicationSet(ctx, "replicationSetName", &ssmincidents.ReplicationSetArgs{
			Regions: ssmincidents.ReplicationSetRegionArray{
				&ssmincidents.ReplicationSetRegionArgs{
					Name:      pulumi.String("us-west-2"),
					KmsKeyArn: exampleKey.Arn,
				},
			},
			Tags: pulumi.StringMap{
				"exampleTag": pulumi.String("exampleValue"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.kms.Key;
import com.pulumi.aws.ssmincidents.ReplicationSet;
import com.pulumi.aws.ssmincidents.ReplicationSetArgs;
import com.pulumi.aws.ssmincidents.inputs.ReplicationSetRegionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var exampleKey = new Key("exampleKey");

        var replicationSetName = new ReplicationSet("replicationSetName", ReplicationSetArgs.builder()
            .regions(ReplicationSetRegionArgs.builder()
                .name("us-west-2")
                .kmsKeyArn(exampleKey.arn())
                .build())
            .tags(Map.of("exampleTag", "exampleValue"))
            .build());

    }
}
```
```yaml
resources:
  exampleKey:
    type: aws:kms:Key
    name: example_key
  replicationSetName:
    type: aws:ssmincidents:ReplicationSet
    properties:
      regions:
        - name: us-west-2
          kmsKeyArn: ${exampleKey.arn}
      tags:
        exampleTag: exampleValue
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import an Incident Manager replication. For example:

```sh
$ pulumi import aws:ssmincidents/replicationSet:ReplicationSet replicationSetName import
```
q
regionsf*d:b
`
ssmincidentsReplicationSetRegion:aws:ssmincidents/ReplicationSetRegion:ReplicationSetRegion
tagsB2" "+
arn"  The ARN of the replication set.
"F
	createdBy" 5The ARN of the user who created the replication set.
"\
deletionProtected
 CIf `true`, the last region in a replication set cannot be deleted.
"V
lastModifiedBy" @A timestamp showing when the replication set was last modified.
"q
regionsf*d:b
`
ssmincidentsReplicationSetRegion:aws:ssmincidents/ReplicationSetRegion:ReplicationSetRegion"{
status" mThe current status of the Region.
* Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
"
tagsB2" "�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*��
H
ssmincidentsResponsePlan*aws:ssmincidents/responsePlan:ResponsePlan��Provides a resource to manage response plans in AWS Systems Manager Incident Manager.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssmincidents.ResponsePlan("example", {
    name: "name",
    incidentTemplate: {
        title: "title",
        impact: 3,
    },
    tags: {
        key: "value",
    },
}, {
    dependsOn: [exampleAwsSsmincidentsReplicationSet],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssmincidents.ResponsePlan("example",
    name="name",
    incident_template={
        "title": "title",
        "impact": 3,
    },
    tags={
        "key": "value",
    },
    opts = pulumi.ResourceOptions(depends_on=[example_aws_ssmincidents_replication_set]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsmIncidents.ResponsePlan("example", new()
    {
        Name = "name",
        IncidentTemplate = new Aws.SsmIncidents.Inputs.ResponsePlanIncidentTemplateArgs
        {
            Title = "title",
            Impact = 3,
        },
        Tags = 
        {
            { "key", "value" },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSsmincidentsReplicationSet,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmincidents"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmincidents.NewResponsePlan(ctx, "example", &ssmincidents.ResponsePlanArgs{
			Name: pulumi.String("name"),
			IncidentTemplate: &ssmincidents.ResponsePlanIncidentTemplateArgs{
				Title:  pulumi.String("title"),
				Impact: pulumi.Int(3),
			},
			Tags: pulumi.StringMap{
				"key": pulumi.String("value"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSsmincidentsReplicationSet,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmincidents.ResponsePlan;
import com.pulumi.aws.ssmincidents.ResponsePlanArgs;
import com.pulumi.aws.ssmincidents.inputs.ResponsePlanIncidentTemplateArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResponsePlan("example", ResponsePlanArgs.builder()
            .name("name")
            .incidentTemplate(ResponsePlanIncidentTemplateArgs.builder()
                .title("title")
                .impact("3")
                .build())
            .tags(Map.of("key", "value"))
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSsmincidentsReplicationSet)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssmincidents:ResponsePlan
    properties:
      name: name
      incidentTemplate:
        title: title
        impact: '3'
      tags:
        key: value
    options:
      dependsOn:
        - ${exampleAwsSsmincidentsReplicationSet}
```
<!--End PulumiCodeChooser -->

### Usage With All Fields

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssmincidents.ResponsePlan("example", {
    name: "name",
    incidentTemplate: {
        title: "title",
        impact: 3,
        dedupeString: "dedupe",
        incidentTags: {
            key: "value",
        },
        notificationTargets: [
            {
                snsTopicArn: example1.arn,
            },
            {
                snsTopicArn: example2.arn,
            },
        ],
        summary: "summary",
    },
    displayName: "display name",
    chatChannels: [topic.arn],
    engagements: ["arn:aws:ssm-contacts:us-east-2:111122223333:contact/test1"],
    action: {
        ssmAutomations: [{
            documentName: document1.name,
            roleArn: role1.arn,
            documentVersion: "version1",
            targetAccount: "RESPONSE_PLAN_OWNER_ACCOUNT",
            parameters: [
                {
                    name: "key",
                    values: [
                        "value1",
                        "value2",
                    ],
                },
                {
                    name: "foo",
                    values: ["bar"],
                },
            ],
            dynamicParameters: {
                someKey: "INVOLVED_RESOURCES",
                anotherKey: "INCIDENT_RECORD_ARN",
            },
        }],
    },
    integration: {
        pagerduties: [{
            name: "pagerdutyIntergration",
            serviceId: "example",
            secretId: "example",
        }],
    },
    tags: {
        key: "value",
    },
}, {
    dependsOn: [exampleAwsSsmincidentsReplicationSet],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssmincidents.ResponsePlan("example",
    name="name",
    incident_template={
        "title": "title",
        "impact": 3,
        "dedupe_string": "dedupe",
        "incident_tags": {
            "key": "value",
        },
        "notification_targets": [
            {
                "sns_topic_arn": example1["arn"],
            },
            {
                "sns_topic_arn": example2["arn"],
            },
        ],
        "summary": "summary",
    },
    display_name="display name",
    chat_channels=[topic["arn"]],
    engagements=["arn:aws:ssm-contacts:us-east-2:111122223333:contact/test1"],
    action={
        "ssm_automations": [{
            "document_name": document1["name"],
            "role_arn": role1["arn"],
            "document_version": "version1",
            "target_account": "RESPONSE_PLAN_OWNER_ACCOUNT",
            "parameters": [
                {
                    "name": "key",
                    "values": [
                        "value1",
                        "value2",
                    ],
                },
                {
                    "name": "foo",
                    "values": ["bar"],
                },
            ],
            "dynamic_parameters": {
                "someKey": "INVOLVED_RESOURCES",
                "anotherKey": "INCIDENT_RECORD_ARN",
            },
        }],
    },
    integration={
        "pagerduties": [{
            "name": "pagerdutyIntergration",
            "service_id": "example",
            "secret_id": "example",
        }],
    },
    tags={
        "key": "value",
    },
    opts = pulumi.ResourceOptions(depends_on=[example_aws_ssmincidents_replication_set]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsmIncidents.ResponsePlan("example", new()
    {
        Name = "name",
        IncidentTemplate = new Aws.SsmIncidents.Inputs.ResponsePlanIncidentTemplateArgs
        {
            Title = "title",
            Impact = 3,
            DedupeString = "dedupe",
            IncidentTags = 
            {
                { "key", "value" },
            },
            NotificationTargets = new[]
            {
                new Aws.SsmIncidents.Inputs.ResponsePlanIncidentTemplateNotificationTargetArgs
                {
                    SnsTopicArn = example1.Arn,
                },
                new Aws.SsmIncidents.Inputs.ResponsePlanIncidentTemplateNotificationTargetArgs
                {
                    SnsTopicArn = example2.Arn,
                },
            },
            Summary = "summary",
        },
        DisplayName = "display name",
        ChatChannels = new[]
        {
            topic.Arn,
        },
        Engagements = new[]
        {
            "arn:aws:ssm-contacts:us-east-2:111122223333:contact/test1",
        },
        Action = new Aws.SsmIncidents.Inputs.ResponsePlanActionArgs
        {
            SsmAutomations = new[]
            {
                new Aws.SsmIncidents.Inputs.ResponsePlanActionSsmAutomationArgs
                {
                    DocumentName = document1.Name,
                    RoleArn = role1.Arn,
                    DocumentVersion = "version1",
                    TargetAccount = "RESPONSE_PLAN_OWNER_ACCOUNT",
                    Parameters = new[]
                    {
                        new Aws.SsmIncidents.Inputs.ResponsePlanActionSsmAutomationParameterArgs
                        {
                            Name = "key",
                            Values = new[]
                            {
                                "value1",
                                "value2",
                            },
                        },
                        new Aws.SsmIncidents.Inputs.ResponsePlanActionSsmAutomationParameterArgs
                        {
                            Name = "foo",
                            Values = new[]
                            {
                                "bar",
                            },
                        },
                    },
                    DynamicParameters = 
                    {
                        { "someKey", "INVOLVED_RESOURCES" },
                        { "anotherKey", "INCIDENT_RECORD_ARN" },
                    },
                },
            },
        },
        Integration = new Aws.SsmIncidents.Inputs.ResponsePlanIntegrationArgs
        {
            Pagerduties = new[]
            {
                new Aws.SsmIncidents.Inputs.ResponsePlanIntegrationPagerdutyArgs
                {
                    Name = "pagerdutyIntergration",
                    ServiceId = "example",
                    SecretId = "example",
                },
            },
        },
        Tags = 
        {
            { "key", "value" },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSsmincidentsReplicationSet,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmincidents"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmincidents.NewResponsePlan(ctx, "example", &ssmincidents.ResponsePlanArgs{
			Name: pulumi.String("name"),
			IncidentTemplate: &ssmincidents.ResponsePlanIncidentTemplateArgs{
				Title:        pulumi.String("title"),
				Impact:       pulumi.Int(3),
				DedupeString: pulumi.String("dedupe"),
				IncidentTags: pulumi.StringMap{
					"key": pulumi.String("value"),
				},
				NotificationTargets: ssmincidents.ResponsePlanIncidentTemplateNotificationTargetArray{
					&ssmincidents.ResponsePlanIncidentTemplateNotificationTargetArgs{
						SnsTopicArn: pulumi.Any(example1.Arn),
					},
					&ssmincidents.ResponsePlanIncidentTemplateNotificationTargetArgs{
						SnsTopicArn: pulumi.Any(example2.Arn),
					},
				},
				Summary: pulumi.String("summary"),
			},
			DisplayName: pulumi.String("display name"),
			ChatChannels: pulumi.StringArray{
				topic.Arn,
			},
			Engagements: pulumi.StringArray{
				pulumi.String("arn:aws:ssm-contacts:us-east-2:111122223333:contact/test1"),
			},
			Action: &ssmincidents.ResponsePlanActionArgs{
				SsmAutomations: ssmincidents.ResponsePlanActionSsmAutomationArray{
					&ssmincidents.ResponsePlanActionSsmAutomationArgs{
						DocumentName:    pulumi.Any(document1.Name),
						RoleArn:         pulumi.Any(role1.Arn),
						DocumentVersion: pulumi.String("version1"),
						TargetAccount:   pulumi.String("RESPONSE_PLAN_OWNER_ACCOUNT"),
						Parameters: ssmincidents.ResponsePlanActionSsmAutomationParameterArray{
							&ssmincidents.ResponsePlanActionSsmAutomationParameterArgs{
								Name: pulumi.String("key"),
								Values: pulumi.StringArray{
									pulumi.String("value1"),
									pulumi.String("value2"),
								},
							},
							&ssmincidents.ResponsePlanActionSsmAutomationParameterArgs{
								Name: pulumi.String("foo"),
								Values: pulumi.StringArray{
									pulumi.String("bar"),
								},
							},
						},
						DynamicParameters: pulumi.StringMap{
							"someKey":    pulumi.String("INVOLVED_RESOURCES"),
							"anotherKey": pulumi.String("INCIDENT_RECORD_ARN"),
						},
					},
				},
			},
			Integration: &ssmincidents.ResponsePlanIntegrationArgs{
				Pagerduties: ssmincidents.ResponsePlanIntegrationPagerdutyArray{
					&ssmincidents.ResponsePlanIntegrationPagerdutyArgs{
						Name:      pulumi.String("pagerdutyIntergration"),
						ServiceId: pulumi.String("example"),
						SecretId:  pulumi.String("example"),
					},
				},
			},
			Tags: pulumi.StringMap{
				"key": pulumi.String("value"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSsmincidentsReplicationSet,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmincidents.ResponsePlan;
import com.pulumi.aws.ssmincidents.ResponsePlanArgs;
import com.pulumi.aws.ssmincidents.inputs.ResponsePlanIncidentTemplateArgs;
import com.pulumi.aws.ssmincidents.inputs.ResponsePlanActionArgs;
import com.pulumi.aws.ssmincidents.inputs.ResponsePlanIntegrationArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResponsePlan("example", ResponsePlanArgs.builder()
            .name("name")
            .incidentTemplate(ResponsePlanIncidentTemplateArgs.builder()
                .title("title")
                .impact("3")
                .dedupeString("dedupe")
                .incidentTags(Map.of("key", "value"))
                .notificationTargets(                
                    ResponsePlanIncidentTemplateNotificationTargetArgs.builder()
                        .snsTopicArn(example1.arn())
                        .build(),
                    ResponsePlanIncidentTemplateNotificationTargetArgs.builder()
                        .snsTopicArn(example2.arn())
                        .build())
                .summary("summary")
                .build())
            .displayName("display name")
            .chatChannels(topic.arn())
            .engagements("arn:aws:ssm-contacts:us-east-2:111122223333:contact/test1")
            .action(ResponsePlanActionArgs.builder()
                .ssmAutomations(ResponsePlanActionSsmAutomationArgs.builder()
                    .documentName(document1.name())
                    .roleArn(role1.arn())
                    .documentVersion("version1")
                    .targetAccount("RESPONSE_PLAN_OWNER_ACCOUNT")
                    .parameters(                    
                        ResponsePlanActionSsmAutomationParameterArgs.builder()
                            .name("key")
                            .values(                            
                                "value1",
                                "value2")
                            .build(),
                        ResponsePlanActionSsmAutomationParameterArgs.builder()
                            .name("foo")
                            .values("bar")
                            .build())
                    .dynamicParameters(Map.ofEntries(
                        Map.entry("someKey", "INVOLVED_RESOURCES"),
                        Map.entry("anotherKey", "INCIDENT_RECORD_ARN")
                    ))
                    .build())
                .build())
            .integration(ResponsePlanIntegrationArgs.builder()
                .pagerduties(ResponsePlanIntegrationPagerdutyArgs.builder()
                    .name("pagerdutyIntergration")
                    .serviceId("example")
                    .secretId("example")
                    .build())
                .build())
            .tags(Map.of("key", "value"))
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSsmincidentsReplicationSet)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssmincidents:ResponsePlan
    properties:
      name: name
      incidentTemplate:
        title: title
        impact: '3'
        dedupeString: dedupe
        incidentTags:
          key: value
        notificationTargets:
          - snsTopicArn: ${example1.arn}
          - snsTopicArn: ${example2.arn}
        summary: summary
      displayName: display name
      chatChannels:
        - ${topic.arn}
      engagements:
        - arn:aws:ssm-contacts:us-east-2:111122223333:contact/test1
      action:
        ssmAutomations:
          - documentName: ${document1.name}
            roleArn: ${role1.arn}
            documentVersion: version1
            targetAccount: RESPONSE_PLAN_OWNER_ACCOUNT
            parameters:
              - name: key
                values:
                  - value1
                  - value2
              - name: foo
                values:
                  - bar
            dynamicParameters:
              someKey: INVOLVED_RESOURCES
              anotherKey: INCIDENT_RECORD_ARN
      integration:
        pagerduties:
          - name: pagerdutyIntergration
            serviceId: example
            secretId: example
      tags:
        key: value
    options:
      dependsOn:
        - ${exampleAwsSsmincidentsReplicationSet}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import an Incident Manager response plan using the response plan ARN. You can find the response plan ARN in the AWS Management Console. For example:

```sh
$ pulumi import aws:ssmincidents/responsePlan:ResponsePlan responsePlanName ARNValue
```
j
action`B^:\
Z
ssmincidentsResponsePlanAction6aws:ssmincidents/ResponsePlanAction:ResponsePlanAction
chatChannelsB*" 
displayNameB" 
engagementsB*" �
incidentTemplate|:z
x
ssmincidentsResponsePlanIncidentTemplateJaws:ssmincidents/ResponsePlanIncidentTemplate:ResponsePlanIncidentTemplate~
integrationoBm:k
i
ssmincidentsResponsePlanIntegration@aws:ssmincidents/ResponsePlanIntegration:ResponsePlanIntegration-
nameB" The name of the response plan.

tagsB2" "j
action`B^:\
Z
ssmincidentsResponsePlanAction6aws:ssmincidents/ResponsePlanAction:ResponsePlanAction")
arn" The ARN of the response plan.
"
chatChannelsB*" "
displayNameB" "
engagementsB*" "�
incidentTemplate|:z
x
ssmincidentsResponsePlanIncidentTemplateJaws:ssmincidents/ResponsePlanIncidentTemplate:ResponsePlanIncidentTemplate"~
integrationoBm:k
i
ssmincidentsResponsePlanIntegration@aws:ssmincidents/ResponsePlanIntegration:ResponsePlanIntegration"+
name" The name of the response plan.
"
tagsB2" "�
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
2�
!getArnaws:index/getArn:getArn�Parses an ARN into its constituent parts.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const dbInstance = aws.getArn({
    arn: "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
});
```
```python
import pulumi
import pulumi_aws as aws

db_instance = aws.get_arn(arn="arn:aws:rds:eu-west-1:123456789012:db:mysql-db")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var dbInstance = Aws.GetArn.Invoke(new()
    {
        Arn = "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetArn(ctx, &aws.GetArnArgs{
			Arn: "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetArnArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var dbInstance = AwsFunctions.getArn(GetArnArgs.builder()
            .arn("arn:aws:rds:eu-west-1:123456789012:db:mysql-db")
            .build());

    }
}
```
```yaml
variables:
  dbInstance:
    fn::invoke:
      function: aws:getArn
      arguments:
        arn: arn:aws:rds:eu-west-1:123456789012:db:mysql-db
```
<!--End PulumiCodeChooser -->

arn" ARN to parse.


idB" "�
account" �The [ID](https://docs.aws.amazon.com/general/latest/gr/acct-identifiers.html) of the AWS account that owns the resource, without the hyphens.
"	
arn" "
id" "4
	partition" #Partition that the resource is in.
"�
region" �Region the resource resides in.
Note that the ARNs for some resources do not require a region, so this component might be omitted.
"�
resource" �Content of this part of the ARN varies by service.
It often includes an indicator of the type of resource—for example, an IAM user or Amazon RDS database —followed by a slash (/) or a colon (:), followed by the resource name itself.
"�
service" �The [service namespace](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces) that identifies the AWS product.
2�[
HgetAvailabilityZone1aws:index/getAvailabilityZone:getAvailabilityZone�J`aws.getAvailabilityZone` provides details about a specific availability zone (AZ)
in the current region.

This can be used both to validate an availability zone given in a variable
and to split the AZ name into its component parts of an AWS region and an
AZ identifier letter. The latter may be useful e.g., for implementing a
consistent subnet numbering scheme across several regions by mapping both
the region and the subnet letter to network numbers.

This is different from the `aws.getAvailabilityZones` (plural) data source,
which provides a list of the available zones.

## Example Usage

The following example shows how this data source might be used to derive
VPC and subnet CIDR prefixes systematically for an availability zone.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const config = new pulumi.Config();
const regionNumber = config.getObject("regionNumber") || {
    "ap-northeast-1": 5,
    "eu-central-1": 4,
    "us-east-1": 1,
    "us-west-1": 2,
    "us-west-2": 3,
};
const azNumber = config.getObject("azNumber") || {
    a: 1,
    b: 2,
    c: 3,
    d: 4,
    e: 5,
    f: 6,
};
// Retrieve the AZ where we want to create network resources
// This must be in the region selected on the AWS provider.
const example = aws.getAvailabilityZone({
    name: "eu-central-1a",
});
// Create a VPC for the region associated with the AZ
const exampleVpc = new aws.ec2.Vpc("example", {cidrBlock: example.then(example => std.cidrsubnet({
    input: "10.0.0.0/8",
    newbits: 4,
    netnum: regionNumber[example.region],
})).then(invoke => invoke.result)});
// Create a subnet for the AZ within the regional VPC
const exampleSubnet = new aws.ec2.Subnet("example", {
    vpcId: exampleVpc.id,
    cidrBlock: pulumi.all([exampleVpc.cidrBlock, example]).apply(([cidrBlock, example]) => std.cidrsubnetOutput({
        input: cidrBlock,
        newbits: 4,
        netnum: azNumber[example.nameSuffix],
    })).apply(invoke => invoke.result),
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

config = pulumi.Config()
region_number = config.get_object("regionNumber")
if region_number is None:
    region_number = {
        "ap-northeast-1": 5,
        "eu-central-1": 4,
        "us-east-1": 1,
        "us-west-1": 2,
        "us-west-2": 3,
    }
az_number = config.get_object("azNumber")
if az_number is None:
    az_number = {
        "a": 1,
        "b": 2,
        "c": 3,
        "d": 4,
        "e": 5,
        "f": 6,
    }
# Retrieve the AZ where we want to create network resources
# This must be in the region selected on the AWS provider.
example = aws.get_availability_zone(name="eu-central-1a")
# Create a VPC for the region associated with the AZ
example_vpc = aws.ec2.Vpc("example", cidr_block=std.cidrsubnet(input="10.0.0.0/8",
    newbits=4,
    netnum=region_number[example.region]).result)
# Create a subnet for the AZ within the regional VPC
example_subnet = aws.ec2.Subnet("example",
    vpc_id=example_vpc.id,
    cidr_block=example_vpc.cidr_block.apply(lambda cidr_block: std.cidrsubnet_output(input=cidr_block,
        newbits=4,
        netnum=az_number[example.name_suffix])).apply(lambda invoke: invoke.result))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var config = new Config();
    var regionNumber = config.GetObject<dynamic>("regionNumber") ?? 
    {
        { "ap-northeast-1", 5 },
        { "eu-central-1", 4 },
        { "us-east-1", 1 },
        { "us-west-1", 2 },
        { "us-west-2", 3 },
    };
    var azNumber = config.GetObject<dynamic>("azNumber") ?? 
    {
        { "a", 1 },
        { "b", 2 },
        { "c", 3 },
        { "d", 4 },
        { "e", 5 },
        { "f", 6 },
    };
    // Retrieve the AZ where we want to create network resources
    // This must be in the region selected on the AWS provider.
    var example = Aws.GetAvailabilityZone.Invoke(new()
    {
        Name = "eu-central-1a",
    });

    // Create a VPC for the region associated with the AZ
    var exampleVpc = new Aws.Ec2.Vpc("example", new()
    {
        CidrBlock = Std.Cidrsubnet.Invoke(new()
        {
            Input = "10.0.0.0/8",
            Newbits = 4,
            Netnum = regionNumber[example.Apply(getAvailabilityZoneResult => getAvailabilityZoneResult.Region)],
        }).Apply(invoke => invoke.Result),
    });

    // Create a subnet for the AZ within the regional VPC
    var exampleSubnet = new Aws.Ec2.Subnet("example", new()
    {
        VpcId = exampleVpc.Id,
        CidrBlock = Output.Tuple(exampleVpc.CidrBlock, example).Apply(values =>
        {
            var cidrBlock = values.Item1;
            var example = values.Item2;
            return Std.Cidrsubnet.Invoke(new()
            {
                Input = cidrBlock,
                Newbits = 4,
                Netnum = azNumber[example.Apply(getAvailabilityZoneResult => getAvailabilityZoneResult.NameSuffix)],
            });
        }).Apply(invoke => invoke.Result),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		cfg := config.New(ctx, "")
		regionNumber := map[string]interface{}{
			"ap-northeast-1": 5,
			"eu-central-1":   4,
			"us-east-1":      1,
			"us-west-1":      2,
			"us-west-2":      3,
		}
		if param := cfg.GetObject("regionNumber"); param != nil {
			regionNumber = param
		}
		azNumber := map[string]interface{}{
			"a": 1,
			"b": 2,
			"c": 3,
			"d": 4,
			"e": 5,
			"f": 6,
		}
		if param := cfg.GetObject("azNumber"); param != nil {
			azNumber = param
		}
		// Retrieve the AZ where we want to create network resources
		// This must be in the region selected on the AWS provider.
		example, err := aws.GetAvailabilityZone(ctx, &aws.GetAvailabilityZoneArgs{
			Name: pulumi.StringRef("eu-central-1a"),
		}, nil)
		if err != nil {
			return err
		}
		invokeCidrsubnet, err := std.Cidrsubnet(ctx, &std.CidrsubnetArgs{
			Input:   "10.0.0.0/8",
			Newbits: 4,
			Netnum:  regionNumber[example.Region],
		}, nil)
		if err != nil {
			return err
		}
		// Create a VPC for the region associated with the AZ
		exampleVpc, err := ec2.NewVpc(ctx, "example", &ec2.VpcArgs{
			CidrBlock: pulumi.String(invokeCidrsubnet.Result),
		})
		if err != nil {
			return err
		}
		// Create a subnet for the AZ within the regional VPC
		_, err = ec2.NewSubnet(ctx, "example", &ec2.SubnetArgs{
			VpcId: exampleVpc.ID(),
			CidrBlock: pulumi.String(exampleVpc.CidrBlock.ApplyT(func(cidrBlock string) (std.CidrsubnetResult, error) {
				return std.CidrsubnetResult(interface{}(std.CidrsubnetOutput(ctx, std.CidrsubnetOutputArgs{
					Input:   cidrBlock,
					Newbits: 4,
					Netnum:  pulumi.Int(azNumber[example.NameSuffix]),
				}, nil))), nil
			}).(std.CidrsubnetResultOutput).ApplyT(func(invoke std.CidrsubnetResult) (*string, error) {
				return invoke.Result, nil
			}).(pulumi.StringPtrOutput)),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZoneArgs;
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var config = ctx.config();
        final var regionNumber = config.get("regionNumber").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        final var azNumber = config.get("azNumber").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        // Retrieve the AZ where we want to create network resources
        // This must be in the region selected on the AWS provider.
        final var example = AwsFunctions.getAvailabilityZone(GetAvailabilityZoneArgs.builder()
            .name("eu-central-1a")
            .build());

        // Create a VPC for the region associated with the AZ
        var exampleVpc = new Vpc("exampleVpc", VpcArgs.builder()
            .cidrBlock(StdFunctions.cidrsubnet(CidrsubnetArgs.builder()
                .input("10.0.0.0/8")
                .newbits(4)
                .netnum(regionNumber[example.applyValue(getAvailabilityZoneResult -> getAvailabilityZoneResult.region())])
                .build()).result())
            .build());

        // Create a subnet for the AZ within the regional VPC
        var exampleSubnet = new Subnet("exampleSubnet", SubnetArgs.builder()
            .vpcId(exampleVpc.id())
            .cidrBlock(exampleVpc.cidrBlock().applyValue(cidrBlock -> StdFunctions.cidrsubnet()).applyValue(invoke -> invoke.result()))
            .build());

    }
}
```
<!--End PulumiCodeChooser -->
�
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
�
filtersbB`*^:\
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter6Configuration block(s) for filtering. Detailed below.
<
nameB" .Full name of the availability zone to select.
z
stateB" kSpecific availability zone state to require. May be any of `"available"`, `"information"` or `"impaired"`.
<
zoneIdB" ,Zone ID of the availability zone to select.
"
allAvailabilityZonesB
 "m
filtersbB`*^:\
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter"�
	groupName" �For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "�

nameSuffix" �Part of the AZ name that appears after the region name, uniquely identifying the AZ within its region.
For Availability Zones this is usually a single letter, for example `a` for the `us-west-2a` zone.
For Local and Wavelength Zones this is a longer string, for example `wl1-sfo-wlz-1` for the `us-west-2-wl1-sfo-wlz-1` zone.
"Y
networkBorderGroup" ?The name of the location from which the address is advertised.
"�
optInStatus" �For Availability Zones, this always has the value of `opt-in-not-required`. For Local Zones, this is the opt in status. The possible values are `opted-in` and `not-opted-in`.
"�
parentZoneId" sID of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"�
parentZoneName" uName of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"�
region" �Region where the selected availability zone resides. This is always the region selected on the provider, since this data source searches only within that region.
"
state" "
zoneId" "c
zoneType" SType of zone. Values are `availability-zone`, `local-zone`, and `wavelength-zone`.
2�\
KgetAvailabilityZones3aws:index/getAvailabilityZones:getAvailabilityZones�PThe Availability Zones data source allows access to the list of AWS
Availability Zones which can be accessed by an AWS account within the region
configured in the provider.

This is different from the `aws.getAvailabilityZone` (singular) data source,
which provides some details about a specific availability zone.

> When [Local Zones](https://aws.amazon.com/about-aws/global-infrastructure/localzones/) are enabled in a region, by default the API and this data source include both Local Zones and Availability Zones. To return only Availability Zones, see the example section below.

## Example Usage

### By State

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// Declare the data source
const available = aws.getAvailabilityZones({
    state: "available",
});
// e.g., Create subnets in the first two available availability zones
const primary = new aws.ec2.Subnet("primary", {availabilityZone: available.then(available => available.names?.[0])});
const secondary = new aws.ec2.Subnet("secondary", {availabilityZone: available.then(available => available.names?.[1])});
```
```python
import pulumi
import pulumi_aws as aws

# Declare the data source
available = aws.get_availability_zones(state="available")
# e.g., Create subnets in the first two available availability zones
primary = aws.ec2.Subnet("primary", availability_zone=available.names[0])
secondary = aws.ec2.Subnet("secondary", availability_zone=available.names[1])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // Declare the data source
    var available = Aws.GetAvailabilityZones.Invoke(new()
    {
        State = "available",
    });

    // e.g., Create subnets in the first two available availability zones
    var primary = new Aws.Ec2.Subnet("primary", new()
    {
        AvailabilityZone = available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[0]),
    });

    var secondary = new Aws.Ec2.Subnet("secondary", new()
    {
        AvailabilityZone = available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[1]),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// Declare the data source
		available, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			State: pulumi.StringRef("available"),
		}, nil)
		if err != nil {
			return err
		}
		// e.g., Create subnets in the first two available availability zones
		_, err = ec2.NewSubnet(ctx, "primary", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(available.Names[0]),
		})
		if err != nil {
			return err
		}
		_, err = ec2.NewSubnet(ctx, "secondary", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(available.Names[1]),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // Declare the data source
        final var available = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .state("available")
            .build());

        // e.g., Create subnets in the first two available availability zones
        var primary = new Subnet("primary", SubnetArgs.builder()
            .availabilityZone(available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[0]))
            .build());

        var secondary = new Subnet("secondary", SubnetArgs.builder()
            .availabilityZone(available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[1]))
            .build());

    }
}
```
```yaml
resources:
  # e.g., Create subnets in the first two available availability zones
  primary:
    type: aws:ec2:Subnet
    properties:
      availabilityZone: ${available.names[0]}
  secondary:
    type: aws:ec2:Subnet
    properties:
      availabilityZone: ${available.names[1]}
variables:
  # Declare the data source
  available:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        state: available
```
<!--End PulumiCodeChooser -->

### By Filter

All Local Zones (regardless of opt-in status):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getAvailabilityZones({
    allAvailabilityZones: true,
    filters: [{
        name: "opt-in-status",
        values: [
            "not-opted-in",
            "opted-in",
        ],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_availability_zones(all_availability_zones=True,
    filters=[{
        "name": "opt-in-status",
        "values": [
            "not-opted-in",
            "opted-in",
        ],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetAvailabilityZones.Invoke(new()
    {
        AllAvailabilityZones = true,
        Filters = new[]
        {
            new Aws.Inputs.GetAvailabilityZonesFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "not-opted-in",
                    "opted-in",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			AllAvailabilityZones: pulumi.BoolRef(true),
			Filters: []aws.GetAvailabilityZonesFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"not-opted-in",
						"opted-in",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .allAvailabilityZones(true)
            .filters(GetAvailabilityZonesFilterArgs.builder()
                .name("opt-in-status")
                .values(                
                    "not-opted-in",
                    "opted-in")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        allAvailabilityZones: true
        filters:
          - name: opt-in-status
            values:
              - not-opted-in
              - opted-in
```
<!--End PulumiCodeChooser -->

Only Availability Zones (no Local Zones):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getAvailabilityZones({
    filters: [{
        name: "opt-in-status",
        values: ["opt-in-not-required"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_availability_zones(filters=[{
    "name": "opt-in-status",
    "values": ["opt-in-not-required"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetAvailabilityZones.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.Inputs.GetAvailabilityZonesFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "opt-in-not-required",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			Filters: []aws.GetAvailabilityZonesFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"opt-in-not-required",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .filters(GetAvailabilityZonesFilterArgs.builder()
                .name("opt-in-status")
                .values("opt-in-not-required")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        filters:
          - name: opt-in-status
            values:
              - opt-in-not-required
```
<!--End PulumiCodeChooser -->
�
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
D
excludeNamesB*" ,List of Availability Zone names to exclude.
D
excludeZoneIdsB*" *List of Availability Zone IDs to exclude.
�
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter6Configuration block(s) for filtering. Detailed below.
�
stateB" �Allows to filter list of Availability Zones based on their
current state. Can be either `"available"`, `"information"`, `"impaired"` or
`"unavailable"`. By default the list includes a complete set of Availability Zones
to which the underlying AWS account has access, regardless of their state.
"
allAvailabilityZonesB
 "
excludeNamesB*" "
excludeZoneIdsB*" "p
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter"�

groupNames*" �A set of the Availability Zone Group names. For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"M
names*" >List of the Availability Zone names available to the account.
"
stateB" "M
zoneIds*" <List of the Availability Zone IDs available to the account.
2�[
WgetBillingServiceAccount;aws:index/getBillingServiceAccount:getBillingServiceAccount�YUse this data source to get the Account ID of the [AWS Billing and Cost Management Service Account](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-getting-started.html#step-2) for the purpose of permitting in S3 bucket policy.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = aws.getBillingServiceAccount({});
const billingLogs = new aws.s3.BucketV2("billing_logs", {bucket: "my-billing-tf-test-bucket"});
const billingLogsAcl = new aws.s3.BucketAclV2("billing_logs_acl", {
    bucket: billingLogs.id,
    acl: "private",
});
const allowBillingLogging = pulumi.all([main, billingLogs.arn, main, billingLogs.arn]).apply(([main, billingLogsArn, main1, billingLogsArn1]) => aws.iam.getPolicyDocumentOutput({
    statements: [
        {
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: [main.arn],
            }],
            actions: [
                "s3:GetBucketAcl",
                "s3:GetBucketPolicy",
            ],
            resources: [billingLogsArn],
        },
        {
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: [main1.arn],
            }],
            actions: ["s3:PutObject"],
            resources: [`${billingLogsArn1}/*`],
        },
    ],
}));
const allowBillingLoggingBucketPolicy = new aws.s3.BucketPolicy("allow_billing_logging", {
    bucket: billingLogs.id,
    policy: allowBillingLogging.apply(allowBillingLogging => allowBillingLogging.json),
});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.get_billing_service_account()
billing_logs = aws.s3.BucketV2("billing_logs", bucket="my-billing-tf-test-bucket")
billing_logs_acl = aws.s3.BucketAclV2("billing_logs_acl",
    bucket=billing_logs.id,
    acl="private")
allow_billing_logging = pulumi.Output.all(
    billingLogsArn=billing_logs.arn,
    billingLogsArn1=billing_logs.arn
).apply(lambda resolved_outputs: aws.iam.get_policy_document_output(statements=[
    {
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": [main.arn],
        }],
        "actions": [
            "s3:GetBucketAcl",
            "s3:GetBucketPolicy",
        ],
        "resources": [resolved_outputs['billingLogsArn']],
    },
    {
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": [main.arn],
        }],
        "actions": ["s3:PutObject"],
        "resources": [f"{resolved_outputs['billingLogsArn1']}/*"],
    },
]))

allow_billing_logging_bucket_policy = aws.s3.BucketPolicy("allow_billing_logging",
    bucket=billing_logs.id,
    policy=allow_billing_logging.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = Aws.GetBillingServiceAccount.Invoke();

    var billingLogs = new Aws.S3.BucketV2("billing_logs", new()
    {
        Bucket = "my-billing-tf-test-bucket",
    });

    var billingLogsAcl = new Aws.S3.BucketAclV2("billing_logs_acl", new()
    {
        Bucket = billingLogs.Id,
        Acl = "private",
    });

    var allowBillingLogging = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            main.Apply(getBillingServiceAccountResult => getBillingServiceAccountResult.Arn),
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:GetBucketAcl",
                    "s3:GetBucketPolicy",
                },
                Resources = new[]
                {
                    billingLogs.Arn,
                },
            },
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            main.Apply(getBillingServiceAccountResult => getBillingServiceAccountResult.Arn),
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:PutObject",
                },
                Resources = new[]
                {
                    $"{billingLogs.Arn}/*",
                },
            },
        },
    });

    var allowBillingLoggingBucketPolicy = new Aws.S3.BucketPolicy("allow_billing_logging", new()
    {
        Bucket = billingLogs.Id,
        Policy = allowBillingLogging.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
main, err := aws.GetBillingServiceAccount(ctx, &aws.GetBillingServiceAccountArgs{
}, nil);
if err != nil {
return err
}
billingLogs, err := s3.NewBucketV2(ctx, "billing_logs", &s3.BucketV2Args{
Bucket: pulumi.String("my-billing-tf-test-bucket"),
})
if err != nil {
return err
}
_, err = s3.NewBucketAclV2(ctx, "billing_logs_acl", &s3.BucketAclV2Args{
Bucket: billingLogs.ID(),
Acl: pulumi.String("private"),
})
if err != nil {
return err
}
allowBillingLogging := pulumi.All(billingLogs.Arn,billingLogs.Arn).ApplyT(func(_args []interface{}) (iam.GetPolicyDocumentResult, error) {
billingLogsArn := _args[0].(string)
billingLogsArn1 := _args[1].(string)
return iam.GetPolicyDocumentResult(interface{}(iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: interface{}{
main.Arn,
},
},
},
Actions: []string{
"s3:GetBucketAcl",
"s3:GetBucketPolicy",
},
Resources: []string{
billingLogsArn,
},
},
{
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: interface{}{
main.Arn,
},
},
},
Actions: []string{
"s3:PutObject",
},
Resources: []string{
fmt.Sprintf("%v/*", billingLogsArn1),
},
},
},
}, nil))), nil
}).(iam.GetPolicyDocumentResultOutput)
_, err = s3.NewBucketPolicy(ctx, "allow_billing_logging", &s3.BucketPolicyArgs{
Bucket: billingLogs.ID(),
Policy: pulumi.String(allowBillingLogging.ApplyT(func(allowBillingLogging iam.GetPolicyDocumentResult) (*string, error) {
return &allowBillingLogging.Json, nil
}).(pulumi.StringPtrOutput)),
})
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetBillingServiceAccountArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketAclV2;
import com.pulumi.aws.s3.BucketAclV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.s3.BucketPolicy;
import com.pulumi.aws.s3.BucketPolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var main = AwsFunctions.getBillingServiceAccount();

        var billingLogs = new BucketV2("billingLogs", BucketV2Args.builder()
            .bucket("my-billing-tf-test-bucket")
            .build());

        var billingLogsAcl = new BucketAclV2("billingLogsAcl", BucketAclV2Args.builder()
            .bucket(billingLogs.id())
            .acl("private")
            .build());

        final var allowBillingLogging = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(            
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers(main.applyValue(getBillingServiceAccountResult -> getBillingServiceAccountResult.arn()))
                        .build())
                    .actions(                    
                        "s3:GetBucketAcl",
                        "s3:GetBucketPolicy")
                    .resources(billingLogs.arn())
                    .build(),
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers(main.applyValue(getBillingServiceAccountResult -> getBillingServiceAccountResult.arn()))
                        .build())
                    .actions("s3:PutObject")
                    .resources(billingLogs.arn().applyValue(arn -> String.format("%s/*", arn)))
                    .build())
            .build());

        var allowBillingLoggingBucketPolicy = new BucketPolicy("allowBillingLoggingBucketPolicy", BucketPolicyArgs.builder()
            .bucket(billingLogs.id())
            .policy(allowBillingLogging.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(allowBillingLogging -> allowBillingLogging.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  billingLogs:
    type: aws:s3:BucketV2
    name: billing_logs
    properties:
      bucket: my-billing-tf-test-bucket
  billingLogsAcl:
    type: aws:s3:BucketAclV2
    name: billing_logs_acl
    properties:
      bucket: ${billingLogs.id}
      acl: private
  allowBillingLoggingBucketPolicy:
    type: aws:s3:BucketPolicy
    name: allow_billing_logging
    properties:
      bucket: ${billingLogs.id}
      policy: ${allowBillingLogging.json}
variables:
  main:
    fn::invoke:
      function: aws:getBillingServiceAccount
      arguments: {}
  allowBillingLogging:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - ${main.arn}
            actions:
              - s3:GetBucketAcl
              - s3:GetBucketPolicy
            resources:
              - ${billingLogs.arn}
          - effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - ${main.arn}
            actions:
              - s3:PutObject
            resources:
              - ${billingLogs.arn}/*
```
<!--End PulumiCodeChooser -->
3
idB" 'ID of the AWS billing service account.
"3
arn" (ARN of the AWS billing service account.
"1
id" 'ID of the AWS billing service account.
2�
BgetCallerIdentity-aws:index/getCallerIdentity:getCallerIdentity�Use this data source to get the access to the effective Account ID, User ID, and ARN in
which this provider is authorized.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getCallerIdentity({});
export const accountId = current.then(current => current.accountId);
export const callerArn = current.then(current => current.arn);
export const callerUser = current.then(current => current.userId);
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_caller_identity()
pulumi.export("accountId", current.account_id)
pulumi.export("callerArn", current.arn)
pulumi.export("callerUser", current.user_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetCallerIdentity.Invoke();

    return new Dictionary<string, object?>
    {
        ["accountId"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId),
        ["callerArn"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.Arn),
        ["callerUser"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.UserId),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		ctx.Export("accountId", current.AccountId)
		ctx.Export("callerArn", current.Arn)
		ctx.Export("callerUser", current.UserId)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getCallerIdentity();

        ctx.export("accountId", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()));
        ctx.export("callerArn", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.arn()));
        ctx.export("callerUser", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.userId()));
    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
outputs:
  accountId: ${current.accountId}
  callerArn: ${current.arn}
  callerUser: ${current.userId}
```
<!--End PulumiCodeChooser -->
W
idB" KAccount ID number of the account that owns or contains the calling entity.
"`
	accountId" OAWS Account ID number of the account that owns or contains the calling entity.
"3
arn" (ARN associated with the calling entity.
"U
id" KAccount ID number of the account that owns or contains the calling entity.
"7
userId" )Unique identifier of the calling entity.
2�
9getDefaultTags'aws:index/getDefaultTags:getDefaultTags�Use this data source to get the default tags configured on the provider.

With this data source, you can apply default tags to resources not _directly_ managed by a resource, such as the instances underneath an Auto Scaling group or the volumes created for an EC2 instance.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getDefaultTags({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_default_tags()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetDefaultTags.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetDefaultTags(ctx, &aws.GetDefaultTagsArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetDefaultTagsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getDefaultTags();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getDefaultTags
      arguments: {}
```
<!--End PulumiCodeChooser -->

### Dynamically Apply Default Tags to Auto Scaling Group

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getDefaultTags({});
const exampleGroup = new aws.autoscaling.Group("example", {tags: .map(entry => ({
    key: entry.key,
    value: entry.value,
    propagateAtLaunch: true,
}))});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_default_tags()
example_group = aws.autoscaling.Group("example", tags=[{
    "key": entry["key"],
    "value": entry["value"],
    "propagate_at_launch": True,
} for entry in [{"key": k, "value": v} for k, v in example.tags]])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetDefaultTags.Invoke();

    var exampleGroup = new Aws.AutoScaling.Group("example", new()
    {
        Tags = ,
    });

});
```
<!--End PulumiCodeChooser -->


idB" "
id" ":
tags2" ,Key-value mapping of provider default tags.
2�;
0getIpRanges!aws:index/getIpRanges:getIpRanges�1Use this data source to get the IP ranges of various AWS products and services. For more information about the contents of this data source and required JSON syntax if referencing a custom URL, see the [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const europeanEc2 = aws.getIpRanges({
    regions: [
        "eu-west-1",
        "eu-central-1",
    ],
    services: ["ec2"],
});
const fromEurope = new aws.ec2.SecurityGroup("from_europe", {
    name: "from_europe",
    ingress: [{
        fromPort: 443,
        toPort: 443,
        protocol: "tcp",
        cidrBlocks: europeanEc2.then(europeanEc2 => europeanEc2.cidrBlocks),
        ipv6CidrBlocks: europeanEc2.then(europeanEc2 => europeanEc2.ipv6CidrBlocks),
    }],
    tags: {
        CreateDate: europeanEc2.then(europeanEc2 => europeanEc2.createDate),
        SyncToken: europeanEc2.then(europeanEc2 => europeanEc2.syncToken),
    },
});
```
```python
import pulumi
import pulumi_aws as aws

european_ec2 = aws.get_ip_ranges(regions=[
        "eu-west-1",
        "eu-central-1",
    ],
    services=["ec2"])
from_europe = aws.ec2.SecurityGroup("from_europe",
    name="from_europe",
    ingress=[{
        "from_port": 443,
        "to_port": 443,
        "protocol": "tcp",
        "cidr_blocks": european_ec2.cidr_blocks,
        "ipv6_cidr_blocks": european_ec2.ipv6_cidr_blocks,
    }],
    tags={
        "CreateDate": european_ec2.create_date,
        "SyncToken": european_ec2.sync_token,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var europeanEc2 = Aws.GetIpRanges.Invoke(new()
    {
        Regions = new[]
        {
            "eu-west-1",
            "eu-central-1",
        },
        Services = new[]
        {
            "ec2",
        },
    });

    var fromEurope = new Aws.Ec2.SecurityGroup("from_europe", new()
    {
        Name = "from_europe",
        Ingress = new[]
        {
            new Aws.Ec2.Inputs.SecurityGroupIngressArgs
            {
                FromPort = 443,
                ToPort = 443,
                Protocol = "tcp",
                CidrBlocks = europeanEc2.Apply(getIpRangesResult => getIpRangesResult.CidrBlocks),
                Ipv6CidrBlocks = europeanEc2.Apply(getIpRangesResult => getIpRangesResult.Ipv6CidrBlocks),
            },
        },
        Tags = 
        {
            { "CreateDate", europeanEc2.Apply(getIpRangesResult => getIpRangesResult.CreateDate) },
            { "SyncToken", europeanEc2.Apply(getIpRangesResult => getIpRangesResult.SyncToken) },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		europeanEc2, err := aws.GetIpRanges(ctx, &aws.GetIpRangesArgs{
			Regions: []string{
				"eu-west-1",
				"eu-central-1",
			},
			Services: []string{
				"ec2",
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = ec2.NewSecurityGroup(ctx, "from_europe", &ec2.SecurityGroupArgs{
			Name: pulumi.String("from_europe"),
			Ingress: ec2.SecurityGroupIngressArray{
				&ec2.SecurityGroupIngressArgs{
					FromPort:       pulumi.Int(443),
					ToPort:         pulumi.Int(443),
					Protocol:       pulumi.String("tcp"),
					CidrBlocks:     interface{}(europeanEc2.CidrBlocks),
					Ipv6CidrBlocks: interface{}(europeanEc2.Ipv6CidrBlocks),
				},
			},
			Tags: pulumi.StringMap{
				"CreateDate": pulumi.String(europeanEc2.CreateDate),
				"SyncToken":  pulumi.Int(europeanEc2.SyncToken),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetIpRangesArgs;
import com.pulumi.aws.ec2.SecurityGroup;
import com.pulumi.aws.ec2.SecurityGroupArgs;
import com.pulumi.aws.ec2.inputs.SecurityGroupIngressArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var europeanEc2 = AwsFunctions.getIpRanges(GetIpRangesArgs.builder()
            .regions(            
                "eu-west-1",
                "eu-central-1")
            .services("ec2")
            .build());

        var fromEurope = new SecurityGroup("fromEurope", SecurityGroupArgs.builder()
            .name("from_europe")
            .ingress(SecurityGroupIngressArgs.builder()
                .fromPort("443")
                .toPort("443")
                .protocol("tcp")
                .cidrBlocks(europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.cidrBlocks()))
                .ipv6CidrBlocks(europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.ipv6CidrBlocks()))
                .build())
            .tags(Map.ofEntries(
                Map.entry("CreateDate", europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.createDate())),
                Map.entry("SyncToken", europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.syncToken()))
            ))
            .build());

    }
}
```
```yaml
resources:
  fromEurope:
    type: aws:ec2:SecurityGroup
    name: from_europe
    properties:
      name: from_europe
      ingress:
        - fromPort: '443'
          toPort: '443'
          protocol: tcp
          cidrBlocks: ${europeanEc2.cidrBlocks}
          ipv6CidrBlocks: ${europeanEc2.ipv6CidrBlocks}
      tags:
        CreateDate: ${europeanEc2.createDate}
        SyncToken: ${europeanEc2.syncToken}
variables:
  europeanEc2:
    fn::invoke:
      function: aws:getIpRanges
      arguments:
        regions:
          - eu-west-1
          - eu-central-1
        services:
          - ec2
```
<!--End PulumiCodeChooser -->


idB" �
regionsB*" �Filter IP ranges by regions (or include all regions, if
omitted). Valid items are `global` (for `cloudfront`) as well as all AWS regions
(e.g., `eu-central-1`)
�
services*" �Filter IP ranges by services. Valid items are `amazon`
(for amazon.com), `amazon_connect`, `api_gateway`, `cloud9`, `cloudfront`,
`codebuild`, `dynamodb`, `ec2`, `ec2_instance_connect`, `globalaccelerator`,
`route53`, `route53_healthchecks`, `s3` and `workspaces_gateways`. See the
[`service` attribute][2] documentation for other possible values.

> **NOTE:** If the specified combination of regions and services does not yield any
CIDR blocks, this call will fail.
�
urlB" �Custom URL for source JSON file. Syntax must match [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html). Defaults to `https://ip-ranges.amazonaws.com/ip-ranges.json`.
";

cidrBlocks*" 'Lexically ordered list of CIDR blocks.
"S

createDate" APublication time of the IP ranges (e.g., `2016-08-03-23-46-05`).
"
id" "D
ipv6CidrBlocks*" ,Lexically ordered list of IPv6 CIDR blocks.
"
regionsB*" "
services*" "d
	syncToken SPublication time of the IP ranges, in Unix epoch time format
(e.g., `1470267965`).
"
urlB" 2�"
3getPartition#aws:index/getPartition:getPartition�Use this data source to lookup information about the current AWS partition in
which the provider is working.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getPartition({});
const s3Policy = current.then(current => aws.iam.getPolicyDocument({
    statements: [{
        sid: "1",
        actions: ["s3:ListBucket"],
        resources: [`arn:${current.partition}:s3:::my-bucket`],
    }],
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_partition()
s3_policy = aws.iam.get_policy_document(statements=[{
    "sid": "1",
    "actions": ["s3:ListBucket"],
    "resources": [f"arn:{current.partition}:s3:::my-bucket"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetPartition.Invoke();

    var s3Policy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "1",
                Actions = new[]
                {
                    "s3:ListBucket",
                },
                Resources = new[]
                {
                    $"arn:{current.Apply(getPartitionResult => getPartitionResult.Partition)}:s3:::my-bucket",
                },
            },
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid: pulumi.StringRef("1"),
					Actions: []string{
						"s3:ListBucket",
					},
					Resources: []string{
						fmt.Sprintf("arn:%v:s3:::my-bucket", current.Partition),
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getPartition();

        final var s3Policy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("1")
                .actions("s3:ListBucket")
                .resources(String.format("arn:%s:s3:::my-bucket", current.applyValue(getPartitionResult -> getPartitionResult.partition())))
                .build())
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
  s3Policy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: '1'
            actions:
              - s3:ListBucket
            resources:
              - arn:${current.partition}:s3:::my-bucket
```
<!--End PulumiCodeChooser -->
h
idB" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"�
	dnsSuffix" {Base DNS domain name for the current partition (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
"f
id" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"m
	partition" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"|
reverseDnsPrefix" dPrefix of service names (e.g., `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
2�
*	getRegionaws:index/getRegion:getRegion�`aws.getRegion` provides details about a specific AWS region.

As well as validating a given region name this resource can be used to
discover the name of the region configured within the provider. The latter
can be useful in a child module which is inheriting an AWS provider
configuration from its parent module.

## Example Usage

The following example shows how the resource might be used to obtain
the name of the AWS region configured on the provider.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
```
<!--End PulumiCodeChooser -->
8
endpointB" &EC2 endpoint of the region to select.


idB" 1
nameB" #Full name of the region to select.
"R
description" ?Region's description in this format: "Location (Region name)".
"6
endpoint" &EC2 endpoint for the selected region.
"
id" ")
name" Name of the selected region.
2�4
-
getRegionsaws:index/getRegions:getRegions�/Provides information about AWS Regions. Can be used to filter regions i.e., by Opt-In status or only regions enabled for current account. To get details like endpoint and description of each region the data source can be combined with the `aws.getRegion` data source.

## Example Usage

Enabled AWS Regions:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions();

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments: {}
```
<!--End PulumiCodeChooser -->

All the regions regardless of the availability

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({
    allRegions: true,
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions(all_regions=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke(new()
    {
        AllRegions = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{
			AllRegions: pulumi.BoolRef(true),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions(GetRegionsArgs.builder()
            .allRegions(true)
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments:
        allRegions: true
```
<!--End PulumiCodeChooser -->

To see regions that are filtered by `"not-opted-in"`, the `all_regions` argument needs to be set to `true` or no results will be returned.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({
    allRegions: true,
    filters: [{
        name: "opt-in-status",
        values: ["not-opted-in"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions(all_regions=True,
    filters=[{
        "name": "opt-in-status",
        "values": ["not-opted-in"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke(new()
    {
        AllRegions = true,
        Filters = new[]
        {
            new Aws.Inputs.GetRegionsFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "not-opted-in",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{
			AllRegions: pulumi.BoolRef(true),
			Filters: []aws.GetRegionsFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"not-opted-in",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions(GetRegionsArgs.builder()
            .allRegions(true)
            .filters(GetRegionsFilterArgs.builder()
                .name("opt-in-status")
                .values("not-opted-in")
                .build())
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments:
        allRegions: true
        filters:
          - name: opt-in-status
            values:
              - not-opted-in
```
<!--End PulumiCodeChooser -->
Z

allRegionsB
 FIf true the source will query all regions regardless of availability.
�
filtersGBE*C:A
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter:Configuration block(s) to use as filters. Detailed below.
h
idB" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"

allRegionsB
 "R
filtersGBE*C:A
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter"f
id" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"9
names*" *Names of regions that meets the criteria.
2�9
-
getServiceaws:index/getService:getService�0Use this data source to compose and decompose AWS service DNS names.

## Example Usage

### Get Service DNS Name

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
const test = current.then(current => aws.getService({
    region: current.name,
    serviceId: "ec2",
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
test = aws.get_service(region=current.name,
    service_id="ec2")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

    var test = Aws.GetService.Invoke(new()
    {
        Region = current.Apply(getRegionResult => getRegionResult.Name),
        ServiceId = "ec2",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = aws.GetService(ctx, &aws.GetServiceArgs{
			Region:    pulumi.StringRef(current.Name),
			ServiceId: pulumi.StringRef("ec2"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

        final var test = AwsFunctions.getService(GetServiceArgs.builder()
            .region(current.applyValue(getRegionResult -> getRegionResult.name()))
            .serviceId("ec2")
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
  test:
    fn::invoke:
      function: aws:getService
      arguments:
        region: ${current.name}
        serviceId: ec2
```
<!--End PulumiCodeChooser -->

### Use Service Reverse DNS Name to Get Components

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3 = aws.getService({
    reverseDnsName: "cn.com.amazonaws.cn-north-1.s3",
});
```
```python
import pulumi
import pulumi_aws as aws

s3 = aws.get_service(reverse_dns_name="cn.com.amazonaws.cn-north-1.s3")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3 = Aws.GetService.Invoke(new()
    {
        ReverseDnsName = "cn.com.amazonaws.cn-north-1.s3",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetService(ctx, &aws.GetServiceArgs{
			ReverseDnsName: pulumi.StringRef("cn.com.amazonaws.cn-north-1.s3"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var s3 = AwsFunctions.getService(GetServiceArgs.builder()
            .reverseDnsName("cn.com.amazonaws.cn-north-1.s3")
            .build());

    }
}
```
```yaml
variables:
  s3:
    fn::invoke:
      function: aws:getService
      arguments:
        reverseDnsName: cn.com.amazonaws.cn-north-1.s3
```
<!--End PulumiCodeChooser -->

### Determine Regional Support for a Service

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3 = aws.getService({
    reverseDnsName: "com.amazonaws.us-gov-west-1.waf",
});
```
```python
import pulumi
import pulumi_aws as aws

s3 = aws.get_service(reverse_dns_name="com.amazonaws.us-gov-west-1.waf")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3 = Aws.GetService.Invoke(new()
    {
        ReverseDnsName = "com.amazonaws.us-gov-west-1.waf",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetService(ctx, &aws.GetServiceArgs{
			ReverseDnsName: pulumi.StringRef("com.amazonaws.us-gov-west-1.waf"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var s3 = AwsFunctions.getService(GetServiceArgs.builder()
            .reverseDnsName("com.amazonaws.us-gov-west-1.waf")
            .build());

    }
}
```
```yaml
variables:
  s3:
    fn::invoke:
      function: aws:getService
      arguments:
        reverseDnsName: com.amazonaws.us-gov-west-1.waf
```
<!--End PulumiCodeChooser -->
�
dnsNameB" �DNS name of the service (_e.g.,_ `rds.us-east-1.amazonaws.com`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.


idB" O
regionB" ?Region of the service (_e.g.,_ `us-west-2`, `ap-northeast-1`).
�
reverseDnsNameB" �Reverse DNS name of the service (_e.g.,_ `com.amazonaws.us-west-2.s3`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.
~
reverseDnsPrefixB" dPrefix of the service (_e.g.,_ `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
�
	serviceIdB" �Service endpoint ID (_e.g.,_ `s3`, `rds`, `ec2`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required. A service's endpoint ID can be found in the [_AWS General Reference_](https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html).
"
dnsName" "
id" "
	partition" "
region" "
reverseDnsName" "
reverseDnsPrefix" "
	serviceId" "�
	supported
 uWhether the service is supported in the region's partition. New services may not be listed immediately as supported.
2�
HgetServicePrincipal1aws:index/getServicePrincipal:getServicePrincipal�Use this data source to create a Service Principal Name for a service in a given region. Service Principal Names should always end in the standard global format: `{servicename}.amazonaws.com`. However, in some AWS partitions, AWS may expect a different format.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getServicePrincipal({
    serviceName: "s3",
});
const test = aws.getServicePrincipal({
    serviceName: "s3",
    region: "us-iso-east-1",
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_service_principal(service_name="s3")
test = aws.get_service_principal(service_name="s3",
    region="us-iso-east-1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetServicePrincipal.Invoke(new()
    {
        ServiceName = "s3",
    });

    var test = Aws.GetServicePrincipal.Invoke(new()
    {
        ServiceName = "s3",
        Region = "us-iso-east-1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetServicePrincipal(ctx, &aws.GetServicePrincipalArgs{
			ServiceName: "s3",
		}, nil)
		if err != nil {
			return err
		}
		_, err = aws.GetServicePrincipal(ctx, &aws.GetServicePrincipalArgs{
			ServiceName: "s3",
			Region:      pulumi.StringRef("us-iso-east-1"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServicePrincipalArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getServicePrincipal(GetServicePrincipalArgs.builder()
            .serviceName("s3")
            .build());

        final var test = AwsFunctions.getServicePrincipal(GetServicePrincipalArgs.builder()
            .serviceName("s3")
            .region("us-iso-east-1")
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getServicePrincipal
      arguments:
        serviceName: s3
  test:
    fn::invoke:
      function: aws:getServicePrincipal
      arguments:
        serviceName: s3
        region: us-iso-east-1
```
<!--End PulumiCodeChooser -->
T
regionB" DRegion you'd like the SPN for. By default, uses the current region.
Z
serviceName" GName of the service you want to generate a Service Principal Name for.
"�
id" �Identifier of the current Service Principal (compound of service, region and suffix). (e.g. `logs.us-east-1.amazonaws.com`in AWS Commercial, `logs.cn-north-1.amazonaws.com.cn` in AWS China).
"y
name" mService Principal Name (e.g., `logs.amazonaws.com` in AWS Commercial, `logs.amazonaws.com.cn` in AWS China).
"w
region" iRegion identifier of the generated SPN (e.g., `us-east-1` in AWS Commercial, `cn-north-1` in AWS China).
"
serviceName" "l
suffix" ^Suffix of the SPN (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
2�
O
sesv2getConfigurationSet1aws:sesv2/getConfigurationSet:getConfigurationSet�Data source for managing an AWS SESv2 (Simple Email V2) Configuration Set.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.sesv2.getConfigurationSet({
    configurationSetName: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.get_configuration_set(configuration_set_name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SesV2.GetConfigurationSet.Invoke(new()
    {
        ConfigurationSetName = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.LookupConfigurationSet(ctx, &sesv2.LookupConfigurationSetArgs{
			ConfigurationSetName: "example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.Sesv2Functions;
import com.pulumi.aws.sesv2.inputs.GetConfigurationSetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = Sesv2Functions.getConfigurationSet(GetConfigurationSetArgs.builder()
            .configurationSetName("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:sesv2:getConfigurationSet
      arguments:
        configurationSetName: example
```
<!--End PulumiCodeChooser -->
?
configurationSetName" #The name of the configuration set.
I
tagsB2" 9Key-value map of resource tags for the container recipe.
"	
arn" "
configurationSetName" "�
deliveryOptions*}:{
y
sesv2!getConfigurationSetDeliveryOptionMaws:sesv2/getConfigurationSetDeliveryOption:getConfigurationSetDeliveryOptiontAn object that defines the dedicated IP pool that is used to send emails that you send using the configuration set.
"E
id" ;The provider-assigned unique ID for this managed resource.
"�
reputationOptions�*�:�

sesv2#getConfigurationSetReputationOptionQaws:sesv2/getConfigurationSetReputationOption:getConfigurationSetReputationOption�An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set.
"�
sendingOptions|*z:x
v
sesv2 getConfigurationSetSendingOptionKaws:sesv2/getConfigurationSetSendingOption:getConfigurationSetSendingOptionkAn object that defines whether or not Amazon SES can send email that you send using the configuration set.
"�
suppressionOptions�*�:�
�
sesv2$getConfigurationSetSuppressionOptionSaws:sesv2/getConfigurationSetSuppressionOption:getConfigurationSetSuppressionOption]An object that contains information about the suppression list preferences for your account.
"G
tags2" 9Key-value map of resource tags for the container recipe.
"�
trackingOptions*}:{
y
sesv2!getConfigurationSetTrackingOptionMaws:sesv2/getConfigurationSetTrackingOption:getConfigurationSetTrackingOptionqAn object that defines the open and click tracking options for emails that you send using the configuration set.
"�

vdmOptionsp*n:l
j
sesv2getConfigurationSetVdmOptionCaws:sesv2/getConfigurationSetVdmOption:getConfigurationSetVdmOptionZAn object that contains information about the VDM preferences for your configuration set.
2�
L
sesv2getDedicatedIpPool/aws:sesv2/getDedicatedIpPool:getDedicatedIpPool�Data source for managing an AWS SESv2 (Simple Email V2) Dedicated IP Pool.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.sesv2.getDedicatedIpPool({
    poolName: "my-pool",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.get_dedicated_ip_pool(pool_name="my-pool")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SesV2.GetDedicatedIpPool.Invoke(new()
    {
        PoolName = "my-pool",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.LookupDedicatedIpPool(ctx, &sesv2.LookupDedicatedIpPoolArgs{
			PoolName: "my-pool",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.Sesv2Functions;
import com.pulumi.aws.sesv2.inputs.GetDedicatedIpPoolArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = Sesv2Functions.getDedicatedIpPool(GetDedicatedIpPoolArgs.builder()
            .poolName("my-pool")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:sesv2:getDedicatedIpPool
      arguments:
        poolName: my-pool
```
<!--End PulumiCodeChooser -->
/
poolName" Name of the dedicated IP pool.
4
tagsB2" $A map of tags attached to the pool.
")
arn" ARN of the Dedicated IP Pool.
"�
dedicatedIpss*q:o
m
sesv2getDedicatedIpPoolDedicatedIpEaws:sesv2/getDedicatedIpPoolDedicatedIp:getDedicatedIpPoolDedicatedIpMA list of objects describing the pool's dedicated IP's. See `dedicated_ips`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
poolName" "Y
scalingMode" F(Optional) IP pool scaling mode. Valid values: `STANDARD`, `MANAGED`.
"2
tags2" $A map of tags attached to the pool.
2�
F
sesv2getEmailIdentity+aws:sesv2/getEmailIdentity:getEmailIdentity�Data source for managing an AWS SESv2 (Simple Email V2) Email Identity.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.sesv2.getEmailIdentity({
    emailIdentity: "example.com",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.get_email_identity(email_identity="example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SesV2.GetEmailIdentity.Invoke(new()
    {
        EmailIdentity = "example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sesv2.LookupEmailIdentity(ctx, &sesv2.LookupEmailIdentityArgs{
			EmailIdentity: "example.com",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.Sesv2Functions;
import com.pulumi.aws.sesv2.inputs.GetEmailIdentityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = Sesv2Functions.getEmailIdentity(GetEmailIdentityArgs.builder()
            .emailIdentity("example.com")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:sesv2:getEmailIdentity
      arguments:
        emailIdentity: example.com
```
<!--End PulumiCodeChooser -->
5
emailIdentity"  The name of the email identity.
4
tagsB2" $Key-value mapping of resource tags.
"&
arn" ARN of the Email Identity.
"
configurationSetName" "�
dkimSigningAttributes�*�:�
�
sesv2$getEmailIdentityDkimSigningAttributeSaws:sesv2/getEmailIdentityDkimSigningAttribute:getEmailIdentityDkimSigningAttribute�A list of objects that contains at most one element with information about the private key and selector that you want to use to configure DKIM for the identity for Bring Your Own DKIM (BYODKIM) for the identity, or, configures the key length to be used for Easy DKIM.
"
emailIdentity" "E
id" ;The provider-assigned unique ID for this managed resource.
"V
identityType" BThe email identity type. Valid values: `EMAIL_ADDRESS`, `DOMAIN`.
"2
tags2" $Key-value mapping of resource tags.
"S
verifiedForSendingStatus
 3Specifies whether or not the identity is verified.
2�
|
sesv2"getEmailIdentityMailFromAttributesOaws:sesv2/getEmailIdentityMailFromAttributes:getEmailIdentityMailFromAttributes�Data source for managing an AWS SESv2 (Simple Email V2) Email Identity Mail From Attributes.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.sesv2.getEmailIdentity({
    emailIdentity: "example.com",
});
const exampleGetEmailIdentityMailFromAttributes = example.then(example => aws.sesv2.getEmailIdentityMailFromAttributes({
    emailIdentity: example.emailIdentity,
}));
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sesv2.get_email_identity(email_identity="example.com")
example_get_email_identity_mail_from_attributes = aws.sesv2.get_email_identity_mail_from_attributes(email_identity=example.email_identity)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SesV2.GetEmailIdentity.Invoke(new()
    {
        EmailIdentity = "example.com",
    });

    var exampleGetEmailIdentityMailFromAttributes = Aws.SesV2.GetEmailIdentityMailFromAttributes.Invoke(new()
    {
        EmailIdentity = example.Apply(getEmailIdentityResult => getEmailIdentityResult.EmailIdentity),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sesv2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sesv2.LookupEmailIdentity(ctx, &sesv2.LookupEmailIdentityArgs{
			EmailIdentity: "example.com",
		}, nil)
		if err != nil {
			return err
		}
		_, err = sesv2.LookupEmailIdentityMailFromAttributes(ctx, &sesv2.LookupEmailIdentityMailFromAttributesArgs{
			EmailIdentity: example.EmailIdentity,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sesv2.Sesv2Functions;
import com.pulumi.aws.sesv2.inputs.GetEmailIdentityArgs;
import com.pulumi.aws.sesv2.inputs.GetEmailIdentityMailFromAttributesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = Sesv2Functions.getEmailIdentity(GetEmailIdentityArgs.builder()
            .emailIdentity("example.com")
            .build());

        final var exampleGetEmailIdentityMailFromAttributes = Sesv2Functions.getEmailIdentityMailFromAttributes(GetEmailIdentityMailFromAttributesArgs.builder()
            .emailIdentity(example.applyValue(getEmailIdentityResult -> getEmailIdentityResult.emailIdentity()))
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:sesv2:getEmailIdentity
      arguments:
        emailIdentity: example.com
  exampleGetEmailIdentityMailFromAttributes:
    fn::invoke:
      function: aws:sesv2:getEmailIdentityMailFromAttributes
      arguments:
        emailIdentity: ${example.emailIdentity}
```
<!--End PulumiCodeChooser -->
5
emailIdentity"  The name of the email identity.
"�
behaviorOnMxFailure" �The action to take if the required MX record isn't found when you send an email. Valid values: `USE_DEFAULT_VALUE`, `REJECT_MESSAGE`.
"
emailIdentity" "E
id" ;The provider-assigned unique ID for this managed resource.
"^
mailFromDomain" HThe custom MAIL FROM domain that you want the verified identity to use.
2�
3
sfngetActivityaws:sfn/getActivity:getActivity�Provides a Step Functions Activity data source

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const sfnActivity = aws.sfn.getActivity({
    name: "my-activity",
});
```
```python
import pulumi
import pulumi_aws as aws

sfn_activity = aws.sfn.get_activity(name="my-activity")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var sfnActivity = Aws.Sfn.GetActivity.Invoke(new()
    {
        Name = "my-activity",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sfn.LookupActivity(ctx, &sfn.LookupActivityArgs{
			Name: pulumi.StringRef("my-activity"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.SfnFunctions;
import com.pulumi.aws.sfn.inputs.GetActivityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var sfnActivity = SfnFunctions.getActivity(GetActivityArgs.builder()
            .name("my-activity")
            .build());

    }
}
```
```yaml
variables:
  sfnActivity:
    fn::invoke:
      function: aws:sfn:getActivity
      arguments:
        name: my-activity
```
<!--End PulumiCodeChooser -->
/
arnB" "ARN that identifies the activity.
1
nameB" #Name that identifies the activity.
"	
arn" "3
creationDate" Date the activity was created.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" 2�
*
sfngetAliasaws:sfn/getAlias:getAlias�Data source for managing an AWS SFN (Step Functions) State Machine Alias.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.sfn.getAlias({
    name: "my_sfn_alias",
    statemachineArn: sfnTest.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sfn.get_alias(name="my_sfn_alias",
    statemachine_arn=sfn_test["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Sfn.GetAlias.Invoke(new()
    {
        Name = "my_sfn_alias",
        StatemachineArn = sfnTest.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sfn.LookupAlias(ctx, &sfn.LookupAliasArgs{
			Name:            "my_sfn_alias",
			StatemachineArn: sfnTest.Arn,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.SfnFunctions;
import com.pulumi.aws.sfn.inputs.GetAliasArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SfnFunctions.getAlias(GetAliasArgs.builder()
            .name("my_sfn_alias")
            .statemachineArn(sfnTest.arn())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:sfn:getAlias
      arguments:
        name: my_sfn_alias
        statemachineArn: ${sfnTest.arn}
```
<!--End PulumiCodeChooser -->
9
descriptionB" $Description of state machine alias.
-
name" !Name of the State Machine alias.
1
statemachineArn" ARN of the State Machine.
"4
arn" )ARN identifying the State Machine alias.
">
creationDate" *Date the state machine Alias was created.
"9
descriptionB" $Description of state machine alias.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "�
routingConfigurationsl*j:h
f
sfngetAliasRoutingConfigurationAaws:sfn/getAliasRoutingConfiguration:getAliasRoutingConfiguration-Routing Configuration of state machine alias
"
statemachineArn" 2�
?
sfngetStateMachine'aws:sfn/getStateMachine:getStateMachine�Use this data source to get the ARN of a State Machine in AWS Step
Function (SFN). By using this data source, you can reference a
state machine without having to hard code the ARNs as input.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.sfn.getStateMachine({
    name: "an_example_sfn_name",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sfn.get_state_machine(name="an_example_sfn_name")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Sfn.GetStateMachine.Invoke(new()
    {
        Name = "an_example_sfn_name",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sfn.LookupStateMachine(ctx, &sfn.LookupStateMachineArgs{
			Name: "an_example_sfn_name",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.SfnFunctions;
import com.pulumi.aws.sfn.inputs.GetStateMachineArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SfnFunctions.getStateMachine(GetStateMachineArgs.builder()
            .name("an_example_sfn_name")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:sfn:getStateMachine
      arguments:
        name: an_example_sfn_name
```
<!--End PulumiCodeChooser -->
9
name" -Friendly name of the state machine to match.
"1
arn" &Set to the arn of the state function.
"8
creationDate" $Date the state machine was created.
"7

definition" %Set to the state machine definition.
"
description" "E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "A

revisionId" /The revision identifier for the state machine.
"?
roleArn" 0Set to the role_arn used by the state function.
">
status" 0Set to the current status of the state machine.
2�
W
sfngetStateMachineVersions7aws:sfn/getStateMachineVersions:getStateMachineVersions�Data source for managing an AWS SFN (Step Functions) State Machine Versions.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.sfn.getStateMachineVersions({
    statemachineArn: testAwsSfnStateMachine.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.sfn.get_state_machine_versions(statemachine_arn=test_aws_sfn_state_machine["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.Sfn.GetStateMachineVersions.Invoke(new()
    {
        StatemachineArn = testAwsSfnStateMachine.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sfn"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sfn.GetStateMachineVersions(ctx, &sfn.GetStateMachineVersionsArgs{
			StatemachineArn: testAwsSfnStateMachine.Arn,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sfn.SfnFunctions;
import com.pulumi.aws.sfn.inputs.GetStateMachineVersionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = SfnFunctions.getStateMachineVersions(GetStateMachineVersionsArgs.builder()
            .statemachineArn(testAwsSfnStateMachine.arn())
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:sfn:getStateMachineVersions
      arguments:
        statemachineArn: ${testAwsSfnStateMachine.arn}
```
<!--End PulumiCodeChooser -->
1
statemachineArn" ARN of the State Machine.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
statemachineArn" "N
statemachineVersions*" 0ARN List identifying the statemachine versions.
2�#
?
shieldgetProtection&aws:shield/getProtection:getProtection� Data source for managing an AWS Shield Protection.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.shield.getProtection({
    protectionId: "abc123",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.shield.get_protection(protection_id="abc123")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Shield.GetProtection.Invoke(new()
    {
        ProtectionId = "abc123",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := shield.LookupProtection(ctx, &shield.LookupProtectionArgs{
			ProtectionId: pulumi.StringRef("abc123"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.shield.ShieldFunctions;
import com.pulumi.aws.shield.inputs.GetProtectionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ShieldFunctions.getProtection(GetProtectionArgs.builder()
            .protectionId("abc123")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:shield:getProtection
      arguments:
        protectionId: abc123
```
<!--End PulumiCodeChooser -->

### By Resource ARN

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.shield.getProtection({
    resourceArn: "arn:aws:globalaccelerator::123456789012:accelerator/1234abcd-abcd-1234-abcd-1234abcdefgh",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.shield.get_protection(resource_arn="arn:aws:globalaccelerator::123456789012:accelerator/1234abcd-abcd-1234-abcd-1234abcdefgh")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Shield.GetProtection.Invoke(new()
    {
        ResourceArn = "arn:aws:globalaccelerator::123456789012:accelerator/1234abcd-abcd-1234-abcd-1234abcdefgh",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/shield"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := shield.LookupProtection(ctx, &shield.LookupProtectionArgs{
			ResourceArn: pulumi.StringRef("arn:aws:globalaccelerator::123456789012:accelerator/1234abcd-abcd-1234-abcd-1234abcdefgh"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.shield.ShieldFunctions;
import com.pulumi.aws.shield.inputs.GetProtectionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ShieldFunctions.getProtection(GetProtectionArgs.builder()
            .resourceArn("arn:aws:globalaccelerator::123456789012:accelerator/1234abcd-abcd-1234-abcd-1234abcdefgh")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:shield:getProtection
      arguments:
        resourceArn: arn:aws:globalaccelerator::123456789012:accelerator/1234abcd-abcd-1234-abcd-1234abcdefgh
```
<!--End PulumiCodeChooser -->
<
protectionIdB" &Unique identifier for the protection.
Q
resourceArnB" <ARN (Amazon Resource Name) of the resource being protected.
"
id" "$
name" Name of the protection.
",
protectionArn" ARN of the protection.
"
protectionId" "
resourceArn" 2�
?
signergetSigningJob&aws:signer/getSigningJob:getSigningJob�Provides information about a Signer Signing Job.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const buildSigningJob = aws.signer.getSigningJob({
    jobId: "9ed7e5c3-b8d4-4da0-8459-44e0b068f7ee",
});
```
```python
import pulumi
import pulumi_aws as aws

build_signing_job = aws.signer.get_signing_job(job_id="9ed7e5c3-b8d4-4da0-8459-44e0b068f7ee")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var buildSigningJob = Aws.Signer.GetSigningJob.Invoke(new()
    {
        JobId = "9ed7e5c3-b8d4-4da0-8459-44e0b068f7ee",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/signer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := signer.LookupSigningJob(ctx, &signer.LookupSigningJobArgs{
			JobId: "9ed7e5c3-b8d4-4da0-8459-44e0b068f7ee",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.signer.SignerFunctions;
import com.pulumi.aws.signer.inputs.GetSigningJobArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var buildSigningJob = SignerFunctions.getSigningJob(GetSigningJobArgs.builder()
            .jobId("9ed7e5c3-b8d4-4da0-8459-44e0b068f7ee")
            .build());

    }
}
```
```yaml
variables:
  buildSigningJob:
    fn::invoke:
      function: aws:signer:getSigningJob
      arguments:
        jobId: 9ed7e5c3-b8d4-4da0-8459-44e0b068f7ee
```
<!--End PulumiCodeChooser -->
.
jobId" !ID of the signing job on output.
"�
completedAt" wDate and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the signing job was completed.
"�
	createdAt" uDate and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the signing job was created.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
jobId" "=

jobInvoker" +IAM entity that initiated the signing job.
"1
jobOwner" !AWS account ID of the job owner.
"k
platformDisplayName" PA human-readable name for the signing platform associated with the signing job.
"P

platformId" >Platform to which your signed code image will be distributed.
"M
profileName" :Name of the profile that initiated the signing operation.
"W
profileVersion" AVersion of the signing profile used to initiate the signing job.
"A
requestedBy" .IAM principal that requested the signing job.
"�
revocationRecordsu*s:q
o
signergetSigningJobRevocationRecordFaws:signer/getSigningJobRevocationRecord:getSigningJobRevocationRecord�Revocation record if the signature generated by the signing job has been revoked. Contains a timestamp and the ID of the IAM entity that revoked the signature.
"P
signatureExpiresAt" 6The time when the signature of a signing job expires.
"�
signedObjectsi*g:e
c
signergetSigningJobSignedObject>aws:signer/getSigningJobSignedObject:getSigningJobSignedObjectLName of the S3 bucket where the signed code image is saved by code signing.
"�
sourcesW*U:S
Q
signergetSigningJobSource2aws:signer/getSigningJobSource:getSigningJobSourceBObject that contains the name of your S3 bucket or your raw code.
")
status" Status of the signing job.
"B
statusReason" .String value that contains the status reason.
2�
K
signergetSigningProfile.aws:signer/getSigningProfile:getSigningProfile�Provides information about a Signer Signing Profile.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const productionSigningProfile = aws.signer.getSigningProfile({
    name: "prod_profile_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK",
});
```
```python
import pulumi
import pulumi_aws as aws

production_signing_profile = aws.signer.get_signing_profile(name="prod_profile_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var productionSigningProfile = Aws.Signer.GetSigningProfile.Invoke(new()
    {
        Name = "prod_profile_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/signer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := signer.LookupSigningProfile(ctx, &signer.LookupSigningProfileArgs{
			Name: "prod_profile_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.signer.SignerFunctions;
import com.pulumi.aws.signer.inputs.GetSigningProfileArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var productionSigningProfile = SignerFunctions.getSigningProfile(GetSigningProfileArgs.builder()
            .name("prod_profile_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK")
            .build());

    }
}
```
```yaml
variables:
  productionSigningProfile:
    fn::invoke:
      function: aws:signer:getSigningProfile
      arguments:
        name: prod_profile_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK
```
<!--End PulumiCodeChooser -->
0
name" $Name of the target signing profile.
B
tagsB2" 2List of tags associated with the signing profile.
"(
arn" ARN for the signing profile.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "o
platformDisplayName" TA human-readable name for the signing platform associated with the signing profile.
"Q

platformId" ?ID of the platform that is used by the target signing profile.
"�
revocationRecords�*:}
{
signer!getSigningProfileRevocationRecordNaws:signer/getSigningProfileRevocationRecord:getSigningProfileRevocationRecord.Revocation information for a signing profile.
"�
signatureValidityPeriods�*�:�
�
signer(getSigningProfileSignatureValidityPeriod\aws:signer/getSigningProfileSignatureValidityPeriod:getSigningProfileSignatureValidityPeriod'The validity period for a signing job.
"4
status" &Status of the target signing profile.
"@
tags2" 2List of tags associated with the signing profile.
"7
version" (Current version of the signing profile.
"F

versionArn" 4Signing profile ARN, including the profile version.
2�
*
snsgetTopicaws:sns/getTopic:getTopic�Use this data source to get the ARN of a topic in AWS Simple Notification
Service (SNS). By using this data source, you can reference SNS topics
without having to hard code the ARNs as input.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.sns.getTopic({
    name: "an_example_topic",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sns.get_topic(name="an_example_topic")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Sns.GetTopic.Invoke(new()
    {
        Name = "an_example_topic",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sns"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sns.LookupTopic(ctx, &sns.LookupTopicArgs{
			Name: "an_example_topic",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sns.SnsFunctions;
import com.pulumi.aws.sns.inputs.GetTopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SnsFunctions.getTopic(GetTopicArgs.builder()
            .name("an_example_topic")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:sns:getTopic
      arguments:
        name: an_example_topic
```
<!--End PulumiCodeChooser -->
1
name" %Friendly name of the topic to match.
.
tagsB2" Map of tags for the resource.
"h
arn" ]ARN of the found topic, suitable for referencing in other resources that support SNS topics.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" ",
tags2" Map of tags for the resource.
2�
*
sqsgetQueueaws:sqs/getQueue:getQueue�Use this data source to get the ARN and URL of queue in AWS Simple Queue Service (SQS).
By using this data source, you can reference SQS queues without having to hardcode
the ARNs as input.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.sqs.getQueue({
    name: "queue",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sqs.get_queue(name="queue")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Sqs.GetQueue.Invoke(new()
    {
        Name = "queue",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sqs.LookupQueue(ctx, &sqs.LookupQueueArgs{
			Name: "queue",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.SqsFunctions;
import com.pulumi.aws.sqs.inputs.GetQueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SqsFunctions.getQueue(GetQueueArgs.builder()
            .name("queue")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:sqs:getQueue
      arguments:
        name: queue
```
<!--End PulumiCodeChooser -->
(
name" Name of the queue to match.
.
tagsB2" Map of tags for the resource.
"
arn" ARN of the queue.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" ",
tags2" Map of tags for the resource.
"
url" URL of the queue.
2�
-
sqs	getQueuesaws:sqs/getQueues:getQueues�Data source for managing an AWS SQS (Simple Queue) Queues.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.sqs.getQueues({
    queueNamePrefix: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.sqs.get_queues(queue_name_prefix="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Sqs.GetQueues.Invoke(new()
    {
        QueueNamePrefix = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := sqs.GetQueues(ctx, &sqs.GetQueuesArgs{
			QueueNamePrefix: pulumi.StringRef("example"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.sqs.SqsFunctions;
import com.pulumi.aws.sqs.inputs.GetQueuesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SqsFunctions.getQueues(GetQueuesArgs.builder()
            .queueNamePrefix("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:sqs:getQueues
      arguments:
        queueNamePrefix: example
```
<!--End PulumiCodeChooser -->
�
queueNamePrefixB" �A string to use for filtering the list results. Only those queues whose name begins with the specified string are returned. Queue URLs and names are case-sensitive.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
queueNamePrefixB" ")
	queueUrls*" A list of queue URLs.
2�
K
ssmgetContactsRotation/aws:ssm/getContactsRotation:getContactsRotation�## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssm.getContactsRotation({
    arn: "arn:aws:ssm-contacts:us-east-1:012345678910:rotation/example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.get_contacts_rotation(arn="arn:aws:ssm-contacts:us-east-1:012345678910:rotation/example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Ssm.GetContactsRotation.Invoke(new()
    {
        Arn = "arn:aws:ssm-contacts:us-east-1:012345678910:rotation/example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.LookupContactsRotation(ctx, &ssm.LookupContactsRotationArgs{
			Arn: "arn:aws:ssm-contacts:us-east-1:012345678910:rotation/example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetContactsRotationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsmFunctions.getContactsRotation(GetContactsRotationArgs.builder()
            .arn("arn:aws:ssm-contacts:us-east-1:012345678910:rotation/example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssm:getContactsRotation
      arguments:
        arn: arn:aws:ssm-contacts:us-east-1:012345678910:rotation/example
```
<!--End PulumiCodeChooser -->
;
arn" 0The Amazon Resource Name (ARN) of the rotation.
"	
arn" "�

contactIds*" �The Amazon Resource Names (ARNs) of the contacts to add to the rotation. The order in which you list the contacts is their shift order in the rotation schedule.
"
id" "'
name" The name for the rotation.
"�
recurrenceso*m:k
i
ssmgetContactsRotationRecurrenceCaws:ssm/getContactsRotationRecurrence:getContactsRotationRecurrence`Information about when an on-call rotation is in effect and how long the rotation period lasts.
"\
	startTime" KThe date and time, in RFC 3339 format, that the rotation goes into effect.
"7
tags2" )A map of tags to assign to the resource.
"{

timeZoneId" iThe time zone to base the rotation’s activity on in Internet Assigned Numbers Authority (IANA) format.
2�(
3
ssmgetDocumentaws:ssm/getDocument:getDocument�#Gets the contents of the specified Systems Manager document.

## Example Usage

To get the contents of the document owned by AWS.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = aws.ssm.getDocument({
    name: "AWS-GatherSoftwareInventory",
    documentFormat: "YAML",
});
export const content = foo.then(foo => foo.content);
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.ssm.get_document(name="AWS-GatherSoftwareInventory",
    document_format="YAML")
pulumi.export("content", foo.content)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var foo = Aws.Ssm.GetDocument.Invoke(new()
    {
        Name = "AWS-GatherSoftwareInventory",
        DocumentFormat = "YAML",
    });

    return new Dictionary<string, object?>
    {
        ["content"] = foo.Apply(getDocumentResult => getDocumentResult.Content),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		foo, err := ssm.LookupDocument(ctx, &ssm.LookupDocumentArgs{
			Name:           "AWS-GatherSoftwareInventory",
			DocumentFormat: pulumi.StringRef("YAML"),
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("content", foo.Content)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetDocumentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var foo = SsmFunctions.getDocument(GetDocumentArgs.builder()
            .name("AWS-GatherSoftwareInventory")
            .documentFormat("YAML")
            .build());

        ctx.export("content", foo.applyValue(getDocumentResult -> getDocumentResult.content()));
    }
}
```
```yaml
variables:
  foo:
    fn::invoke:
      function: aws:ssm:getDocument
      arguments:
        name: AWS-GatherSoftwareInventory
        documentFormat: YAML
outputs:
  content: ${foo.content}
```
<!--End PulumiCodeChooser -->

To get the contents of the custom document.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.ssm.getDocument({
    name: testAwsSsmDocument.name,
    documentFormat: "JSON",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.ssm.get_document(name=test_aws_ssm_document["name"],
    document_format="JSON")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.Ssm.GetDocument.Invoke(new()
    {
        Name = testAwsSsmDocument.Name,
        DocumentFormat = "JSON",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.LookupDocument(ctx, &ssm.LookupDocumentArgs{
			Name:           testAwsSsmDocument.Name,
			DocumentFormat: pulumi.StringRef("JSON"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetDocumentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = SsmFunctions.getDocument(GetDocumentArgs.builder()
            .name(testAwsSsmDocument.name())
            .documentFormat("JSON")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:ssm:getDocument
      arguments:
        name: ${testAwsSsmDocument.name}
        documentFormat: JSON
```
<!--End PulumiCodeChooser -->
Z
documentFormatB" BThe format of the document. Valid values: `JSON`, `TEXT`, `YAML`.
/
documentVersionB" The document version.
&
name" The name of the document.
"�
arn" }ARN of the document. If the document is an AWS managed document, this value will be set to the name of the document instead.
"H
content" 9The content for the SSM document in JSON or YAML format.
"
documentFormatB" ".
documentType" The type of the document.
"
documentVersionB" "E
id" ;The provider-assigned unique ID for this managed resource.
"

name" 2�
6
ssmgetInstances!aws:ssm/getInstances:getInstances�Use this data source to get the instance IDs of SSM managed instances.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssm.getInstances({
    filters: [{
        name: "PlatformTypes",
        values: ["Linux"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.get_instances(filters=[{
    "name": "PlatformTypes",
    "values": ["Linux"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Ssm.GetInstances.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.Ssm.Inputs.GetInstancesFilterInputArgs
            {
                Name = "PlatformTypes",
                Values = new[]
                {
                    "Linux",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.GetInstances(ctx, &ssm.GetInstancesArgs{
			Filters: []ssm.GetInstancesFilter{
				{
					Name: "PlatformTypes",
					Values: []string{
						"Linux",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetInstancesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsmFunctions.getInstances(GetInstancesArgs.builder()
            .filters(GetInstancesFilterArgs.builder()
                .name("PlatformTypes")
                .values("Linux")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssm:getInstances
      arguments:
        filters:
          - name: PlatformTypes
            values:
              - Linux
```
<!--End PulumiCodeChooser -->
�
filtersPBN*L:J
H
ssmgetInstancesFilter-aws:ssm/getInstancesFilter:getInstancesFilter6Configuration block(s) for filtering. Detailed below.
"[
filtersPBN*L:J
H
ssmgetInstancesFilter-aws:ssm/getInstancesFilter:getInstancesFilter"E
id" ;The provider-assigned unique ID for this managed resource.
"G
ids*" :Set of instance IDs of the matched SSM managed instances.
2�
Q
ssmgetMaintenanceWindows3aws:ssm/getMaintenanceWindows:getMaintenanceWindows�Use this data source to get the window IDs of SSM maintenance windows.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssm.getMaintenanceWindows({
    filters: [{
        name: "Enabled",
        values: ["true"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.get_maintenance_windows(filters=[{
    "name": "Enabled",
    "values": ["true"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Ssm.GetMaintenanceWindows.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.Ssm.Inputs.GetMaintenanceWindowsFilterInputArgs
            {
                Name = "Enabled",
                Values = new[]
                {
                    "true",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.GetMaintenanceWindows(ctx, &ssm.GetMaintenanceWindowsArgs{
			Filters: []ssm.GetMaintenanceWindowsFilter{
				{
					Name: "Enabled",
					Values: []string{
						"true",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetMaintenanceWindowsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsmFunctions.getMaintenanceWindows(GetMaintenanceWindowsArgs.builder()
            .filters(GetMaintenanceWindowsFilterArgs.builder()
                .name("Enabled")
                .values("true")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssm:getMaintenanceWindows
      arguments:
        filters:
          - name: Enabled
            values:
              - 'true'
```
<!--End PulumiCodeChooser -->
�
filterskBi*g:e
c
ssmgetMaintenanceWindowsFilter?aws:ssm/getMaintenanceWindowsFilter:getMaintenanceWindowsFilter6Configuration block(s) for filtering. Detailed below.
"v
filterskBi*g:e
c
ssmgetMaintenanceWindowsFilter?aws:ssm/getMaintenanceWindowsFilter:getMaintenanceWindowsFilter"E
id" ;The provider-assigned unique ID for this managed resource.
"H
ids*" ;List of window IDs of the matched SSM maintenance windows.
2�
6
ssmgetParameter!aws:ssm/getParameter:getParameter�Provides an SSM Parameter data source.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = aws.ssm.getParameter({
    name: "foo",
});
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.ssm.get_parameter(name="foo")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var foo = Aws.Ssm.GetParameter.Invoke(new()
    {
        Name = "foo",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.LookupParameter(ctx, &ssm.LookupParameterArgs{
			Name: "foo",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetParameterArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var foo = SsmFunctions.getParameter(GetParameterArgs.builder()
            .name("foo")
            .build());

    }
}
```
```yaml
variables:
  foo:
    fn::invoke:
      function: aws:ssm:getParameter
      arguments:
        name: foo
```
<!--End PulumiCodeChooser -->

> **Note:** The unencrypted value of a SecureString will be stored in the raw state as plain-text.
#
name" Name of the parameter.
�
withDecryptionB
 �Whether to return decrypted `SecureString` value. Defaults to `true`.

In addition to all arguments above, the following attributes are exported:
"	
arn" "E
id" ;The provider-assigned unique ID for this managed resource.
"
insecureValue" "

name" "

type" "
value" "
version "
withDecryptionB
 2�
K
ssmgetParametersByPath/aws:ssm/getParametersByPath:getParametersByPath�
path" �The hierarchy for the parameter. Hierarchies start with a forward slash (/). The hierarchy is the parameter name except the last part of the parameter. The last part of the parameter name can't be in the path. A parameter name hierarchy can have a maximum of 15 levels. **Note:** If the parameter name (e.g., `/my-app/my-param`) is specified, the data source will not retrieve any value as designed, unless there are other parameters that happen to use the former path in their hierarchy (e.g., `/my-app/my-param/my-actual-param`).
a
	recursiveB
 NWhether to retrieve all parameters within the hirerachy. Defaults to `false`.
�
withDecryptionB
 �Whether to retrieve all parameters in the hierarchy, particularly those of `SecureString` type, with their value decrypted. Defaults to `true`.
"a
arns*" SA list that contains the Amazon Resource Names (ARNs) of the retrieved parameters.
"E
id" ;The provider-assigned unique ID for this managed resource.
"K
names*" <A list that contains the names of the retrieved parameters.
"

path" "
	recursiveB
 "s
types*" dA list that contains the types (`String`, `StringList`, or `SecureString`) of retrieved parameters.
"�
values*" �A list that contains the retrieved parameter values. **Note:** This value is always marked as sensitive in the pulumi preview output, regardless of whether any retrieved parameters are of `SecureString` type. Use the `nonsensitive` function to override the behavior at your own risk and discretion, if you are certain that there are no sensitive values being retrieved.
"
withDecryptionB
 2�4
B
ssmgetPatchBaseline)aws:ssm/getPatchBaseline:getPatchBaseline�%Provides an SSM Patch Baseline data source. Useful if you wish to reuse the default baselines provided.

## Example Usage

To retrieve a baseline provided by AWS:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const centos = aws.ssm.getPatchBaseline({
    owner: "AWS",
    namePrefix: "AWS-",
    operatingSystem: "CENTOS",
});
```
```python
import pulumi
import pulumi_aws as aws

centos = aws.ssm.get_patch_baseline(owner="AWS",
    name_prefix="AWS-",
    operating_system="CENTOS")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var centos = Aws.Ssm.GetPatchBaseline.Invoke(new()
    {
        Owner = "AWS",
        NamePrefix = "AWS-",
        OperatingSystem = "CENTOS",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.LookupPatchBaseline(ctx, &ssm.LookupPatchBaselineArgs{
			Owner:           "AWS",
			NamePrefix:      pulumi.StringRef("AWS-"),
			OperatingSystem: pulumi.StringRef("CENTOS"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetPatchBaselineArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var centos = SsmFunctions.getPatchBaseline(GetPatchBaselineArgs.builder()
            .owner("AWS")
            .namePrefix("AWS-")
            .operatingSystem("CENTOS")
            .build());

    }
}
```
```yaml
variables:
  centos:
    fn::invoke:
      function: aws:ssm:getPatchBaseline
      arguments:
        owner: AWS
        namePrefix: AWS-
        operatingSystem: CENTOS
```
<!--End PulumiCodeChooser -->

To retrieve a baseline on your account:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const defaultCustom = aws.ssm.getPatchBaseline({
    owner: "Self",
    namePrefix: "MyCustomBaseline",
    defaultBaseline: true,
    operatingSystem: "WINDOWS",
});
```
```python
import pulumi
import pulumi_aws as aws

default_custom = aws.ssm.get_patch_baseline(owner="Self",
    name_prefix="MyCustomBaseline",
    default_baseline=True,
    operating_system="WINDOWS")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var defaultCustom = Aws.Ssm.GetPatchBaseline.Invoke(new()
    {
        Owner = "Self",
        NamePrefix = "MyCustomBaseline",
        DefaultBaseline = true,
        OperatingSystem = "WINDOWS",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.LookupPatchBaseline(ctx, &ssm.LookupPatchBaselineArgs{
			Owner:           "Self",
			NamePrefix:      pulumi.StringRef("MyCustomBaseline"),
			DefaultBaseline: pulumi.BoolRef(true),
			OperatingSystem: pulumi.StringRef("WINDOWS"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetPatchBaselineArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var defaultCustom = SsmFunctions.getPatchBaseline(GetPatchBaselineArgs.builder()
            .owner("Self")
            .namePrefix("MyCustomBaseline")
            .defaultBaseline(true)
            .operatingSystem("WINDOWS")
            .build());

    }
}
```
```yaml
variables:
  defaultCustom:
    fn::invoke:
      function: aws:ssm:getPatchBaseline
      arguments:
        owner: Self
        namePrefix: MyCustomBaseline
        defaultBaseline: true
        operatingSystem: WINDOWS
```
<!--End PulumiCodeChooser -->
[
defaultBaselineB
 BFilters the results against the baselines default_baseline field.
@

namePrefixB" ,Filter results by the baseline name prefix.
�
operatingSystemB" �Specified OS for the baseline. Valid values: `AMAZON_LINUX`, `AMAZON_LINUX_2`, `UBUNTU`, `REDHAT_ENTERPRISE_LINUX`, `SUSE`, `CENTOS`, `ORACLE_LINUX`, `DEBIAN`, `MACOS`, `RASPBIAN` and `ROCKY_LINUX`.
�
owner" xOwner of the baseline. Valid values: `All`, `AWS`, `Self` (the current account).

The following arguments are optional:
"�
approvalRulesl*j:h
f
ssmgetPatchBaselineApprovalRuleAaws:ssm/getPatchBaselineApprovalRule:getPatchBaselineApprovalRule7List of rules used to include patches in the baseline.
"O
approvedPatches*" 6List of explicitly approved patches for the baseline.
"M
approvedPatchesComplianceLevel" 'Compliance level for approved patches.
"�
 approvedPatchesEnableNonSecurity
 vIndicates whether the list of approved patches includes non-security updates that should be applied to the instances.
"
defaultBaselineB
 "0
description" Description of the baseline.
"�
globalFiltersl*j:h
f
ssmgetPatchBaselineGlobalFilterAaws:ssm/getPatchBaselineGlobalFilter:getPatchBaselineGlobalFilterASet of global filters used to exclude patches from the baseline.
"E
id" ;The provider-assigned unique ID for this managed resource.
"1
json" %JSON representation of the baseline.
"9
name" -Name specified to identify the patch source.
"

namePrefixB" "
operatingSystemB" "
owner" "3
rejectedPatches*" List of rejected patches.
"j
rejectedPatchesAction" MAction specified to take on patches included in the `rejected_patches` list.
"�
sourcesZ*X:V
T
ssmgetPatchBaselineSource5aws:ssm/getPatchBaselineSource:getPatchBaselineSource~Information about the patches to use to update the managed nodes, including target operating systems and source repositories.
2�+
E
ssmgetPatchBaselines+aws:ssm/getPatchBaselines:getPatchBaselines�%Data source for retrieving AWS SSM (Systems Manager) Patch Baselines.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssm.getPatchBaselines({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.get_patch_baselines()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Ssm.GetPatchBaselines.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.GetPatchBaselines(ctx, &ssm.GetPatchBaselinesArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetPatchBaselinesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsmFunctions.getPatchBaselines();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssm:getPatchBaselines
      arguments: {}
```
<!--End PulumiCodeChooser -->

### With Filters

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssm.getPatchBaselines({
    filters: [
        {
            key: "OWNER",
            values: ["AWS"],
        },
        {
            key: "OPERATING_SYSTEM",
            values: ["WINDOWS"],
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssm.get_patch_baselines(filters=[
    {
        "key": "OWNER",
        "values": ["AWS"],
    },
    {
        "key": "OPERATING_SYSTEM",
        "values": ["WINDOWS"],
    },
])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Ssm.GetPatchBaselines.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.Ssm.Inputs.GetPatchBaselinesFilterInputArgs
            {
                Key = "OWNER",
                Values = new[]
                {
                    "AWS",
                },
            },
            new Aws.Ssm.Inputs.GetPatchBaselinesFilterInputArgs
            {
                Key = "OPERATING_SYSTEM",
                Values = new[]
                {
                    "WINDOWS",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssm.GetPatchBaselines(ctx, &ssm.GetPatchBaselinesArgs{
			Filters: []ssm.GetPatchBaselinesFilter{
				{
					Key: "OWNER",
					Values: []string{
						"AWS",
					},
				},
				{
					Key: "OPERATING_SYSTEM",
					Values: []string{
						"WINDOWS",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetPatchBaselinesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsmFunctions.getPatchBaselines(GetPatchBaselinesArgs.builder()
            .filters(            
                GetPatchBaselinesFilterArgs.builder()
                    .key("OWNER")
                    .values("AWS")
                    .build(),
                GetPatchBaselinesFilterArgs.builder()
                    .key("OPERATING_SYSTEM")
                    .values("WINDOWS")
                    .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssm:getPatchBaselines
      arguments:
        filters:
          - key: OWNER
            values:
              - AWS
          - key: OPERATING_SYSTEM
            values:
              - WINDOWS
```
<!--End PulumiCodeChooser -->
^
defaultBaselinesB
 DOnly return baseline identities where `default_baseline` is `true`.
�
filters_B]*[:Y
W
ssmgetPatchBaselinesFilter7aws:ssm/getPatchBaselinesFilter:getPatchBaselinesFilter@Key-value pairs used to filter the results. See `filter` below.
"�
baselineIdentities{*y:w
u
ssm!getPatchBaselinesBaselineIdentityKaws:ssm/getPatchBaselinesBaselineIdentity:getPatchBaselinesBaselineIdentity>List of baseline identities. See `baseline_identities` below.
"
defaultBaselinesB
 "j
filters_B]*[:Y
W
ssmgetPatchBaselinesFilter7aws:ssm/getPatchBaselinesFilter:getPatchBaselinesFilter"E
id" ;The provider-assigned unique ID for this managed resource.
2�
@
ssmcontacts
getContact%aws:ssmcontacts/getContact:getContact�Data source for managing an AWS SSM Contact.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssmcontacts.getContact({
    arn: "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssmcontacts.get_contact(arn="arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsmContacts.GetContact.Invoke(new()
    {
        Arn = "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmcontacts.LookupContact(ctx, &ssmcontacts.LookupContactArgs{
			Arn: "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.SsmcontactsFunctions;
import com.pulumi.aws.ssmcontacts.inputs.GetContactArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsmcontactsFunctions.getContact(GetContactArgs.builder()
            .arn("arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssmcontacts:getContact
      arguments:
        arn: arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias
```
<!--End PulumiCodeChooser -->
M
arn" BThe Amazon Resource Name (ARN) of the contact or escalation plan.
7
tagsB2" 'Map of tags to assign to the resource.
"P
alias" CA unique and identifiable alias of the contact or escalation plan.
"	
arn" "I
displayName" 6Full friendly name of the contact or escalation plan.
"E
id" ;The provider-assigned unique ID for this managed resource.
"5
tags2" 'Map of tags to assign to the resource.
"z
type" nThe type of contact engaged. A single contact is type `PERSONAL` and an escalation plan is type `ESCALATION`.
2�
U
ssmcontactsgetContactChannel3aws:ssmcontacts/getContactChannel:getContactChannel�Data source for managing an AWS SSM Contacts Contact Channel.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssmcontacts.getContactChannel({
    arn: "arn:aws:ssm-contacts:us-west-2:123456789012:contact-channel/example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssmcontacts.get_contact_channel(arn="arn:aws:ssm-contacts:us-west-2:123456789012:contact-channel/example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsmContacts.GetContactChannel.Invoke(new()
    {
        Arn = "arn:aws:ssm-contacts:us-west-2:123456789012:contact-channel/example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmcontacts.LookupContactChannel(ctx, &ssmcontacts.LookupContactChannelArgs{
			Arn: "arn:aws:ssm-contacts:us-west-2:123456789012:contact-channel/example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.SsmcontactsFunctions;
import com.pulumi.aws.ssmcontacts.inputs.GetContactChannelArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsmcontactsFunctions.getContactChannel(GetContactChannelArgs.builder()
            .arn("arn:aws:ssm-contacts:us-west-2:123456789012:contact-channel/example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssmcontacts:getContactChannel
      arguments:
        arn: arn:aws:ssm-contacts:us-west-2:123456789012:contact-channel/example
```
<!--End PulumiCodeChooser -->
>
arn" 3Amazon Resource Name (ARN) of the contact channel.
"B
activationStatus" *Whether the contact channel is activated.
"	
arn" "h
	contactId" WAmazon Resource Name (ARN) of the AWS SSM Contact that the contact channel belongs to.
"�
deliveryAddresses�*�:�
�
ssmcontacts getContactChannelDeliveryAddressQaws:ssmcontacts/getContactChannelDeliveryAddress:getContactChannelDeliveryAddress,Details used to engage the contact channel.
"E
id" ;The provider-assigned unique ID for this managed resource.
")
name" Name of the contact channel.
")
type" Type of the contact channel.
2�
7
ssmcontactsgetPlanaws:ssmcontacts/getPlan:getPlan�Data source for managing a Plan of an AWS SSM Contact.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.ssmcontacts.getPlan({
    contactId: "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.ssmcontacts.get_plan(contact_id="arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.SsmContacts.GetPlan.Invoke(new()
    {
        ContactId = "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmcontacts"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmcontacts.LookupPlan(ctx, &ssmcontacts.LookupPlanArgs{
			ContactId: "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmcontacts.SsmcontactsFunctions;
import com.pulumi.aws.ssmcontacts.inputs.GetPlanArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = SsmcontactsFunctions.getPlan(GetPlanArgs.builder()
            .contactId("arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:ssmcontacts:getPlan
      arguments:
        contactId: arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias
```
<!--End PulumiCodeChooser -->
S
	contactId" BThe Amazon Resource Name (ARN) of the contact or escalation plan.
"
	contactId" "E
id" ;The provider-assigned unique ID for this managed resource.
"�
stagesL*J:H
F
ssmcontactsgetPlanStage)aws:ssmcontacts/getPlanStage:getPlanStage�List of stages. A contact has an engagement plan with stages that contact specified contact channels. An escalation plan uses stages that contact specified contacts.
2�
W
ssmincidentsgetReplicationSet4aws:ssmincidents/getReplicationSet:getReplicationSet�> **NOTE:** The AWS Region specified by a provider must always be one of the Regions specified for the replication set.

Use this data source to manage a replication set in AWS Systems Manager Incident Manager.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssmincidents.getReplicationSet({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssmincidents.get_replication_set()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsmIncidents.GetReplicationSet.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssmincidents"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssmincidents.LookupReplicationSet(ctx, &ssmincidents.LookupReplicationSetArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssmincidents.SsmincidentsFunctions;
import com.pulumi.aws.ssmincidents.inputs.GetReplicationSetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsmincidentsFunctions.getReplicationSet();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssmincidents:getReplicationSet
      arguments: {}
```
<!--End PulumiCodeChooser -->
9
tagsB2" )All tags applied to the replication set.
"A
arn" 6The Amazon Resouce Name (ARN) of the replication set.
"F
	createdBy" 5The ARN of the user who created the replication set.
"g
deletionProtected
 NIf `true`, the last remaining Region in a replication set can’t be deleted.
"E
id" ;The provider-assigned unique ID for this managed resource.
"Q
lastModifiedBy" ;The ARN of the user who last modified the replication set.
"z
regionso*m:k
i
ssmincidentsgetReplicationSetRegion@aws:ssmincidents/getReplicationSetRegion:getReplicationSetRegion"{
status" mThe current status of the Region.
* Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
"7
tags2" )All tags applied to the replication set.
2�
Q
ssmincidentsgetResponsePlan0aws:ssmincidents/getResponsePlan:getResponsePlan�Use this data source to manage a response plan in AWS Systems Manager Incident Manager.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```yaml
resources:
  example:
    type: aws:ssmincidents:ResponsePlan
    properties:
      arn: exampleARN
```
<!--End PulumiCodeChooser -->
@
arn" 5The Amazon Resource Name (ARN) of the response plan.
7
tagsB2" 'The tags applied to the response plan.
"�
actionsi*g:e
c
ssmincidentsgetResponsePlanAction<aws:ssmincidents/getResponsePlanAction:getResponsePlanActionV(Optional) The actions that the response plan starts at the beginning of an incident.
"	
arn" "Z
chatChannels*" DThe Chatbot chat channel used for collaboration during an incident.
"]
displayName" JThe long format of the response plan name. This field can contain spaces.
"�
engagements*" xThe Amazon Resource Name (ARN) for the contacts and escalation plans that the response plan engages during an incident.
"E
id" ;The provider-assigned unique ID for this managed resource.
"�
incidentTemplates�*�:�
�
ssmincidentsgetResponsePlanIncidentTemplatePaws:ssmincidents/getResponsePlanIncidentTemplate:getResponsePlanIncidentTemplate"�
integrationsx*v:t
r
ssmincidentsgetResponsePlanIntegrationFaws:ssmincidents/getResponsePlanIntegration:getResponsePlanIntegrationnInformation about third-party services integrated into the response plan. The following values are supported:
"5
name" )The name of the PagerDuty configuration.
"5
tags2" 'The tags applied to the response plan.
:�
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter�
��
name" �Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter�
��
name" �Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter�
�p
name" dName of the filter field. Valid values can be found in the [describe-regions AWS CLI Reference][1].
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
�
sesv2'AccountVdmAttributesDashboardAttributesYaws:sesv2/AccountVdmAttributesDashboardAttributes:AccountVdmAttributesDashboardAttributes�
��
engagementMetricsB" eSpecifies the status of your VDM engagement metrics collection. Valid values: `ENABLED`, `DISABLED`.
:�
�
sesv2&AccountVdmAttributesGuardianAttributesWaws:sesv2/AccountVdmAttributesGuardianAttributes:AccountVdmAttributesGuardianAttributes�
��
optimizedSharedDeliveryB" aSpecifies the status of your VDM optimized shared delivery. Valid values: `ENABLED`, `DISABLED`.
:�
s
sesv2ConfigurationSetDeliveryOptionsIaws:sesv2/ConfigurationSetDeliveryOptions:ConfigurationSetDeliveryOptions�
��
maxDeliverySecondsB �The maximum amount of time, in seconds, that Amazon SES API v2 will attempt delivery of email. If specified, the value must greater than or equal to 300 seconds (5 minutes) and less than or equal to 50400 seconds (840 minutes).
d
sendingPoolNameB" KThe name of the dedicated IP pool to associate with the configuration set.
�
	tlsPolicyB" �Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS). Valid values: `REQUIRE`, `OPTIONAL`.
:�
�
sesv20ConfigurationSetEventDestinationEventDestinationkaws:sesv2/ConfigurationSetEventDestinationEventDestination:ConfigurationSetEventDestinationEventDestination�
��
cloudWatchDestination�B�:�
�
sesv2EConfigurationSetEventDestinationEventDestinationCloudWatchDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationCloudWatchDestination:ConfigurationSetEventDestinationEventDestinationCloudWatchDestination{An object that defines an Amazon CloudWatch destination for email events. See `cloud_watch_destination` Block for details.
�
enabledB
 qWhen the event destination is enabled, the specified event types are sent to the destinations. Default: `false`.
�
eventBridgeDestination�B�:�
�
sesv2FConfigurationSetEventDestinationEventDestinationEventBridgeDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationEventBridgeDestination:ConfigurationSetEventDestinationEventDestinationEventBridgeDestination�
kinesisFirehoseDestination�B�:�
�
sesv2JConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestination:ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestination�An object that defines an Amazon Kinesis Data Firehose destination for email events. See `kinesis_firehose_destination` Block for details.
�
matchingEventTypes*" �An array that specifies which events the Amazon SES API v2 should send to the destinations. Valid values: `SEND`, `REJECT`, `BOUNCE`, `COMPLAINT`, `DELIVERY`, `OPEN`, `CLICK`, `RENDERING_FAILURE`, `DELIVERY_DELAY`, `SUBSCRIPTION`.
�
pinpointDestination�B�:�
�
sesv2CConfigurationSetEventDestinationEventDestinationPinpointDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationPinpointDestination:ConfigurationSetEventDestinationEventDestinationPinpointDestination~An object that defines an Amazon Pinpoint project destination for email events. See `pinpoint_destination` Block for details.
�
snsDestination�B�:�
�
sesv2>ConfigurationSetEventDestinationEventDestinationSnsDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationSnsDestination:ConfigurationSetEventDestinationEventDestinationSnsDestinationlAn object that defines an Amazon SNS destination for email events. See `sns_destination` Block for details.
:�
�
sesv2EConfigurationSetEventDestinationEventDestinationCloudWatchDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationCloudWatchDestination:ConfigurationSetEventDestinationEventDestinationCloudWatchDestination�
��
dimensionConfigurations�*�:�
�
sesv2[ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration�aws:sesv2/ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration:ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration�An array of objects that define the dimensions to use when you send email events to Amazon CloudWatch. See `dimension_configuration` Block for details.
:�
�
sesv2[ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration�aws:sesv2/ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration:ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration�
��
defaultDimensionValue" �The default value of the dimension that is published to Amazon CloudWatch if you don't provide the value of the dimension when you send an email.
i
dimensionName" TThe name of an Amazon CloudWatch dimension associated with an email sending metric.
�
dimensionValueSource" �The location where the Amazon SES API v2 finds the value of a dimension to publish to Amazon CloudWatch. Valid values: `MESSAGE_TAG`, `EMAIL_HEADER`, `LINK_TAG`.
:�
�
sesv2FConfigurationSetEventDestinationEventDestinationEventBridgeDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationEventBridgeDestination:ConfigurationSetEventDestinationEventDestinationEventBridgeDestination�
��
eventBusArn" |The Amazon Resource Name (ARN) of the Amazon EventBridge bus to publish email events to. Only the default bus is supported.
:�
�
sesv2JConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestination:ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestination�
��
deliveryStreamArn" |The Amazon Resource Name (ARN) of the Amazon Kinesis Data Firehose stream that the Amazon SES API v2 sends email events to.
�

iamRoleArn" �The Amazon Resource Name (ARN) of the IAM role that the Amazon SES API v2 uses to send email events to the Amazon Kinesis Data Firehose stream.
:�
�
sesv2CConfigurationSetEventDestinationEventDestinationPinpointDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationPinpointDestination:ConfigurationSetEventDestinationEventDestinationPinpointDestination

applicationArn" :�
�
sesv2>ConfigurationSetEventDestinationEventDestinationSnsDestination�aws:sesv2/ConfigurationSetEventDestinationEventDestinationSnsDestination:ConfigurationSetEventDestinationEventDestinationSnsDestinationg
ec
topicArn" SThe Amazon Resource Name (ARN) of the Amazon SNS topic to publish email events to.
:�
y
sesv2!ConfigurationSetReputationOptionsMaws:sesv2/ConfigurationSetReputationOptions:ConfigurationSetReputationOptions�
��
lastFreshStartB" �The date and time (in Unix time) when the reputation metrics were last given a fresh start. When your account is given a fresh start, your reputation metrics are calculated starting from the date of the fresh start.
�
reputationMetricsEnabledB
 �If `true`, tracking of reputation metrics is enabled for the configuration set. If `false`, tracking of reputation metrics is disabled for the configuration set.
:�
p
sesv2ConfigurationSetSendingOptionsGaws:sesv2/ConfigurationSetSendingOptions:ConfigurationSetSendingOptions�
��
sendingEnabledB
 �If `true`, email sending is enabled for the configuration set. If `false`, email sending is disabled for the configuration set.
:�
|
sesv2"ConfigurationSetSuppressionOptionsOaws:sesv2/ConfigurationSetSuppressionOptions:ConfigurationSetSuppressionOptions�
��
suppressedReasonsB*" �A list that contains the reasons that email addresses are automatically added to the suppression list for your account. Valid values: `BOUNCE`, `COMPLAINT`.
:�
s
sesv2ConfigurationSetTrackingOptionsIaws:sesv2/ConfigurationSetTrackingOptions:ConfigurationSetTrackingOptions�
�R
customRedirectDomain" 6The domain to use for tracking open and click events.
�
httpsPolicyB" {The https policy to use for tracking open and click events. Valid values are `REQUIRE`, `REQUIRE_OPEN_ONLY` or `OPTIONAL`.
:�
d
sesv2ConfigurationSetVdmOptions?aws:sesv2/ConfigurationSetVdmOptions:ConfigurationSetVdmOptions�
��
dashboardOptions�B�:�
�
sesv2*ConfigurationSetVdmOptionsDashboardOptions_aws:sesv2/ConfigurationSetVdmOptionsDashboardOptions:ConfigurationSetVdmOptionsDashboardOptions�Specifies additional settings for your VDM configuration as applicable to the Dashboard. See `dashboard_options` Block for details.
�
guardianOptions�B�:�
�
sesv2)ConfigurationSetVdmOptionsGuardianOptions]aws:sesv2/ConfigurationSetVdmOptionsGuardianOptions:ConfigurationSetVdmOptionsGuardianOptions�Specifies additional settings for your VDM configuration as applicable to the Guardian. See `guardian_options` Block for details.
:�
�
sesv2*ConfigurationSetVdmOptionsDashboardOptions_aws:sesv2/ConfigurationSetVdmOptionsDashboardOptions:ConfigurationSetVdmOptionsDashboardOptions�
��
engagementMetricsB" eSpecifies the status of your VDM engagement metrics collection. Valid values: `ENABLED`, `DISABLED`.
:�
�
sesv2)ConfigurationSetVdmOptionsGuardianOptions]aws:sesv2/ConfigurationSetVdmOptionsGuardianOptions:ConfigurationSetVdmOptionsGuardianOptions�
��
optimizedSharedDeliveryB" aSpecifies the status of your VDM optimized shared delivery. Valid values: `ENABLED`, `DISABLED`.
:�
F
sesv2ContactListTopic+aws:sesv2/ContactListTopic:ContactListTopic�
��
defaultSubscriptionStatus" �Default subscription status to be applied to a contact if the contact has not noted their preference for subscribing to a topic.
Y
descriptionB" DDescription of what the topic is about, which the contact will see.
;
displayName" (Name of the topic the contact will see.
K
	topicName" :Name of the topic.

The following arguments are optional:
:�
|
sesv2"EmailIdentityDkimSigningAttributesOaws:sesv2/EmailIdentityDkimSigningAttributes:EmailIdentityDkimSigningAttributes�
�Y
currentSigningKeyLengthB" 8[Easy DKIM] The key length of the DKIM key pair in use.
�
domainSigningPrivateKeyB" �[Bring Your Own DKIM] A private key that's used to generate a DKIM signature. The private key must use 1024 or 2048-bit RSA encryption, and must be encoded using base64 encoding.

> **NOTE:** You have to delete the first and last lines ('-----BEGIN PRIVATE KEY-----' and '-----END PRIVATE KEY-----', respectively) of the generated private key. Additionally, you have to remove the line breaks in the generated private key. The resulting value is a string of characters with no spaces or line breaks.
�
domainSigningSelectorB" k[Bring Your Own DKIM] A string that's used to identify a public key in the DNS configuration for a domain.
j
lastKeyGenerationTimestampB" F[Easy DKIM] The last time a key pair was generated for this identity.
�
nextSigningKeyLengthB" �[Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day. Valid values: `RSA_1024_BIT`, `RSA_2048_BIT`.
�
signingAttributesOriginB" �A string that indicates how DKIM was configured for the identity. `AWS_SES` indicates that DKIM was configured for the identity by using Easy DKIM. `EXTERNAL` indicates that DKIM was configured for the identity by using Bring Your Own DKIM (BYODKIM).
�
statusB" �Describes whether or not Amazon SES has successfully located the DKIM records in the DNS records for the domain. See the [AWS SES API v2 Reference](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DkimAttributes.html#SES-Type-DkimAttributes-Status) for supported statuses.
�
tokensB*" �If you used Easy DKIM to configure DKIM authentication for the domain, then this object contains a set of unique strings that you use to create a set of CNAME records that you add to the DNS configuration for your domain. When Amazon SES detects these records in the DNS configuration for your domain, the DKIM authentication process is complete. If you configured DKIM authentication for the domain by providing your own public-private key pair, then this object contains the selector for the public key.
:�
y
sesv2!getConfigurationSetDeliveryOptionMaws:sesv2/getConfigurationSetDeliveryOption:getConfigurationSetDeliveryOption�
��
maxDeliverySeconds �The maximum amount of time, in seconds, that Amazon SES API v2 will attempt delivery of email. If specified, the value must greater than or equal to 300 seconds (5 minutes) and less than or equal to 50400 seconds (840 minutes).
b
sendingPoolName" KThe name of the dedicated IP pool to associate with the configuration set.

	tlsPolicy" nSpecifies whether messages that use the configuration set are required to use Transport Layer Security (TLS).
:�

sesv2#getConfigurationSetReputationOptionQaws:sesv2/getConfigurationSetReputationOption:getConfigurationSetReputationOption�
�r
lastFreshStart" \The date and time (in Unix time) when the reputation metrics were last given a fresh start.
]
reputationMetricsEnabled
 =Specifies whether tracking of reputation metrics is enabled.
:�
v
sesv2 getConfigurationSetSendingOptionKaws:sesv2/getConfigurationSetSendingOption:getConfigurationSetSendingOptionF
DB
sendingEnabled
 ,Specifies whether email sending is enabled.
:�
�
sesv2$getConfigurationSetSuppressionOptionSaws:sesv2/getConfigurationSetSuppressionOption:getConfigurationSetSuppressionOption�
��
suppressedReasons*" xA list that contains the reasons that email addresses are automatically added to the suppression list for your account.
:�
y
sesv2!getConfigurationSetTrackingOptionMaws:sesv2/getConfigurationSetTrackingOption:getConfigurationSetTrackingOption�
�R
customRedirectDomain" 6The domain to use for tracking open and click events.
�
httpsPolicy" {The https policy to use for tracking open and click events. Valid values are `REQUIRE`, `REQUIRE_OPEN_ONLY` or `OPTIONAL`.
:�
j
sesv2getConfigurationSetVdmOptionCaws:sesv2/getConfigurationSetVdmOption:getConfigurationSetVdmOption�
��
dashboardOptions�*�:�
�
sesv2+getConfigurationSetVdmOptionDashboardOptionaaws:sesv2/getConfigurationSetVdmOptionDashboardOption:getConfigurationSetVdmOptionDashboardOptionYSpecifies additional settings for your VDM configuration as applicable to the Dashboard.
�
guardianOptions�*�:�
�
sesv2*getConfigurationSetVdmOptionGuardianOption_aws:sesv2/getConfigurationSetVdmOptionGuardianOption:getConfigurationSetVdmOptionGuardianOptionXSpecifies additional settings for your VDM configuration as applicable to the Guardian.
:�
�
sesv2+getConfigurationSetVdmOptionDashboardOptionaaws:sesv2/getConfigurationSetVdmOptionDashboardOption:getConfigurationSetVdmOptionDashboardOption]
[Y
engagementMetrics" @Specifies the status of your VDM engagement metrics collection.
:�
�
sesv2*getConfigurationSetVdmOptionGuardianOption_aws:sesv2/getConfigurationSetVdmOptionGuardianOption:getConfigurationSetVdmOptionGuardianOption_
][
optimizedSharedDelivery" <Specifies the status of your VDM optimized shared delivery.
:�
m
sesv2getDedicatedIpPoolDedicatedIpEaws:sesv2/getDedicatedIpPoolDedicatedIp:getDedicatedIpPoolDedicatedIp�
�
ip" IPv4 address.
�
warmupPercentage �Indicates how complete the dedicated IP warm-up process is. When this value equals `1`, the address has completed the warm-up process and is ready for use.
g
warmupStatus" SThe warm-up status of a dedicated IP address. Valid values: `IN_PROGRESS`, `DONE`.
:�
�
sesv2$getEmailIdentityDkimSigningAttributeSaws:sesv2/getEmailIdentityDkimSigningAttribute:getEmailIdentityDkimSigningAttribute�
�W
currentSigningKeyLength" 8[Easy DKIM] The key length of the DKIM key pair in use.

domainSigningPrivateKey" 
domainSigningSelector" h
lastKeyGenerationTimestamp" F[Easy DKIM] The last time a key pair was generated for this identity.
�
nextSigningKeyLength" r[Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day.
�
signingAttributesOrigin" �A string that indicates how DKIM was configured for the identity. `AWS_SES` indicates that DKIM was configured for the identity by using Easy DKIM. `EXTERNAL` indicates that DKIM was configured for the identity by using Bring Your Own DKIM (BYODKIM).
�
status" �Describes whether or not Amazon SES has successfully located the DKIM records in the DNS records for the domain. See the [AWS SES API v2 Reference](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DkimAttributes.html#SES-Type-DkimAttributes-Status) for supported statuses.
�
tokens*" �If you used Easy DKIM to configure DKIM authentication for the domain, then this object contains a set of unique strings that you use to create a set of CNAME records that you add to the DNS configuration for your domain. When Amazon SES detects these records in the DNS configuration for your domain, the DKIM authentication process is complete. If you configured DKIM authentication for the domain by providing your own public-private key pair, then this object contains the selector for the public key.
:�
o
sfnActivityEncryptionConfigurationGaws:sfn/ActivityEncryptionConfiguration:ActivityEncryptionConfiguration�
��
kmsDataKeyReusePeriodSecondsB �Maximum duration for which Activities will reuse data keys. When the period expires, Activities will call GenerateDataKey. This setting only applies to customer managed KMS key and does not apply to AWS owned KMS key.
�
kmsKeyIdB" �The alias, alias ARN, key ID, or key ARN of the symmetric encryption KMS key that encrypts the data key. To specify a KMS key in a different AWS account, the customer must use the key ARN or alias ARN. For more information regarding kms_key_id, see [KeyId](https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters) in the KMS documentation.
x
typeB" jThe encryption option specified for the activity. Valid values: `AWS_KMS_KEY`, `CUSTOMER_MANAGED_KMS_KEY`
:�
]
sfnAliasRoutingConfiguration;aws:sfn/AliasRoutingConfiguration:AliasRoutingConfiguration�
�[
stateMachineVersionArn" =The Amazon Resource Name (ARN) of the state machine version.
I
weight ;Percentage of traffic routed to the state machine version.
:�
{
sfn#StateMachineEncryptionConfigurationOaws:sfn/StateMachineEncryptionConfiguration:StateMachineEncryptionConfiguration�
��
kmsDataKeyReusePeriodSecondsB �Maximum duration for which Step Functions will reuse data keys. When the period expires, Step Functions will call GenerateDataKey. This setting only applies to customer managed KMS key and does not apply when `type` is `AWS_OWNED_KEY`.
�
kmsKeyIdB" �The alias, alias ARN, key ID, or key ARN of the symmetric encryption KMS key that encrypts the data key. To specify a KMS key in a different AWS account, the customer must use the key ARN or alias ARN. For more information regarding kms_key_id, see [KeyId](https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters) in the KMS documentation.

typeB" qThe encryption option specified for the state machine. Valid values: `AWS_OWNED_KEY`, `CUSTOMER_MANAGED_KMS_KEY`
:�
r
sfn StateMachineLoggingConfigurationIaws:sfn/StateMachineLoggingConfiguration:StateMachineLoggingConfiguration�
��
includeExecutionDataB
 bDetermines whether execution data is included in your log. When set to `false`, data is excluded.
{
levelB" lDefines which category of execution history events are logged. Valid values: `ALL`, `ERROR`, `FATAL`, `OFF`
�
logDestinationB" �Amazon Resource Name (ARN) of a CloudWatch log group. Make sure the State Machine has the correct IAM policies for logging. The ARN must end with `:*`
:�
r
sfn StateMachineTracingConfigurationIaws:sfn/StateMachineTracingConfiguration:StateMachineTracingConfiguration�
��
enabledB
 �When set to `true`, AWS X-Ray tracing is enabled. Make sure the State Machine has the correct IAM policies for logging. See the [AWS Step Functions Developer Guide](https://docs.aws.amazon.com/step-functions/latest/dg/xray-iam.html) for details.
:�
f
sfngetAliasRoutingConfigurationAaws:sfn/getAliasRoutingConfiguration:getAliasRoutingConfiguration.
,
stateMachineVersionArn" 
weight :�
�
shield)ApplicationLayerAutomaticResponseTimeouts^aws:shield/ApplicationLayerAutomaticResponseTimeouts:ApplicationLayerAutomaticResponseTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
�
shield%DrtAccessLogBucketAssociationTimeoutsVaws:shield/DrtAccessLogBucketAssociationTimeouts:DrtAccessLogBucketAssociationTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
:�
�
shield#DrtAccessRoleArnAssociationTimeoutsRaws:shield/DrtAccessRoleArnAssociationTimeouts:DrtAccessRoleArnAssociationTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
�
shield#ProactiveEngagementEmergencyContactRaws:shield/ProactiveEngagementEmergencyContact:ProactiveEngagementEmergencyContact�
�>
contactNotesB" (Additional notes regarding the contact.
N
emailAddress" :A valid email address that will be used for this contact.
o
phoneNumberB" ZA phone number, starting with `+` and up to 15 digits that will be used for this contact.
:�
W
signerSigningJobDestination6aws:signer/SigningJobDestination:SigningJobDestination�
��
s3a:_
]
signerSigningJobDestinationS3:aws:signer/SigningJobDestinationS3:SigningJobDestinationS3bA configuration block describing the S3 Destination object: See S3 Destination below for details.
:�
]
signerSigningJobDestinationS3:aws:signer/SigningJobDestinationS3:SigningJobDestinationS3�
�
bucket" �
prefixB" qAn Amazon S3 object key prefix that you can use to limit signed objects keys to begin with the specified prefix.
:�
f
signerSigningJobRevocationRecord@aws:signer/SigningJobRevocationRecord:SigningJobRevocationRecord8
6
reasonB" 
	revokedAtB" 
	revokedByB" :�
Z
signerSigningJobSignedObject8aws:signer/SigningJobSignedObject:SigningJobSignedObjects
qo
s3shBf*d:b
`
signerSigningJobSignedObjectS3<aws:signer/SigningJobSignedObjectS3:SigningJobSignedObjectS3:�
`
signerSigningJobSignedObjectS3<aws:signer/SigningJobSignedObjectS3:SigningJobSignedObjectS3Z
X
bucketB" F
keyB" 9Key name of the object that contains your unsigned code.
:�
H
signerSigningJobSource,aws:signer/SigningJobSource:SigningJobSource�
��
s3R:P
N
signerSigningJobSourceS30aws:signer/SigningJobSourceS3:SigningJobSourceS3XA configuration block describing the S3 Source object: See S3 Source below for details.
:�
N
signerSigningJobSourceS30aws:signer/SigningJobSourceS3:SigningJobSourceS3�
�
bucket" D
key" 9Key name of the object that contains your unsigned code.
O
version" @Version of your source image in your version enabled S3 bucket.
:�
r
signerSigningProfileRevocationRecordHaws:signer/SigningProfileRevocationRecord:SigningProfileRevocationRecord�
�M
revocationEffectiveFromB" ,The time when revocation becomes effective.
B
	revokedAtB" /The time when the signing profile was revoked.
0
	revokedByB" The identity of the revoker.
:�
�
signer%SigningProfileSignatureValidityPeriodVaws:signer/SigningProfileSignatureValidityPeriod:SigningProfileSignatureValidityPeriod�
�[
type" OThe time unit for signature validity. Valid values: `DAYS`, `MONTHS`, `YEARS`.
J
value =The numerical value of the time unit for signature validity.
:�
o
signerSigningProfileSigningMaterialFaws:signer/SigningProfileSigningMaterial:SigningProfileSigningMaterialm
ki
certificateArn" SThe Amazon Resource Name (ARN) of the certificates that is used to sign your code.
:�
o
signergetSigningJobRevocationRecordFaws:signer/getSigningJobRevocationRecord:getSigningJobRevocationRecord2
0
reason" 
	revokedAt" 
	revokedBy" :�
c
signergetSigningJobSignedObject>aws:signer/getSigningJobSignedObject:getSigningJobSignedObjectz
xv
s3so*m:k
i
signergetSigningJobSignedObjectS3Baws:signer/getSigningJobSignedObjectS3:getSigningJobSignedObjectS3:�
i
signergetSigningJobSignedObjectS3Baws:signer/getSigningJobSignedObjectS3:getSigningJobSignedObjectS3

bucket" 	
key" :�
Q
signergetSigningJobSource2aws:signer/getSigningJobSource:getSigningJobSourceh
fd
s3s]*[:Y
W
signergetSigningJobSourceS36aws:signer/getSigningJobSourceS3:getSigningJobSourceS3:�
W
signergetSigningJobSourceS36aws:signer/getSigningJobSourceS3:getSigningJobSourceS3*
(
bucket" 	
key" 
version" :�
{
signer!getSigningProfileRevocationRecordNaws:signer/getSigningProfileRevocationRecord:getSigningProfileRevocationRecordC
A
revocationEffectiveFrom" 
	revokedAt" 
	revokedBy" :�
�
signer(getSigningProfileSignatureValidityPeriod\aws:signer/getSigningProfileSignatureValidityPeriod:getSigningProfileSignatureValidityPeriod


type" 
value :�
]
ssmAssociationOutputLocation;aws:ssm/AssociationOutputLocation:AssociationOutputLocation�
�(
s3BucketName" The S3 bucket name.
Y
s3KeyPrefixB" DThe S3 bucket prefix. Results stored in the root if not configured.
�
s3RegionB" nThe S3 bucket region.

Targets specify what instance IDs or tags to apply the document to and has these keys:
:�
E
ssmAssociationTarget+aws:ssm/AssociationTarget:AssociationTarget�
�I
key" >Either `InstanceIds` or `tag:Tag Name` to specify an EC2 tag.
h
values*" XA list of instance IDs or tag values. AWS currently limits this list size to one value.
:�

`
ssmContactsRotationRecurrence=aws:ssm/ContactsRotationRecurrence:ContactsRotationRecurrence�	
�	�
dailySettings�B�*�:�
�
ssm&ContactsRotationRecurrenceDailySettingUaws:ssm/ContactsRotationRecurrenceDailySetting:ContactsRotationRecurrenceDailySetting�
monthlySettings�B�*�:�
�
ssm(ContactsRotationRecurrenceMonthlySettingYaws:ssm/ContactsRotationRecurrenceMonthlySetting:ContactsRotationRecurrenceMonthlySettingj(Optional) Information about on-call rotations that recur monthly. See Monthly Settings for more details.
�
numberOfOnCalls o(Required) The number of contacts, or shift team members designated to be on call concurrently during a shift.
e
recurrenceMultiplier I(Required) The number of days, weeks, or months a single rotation lasts.
�
shiftCoverages�B�*�:�
�
ssm'ContactsRotationRecurrenceShiftCoverageWaws:ssm/ContactsRotationRecurrenceShiftCoverage:ContactsRotationRecurrenceShiftCoverage�(Optional) Information about the days of the week that the on-call rotation coverage includes. See Shift Coverages for more details.
�
weeklySettings�B�*�:�
�
ssm'ContactsRotationRecurrenceWeeklySettingWaws:ssm/ContactsRotationRecurrenceWeeklySetting:ContactsRotationRecurrenceWeeklySettingh(Optional) Information about on-call rotations that recur weekly. See Weekly Settings for more details.
:�
�
ssm&ContactsRotationRecurrenceDailySettingUaws:ssm/ContactsRotationRecurrenceDailySetting:ContactsRotationRecurrenceDailySettingo
m1
	hourOfDay  (Required) The hour of the day.
8
minuteOfHour $(Required) The minutes of the hour.
:�
�
ssm(ContactsRotationRecurrenceMonthlySettingYaws:ssm/ContactsRotationRecurrenceMonthlySetting:ContactsRotationRecurrenceMonthlySetting�
�b

dayOfMonth P(Required) The day of the month when monthly recurring on-call rotations begin.
�
handOffTime�B�:�
�
ssm3ContactsRotationRecurrenceMonthlySettingHandOffTimeoaws:ssm/ContactsRotationRecurrenceMonthlySettingHandOffTime:ContactsRotationRecurrenceMonthlySettingHandOffTimeB(Required) The hand off time. See Hand Off Time for more details.
:�
�
ssm3ContactsRotationRecurrenceMonthlySettingHandOffTimeoaws:ssm/ContactsRotationRecurrenceMonthlySettingHandOffTime:ContactsRotationRecurrenceMonthlySettingHandOffTimeo
m1
	hourOfDay  (Required) The hour of the day.
8
minuteOfHour $(Required) The minutes of the hour.
:�
�
ssm'ContactsRotationRecurrenceShiftCoverageWaws:ssm/ContactsRotationRecurrenceShiftCoverage:ContactsRotationRecurrenceShiftCoverage�
��
coverageTimes�B�*�:�
�
ssm3ContactsRotationRecurrenceShiftCoverageCoverageTimeoaws:ssm/ContactsRotationRecurrenceShiftCoverageCoverageTime:ContactsRotationRecurrenceShiftCoverageCoverageTimei(Required) Information about when an on-call shift begins and ends. See Coverage Times for more details.

mapBlockKey" :�
�
ssm3ContactsRotationRecurrenceShiftCoverageCoverageTimeoaws:ssm/ContactsRotationRecurrenceShiftCoverageCoverageTime:ContactsRotationRecurrenceShiftCoverageCoverageTime�
��
end�B�:�
�
ssm6ContactsRotationRecurrenceShiftCoverageCoverageTimeEnduaws:ssm/ContactsRotationRecurrenceShiftCoverageCoverageTimeEnd:ContactsRotationRecurrenceShiftCoverageCoverageTimeEndR(Required) The end time of the on-call shift. See Hand Off Time for more details.
�
start�B�:�
�
ssm8ContactsRotationRecurrenceShiftCoverageCoverageTimeStartyaws:ssm/ContactsRotationRecurrenceShiftCoverageCoverageTimeStart:ContactsRotationRecurrenceShiftCoverageCoverageTimeStartT(Required) The start time of the on-call shift. See Hand Off Time for more details.
:�
�
ssm6ContactsRotationRecurrenceShiftCoverageCoverageTimeEnduaws:ssm/ContactsRotationRecurrenceShiftCoverageCoverageTimeEnd:ContactsRotationRecurrenceShiftCoverageCoverageTimeEndo
m1
	hourOfDay  (Required) The hour of the day.
8
minuteOfHour $(Required) The minutes of the hour.
:�
�
ssm8ContactsRotationRecurrenceShiftCoverageCoverageTimeStartyaws:ssm/ContactsRotationRecurrenceShiftCoverageCoverageTimeStart:ContactsRotationRecurrenceShiftCoverageCoverageTimeStarto
m1
	hourOfDay  (Required) The hour of the day.
8
minuteOfHour $(Required) The minutes of the hour.
:�
�
ssm'ContactsRotationRecurrenceWeeklySettingWaws:ssm/ContactsRotationRecurrenceWeeklySetting:ContactsRotationRecurrenceWeeklySetting�
�P
	dayOfWeek" ?(Required) The day of the week when the shift coverage occurs.
�
handOffTime�B�:�
�
ssm2ContactsRotationRecurrenceWeeklySettingHandOffTimemaws:ssm/ContactsRotationRecurrenceWeeklySettingHandOffTime:ContactsRotationRecurrenceWeeklySettingHandOffTimeB(Required) The hand off time. See Hand Off Time for more details.
:�
�
ssm2ContactsRotationRecurrenceWeeklySettingHandOffTimemaws:ssm/ContactsRotationRecurrenceWeeklySettingHandOffTime:ContactsRotationRecurrenceWeeklySettingHandOffTimeo
m1
	hourOfDay  (Required) The hour of the day.
8
minuteOfHour $(Required) The minutes of the hour.
:�
]
ssmDocumentAttachmentsSource;aws:ssm/DocumentAttachmentsSource:DocumentAttachmentsSource�
��
key" �The key of a key-value pair that identifies the location of an attachment to the document. Valid values: `SourceUrl`, `S3FileUrl`, `AttachmentReference`.
8
nameB" *The name of the document attachment file.
�
values*" �The value of a key-value pair that identifies the location of an attachment to the document. The argument format is a list of a single string that depends on the type of key you specify - see the [API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_AttachmentsSource.html) for details.
:�
E
ssmDocumentParameter+aws:ssm/DocumentParameter:DocumentParameter�
��
defaultValueB" �If specified, the default values for the parameters. Parameters without a default value are required. Parameters with a default value are optional.
�
descriptionB" zA description of what the parameter does, how to use it, the default value, and whether or not the parameter is optional.
(
nameB" The name of the document.
K
typeB" =The type of parameter. Valid values: `String`, `StringList`.
:�
i
ssmMaintenanceWindowTargetTargetCaws:ssm/MaintenanceWindowTargetTarget:MaintenanceWindowTargetTarget
	
key" 
values*" :�
c
ssmMaintenanceWindowTaskTarget?aws:ssm/MaintenanceWindowTaskTarget:MaintenanceWindowTaskTarget5
3	
key" &
values*" The array of strings.
:�
�
ssm-MaintenanceWindowTaskTaskInvocationParameterscaws:ssm/MaintenanceWindowTaskTaskInvocationParameters:MaintenanceWindowTaskTaskInvocationParameters�	
�	�
automationParameters�B�:�
�
ssmAMaintenanceWindowTaskTaskInvocationParametersAutomationParameters�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersAutomationParameters:MaintenanceWindowTaskTaskInvocationParametersAutomationParameters>The parameters for an AUTOMATION task type. Documented below.
�
lambdaParameters�B�:�
�
ssm=MaintenanceWindowTaskTaskInvocationParametersLambdaParameters�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersLambdaParameters:MaintenanceWindowTaskTaskInvocationParametersLambdaParameters9The parameters for a LAMBDA task type. Documented below.
�
runCommandParameters�B�:�
�
ssmAMaintenanceWindowTaskTaskInvocationParametersRunCommandParameters�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters:MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters>The parameters for a RUN_COMMAND task type. Documented below.
�
stepFunctionsParameters�B�:�
�
ssmDMaintenanceWindowTaskTaskInvocationParametersStepFunctionsParameters�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParameters:MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParametersAThe parameters for a STEP_FUNCTIONS task type. Documented below.
:�
�
ssmAMaintenanceWindowTaskTaskInvocationParametersAutomationParameters�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersAutomationParameters:MaintenanceWindowTaskTaskInvocationParametersAutomationParameters�
�]
documentVersionB" DThe version of an Automation document to use during task execution.
�

parameters�B�*�:�
�
ssmJMaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameter�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameter:MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameterEThe parameters for the RUN_COMMAND task execution. Documented below.
:�
�
ssmJMaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameter�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameter:MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameterL
J 
name" The parameter name.
&
values*" The array of strings.
:�
�
ssm=MaintenanceWindowTaskTaskInvocationParametersLambdaParameters�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersLambdaParameters:MaintenanceWindowTaskTaskInvocationParametersLambdaParameters�
�f
clientContextB" OPass client-specific information to the Lambda function that you are invoking.
C
payloadB" 2JSON to provide to your Lambda function as input.
D
	qualifierB" 1Specify a Lambda function version or alias name.
:�
�
ssmAMaintenanceWindowTaskTaskInvocationParametersRunCommandParameters�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters:MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters�
��
cloudwatchConfig�B�:�
�
ssmQMaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfig�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfig:MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfigWConfiguration options for sending command output to CloudWatch Logs. Documented below.
>
commentB" -Information about the command(s) to execute.
�
documentHashB" rThe SHA-256 or SHA-1 hash created by the system when the document was created. SHA-1 hashes have been deprecated.
q
documentHashTypeB" WSHA-256 or SHA-1. SHA-1 hashes have been deprecated. Valid values: `Sha256` and `Sha1`
]
documentVersionB" DThe version of an Automation document to use during task execution.
�
notificationConfig�B�:�
�
ssmSMaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfig�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfig:MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfigqConfigurations for sending notifications about command status changes on a per-instance basis. Documented below.
:
outputS3BucketB" "The name of the Amazon S3 bucket.
;
outputS3KeyPrefixB"  The Amazon S3 bucket subfolder.
�

parameters�B�*�:�
�
ssmJMaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameter�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameter:MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameterEThe parameters for the RUN_COMMAND task execution. Documented below.
�
serviceRoleArnB" �The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.
s
timeoutSecondsB [If this time is reached and the command has not already started executing, it doesn't run.
:�
�
ssmQMaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfig�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfig:MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfig�
��
cloudwatchLogGroupNameB" �The name of the CloudWatch log group where you want to send command output. If you don't specify a group name, Systems Manager automatically creates a log group for you. The log group uses the following naming format: aws/ssm/SystemsManagerDocumentName.
d
cloudwatchOutputEnabledB
 CEnables Systems Manager to send command output to CloudWatch Logs.
:�
�
ssmSMaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfig�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfig:MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfig�
��
notificationArnB" �An Amazon Resource Name (ARN) for a Simple Notification Service (SNS) topic. Run Command pushes notifications about command status changes to this topic.
�
notificationEventsB*" �The different events for which you can receive notifications. Valid values: `All`, `InProgress`, `Success`, `TimedOut`, `Cancelled`, and `Failed`
�
notificationTypeB" �When specified with `Command`, receive notification when the status of a command changes. When specified with `Invocation`, for commands sent to multiple instances, receive notification on a per-instance basis when the status of a command changes. Valid values: `Command` and `Invocation`
:�
�
ssmJMaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameter�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameter:MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameterL
J 
name" The parameter name.
&
values*" The array of strings.
:�
�
ssmDMaintenanceWindowTaskTaskInvocationParametersStepFunctionsParameters�aws:ssm/MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParameters:MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParametersn
l6
inputB" 'The inputs for the STEP_FUNCTION task.
2
nameB" $The name of the STEP_FUNCTION task.
:�
]
ssmPatchBaselineApprovalRule;aws:ssm/PatchBaselineApprovalRule:PatchBaselineApprovalRule�
��
approveAfterDaysB �Number of days after the release date of each patch matched by the rule the patch is marked as approved in the patch baseline. Valid Range: 0 to 360. Conflicts with `approve_until_date`.
�
approveUntilDateB" �Cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Date is formatted as `YYYY-MM-DD`. Conflicts with `approve_after_days`
�
complianceLevelB" �Compliance level for patches approved by this rule. Valid values are `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`, `INFORMATIONAL`, and `UNSPECIFIED`. The default value is `UNSPECIFIED`.
�
enableNonSecurityB
 xBoolean enabling the application of non-security updates. The default value is `false`. Valid for Linux instances only.
�
patchFilters�*�:�
~
ssm$PatchBaselineApprovalRulePatchFilterQaws:ssm/PatchBaselineApprovalRulePatchFilter:PatchBaselineApprovalRulePatchFilter�Patch filter group that defines the criteria for the rule. Up to 5 patch filters can be specified per approval rule using Key/Value pairs. Valid combinations of these Keys and the `operating_system` value can be found in the [SSM DescribePatchProperties API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribePatchProperties.html). Valid Values are exact values for the patch property given as the key, or a wildcard `*`, which matches all values. `PATCH_SET` defaults to `OS` if unspecified
:�
~
ssm$PatchBaselineApprovalRulePatchFilterQaws:ssm/PatchBaselineApprovalRulePatchFilter:PatchBaselineApprovalRulePatchFilter
	
key" 
values*" :~
]
ssmPatchBaselineGlobalFilter;aws:ssm/PatchBaselineGlobalFilter:PatchBaselineGlobalFilter
	
key" 
values*" :�
K
ssmPatchBaselineSource/aws:ssm/PatchBaselineSource:PatchBaselineSource�
��
configuration" �Value of the yum repo configuration. For information about other options available for your yum repository configuration, see the [`dnf.conf` documentation](https://man7.org/linux/man-pages/man5/dnf.conf.5.html)
9
name" -Name specified to identify the patch source.
�
products*" �Specific operating system versions a patch repository applies to, such as `"Ubuntu16.04"`, `"AmazonLinux2016.09"`, `"RedhatEnterpriseLinux7.2"` or `"Suse12.7"`. For lists of supported product values, see [PatchFilter](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PatchFilter.html).
:�
�
ssm5QuicksetupConfigurationManagerConfigurationDefinitionsaws:ssm/QuicksetupConfigurationManagerConfigurationDefinition:QuicksetupConfigurationManagerConfigurationDefinition�
�

idB" ,
$localDeploymentAdministrationRoleArnB" d
 localDeploymentExecutionRoleNameB" :Name of the IAM role used to deploy local configurations.
�

parameters2" �Parameters for the configuration definition type. Parameters for configuration definitions vary based the configuration type. See the [AWS API documentation](https://docs.aws.amazon.com/quick-setup/latest/APIReference/API_ConfigurationDefinitionInput.html) for a complete list of parameters for each configuration type.
3
type" 'Type of the Quick Setup configuration.
=
typeVersionB" (Version of the Quick Setup type to use.
:�
�
ssm+QuicksetupConfigurationManagerStatusSummary_aws:ssm/QuicksetupConfigurationManagerStatusSummary:QuicksetupConfigurationManagerStatusSummary�
�
status" Current status.
�
statusMessage" When applicable, returns an informational message relevant to the current status and status type of the status summary object.
,

statusType" Type of a status summary.
:�
�
ssm&QuicksetupConfigurationManagerTimeoutsUaws:ssm/QuicksetupConfigurationManagerTimeouts:QuicksetupConfigurationManagerTimeouts�
��
createB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
�
deleteB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
�
updateB" �A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:�
i
ssmResourceDataSyncS3DestinationCaws:ssm/ResourceDataSyncS3Destination:ResourceDataSyncS3Destination�
�I

bucketName" 7Name of S3 bucket where the aggregated data is stored.
L
	kmsKeyArnB" 9ARN of an encryption key for a destination in Amazon S3.
'
prefixB" Prefix for the bucket.
I
region" ;Region with the bucket targeted by the Resource Data Sync.
k

syncFormatB" WA supported sync format. Only JsonSerDe is currently supported. Defaults to JsonSerDe.
:�
i
ssmgetContactsRotationRecurrenceCaws:ssm/getContactsRotationRecurrence:getContactsRotationRecurrence�
��
dailySettings�*�:�
�
ssm)getContactsRotationRecurrenceDailySetting[aws:ssm/getContactsRotationRecurrenceDailySetting:getContactsRotationRecurrenceDailySetting�
monthlySettings�*�:�
�
ssm+getContactsRotationRecurrenceMonthlySetting_aws:ssm/getContactsRotationRecurrenceMonthlySetting:getContactsRotationRecurrenceMonthlySetting
numberOfOnCalls 
recurrenceMultiplier �
shiftCoverages�*�:�
�
ssm*getContactsRotationRecurrenceShiftCoverage]aws:ssm/getContactsRotationRecurrenceShiftCoverage:getContactsRotationRecurrenceShiftCoverage�
weeklySettings�*�:�
�
ssm*getContactsRotationRecurrenceWeeklySetting]aws:ssm/getContactsRotationRecurrenceWeeklySetting:getContactsRotationRecurrenceWeeklySetting:�
�
ssm)getContactsRotationRecurrenceDailySetting[aws:ssm/getContactsRotationRecurrenceDailySetting:getContactsRotationRecurrenceDailySetting'
%
	hourOfDay 
minuteOfHour :�
�
ssm+getContactsRotationRecurrenceMonthlySetting_aws:ssm/getContactsRotationRecurrenceMonthlySetting:getContactsRotationRecurrenceMonthlySetting�
�

dayOfMonth �
handOffTimes�*�:�
�
ssm6getContactsRotationRecurrenceMonthlySettingHandOffTimeuaws:ssm/getContactsRotationRecurrenceMonthlySettingHandOffTime:getContactsRotationRecurrenceMonthlySettingHandOffTime:�
�
ssm6getContactsRotationRecurrenceMonthlySettingHandOffTimeuaws:ssm/getContactsRotationRecurrenceMonthlySettingHandOffTime:getContactsRotationRecurrenceMonthlySettingHandOffTime'
%
	hourOfDay 
minuteOfHour :�
�
ssm*getContactsRotationRecurrenceShiftCoverage]aws:ssm/getContactsRotationRecurrenceShiftCoverage:getContactsRotationRecurrenceShiftCoverage�
��
coverageTimes�*�:�
�
ssm6getContactsRotationRecurrenceShiftCoverageCoverageTimeuaws:ssm/getContactsRotationRecurrenceShiftCoverageCoverageTime:getContactsRotationRecurrenceShiftCoverageCoverageTime
mapBlockKey" :�
�
ssm6getContactsRotationRecurrenceShiftCoverageCoverageTimeuaws:ssm/getContactsRotationRecurrenceShiftCoverageCoverageTime:getContactsRotationRecurrenceShiftCoverageCoverageTime�
��
ends�*�:�
�
ssm9getContactsRotationRecurrenceShiftCoverageCoverageTimeEnd{aws:ssm/getContactsRotationRecurrenceShiftCoverageCoverageTimeEnd:getContactsRotationRecurrenceShiftCoverageCoverageTimeEnd�
starts�*�:�
�
ssm;getContactsRotationRecurrenceShiftCoverageCoverageTimeStartaws:ssm/getContactsRotationRecurrenceShiftCoverageCoverageTimeStart:getContactsRotationRecurrenceShiftCoverageCoverageTimeStart:�
�
ssm9getContactsRotationRecurrenceShiftCoverageCoverageTimeEnd{aws:ssm/getContactsRotationRecurrenceShiftCoverageCoverageTimeEnd:getContactsRotationRecurrenceShiftCoverageCoverageTimeEnd'
%
	hourOfDay 
minuteOfHour :�
�
ssm;getContactsRotationRecurrenceShiftCoverageCoverageTimeStartaws:ssm/getContactsRotationRecurrenceShiftCoverageCoverageTimeStart:getContactsRotationRecurrenceShiftCoverageCoverageTimeStart'
%
	hourOfDay 
minuteOfHour :�
�
ssm*getContactsRotationRecurrenceWeeklySetting]aws:ssm/getContactsRotationRecurrenceWeeklySetting:getContactsRotationRecurrenceWeeklySetting�
�
	dayOfWeek" �
handOffTimes�*�:�
�
ssm5getContactsRotationRecurrenceWeeklySettingHandOffTimesaws:ssm/getContactsRotationRecurrenceWeeklySettingHandOffTime:getContactsRotationRecurrenceWeeklySettingHandOffTime:�
�
ssm5getContactsRotationRecurrenceWeeklySettingHandOffTimesaws:ssm/getContactsRotationRecurrenceWeeklySettingHandOffTime:getContactsRotationRecurrenceWeeklySettingHandOffTime'
%
	hourOfDay 
minuteOfHour :�
H
ssmgetInstancesFilter-aws:ssm/getInstancesFilter:getInstancesFilter�
��
name" �Name of the filter field. Valid values can be found in the [SSM InstanceInformationStringFilter API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_InstanceInformationStringFilter.html).
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
c
ssmgetMaintenanceWindowsFilter?aws:ssm/getMaintenanceWindowsFilter:getMaintenanceWindowsFilter�
��
name" �Name of the filter field. Valid values can be found in the [SSM DescribeMaintenanceWindows API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribeMaintenanceWindows.html#API_DescribeMaintenanceWindows_RequestSyntax).
�
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:�
f
ssmgetPatchBaselineApprovalRuleAaws:ssm/getPatchBaselineApprovalRule:getPatchBaselineApprovalRule�
��
approveAfterDays Number of days after the release date of each patch matched by the rule the patch is marked as approved in the patch baseline.
�
approveUntilDate" �Cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Date is formatted as `YYYY-MM-DD`. Conflicts with `approve_after_days`
K
complianceLevel" 4Compliance level for patches approved by this rule.
S
enableNonSecurity
 :Boolean enabling the application of non-security updates.
�
patchFilters�*�:�
�
ssm'getPatchBaselineApprovalRulePatchFilterWaws:ssm/getPatchBaselineApprovalRulePatchFilter:getPatchBaselineApprovalRulePatchFilter;Patch filter group that defines the criteria for the rule.
:�
�
ssm'getPatchBaselineApprovalRulePatchFilterWaws:ssm/getPatchBaselineApprovalRulePatchFilter:getPatchBaselineApprovalRulePatchFilterK
I
key" Key for the filter.
&
values*" Value for the filter.
:�
f
ssmgetPatchBaselineGlobalFilterAaws:ssm/getPatchBaselineGlobalFilter:getPatchBaselineGlobalFilterK
I
key" Key for the filter.
&
values*" Value for the filter.
:�
T
ssmgetPatchBaselineSource5aws:ssm/getPatchBaselineSource:getPatchBaselineSource�
�:
configuration" %Value of the yum repo configuration.
9
name" -Name specified to identify the patch source.
T
products*" BSpecific operating system versions a patch repository applies to.
:�
u
ssm!getPatchBaselinesBaselineIdentityKaws:ssm/getPatchBaselinesBaselineIdentity:getPatchBaselinesBaselineIdentity�
�>
baselineDescription" #Description of the patch baseline.
,

baselineId" ID of the patch baseline.
0
baselineName" Name of the patch baseline.
�
defaultBaseline
 �Indicates whether this is the default baseline. AWS Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system.
G
operatingSystem" 0Operating system the patch baseline applies to.
:�
W
ssmgetPatchBaselinesFilter7aws:ssm/getPatchBaselinesFilter:getPatchBaselinesFilter�
��
key" �Filter key. See the [AWS SSM documentation](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribePatchBaselines.html) for valid values.
�
values*" �Filter values. See the [AWS SSM documentation](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribePatchBaselines.html) for example values.
:�
y
ssmcontactsContactChannelDeliveryAddressKaws:ssmcontacts/ContactChannelDeliveryAddress:ContactChannelDeliveryAddress�
��
simpleAddress" �Details to engage this contact channel. The expected format depends on the contact channel type and is described in the [`ContactChannelAddress` section of the SSM Contacts API Reference](https://docs.aws.amazon.com/incident-manager/latest/APIReference/API_SSMContacts_ContactChannelAddress.html).
:�
=
ssmcontacts	PlanStage#aws:ssmcontacts/PlanStage:PlanStage�
��
durationInMinutes mThe time to wait until beginning the next stage. The duration can only be set to 0 if a target is specified.
�
targetsWBU*S:Q
O
ssmcontactsPlanStageTarget/aws:ssmcontacts/PlanStageTarget:PlanStageTarget�One or more configuration blocks for specifying the contacts or contact methods that the escalation plan or engagement plan is engaging. See Target below for more details.
:�
O
ssmcontactsPlanStageTarget/aws:ssmcontacts/PlanStageTarget:PlanStageTarget�
��
channelTargetInfo�B�:�
�
ssmcontacts PlanStageTargetChannelTargetInfoQaws:ssmcontacts/PlanStageTargetChannelTargetInfo:PlanStageTargetChannelTargetInfo�A configuration block for specifying information about the contact channel that Incident Manager engages. See Channel Target Info for more details.
�
contactTargetInfo�B�:�
�
ssmcontacts PlanStageTargetContactTargetInfoQaws:ssmcontacts/PlanStageTargetContactTargetInfo:PlanStageTargetContactTargetInfo�A configuration block for specifying information about the contact that Incident Manager engages. See Contact Target Info for more details.
:�
�
ssmcontacts PlanStageTargetChannelTargetInfoQaws:ssmcontacts/PlanStageTargetChannelTargetInfo:PlanStageTargetChannelTargetInfo�
�O
contactChannelId" 7The Amazon Resource Name (ARN) of the contact channel.
�
retryIntervalInMinutesB eThe number of minutes to wait before retrying to send engagement if the engagement initially failed.
:�
�
ssmcontacts PlanStageTargetContactTargetInfoQaws:ssmcontacts/PlanStageTargetContactTargetInfo:PlanStageTargetContactTargetInfo�
�B
	contactIdB" /The Amazon Resource Name (ARN) of the contact.
z
isEssential
 gA Boolean value determining if the contact's acknowledgement stops the progress of stages in the plan.
:�
�
ssmcontacts getContactChannelDeliveryAddressQaws:ssmcontacts/getContactChannelDeliveryAddress:getContactChannelDeliveryAddress

simpleAddress" :�
F
ssmcontactsgetPlanStage)aws:ssmcontacts/getPlanStage:getPlanStage�
�
durationInMinutes i
targets^*\:Z
X
ssmcontactsgetPlanStageTarget5aws:ssmcontacts/getPlanStageTarget:getPlanStageTarget:�
X
ssmcontactsgetPlanStageTarget5aws:ssmcontacts/getPlanStageTarget:getPlanStageTarget�
��
channelTargetInfos�*�:�
�
ssmcontacts#getPlanStageTargetChannelTargetInfoWaws:ssmcontacts/getPlanStageTargetChannelTargetInfo:getPlanStageTargetChannelTargetInfo�
contactTargetInfos�*�:�
�
ssmcontacts#getPlanStageTargetContactTargetInfoWaws:ssmcontacts/getPlanStageTargetContactTargetInfo:getPlanStageTargetContactTargetInfo:�
�
ssmcontacts#getPlanStageTargetChannelTargetInfoWaws:ssmcontacts/getPlanStageTargetChannelTargetInfo:getPlanStageTargetChannelTargetInfo8
6
contactChannelId" 
retryIntervalInMinutes :�
�
ssmcontacts#getPlanStageTargetContactTargetInfoWaws:ssmcontacts/getPlanStageTargetContactTargetInfo:getPlanStageTargetContactTargetInfoj
hS
	contactId" BThe Amazon Resource Name (ARN) of the contact or escalation plan.

isEssential
 :�
`
ssmincidentsReplicationSetRegion:aws:ssmincidents/ReplicationSetRegion:ReplicationSetRegion�
��
	kmsKeyArnB" �The Amazon Resource name (ARN) of the customer managed key. If omitted, AWS manages the AWS KMS keys for you, using an AWS owned key, as indicated by a default value of `DefaultKey`.

The following arguments are optional:
>
name" 2The name of the Region, such as `ap-southeast-2`.
}
statusB" mThe current status of the Region.
* Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
F
statusMessageB" /More information about the status of a Region.
:�
Z
ssmincidentsResponsePlanAction6aws:ssmincidents/ResponsePlanAction:ResponsePlanAction�
��
ssmAutomations�B�*�:�
�
ssmincidentsResponsePlanActionSsmAutomationPaws:ssmincidents/ResponsePlanActionSsmAutomation:ResponsePlanActionSsmAutomation�The Systems Manager automation document to start as the runbook at the beginning of the incident. The following values are supported:
:�
�
ssmincidentsResponsePlanActionSsmAutomationPaws:ssmincidents/ResponsePlanActionSsmAutomation:ResponsePlanActionSsmAutomation�
�4
documentName"  The automation document's name.
S
documentVersionB" :The version of the automation document to use at runtime.
�
dynamicParametersB2" mThe key-value pair to resolve dynamic parameter values when processing a Systems Manager Automation runbook.
�

parameters�B�*�:�
�
ssmincidents(ResponsePlanActionSsmAutomationParameterbaws:ssmincidents/ResponsePlanActionSsmAutomationParameter:ResponsePlanActionSsmAutomationParameterlThe key-value pair parameters to use when the automation document runs. The following values are supported:
v
roleArn" gThe Amazon Resource Name (ARN) of the role that the automation document assumes when it runs commands.
�
targetAccountB" zThe account that the automation document runs in. This can be in either the management account or an application account.
:�
�
ssmincidents(ResponsePlanActionSsmAutomationParameterbaws:ssmincidents/ResponsePlanActionSsmAutomationParameter:ResponsePlanActionSsmAutomationParametero
m+
name" The name of the response plan.
>
values*" .The values for the associated parameter name.
:�
x
ssmincidentsResponsePlanIncidentTemplateJaws:ssmincidents/ResponsePlanIncidentTemplate:ResponsePlanIncidentTemplate�
�|
dedupeStringB" fA string used to stop Incident Manager from creating multiple incident records for the same incident.
\
impact NThe impact value of a generated incident. The following values are supported:
�
incidentTagsB2" �The tags assigned to an incident template. When an incident starts, Incident Manager assigns the tags specified in the template to the incident.
�
notificationTargets�B�*�:�
�
ssmincidents.ResponsePlanIncidentTemplateNotificationTargetnaws:ssmincidents/ResponsePlanIncidentTemplateNotificationTarget:ResponsePlanIncidentTemplateNotificationTarget�The Amazon Simple Notification Service (Amazon SNS) targets that this incident notifies when it is updated. The `notification_target` configuration block supports the following argument:
-
summaryB" The summary of an incident.
0
title" #The title of a generated incident.
:�
�
ssmincidents.ResponsePlanIncidentTemplateNotificationTargetnaws:ssmincidents/ResponsePlanIncidentTemplateNotificationTarget:ResponsePlanIncidentTemplateNotificationTarget_
][
snsTopicArn" HThe ARN of the Amazon SNS topic.

The following arguments are optional:
:�
i
ssmincidentsResponsePlanIntegration@aws:ssmincidents/ResponsePlanIntegration:ResponsePlanIntegration�
��
pagerduties�B�*�:�
�
ssmincidents ResponsePlanIntegrationPagerdutyRaws:ssmincidents/ResponsePlanIntegrationPagerduty:ResponsePlanIntegrationPagerdutycDetails about the PagerDuty configuration for a response plan. The following values are supported:
:�
�
ssmincidents ResponsePlanIntegrationPagerdutyRaws:ssmincidents/ResponsePlanIntegrationPagerduty:ResponsePlanIntegrationPagerduty�
�+
name" The name of the response plan.
�
secretId" �The ID of the AWS Secrets Manager secret that stores your PagerDuty key &mdash; either a General Access REST API Key or User Token REST API Key &mdash; and other user credentials.

For more information about the constraints for each field, see [CreateResponsePlan](https://docs.aws.amazon.com/incident-manager/latest/APIReference/API_CreateResponsePlan.html) in the *AWS Systems Manager Incident Manager API Reference*.
p
	serviceId" _The ID of the PagerDuty service that the response plan associated with the incident at launch.
:�
i
ssmincidentsgetReplicationSetRegion@aws:ssmincidents/getReplicationSetRegion:getReplicationSetRegion�
�U
	kmsKeyArn" DThe ARN of the AWS Key Management Service (AWS KMS) encryption key.
$
name" The name of the Region.
{
status" mThe current status of the Region.
* Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
D
statusMessage" /More information about the status of a Region.
:�
c
ssmincidentsgetResponsePlanAction<aws:ssmincidents/getResponsePlanAction:getResponsePlanAction�
��
ssmAutomations�*�:�
�
ssmincidents"getResponsePlanActionSsmAutomationVaws:ssmincidents/getResponsePlanActionSsmAutomation:getResponsePlanActionSsmAutomation�The Systems Manager automation document to start as the runbook at the beginning of the incident. The following values are supported:
:�
�
ssmincidents"getResponsePlanActionSsmAutomationVaws:ssmincidents/getResponsePlanActionSsmAutomation:getResponsePlanActionSsmAutomation�
�4
documentName"  The automation document's name.
Q
documentVersion" :The version of the automation document to use at runtime.
�
dynamicParameters2" rThe key-value pair used to resolve dynamic parameter values when processing a Systems Manager Automation runbook.
�

parameters�*�:�
�
ssmincidents+getResponsePlanActionSsmAutomationParameterhaws:ssmincidents/getResponsePlanActionSsmAutomationParameter:getResponsePlanActionSsmAutomationParameterjThe key-value pair parameters used when the automation document runs. The following values are supported:
v
roleArn" gThe Amazon Resource Name (ARN) of the role that the automation document assumes when it runs commands.
�
targetAccount" wThe account that runs the automation document. This can be in either the management account or an application account.
:�
�
ssmincidents+getResponsePlanActionSsmAutomationParameterhaws:ssmincidents/getResponsePlanActionSsmAutomationParameter:getResponsePlanActionSsmAutomationParametery
w5
name" )The name of the PagerDuty configuration.
>
values*" .The values for the associated parameter name.
:�
�
ssmincidentsgetResponsePlanIncidentTemplatePaws:ssmincidents/getResponsePlanIncidentTemplate:getResponsePlanIncidentTemplate�
�z
dedupeString" fA string used to stop Incident Manager from creating multiple incident records for the same incident.
\
impact NThe impact value of a generated incident. The following values are supported:
�
incidentTags2" �The tags assigned to an incident template. When an incident starts, Incident Manager assigns the tags specified in the template to the incident.
�
notificationTargets�*�:�
�
ssmincidents1getResponsePlanIncidentTemplateNotificationTargettaws:ssmincidents/getResponsePlanIncidentTemplateNotificationTarget:getResponsePlanIncidentTemplateNotificationTarget�The Amazon Simple Notification Service (Amazon SNS) targets that this incident notifies when it is updated. The `notification_target` configuration block supports the following argument:
+
summary" The summary of an incident.
0
title" #The title of a generated incident.
:�
�
ssmincidents1getResponsePlanIncidentTemplateNotificationTargettaws:ssmincidents/getResponsePlanIncidentTemplateNotificationTarget:getResponsePlanIncidentTemplateNotificationTarget8
64
snsTopicArn" !The ARN of the Amazon SNS topic.
:�
r
ssmincidentsgetResponsePlanIntegrationFaws:ssmincidents/getResponsePlanIntegration:getResponsePlanIntegration�
��
pagerduties�*�:�
�
ssmincidents#getResponsePlanIntegrationPagerdutyXaws:ssmincidents/getResponsePlanIntegrationPagerduty:getResponsePlanIntegrationPagerdutycDetails about the PagerDuty configuration for a response plan. The following values are supported:
:�
�
ssmincidents#getResponsePlanIntegrationPagerdutyXaws:ssmincidents/getResponsePlanIntegrationPagerduty:getResponsePlanIntegrationPagerduty�
�5
name" )The name of the PagerDuty configuration.
�
secretId" �The ID of the AWS Secrets Manager secret that stores your PagerDuty key &mdash; either a General Access REST API Key or User Token REST API Key &mdash; and other user credentials.
v
	serviceId" eThe ID of the PagerDuty service that the response plan associates with an incident when it launches.
