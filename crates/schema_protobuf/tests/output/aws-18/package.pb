
awsAWS"6.66.2*k
6
	schedulerScheduleaws:scheduler/schedule:ScheduleüKProvides an EventBridge Scheduler Schedule resource.

You can find out more about EventBridge Scheduler in the [User Guide](https://docs.aws.amazon.com/scheduler/latest/UserGuide/what-is-scheduler.html).

> **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.scheduler.Schedule("example", {
    name: "my-schedule",
    groupName: "default",
    flexibleTimeWindow: {
        mode: "OFF",
    },
    scheduleExpression: "rate(1 hours)",
    target: {
        arn: exampleAwsSqsQueue.arn,
        roleArn: exampleAwsIamRole.arn,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.scheduler.Schedule("example",
    name="my-schedule",
    group_name="default",
    flexible_time_window={
        "mode": "OFF",
    },
    schedule_expression="rate(1 hours)",
    target={
        "arn": example_aws_sqs_queue["arn"],
        "role_arn": example_aws_iam_role["arn"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Scheduler.Schedule("example", new()
    {
        Name = "my-schedule",
        GroupName = "default",
        FlexibleTimeWindow = new Aws.Scheduler.Inputs.ScheduleFlexibleTimeWindowArgs
        {
            Mode = "OFF",
        },
        ScheduleExpression = "rate(1 hours)",
        Target = new Aws.Scheduler.Inputs.ScheduleTargetArgs
        {
            Arn = exampleAwsSqsQueue.Arn,
            RoleArn = exampleAwsIamRole.Arn,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/scheduler"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := scheduler.NewSchedule(ctx, "example", &scheduler.ScheduleArgs{
			Name:      pulumi.String("my-schedule"),
			GroupName: pulumi.String("default"),
			FlexibleTimeWindow: &scheduler.ScheduleFlexibleTimeWindowArgs{
				Mode: pulumi.String("OFF"),
			},
			ScheduleExpression: pulumi.String("rate(1 hours)"),
			Target: &scheduler.ScheduleTargetArgs{
				Arn:     pulumi.Any(exampleAwsSqsQueue.Arn),
				RoleArn: pulumi.Any(exampleAwsIamRole.Arn),
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
import com.pulumi.aws.scheduler.Schedule;
import com.pulumi.aws.scheduler.ScheduleArgs;
import com.pulumi.aws.scheduler.inputs.ScheduleFlexibleTimeWindowArgs;
import com.pulumi.aws.scheduler.inputs.ScheduleTargetArgs;
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
        var example = new Schedule("example", ScheduleArgs.builder()
            .name("my-schedule")
            .groupName("default")
            .flexibleTimeWindow(ScheduleFlexibleTimeWindowArgs.builder()
                .mode("OFF")
                .build())
            .scheduleExpression("rate(1 hours)")
            .target(ScheduleTargetArgs.builder()
                .arn(exampleAwsSqsQueue.arn())
                .roleArn(exampleAwsIamRole.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:scheduler:Schedule
    properties:
      name: my-schedule
      groupName: default
      flexibleTimeWindow:
        mode: OFF
      scheduleExpression: rate(1 hours)
      target:
        arn: ${exampleAwsSqsQueue.arn}
        roleArn: ${exampleAwsIamRole.arn}
```
<!--End PulumiCodeChooser -->

### Universal Target

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.sqs.Queue("example", {});
const exampleSchedule = new aws.scheduler.Schedule("example", {
    name: "my-schedule",
    flexibleTimeWindow: {
        mode: "OFF",
    },
    scheduleExpression: "rate(1 hours)",
    target: {
        arn: "arn:aws:scheduler:::aws-sdk:sqs:sendMessage",
        roleArn: exampleAwsIamRole.arn,
        input: pulumi.jsonStringify({
            MessageBody: "Greetings, programs!",
            QueueUrl: example.url,
        }),
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.sqs.Queue("example")
example_schedule = aws.scheduler.Schedule("example",
    name="my-schedule",
    flexible_time_window={
        "mode": "OFF",
    },
    schedule_expression="rate(1 hours)",
    target={
        "arn": "arn:aws:scheduler:::aws-sdk:sqs:sendMessage",
        "role_arn": example_aws_iam_role["arn"],
        "input": pulumi.Output.json_dumps({
            "MessageBody": "Greetings, programs!",
            "QueueUrl": example.url,
        }),
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
    var example = new Aws.Sqs.Queue("example");

    var exampleSchedule = new Aws.Scheduler.Schedule("example", new()
    {
        Name = "my-schedule",
        FlexibleTimeWindow = new Aws.Scheduler.Inputs.ScheduleFlexibleTimeWindowArgs
        {
            Mode = "OFF",
        },
        ScheduleExpression = "rate(1 hours)",
        Target = new Aws.Scheduler.Inputs.ScheduleTargetArgs
        {
            Arn = "arn:aws:scheduler:::aws-sdk:sqs:sendMessage",
            RoleArn = exampleAwsIamRole.Arn,
            Input = Output.JsonSerialize(Output.Create(new Dictionary<string, object?>
            {
                ["MessageBody"] = "Greetings, programs!",
                ["QueueUrl"] = example.Url,
            })),
        },
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/scheduler"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/sqs"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := sqs.NewQueue(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = scheduler.NewSchedule(ctx, "example", &scheduler.ScheduleArgs{
			Name: pulumi.String("my-schedule"),
			FlexibleTimeWindow: &scheduler.ScheduleFlexibleTimeWindowArgs{
				Mode: pulumi.String("OFF"),
			},
			ScheduleExpression: pulumi.String("rate(1 hours)"),
			Target: &scheduler.ScheduleTargetArgs{
				Arn:     pulumi.String("arn:aws:scheduler:::aws-sdk:sqs:sendMessage"),
				RoleArn: pulumi.Any(exampleAwsIamRole.Arn),
				Input: example.Url.ApplyT(func(url string) (pulumi.String, error) {
					var _zero pulumi.String
					tmpJSON0, err := json.Marshal(map[string]interface{}{
						"MessageBody": "Greetings, programs!",
						"QueueUrl":    url,
					})
					if err != nil {
						return _zero, err
					}
					json0 := string(tmpJSON0)
					return pulumi.String(json0), nil
				}).(pulumi.StringOutput),
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
import com.pulumi.aws.scheduler.Schedule;
import com.pulumi.aws.scheduler.ScheduleArgs;
import com.pulumi.aws.scheduler.inputs.ScheduleFlexibleTimeWindowArgs;
import com.pulumi.aws.scheduler.inputs.ScheduleTargetArgs;
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
        var example = new Queue("example");

        var exampleSchedule = new Schedule("exampleSchedule", ScheduleArgs.builder()
            .name("my-schedule")
            .flexibleTimeWindow(ScheduleFlexibleTimeWindowArgs.builder()
                .mode("OFF")
                .build())
            .scheduleExpression("rate(1 hours)")
            .target(ScheduleTargetArgs.builder()
                .arn("arn:aws:scheduler:::aws-sdk:sqs:sendMessage")
                .roleArn(exampleAwsIamRole.arn())
                .input(example.url().applyValue(url -> serializeJson(
                    jsonObject(
                        jsonProperty("MessageBody", "Greetings, programs!"),
                        jsonProperty("QueueUrl", url)
                    ))))
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:sqs:Queue
  exampleSchedule:
    type: aws:scheduler:Schedule
    name: example
    properties:
      name: my-schedule
      flexibleTimeWindow:
        mode: OFF
      scheduleExpression: rate(1 hours)
      target:
        arn: arn:aws:scheduler:::aws-sdk:sqs:sendMessage
        roleArn: ${exampleAwsIamRole.arn}
        input:
          fn::toJSON:
            MessageBody: Greetings, programs!
            QueueUrl: ${example.url}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import schedules using the combination `group_name/name`. For example:

```sh
$ pulumi import aws:scheduler/schedule:Schedule example my-schedule-group/my-schedule
```
8
descriptionB" #Brief description of the schedule.
¨
endDateB" öThe date, in UTC, before which the schedule can invoke its target. Depending on the schedule's recurrence expression, invocations might stop on, or before, the end date you specify. EventBridge Scheduler ignores the end date for one-time schedules. Example: `2030-01-01T01:00:00Z`.
Í
flexibleTimeWindowp:n
l
	schedulerScheduleFlexibleTimeWindowCaws:scheduler/ScheduleFlexibleTimeWindow:ScheduleFlexibleTimeWindowbConfigures a time window during which EventBridge Scheduler invokes the schedule. Detailed below.
É
	groupNameB" pName of the schedule group to associate with this schedule. When omitted, the `default` schedule group is used.
~
	kmsKeyArnB" kARN for the customer managed KMS key that EventBridge Scheduler will use to encrypt and decrypt your data.
~
nameB" pName of the schedule. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
ø
scheduleExpression" §Defines when the schedule runs. Read more in [Schedule types on EventBridge Scheduler](https://docs.aws.amazon.com/scheduler/latest/UserGuide/schedule-types.html).
é
scheduleExpressionTimezoneB" jTimezone in which the scheduling expression is evaluated. Defaults to `UTC`. Example: `Australia/Sydney`.
π
	startDateB" •The date, in UTC, after which the schedule can begin invoking its target. Depending on the schedule's recurrence expression, invocations might occur on, or after, the start date you specify. EventBridge Scheduler ignores the start date for one-time schedules. Example: `2030-01-01T01:00:00Z`.
o
stateB" `Specifies whether the schedule is enabled or disabled. One of: `ENABLED` (default), `DISABLED`.
∂
targetL:J
H
	schedulerScheduleTarget+aws:scheduler/ScheduleTarget:ScheduleTarget^Configures the target of the schedule. Detailed below.

The following arguments are optional:
" 
arn" ARN of the schedule.
"8
descriptionB" #Brief description of the schedule.
"¨
endDateB" öThe date, in UTC, before which the schedule can invoke its target. Depending on the schedule's recurrence expression, invocations might stop on, or before, the end date you specify. EventBridge Scheduler ignores the end date for one-time schedules. Example: `2030-01-01T01:00:00Z`.
"Í
flexibleTimeWindowp:n
l
	schedulerScheduleFlexibleTimeWindowCaws:scheduler/ScheduleFlexibleTimeWindow:ScheduleFlexibleTimeWindowbConfigures a time window during which EventBridge Scheduler invokes the schedule. Detailed below.
"Å
	groupName" pName of the schedule group to associate with this schedule. When omitted, the `default` schedule group is used.
"~
	kmsKeyArnB" kARN for the customer managed KMS key that EventBridge Scheduler will use to encrypt and decrypt your data.
"|
name" pName of the schedule. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"ø
scheduleExpression" §Defines when the schedule runs. Read more in [Schedule types on EventBridge Scheduler](https://docs.aws.amazon.com/scheduler/latest/UserGuide/schedule-types.html).
"é
scheduleExpressionTimezoneB" jTimezone in which the scheduling expression is evaluated. Defaults to `UTC`. Example: `Australia/Sydney`.
"π
	startDateB" •The date, in UTC, after which the schedule can begin invoking its target. Depending on the schedule's recurrence expression, invocations might occur on, or after, the start date you specify. EventBridge Scheduler ignores the start date for one-time schedules. Example: `2030-01-01T01:00:00Z`.
"o
stateB" `Specifies whether the schedule is enabled or disabled. One of: `ENABLED` (default), `DISABLED`.
"∂
targetL:J
H
	schedulerScheduleTarget+aws:scheduler/ScheduleTarget:ScheduleTarget^Configures the target of the schedule. Detailed below.

The following arguments are optional:
*
E
	schedulerScheduleGroup)aws:scheduler/scheduleGroup:ScheduleGroupùProvides an EventBridge Scheduler Schedule Group resource.

You can find out more about EventBridge Scheduler in the [User Guide](https://docs.aws.amazon.com/scheduler/latest/UserGuide/what-is-scheduler.html).

> **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.scheduler.ScheduleGroup("example", {name: "my-schedule-group"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.scheduler.ScheduleGroup("example", name="my-schedule-group")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Scheduler.ScheduleGroup("example", new()
    {
        Name = "my-schedule-group",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/scheduler"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := scheduler.NewScheduleGroup(ctx, "example", &scheduler.ScheduleGroupArgs{
			Name: pulumi.String("my-schedule-group"),
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
import com.pulumi.aws.scheduler.ScheduleGroup;
import com.pulumi.aws.scheduler.ScheduleGroupArgs;
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
        var example = new ScheduleGroup("example", ScheduleGroupArgs.builder()
            .name("my-schedule-group")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:scheduler:ScheduleGroup
    properties:
      name: my-schedule-group
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import schedule groups using the `name`. For example:

```sh
$ pulumi import aws:scheduler/scheduleGroup:ScheduleGroup example my-schedule-group
```
Ñ
nameB" vName of the schedule group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"&
arn" ARN of the schedule group.
"B
creationDate" .Time at which the schedule group was created.
"P
lastModificationDate" 4Time at which the schedule group was last modified.
"Ç
name" vName of the schedule group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"I
state" <State of the schedule group. Can be `ACTIVE` or `DELETING`.
"À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*È 
8
schemas
Discoverer!aws:schemas/discoverer:Discoverer˝Provides an EventBridge Schema Discoverer resource.

> **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const messenger = new aws.cloudwatch.EventBus("messenger", {name: "chat-messages"});
const test = new aws.schemas.Discoverer("test", {
    sourceArn: messenger.arn,
    description: "Auto discover event schemas",
});
```
```python
import pulumi
import pulumi_aws as aws

messenger = aws.cloudwatch.EventBus("messenger", name="chat-messages")
test = aws.schemas.Discoverer("test",
    source_arn=messenger.arn,
    description="Auto discover event schemas")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var messenger = new Aws.CloudWatch.EventBus("messenger", new()
    {
        Name = "chat-messages",
    });

    var test = new Aws.Schemas.Discoverer("test", new()
    {
        SourceArn = messenger.Arn,
        Description = "Auto discover event schemas",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudwatch"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/schemas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		messenger, err := cloudwatch.NewEventBus(ctx, "messenger", &cloudwatch.EventBusArgs{
			Name: pulumi.String("chat-messages"),
		})
		if err != nil {
			return err
		}
		_, err = schemas.NewDiscoverer(ctx, "test", &schemas.DiscovererArgs{
			SourceArn:   messenger.Arn,
			Description: pulumi.String("Auto discover event schemas"),
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
import com.pulumi.aws.cloudwatch.EventBus;
import com.pulumi.aws.cloudwatch.EventBusArgs;
import com.pulumi.aws.schemas.Discoverer;
import com.pulumi.aws.schemas.DiscovererArgs;
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
        var messenger = new EventBus("messenger", EventBusArgs.builder()
            .name("chat-messages")
            .build());

        var test = new Discoverer("test", DiscovererArgs.builder()
            .sourceArn(messenger.arn())
            .description("Auto discover event schemas")
            .build());

    }
}
```
```yaml
resources:
  messenger:
    type: aws:cloudwatch:EventBus
    properties:
      name: chat-messages
  test:
    type: aws:schemas:Discoverer
    properties:
      sourceArn: ${messenger.arn}
      description: Auto discover event schemas
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import EventBridge discoverers using the `id`. For example:

```sh
$ pulumi import aws:schemas/discoverer:Discoverer test 123
```
S
descriptionB" >The description of the discoverer. Maximum of 256 characters.
H
	sourceArn" 7The ARN of the event bus to discover event schemas on.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"=
arn" 2The Amazon Resource Name (ARN) of the discoverer.
"S
descriptionB" >The description of the discoverer. Maximum of 256 characters.
"H
	sourceArn" 7The ARN of the event bus to discover event schemas on.
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*÷
2
schemasRegistryaws:schemas/registry:RegistryÃProvides an EventBridge Custom Schema Registry resource.

> **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.schemas.Registry("test", {
    name: "my_own_registry",
    description: "A custom schema registry",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.schemas.Registry("test",
    name="my_own_registry",
    description="A custom schema registry")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Schemas.Registry("test", new()
    {
        Name = "my_own_registry",
        Description = "A custom schema registry",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/schemas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := schemas.NewRegistry(ctx, "test", &schemas.RegistryArgs{
			Name:        pulumi.String("my_own_registry"),
			Description: pulumi.String("A custom schema registry"),
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
import com.pulumi.aws.schemas.Registry;
import com.pulumi.aws.schemas.RegistryArgs;
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
        var test = new Registry("test", RegistryArgs.builder()
            .name("my_own_registry")
            .description("A custom schema registry")
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:schemas:Registry
    properties:
      name: my_own_registry
      description: A custom schema registry
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import EventBridge schema registries using the `name`. For example:

```sh
$ pulumi import aws:schemas/registry:Registry test my_own_registry
```
S
descriptionB" >The description of the discoverer. Maximum of 256 characters.
ö
nameB" ãThe name of the custom event schema registry. Maximum of 64 characters consisting of lower case letters, upper case letters, 0-9, ., -, _.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"=
arn" 2The Amazon Resource Name (ARN) of the discoverer.
"S
descriptionB" >The description of the discoverer. Maximum of 256 characters.
"ò
name" ãThe name of the custom event schema registry. Maximum of 64 characters consisting of lower case letters, upper case letters, 0-9, ., -, _.
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*ó4
D
schemasRegistryPolicy)aws:schemas/registryPolicy:RegistryPolicy⁄1Resource for managing an AWS EventBridge Schemas Registry Policy.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.iam.getPolicyDocument({
    statements: [{
        sid: "example",
        effect: "Allow",
        principals: [{
            type: "AWS",
            identifiers: ["109876543210"],
        }],
        actions: ["schemas:*"],
        resources: [
            "arn:aws:schemas:us-east-1:123456789012:registry/example",
            "arn:aws:schemas:us-east-1:123456789012:schema/example*",
        ],
    }],
});
const exampleRegistryPolicy = new aws.schemas.RegistryPolicy("example", {
    registryName: "example",
    policy: example.then(example => example.json),
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.iam.get_policy_document(statements=[{
    "sid": "example",
    "effect": "Allow",
    "principals": [{
        "type": "AWS",
        "identifiers": ["109876543210"],
    }],
    "actions": ["schemas:*"],
    "resources": [
        "arn:aws:schemas:us-east-1:123456789012:registry/example",
        "arn:aws:schemas:us-east-1:123456789012:schema/example*",
    ],
}])
example_registry_policy = aws.schemas.RegistryPolicy("example",
    registry_name="example",
    policy=example.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "example",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            "109876543210",
                        },
                    },
                },
                Actions = new[]
                {
                    "schemas:*",
                },
                Resources = new[]
                {
                    "arn:aws:schemas:us-east-1:123456789012:registry/example",
                    "arn:aws:schemas:us-east-1:123456789012:schema/example*",
                },
            },
        },
    });

    var exampleRegistryPolicy = new Aws.Schemas.RegistryPolicy("example", new()
    {
        RegistryName = "example",
        Policy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/schemas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid:    pulumi.StringRef("example"),
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "AWS",
							Identifiers: []string{
								"109876543210",
							},
						},
					},
					Actions: []string{
						"schemas:*",
					},
					Resources: []string{
						"arn:aws:schemas:us-east-1:123456789012:registry/example",
						"arn:aws:schemas:us-east-1:123456789012:schema/example*",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = schemas.NewRegistryPolicy(ctx, "example", &schemas.RegistryPolicyArgs{
			RegistryName: pulumi.String("example"),
			Policy:       pulumi.String(example.Json),
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
import com.pulumi.aws.schemas.RegistryPolicy;
import com.pulumi.aws.schemas.RegistryPolicyArgs;
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
        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("example")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("AWS")
                    .identifiers("109876543210")
                    .build())
                .actions("schemas:*")
                .resources(                
                    "arn:aws:schemas:us-east-1:123456789012:registry/example",
                    "arn:aws:schemas:us-east-1:123456789012:schema/example*")
                .build())
            .build());

        var exampleRegistryPolicy = new RegistryPolicy("exampleRegistryPolicy", RegistryPolicyArgs.builder()
            .registryName("example")
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

    }
}
```
```yaml
resources:
  exampleRegistryPolicy:
    type: aws:schemas:RegistryPolicy
    name: example
    properties:
      registryName: example
      policy: ${example.json}
variables:
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: example
            effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - '109876543210'
            actions:
              - schemas:*
            resources:
              - arn:aws:schemas:us-east-1:123456789012:registry/example
              - arn:aws:schemas:us-east-1:123456789012:schema/example*
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import EventBridge Schema Registry Policy using the `registry_name`. For example:

```sh
$ pulumi import aws:schemas/registryPolicy:RegistryPolicy example example
```
>
policy" 0Resource Policy for EventBridge Schema Registry
8
registryName" $Name of EventBridge Schema Registry
">
policy" 0Resource Policy for EventBridge Schema Registry
"8
registryName" $Name of EventBridge Schema Registry
*œH
,
schemasSchemaaws:schemas/schema:SchemaÓ:Provides an EventBridge Schema resource.

> **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.schemas.Registry("test", {name: "my_own_registry"});
const testSchema = new aws.schemas.Schema("test", {
    name: "my_schema",
    registryName: test.name,
    type: "OpenApi3",
    description: "The schema definition for my event",
    content: JSON.stringify({
        openapi: "3.0.0",
        info: {
            version: "1.0.0",
            title: "Event",
        },
        paths: {},
        components: {
            schemas: {
                Event: {
                    type: "object",
                    properties: {
                        name: {
                            type: "string",
                        },
                    },
                },
            },
        },
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

test = aws.schemas.Registry("test", name="my_own_registry")
test_schema = aws.schemas.Schema("test",
    name="my_schema",
    registry_name=test.name,
    type="OpenApi3",
    description="The schema definition for my event",
    content=json.dumps({
        "openapi": "3.0.0",
        "info": {
            "version": "1.0.0",
            "title": "Event",
        },
        "paths": {},
        "components": {
            "schemas": {
                "Event": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                        },
                    },
                },
            },
        },
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
    var test = new Aws.Schemas.Registry("test", new()
    {
        Name = "my_own_registry",
    });

    var testSchema = new Aws.Schemas.Schema("test", new()
    {
        Name = "my_schema",
        RegistryName = test.Name,
        Type = "OpenApi3",
        Description = "The schema definition for my event",
        Content = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["openapi"] = "3.0.0",
            ["info"] = new Dictionary<string, object?>
            {
                ["version"] = "1.0.0",
                ["title"] = "Event",
            },
            ["paths"] = new Dictionary<string, object?>
            {
            },
            ["components"] = new Dictionary<string, object?>
            {
                ["schemas"] = new Dictionary<string, object?>
                {
                    ["Event"] = new Dictionary<string, object?>
                    {
                        ["type"] = "object",
                        ["properties"] = new Dictionary<string, object?>
                        {
                            ["name"] = new Dictionary<string, object?>
                            {
                                ["type"] = "string",
                            },
                        },
                    },
                },
            },
        }),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/schemas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		test, err := schemas.NewRegistry(ctx, "test", &schemas.RegistryArgs{
			Name: pulumi.String("my_own_registry"),
		})
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"openapi": "3.0.0",
			"info": map[string]interface{}{
				"version": "1.0.0",
				"title":   "Event",
			},
			"paths": map[string]interface{}{},
			"components": map[string]interface{}{
				"schemas": map[string]interface{}{
					"Event": map[string]interface{}{
						"type": "object",
						"properties": map[string]interface{}{
							"name": map[string]interface{}{
								"type": "string",
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = schemas.NewSchema(ctx, "test", &schemas.SchemaArgs{
			Name:         pulumi.String("my_schema"),
			RegistryName: test.Name,
			Type:         pulumi.String("OpenApi3"),
			Description:  pulumi.String("The schema definition for my event"),
			Content:      pulumi.String(json0),
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
import com.pulumi.aws.schemas.Registry;
import com.pulumi.aws.schemas.RegistryArgs;
import com.pulumi.aws.schemas.Schema;
import com.pulumi.aws.schemas.SchemaArgs;
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
        var test = new Registry("test", RegistryArgs.builder()
            .name("my_own_registry")
            .build());

        var testSchema = new Schema("testSchema", SchemaArgs.builder()
            .name("my_schema")
            .registryName(test.name())
            .type("OpenApi3")
            .description("The schema definition for my event")
            .content(serializeJson(
                jsonObject(
                    jsonProperty("openapi", "3.0.0"),
                    jsonProperty("info", jsonObject(
                        jsonProperty("version", "1.0.0"),
                        jsonProperty("title", "Event")
                    )),
                    jsonProperty("paths", jsonObject(

                    )),
                    jsonProperty("components", jsonObject(
                        jsonProperty("schemas", jsonObject(
                            jsonProperty("Event", jsonObject(
                                jsonProperty("type", "object"),
                                jsonProperty("properties", jsonObject(
                                    jsonProperty("name", jsonObject(
                                        jsonProperty("type", "string")
                                    ))
                                ))
                            ))
                        ))
                    ))
                )))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:schemas:Registry
    properties:
      name: my_own_registry
  testSchema:
    type: aws:schemas:Schema
    name: test
    properties:
      name: my_schema
      registryName: ${test.name}
      type: OpenApi3
      description: The schema definition for my event
      content:
        fn::toJSON:
          openapi: 3.0.0
          info:
            version: 1.0.0
            title: Event
          paths: {}
          components:
            schemas:
              Event:
                type: object
                properties:
                  name:
                    type: string
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import EventBridge schema using the `name` and `registry_name`. For example:

```sh
$ pulumi import aws:schemas/schema:Schema test name/registry
```
L
content" =The schema specification. Must be a valid Open API 3.0 spec.
O
descriptionB" :The description of the schema. Maximum of 256 characters.
Ç
nameB" tThe name of the schema. Maximum of 385 characters consisting of lower case letters, upper case letters, ., -, _, @.
K
registryName" 7The name of the registry in which this schema belongs.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
T
type" HThe type of the schema. Valid values: `OpenApi3` or `JSONSchemaDraft4`.
"=
arn" 2The Amazon Resource Name (ARN) of the discoverer.
"L
content" =The schema specification. Must be a valid Open API 3.0 spec.
"O
descriptionB" :The description of the schema. Maximum of 256 characters.
":
lastModified" &The last modified date of the schema.
"Ä
name" tThe name of the schema. Maximum of 385 characters consisting of lower case letters, upper case letters, ., -, _, @.
"K
registryName" 7The name of the registry in which this schema belongs.
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"T
type" HThe type of the schema. Valid values: `OpenApi3` or `JSONSchemaDraft4`.
"*
version" The version of the schema.
"I
versionCreatedDate" /The created date of the version of the schema.
*ç4
:
secretsmanagerSecret aws:secretsmanager/secret:Secret“Provides a resource to manage AWS Secrets Manager secret metadata. To manage secret rotation, see the `aws.secretsmanager.SecretRotation` resource. To manage a secret value, see the `aws.secretsmanager.SecretVersion` resource.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.secretsmanager.Secret("example", {name: "example"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.secretsmanager.Secret("example", name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecretsManager.Secret("example", new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.NewSecret(ctx, "example", &secretsmanager.SecretArgs{
			Name: pulumi.String("example"),
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
import com.pulumi.aws.secretsmanager.Secret;
import com.pulumi.aws.secretsmanager.SecretArgs;
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
        var example = new Secret("example", SecretArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:secretsmanager:Secret
    properties:
      name: example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_secretsmanager_secret` using the secret Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:secretsmanager/secret:Secret example arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456
```
0
descriptionB" Description of the secret.
í
forceOverwriteReplicaSecretB
 mAccepts boolean value to specify whether to overwrite a secret with the same name in the destination Region.
Á
kmsKeyIdB" ‘ARN or Id of the AWS KMS key to be used to encrypt the secret values in the versions stored in this secret. If you need to reference a CMK in a different account, you can use only the key ARN. If you don't specify this value, then Secrets Manager defaults to using the AWS account's default KMS key (the one named `aws/secretsmanager`). If the default KMS key with that name doesn't yet exist, then AWS Secrets Manager creates it for you automatically the first time.
 
nameB" ªFriendly name of the new secret. The secret name can consist of uppercase letters, lowercase letters, digits, and any of the following characters: `/_+=.@-` Conflicts with `name_prefix`.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
√
policyB" ≤Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Removing `policy` from your configuration or setting `policy` to null or an empty string (i.e., `policy = ""`) _will not_ delete the policy since it could have been set by `aws.secretsmanager.SecretPolicy`. To delete the `policy`, set it to `"{}"` (an empty JSON document).
‚
recoveryWindowInDaysB √Number of days that AWS Secrets Manager waits before it can delete the secret. This value can be `0` to force deletion without recovery or range from `7` to `30` days. The default value is `30`.
´
replicasWBU*S:Q
O
secretsmanagerSecretReplica.aws:secretsmanager/SecretReplica:SecretReplicaFConfiguration block to support secret replication. See details below.
Î
tagsB2" ⁄Key-value map of user-defined tags that are attached to the secret. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
arn" ARN of the secret.
"0
descriptionB" Description of the secret.
"í
forceOverwriteReplicaSecretB
 mAccepts boolean value to specify whether to overwrite a secret with the same name in the destination Region.
"Á
kmsKeyIdB" ‘ARN or Id of the AWS KMS key to be used to encrypt the secret values in the versions stored in this secret. If you need to reference a CMK in a different account, you can use only the key ARN. If you don't specify this value, then Secrets Manager defaults to using the AWS account's default KMS key (the one named `aws/secretsmanager`). If the default KMS key with that name doesn't yet exist, then AWS Secrets Manager creates it for you automatically the first time.
"»
name" ªFriendly name of the new secret. The secret name can consist of uppercase letters, lowercase letters, digits, and any of the following characters: `/_+=.@-` Conflicts with `name_prefix`.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"¡
policy" ≤Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Removing `policy` from your configuration or setting `policy` to null or an empty string (i.e., `policy = ""`) _will not_ delete the policy since it could have been set by `aws.secretsmanager.SecretPolicy`. To delete the `policy`, set it to `"{}"` (an empty JSON document).
"‚
recoveryWindowInDaysB √Number of days that AWS Secrets Manager waits before it can delete the secret. This value can be `0` to force deletion without recovery or range from `7` to `30` days. The default value is `30`.
"©
replicasU*S:Q
O
secretsmanagerSecretReplica.aws:secretsmanager/SecretReplica:SecretReplicaFConfiguration block to support secret replication. See details below.
"Î
tagsB2" ⁄Key-value map of user-defined tags that are attached to the secret. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*ø>
L
secretsmanagerSecretPolicy,aws:secretsmanager/secretPolicy:SecretPolicy∏5Provides a resource to manage AWS Secrets Manager secret policy.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleSecret = new aws.secretsmanager.Secret("example", {name: "example"});
const example = aws.iam.getPolicyDocument({
    statements: [{
        sid: "EnableAnotherAWSAccountToReadTheSecret",
        effect: "Allow",
        principals: [{
            type: "AWS",
            identifiers: ["arn:aws:iam::123456789012:root"],
        }],
        actions: ["secretsmanager:GetSecretValue"],
        resources: ["*"],
    }],
});
const exampleSecretPolicy = new aws.secretsmanager.SecretPolicy("example", {
    secretArn: exampleSecret.arn,
    policy: example.then(example => example.json),
});
```
```python
import pulumi
import pulumi_aws as aws

example_secret = aws.secretsmanager.Secret("example", name="example")
example = aws.iam.get_policy_document(statements=[{
    "sid": "EnableAnotherAWSAccountToReadTheSecret",
    "effect": "Allow",
    "principals": [{
        "type": "AWS",
        "identifiers": ["arn:aws:iam::123456789012:root"],
    }],
    "actions": ["secretsmanager:GetSecretValue"],
    "resources": ["*"],
}])
example_secret_policy = aws.secretsmanager.SecretPolicy("example",
    secret_arn=example_secret.arn,
    policy=example.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleSecret = new Aws.SecretsManager.Secret("example", new()
    {
        Name = "example",
    });

    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "EnableAnotherAWSAccountToReadTheSecret",
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            "arn:aws:iam::123456789012:root",
                        },
                    },
                },
                Actions = new[]
                {
                    "secretsmanager:GetSecretValue",
                },
                Resources = new[]
                {
                    "*",
                },
            },
        },
    });

    var exampleSecretPolicy = new Aws.SecretsManager.SecretPolicy("example", new()
    {
        SecretArn = exampleSecret.Arn,
        Policy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleSecret, err := secretsmanager.NewSecret(ctx, "example", &secretsmanager.SecretArgs{
			Name: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		example, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid:    pulumi.StringRef("EnableAnotherAWSAccountToReadTheSecret"),
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "AWS",
							Identifiers: []string{
								"arn:aws:iam::123456789012:root",
							},
						},
					},
					Actions: []string{
						"secretsmanager:GetSecretValue",
					},
					Resources: []string{
						"*",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = secretsmanager.NewSecretPolicy(ctx, "example", &secretsmanager.SecretPolicyArgs{
			SecretArn: exampleSecret.Arn,
			Policy:    pulumi.String(example.Json),
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
import com.pulumi.aws.secretsmanager.Secret;
import com.pulumi.aws.secretsmanager.SecretArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.secretsmanager.SecretPolicy;
import com.pulumi.aws.secretsmanager.SecretPolicyArgs;
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
        var exampleSecret = new Secret("exampleSecret", SecretArgs.builder()
            .name("example")
            .build());

        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("EnableAnotherAWSAccountToReadTheSecret")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("AWS")
                    .identifiers("arn:aws:iam::123456789012:root")
                    .build())
                .actions("secretsmanager:GetSecretValue")
                .resources("*")
                .build())
            .build());

        var exampleSecretPolicy = new SecretPolicy("exampleSecretPolicy", SecretPolicyArgs.builder()
            .secretArn(exampleSecret.arn())
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

    }
}
```
```yaml
resources:
  exampleSecret:
    type: aws:secretsmanager:Secret
    name: example
    properties:
      name: example
  exampleSecretPolicy:
    type: aws:secretsmanager:SecretPolicy
    name: example
    properties:
      secretArn: ${exampleSecret.arn}
      policy: ${example.json}
variables:
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: EnableAnotherAWSAccountToReadTheSecret
            effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - arn:aws:iam::123456789012:root
            actions:
              - secretsmanager:GetSecretValue
            resources:
              - '*'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_secretsmanager_secret_policy` using the secret Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:secretsmanager/secretPolicy:SecretPolicy example arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456
```
â
blockPublicPolicyB
 nMakes an optional API call to Zelkova to validate the Resource Policy to prevent broad access to your secret.
∆
policy" ∑Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Unlike `aws.secretsmanager.Secret`, where `policy` can be set to `"{}"` to delete the policy, `"{}"` is not a valid policy since `policy` is required.
D
	secretArn" 3Secret ARN.

The following arguments are optional:
"â
blockPublicPolicyB
 nMakes an optional API call to Zelkova to validate the Resource Policy to prevent broad access to your secret.
"∆
policy" ∑Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Unlike `aws.secretsmanager.Secret`, where `policy` can be set to `"{}"` to delete the policy, `"{}"` is not a valid policy since `policy` is required.
"D
	secretArn" 3Secret ARN.

The following arguments are optional:
*€:
R
secretsmanagerSecretRotation0aws:secretsmanager/secretRotation:SecretRotationä'Provides a resource to manage AWS Secrets Manager secret rotation. To manage a secret, see the `aws.secretsmanager.Secret` resource. To manage a secret value, see the `aws.secretsmanager.SecretVersion` resource.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.secretsmanager.SecretRotation("example", {
    secretId: exampleAwsSecretsmanagerSecret.id,
    rotationLambdaArn: exampleAwsLambdaFunction.arn,
    rotationRules: {
        automaticallyAfterDays: 30,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.secretsmanager.SecretRotation("example",
    secret_id=example_aws_secretsmanager_secret["id"],
    rotation_lambda_arn=example_aws_lambda_function["arn"],
    rotation_rules={
        "automatically_after_days": 30,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecretsManager.SecretRotation("example", new()
    {
        SecretId = exampleAwsSecretsmanagerSecret.Id,
        RotationLambdaArn = exampleAwsLambdaFunction.Arn,
        RotationRules = new Aws.SecretsManager.Inputs.SecretRotationRotationRulesArgs
        {
            AutomaticallyAfterDays = 30,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.NewSecretRotation(ctx, "example", &secretsmanager.SecretRotationArgs{
			SecretId:          pulumi.Any(exampleAwsSecretsmanagerSecret.Id),
			RotationLambdaArn: pulumi.Any(exampleAwsLambdaFunction.Arn),
			RotationRules: &secretsmanager.SecretRotationRotationRulesArgs{
				AutomaticallyAfterDays: pulumi.Int(30),
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
import com.pulumi.aws.secretsmanager.SecretRotation;
import com.pulumi.aws.secretsmanager.SecretRotationArgs;
import com.pulumi.aws.secretsmanager.inputs.SecretRotationRotationRulesArgs;
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
        var example = new SecretRotation("example", SecretRotationArgs.builder()
            .secretId(exampleAwsSecretsmanagerSecret.id())
            .rotationLambdaArn(exampleAwsLambdaFunction.arn())
            .rotationRules(SecretRotationRotationRulesArgs.builder()
                .automaticallyAfterDays(30)
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:secretsmanager:SecretRotation
    properties:
      secretId: ${exampleAwsSecretsmanagerSecret.id}
      rotationLambdaArn: ${exampleAwsLambdaFunction.arn}
      rotationRules:
        automaticallyAfterDays: 30
```
<!--End PulumiCodeChooser -->

### Rotation Configuration

To enable automatic secret rotation, the Secrets Manager service requires usage of a Lambda function. The [Rotate Secrets section in the Secrets Manager User Guide](https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotating-secrets.html) provides additional information about deploying a prebuilt Lambda functions for supported credential rotation (e.g., RDS) or deploying a custom Lambda function.

> **NOTE:** Configuring rotation causes the secret to rotate once as soon as you enable rotation. Before you do this, you must ensure that all of your applications that use the credentials stored in the secret are updated to retrieve the secret from AWS Secrets Manager. The old credentials might no longer be usable after the initial rotation and any applications that you fail to update will break as soon as the old credentials are no longer valid.

> **NOTE:** If you cancel a rotation that is in progress (by removing the `rotation` configuration), it can leave the VersionStage labels in an unexpected state. Depending on what step of the rotation was in progress, you might need to remove the staging label AWSPENDING from the partially created version, specified by the SecretVersionId response value. You should also evaluate the partially rotated new version to see if it should be deleted, which you can do by removing all staging labels from the new version's VersionStage field.

## Import

Using `pulumi import`, import `aws_secretsmanager_secret_rotation` using the secret Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:secretsmanager/secretRotation:SecretRotation example arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456
```
√
rotateImmediatelyB
 ßSpecifies whether to rotate the secret immediately or wait until the next scheduled rotation window. The rotation schedule is defined in `rotation_rules`. For secrets that use a Lambda rotation function to rotate, if you don't immediately rotate the secret, Secrets Manager tests the rotation configuration by running the testSecret step (https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html) of the Lambda rotation function. The test creates an AWSPENDING version of the secret and then removes it. Defaults to `true`.
ñ
rotationLambdaArnB" {Specifies the ARN of the Lambda function that can rotate the secret. Must be supplied if the secret is not managed by AWS.
‰
rotationRules}:{
y
secretsmanagerSecretRotationRotationRulesJaws:secretsmanager/SecretRotationRotationRules:SecretRotationRotationRulesTA structure that defines the rotation configuration for this secret. Defined below.
«
secretId" ∂Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
"√
rotateImmediatelyB
 ßSpecifies whether to rotate the secret immediately or wait until the next scheduled rotation window. The rotation schedule is defined in `rotation_rules`. For secrets that use a Lambda rotation function to rotate, if you don't immediately rotate the secret, Secrets Manager tests the rotation configuration by running the testSecret step (https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html) of the Lambda rotation function. The test creates an AWSPENDING version of the secret and then removes it. Defaults to `true`.
"X
rotationEnabled
 ASpecifies whether automatic rotation is enabled for this secret.
"ñ
rotationLambdaArnB" {Specifies the ARN of the Lambda function that can rotate the secret. Must be supplied if the secret is not managed by AWS.
"‰
rotationRules}:{
y
secretsmanagerSecretRotationRotationRulesJaws:secretsmanager/SecretRotationRotationRules:SecretRotationRotationRulesTA structure that defines the rotation configuration for this secret. Defined below.
"«
secretId" ∂Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
*ŒN
O
secretsmanagerSecretVersion.aws:secretsmanager/secretVersion:SecretVersion£8Provides a resource to manage AWS Secrets Manager secret version including its secret value. To manage secret metadata, see the `aws.secretsmanager.Secret` resource.

> **NOTE:** If the `AWSCURRENT` staging label is present on this version during resource deletion, that label cannot be removed and will be skipped to prevent errors when fully deleting the secret. That label will leave this secret version active even after the resource is deleted from this provider unless the secret itself is deleted. Move the `AWSCURRENT` staging label before or after deleting this resource from this provider to fully trigger version deprecation if necessary.

## Example Usage

### Simple String Value

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.secretsmanager.SecretVersion("example", {
    secretId: exampleAwsSecretsmanagerSecret.id,
    secretString: "example-string-to-protect",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.secretsmanager.SecretVersion("example",
    secret_id=example_aws_secretsmanager_secret["id"],
    secret_string="example-string-to-protect")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecretsManager.SecretVersion("example", new()
    {
        SecretId = exampleAwsSecretsmanagerSecret.Id,
        SecretString = "example-string-to-protect",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.NewSecretVersion(ctx, "example", &secretsmanager.SecretVersionArgs{
			SecretId:     pulumi.Any(exampleAwsSecretsmanagerSecret.Id),
			SecretString: pulumi.String("example-string-to-protect"),
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
import com.pulumi.aws.secretsmanager.SecretVersion;
import com.pulumi.aws.secretsmanager.SecretVersionArgs;
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
        var example = new SecretVersion("example", SecretVersionArgs.builder()
            .secretId(exampleAwsSecretsmanagerSecret.id())
            .secretString("example-string-to-protect")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:secretsmanager:SecretVersion
    properties:
      secretId: ${exampleAwsSecretsmanagerSecret.id}
      secretString: example-string-to-protect
```
<!--End PulumiCodeChooser -->

### Key-Value Pairs

Secrets Manager also accepts key-value pairs in JSON.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const config = new pulumi.Config();
const example = config.getObject<Record<string, string>>("example") || {
    key1: "value1",
    key2: "value2",
};
const exampleSecretVersion = new aws.secretsmanager.SecretVersion("example", {
    secretId: exampleAwsSecretsmanagerSecret.id,
    secretString: JSON.stringify(example),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

config = pulumi.Config()
example = config.get_object("example")
if example is None:
    example = {
        "key1": "value1",
        "key2": "value2",
    }
example_secret_version = aws.secretsmanager.SecretVersion("example",
    secret_id=example_aws_secretsmanager_secret["id"],
    secret_string=json.dumps(example))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var config = new Config();
    var example = config.GetObject<Dictionary<string, string>>("example") ?? 
    {
        { "key1", "value1" },
        { "key2", "value2" },
    };
    var exampleSecretVersion = new Aws.SecretsManager.SecretVersion("example", new()
    {
        SecretId = exampleAwsSecretsmanagerSecret.Id,
        SecretString = JsonSerializer.Serialize(example),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		cfg := config.New(ctx, "")
		example := map[string]interface{}{
			"key1": "value1",
			"key2": "value2",
		}
		if param := cfg.GetObject("example"); param != nil {
			example = param
		}
		tmpJSON0, err := json.Marshal(example)
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = secretsmanager.NewSecretVersion(ctx, "example", &secretsmanager.SecretVersionArgs{
			SecretId:     pulumi.Any(exampleAwsSecretsmanagerSecret.Id),
			SecretString: pulumi.String(json0),
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
import com.pulumi.aws.secretsmanager.SecretVersion;
import com.pulumi.aws.secretsmanager.SecretVersionArgs;
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
        final var config = ctx.config();
        final var example = config.get("example").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        var exampleSecretVersion = new SecretVersion("exampleSecretVersion", SecretVersionArgs.builder()
            .secretId(exampleAwsSecretsmanagerSecret.id())
            .secretString(serializeJson(
                example))
            .build());

    }
}
```
```yaml
configuration:
  # The map here can come from other supported configurations
  # like locals, resource attribute, map() built-in, etc.
  example:
    type: map(string)
    default:
      key1: value1
      key2: value2
resources:
  exampleSecretVersion:
    type: aws:secretsmanager:SecretVersion
    name: example
    properties:
      secretId: ${exampleAwsSecretsmanagerSecret.id}
      secretString:
        fn::toJSON: ${example}
```
<!--End PulumiCodeChooser -->


Reading key-value pairs from JSON back into a native map

## Import

Using `pulumi import`, import `aws_secretsmanager_secret_version` using the secret ID and version ID. For example:

```sh
$ pulumi import aws:secretsmanager/secretVersion:SecretVersion example 'arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456|xxxxx-xxxxxxx-xxxxxxx-xxxxx'
```
æ
secretBinaryB" ßSpecifies binary data that you want to encrypt and store in this version of the secret. This is required if `secret_string` is not set. Needs to be encoded to base64.
«
secretId" ∂Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
ù
secretStringB" ÜSpecifies text data that you want to encrypt and store in this version of the secret. This is required if `secret_binary` is not set.
â
versionStagesB*" ÔSpecifies a list of staging labels that are attached to this version of the secret. A staging label must be unique to a single version of the secret. If you specify a staging label that's already associated with a different version of the same secret then that staging label is automatically removed from the other version and attached to this version. If you do not specify a value, then AWS Secrets Manager automatically moves the staging label `AWSCURRENT` to this new version on creation.

> **NOTE:** If `version_stages` is configured, you must include the `AWSCURRENT` staging label if this secret version is the only version or if the label is currently present on this secret version, otherwise this provider will show a perpetual difference.
""
arn" The ARN of the secret.
"æ
secretBinaryB" ßSpecifies binary data that you want to encrypt and store in this version of the secret. This is required if `secret_string` is not set. Needs to be encoded to base64.
"«
secretId" ∂Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
"ù
secretStringB" ÜSpecifies text data that you want to encrypt and store in this version of the secret. This is required if `secret_binary` is not set.
"E
	versionId" 4The unique identifier of the version of the secret.
"á
versionStages*" ÔSpecifies a list of staging labels that are attached to this version of the secret. A staging label must be unique to a single version of the secret. If you specify a staging label that's already associated with a different version of the same secret then that staging label is automatically removed from the other version and attached to this version. If you do not specify a value, then AWS Secrets Manager automatically moves the staging label `AWSCURRENT` to this new version on creation.

> **NOTE:** If `version_stages` is configured, you must include the `AWSCURRENT` staging label if this secret version is the only version or if the label is currently present on this secret version, otherwise this provider will show a perpetual difference.
*â 
7
securityhubAccountaws:securityhub/account:AccountÒEnables Security Hub for this AWS account.

> **NOTE:** Destroying this resource will disable Security Hub for this AWS account.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securityhub.NewAccount(ctx, "example", nil)
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
import com.pulumi.aws.securityhub.Account;
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
        var example = new Account("example");

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import an existing Security Hub enabled account using the AWS account ID. For example:

```sh
$ pulumi import aws:securityhub/account:Account example 123456789012
```
á
autoEnableControlsB
 ÍWhether to automatically enable new controls when they are added to standards that are enabled. By default, this is set to true, and new controls are enabled automatically. To not automatically enable new controls, set this to false.
Ω
controlFindingGeneratorB" õUpdates whether the calling account has consolidated control findings turned on. If the value for this field is set to `SECURITY_CONTROL`, Security Hub generates a single finding for a control check even when the check applies to multiple enabled standards. If the value for this field is set to `STANDARD_CONTROL`, Security Hub generates separate findings for a control check when the check applies to multiple enabled standards. For accounts that are part of an organization, this value can only be updated in the administrator account.
Ç
enableDefaultStandardsB
 ·Whether to enable the security standards that Security Hub has designated as automatically enabled including: ` AWS Foundational Security Best Practices v1.0.0` and `CIS AWS Foundations Benchmark v1.2.0`. Defaults to `true`.
">
arn" 3ARN of the SecurityHub Hub created in the account.
"á
autoEnableControlsB
 ÍWhether to automatically enable new controls when they are added to standards that are enabled. By default, this is set to true, and new controls are enabled automatically. To not automatically enable new controls, set this to false.
"ª
controlFindingGenerator" õUpdates whether the calling account has consolidated control findings turned on. If the value for this field is set to `SECURITY_CONTROL`, Security Hub generates a single finding for a control check even when the check applies to multiple enabled standards. If the value for this field is set to `STANDARD_CONTROL`, Security Hub generates separate findings for a control check when the check applies to multiple enabled standards. For accounts that are part of an organization, this value can only be updated in the administrator account.
"Ç
enableDefaultStandardsB
 ·Whether to enable the security standards that Security Hub has designated as automatically enabled including: ` AWS Foundational Security Best Practices v1.0.0` and `CIS AWS Foundations Benchmark v1.2.0`. Defaults to `true`.
*◊!
F
securityhubActionTarget)aws:securityhub/actionTarget:ActionTargetÿCreates Security Hub custom action.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleActionTarget = new aws.securityhub.ActionTarget("example", {
    name: "Send notification to chat",
    identifier: "SendToChat",
    description: "This is custom action sends selected findings to chat",
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_action_target = aws.securityhub.ActionTarget("example",
    name="Send notification to chat",
    identifier="SendToChat",
    description="This is custom action sends selected findings to chat",
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleActionTarget = new Aws.SecurityHub.ActionTarget("example", new()
    {
        Name = "Send notification to chat",
        Identifier = "SendToChat",
        Description = "This is custom action sends selected findings to chat",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewActionTarget(ctx, "example", &securityhub.ActionTargetArgs{
			Name:        pulumi.String("Send notification to chat"),
			Identifier:  pulumi.String("SendToChat"),
			Description: pulumi.String("This is custom action sends selected findings to chat"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.ActionTarget;
import com.pulumi.aws.securityhub.ActionTargetArgs;
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
        var example = new Account("example");

        var exampleActionTarget = new ActionTarget("exampleActionTarget", ActionTargetArgs.builder()
            .name("Send notification to chat")
            .identifier("SendToChat")
            .description("This is custom action sends selected findings to chat")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleActionTarget:
    type: aws:securityhub:ActionTarget
    name: example
    properties:
      name: Send notification to chat
      identifier: SendToChat
      description: This is custom action sends selected findings to chat
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Hub custom action using the action target ARN. For example:

```sh
$ pulumi import aws:securityhub/actionTarget:ActionTarget example arn:aws:securityhub:eu-west-1:312940875350:action/custom/a
```
9
description" &The name of the custom action target.
7

identifier" %The ID for the custom action target.
<
nameB" .The description for the custom action target.
"P
arn" EAmazon Resource Name (ARN) of the Security Hub custom action target.
"9
description" &The name of the custom action target.
"7

identifier" %The ID for the custom action target.
":
name" .The description for the custom action target.
*ì]
L
securityhubAutomationRule-aws:securityhub/automationRule:AutomationRuleèKResource for managing an AWS Security Hub Automation Rule.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.AutomationRule("example", {
    description: "Elevate finding severity to CRITICAL when specific resources such as an S3 bucket is at risk",
    ruleName: "Elevate severity of findings that relate to important resources",
    ruleOrder: 1,
    actions: [{
        findingFieldsUpdate: {
            severity: {
                label: "CRITICAL",
                product: 0,
            },
            note: {
                text: "This is a critical resource. Please review ASAP.",
                updatedBy: "sechub-automation",
            },
            types: ["Software and Configuration Checks/Industry and Regulatory Standards"],
            userDefinedFields: {
                key: "value",
            },
        },
        type: "FINDING_FIELDS_UPDATE",
    }],
    criteria: {
        resourceIds: [{
            comparison: "EQUALS",
            value: "arn:aws:s3:::examplebucket/*",
        }],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.AutomationRule("example",
    description="Elevate finding severity to CRITICAL when specific resources such as an S3 bucket is at risk",
    rule_name="Elevate severity of findings that relate to important resources",
    rule_order=1,
    actions=[{
        "finding_fields_update": {
            "severity": {
                "label": "CRITICAL",
                "product": 0,
            },
            "note": {
                "text": "This is a critical resource. Please review ASAP.",
                "updated_by": "sechub-automation",
            },
            "types": ["Software and Configuration Checks/Industry and Regulatory Standards"],
            "user_defined_fields": {
                "key": "value",
            },
        },
        "type": "FINDING_FIELDS_UPDATE",
    }],
    criteria={
        "resource_ids": [{
            "comparison": "EQUALS",
            "value": "arn:aws:s3:::examplebucket/*",
        }],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.AutomationRule("example", new()
    {
        Description = "Elevate finding severity to CRITICAL when specific resources such as an S3 bucket is at risk",
        RuleName = "Elevate severity of findings that relate to important resources",
        RuleOrder = 1,
        Actions = new[]
        {
            new Aws.SecurityHub.Inputs.AutomationRuleActionArgs
            {
                FindingFieldsUpdate = new Aws.SecurityHub.Inputs.AutomationRuleActionFindingFieldsUpdateArgs
                {
                    Severity = new Aws.SecurityHub.Inputs.AutomationRuleActionFindingFieldsUpdateSeverityArgs
                    {
                        Label = "CRITICAL",
                        Product = 0,
                    },
                    Note = new Aws.SecurityHub.Inputs.AutomationRuleActionFindingFieldsUpdateNoteArgs
                    {
                        Text = "This is a critical resource. Please review ASAP.",
                        UpdatedBy = "sechub-automation",
                    },
                    Types = new[]
                    {
                        "Software and Configuration Checks/Industry and Regulatory Standards",
                    },
                    UserDefinedFields = 
                    {
                        { "key", "value" },
                    },
                },
                Type = "FINDING_FIELDS_UPDATE",
            },
        },
        Criteria = new Aws.SecurityHub.Inputs.AutomationRuleCriteriaArgs
        {
            ResourceIds = new[]
            {
                new Aws.SecurityHub.Inputs.AutomationRuleCriteriaResourceIdArgs
                {
                    Comparison = "EQUALS",
                    Value = "arn:aws:s3:::examplebucket/*",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securityhub.NewAutomationRule(ctx, "example", &securityhub.AutomationRuleArgs{
			Description: pulumi.String("Elevate finding severity to CRITICAL when specific resources such as an S3 bucket is at risk"),
			RuleName:    pulumi.String("Elevate severity of findings that relate to important resources"),
			RuleOrder:   pulumi.Int(1),
			Actions: securityhub.AutomationRuleActionArray{
				&securityhub.AutomationRuleActionArgs{
					FindingFieldsUpdate: &securityhub.AutomationRuleActionFindingFieldsUpdateArgs{
						Severity: &securityhub.AutomationRuleActionFindingFieldsUpdateSeverityArgs{
							Label:   pulumi.String("CRITICAL"),
							Product: pulumi.Float64(0),
						},
						Note: &securityhub.AutomationRuleActionFindingFieldsUpdateNoteArgs{
							Text:      pulumi.String("This is a critical resource. Please review ASAP."),
							UpdatedBy: pulumi.String("sechub-automation"),
						},
						Types: pulumi.StringArray{
							pulumi.String("Software and Configuration Checks/Industry and Regulatory Standards"),
						},
						UserDefinedFields: pulumi.StringMap{
							"key": pulumi.String("value"),
						},
					},
					Type: pulumi.String("FINDING_FIELDS_UPDATE"),
				},
			},
			Criteria: &securityhub.AutomationRuleCriteriaArgs{
				ResourceIds: securityhub.AutomationRuleCriteriaResourceIdArray{
					&securityhub.AutomationRuleCriteriaResourceIdArgs{
						Comparison: pulumi.String("EQUALS"),
						Value:      pulumi.String("arn:aws:s3:::examplebucket/*"),
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
import com.pulumi.aws.securityhub.AutomationRule;
import com.pulumi.aws.securityhub.AutomationRuleArgs;
import com.pulumi.aws.securityhub.inputs.AutomationRuleActionArgs;
import com.pulumi.aws.securityhub.inputs.AutomationRuleActionFindingFieldsUpdateArgs;
import com.pulumi.aws.securityhub.inputs.AutomationRuleActionFindingFieldsUpdateSeverityArgs;
import com.pulumi.aws.securityhub.inputs.AutomationRuleActionFindingFieldsUpdateNoteArgs;
import com.pulumi.aws.securityhub.inputs.AutomationRuleCriteriaArgs;
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
        var example = new AutomationRule("example", AutomationRuleArgs.builder()
            .description("Elevate finding severity to CRITICAL when specific resources such as an S3 bucket is at risk")
            .ruleName("Elevate severity of findings that relate to important resources")
            .ruleOrder(1)
            .actions(AutomationRuleActionArgs.builder()
                .findingFieldsUpdate(AutomationRuleActionFindingFieldsUpdateArgs.builder()
                    .severity(AutomationRuleActionFindingFieldsUpdateSeverityArgs.builder()
                        .label("CRITICAL")
                        .product("0.0")
                        .build())
                    .note(AutomationRuleActionFindingFieldsUpdateNoteArgs.builder()
                        .text("This is a critical resource. Please review ASAP.")
                        .updatedBy("sechub-automation")
                        .build())
                    .types("Software and Configuration Checks/Industry and Regulatory Standards")
                    .userDefinedFields(Map.of("key", "value"))
                    .build())
                .type("FINDING_FIELDS_UPDATE")
                .build())
            .criteria(AutomationRuleCriteriaArgs.builder()
                .resourceIds(AutomationRuleCriteriaResourceIdArgs.builder()
                    .comparison("EQUALS")
                    .value("arn:aws:s3:::examplebucket/*")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:AutomationRule
    properties:
      description: Elevate finding severity to CRITICAL when specific resources such as an S3 bucket is at risk
      ruleName: Elevate severity of findings that relate to important resources
      ruleOrder: 1
      actions:
        - findingFieldsUpdate:
            severity:
              label: CRITICAL
              product: '0.0'
            note:
              text: This is a critical resource. Please review ASAP.
              updatedBy: sechub-automation
            types:
              - Software and Configuration Checks/Industry and Regulatory Standards
            userDefinedFields:
              key: value
          type: FINDING_FIELDS_UPDATE
      criteria:
        resourceIds:
          - comparison: EQUALS
            value: arn:aws:s3:::examplebucket/*
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Hub automation rule using their ARN. For example:

```sh
$ pulumi import aws:securityhub/automationRule:AutomationRule example arn:aws:securityhub:us-west-2:123456789012:automation-rule/473eddde-f5c4-4ae5-85c7-e922f271fffc
```
á
actionsfBd*b:`
^
securityhubAutomationRuleAction9aws:securityhub/AutomationRuleAction:AutomationRuleActionìA block that specifies one or more actions to update finding fields if a finding matches the conditions specified in `Criteria`. Documented below.
ñ
criteriajBh:f
d
securityhubAutomationRuleCriteria=aws:securityhub/AutomationRuleCriteria:AutomationRuleCriteriaùA block that specifies a set of ASFF finding field attributes and corresponding expected values that Security Hub uses to filter findings. Documented below.
0
description" The description of the rule.
ó

isTerminalB
 ÇSpecifies whether a rule is the last to be applied with respect to a finding that matches the rule criteria. Defaults to `false`.
&
ruleName" The name of the rule.
∆
	ruleOrder ¥An integer ranging from 1 to 1000 that represents the order in which the rule action is applied to findings. Security Hub applies rules with lower values for this parameter first.
D

ruleStatusB" 0Whether the rule is active after it is created.

tagsB2" "á
actionsfBd*b:`
^
securityhubAutomationRuleAction9aws:securityhub/AutomationRuleAction:AutomationRuleActionìA block that specifies one or more actions to update finding fields if a finding matches the conditions specified in `Criteria`. Documented below.
"8
arn" -The ARN of the Security Hub automation rule.
"ñ
criteriajBh:f
d
securityhubAutomationRuleCriteria=aws:securityhub/AutomationRuleCriteria:AutomationRuleCriteriaùA block that specifies a set of ASFF finding field attributes and corresponding expected values that Security Hub uses to filter findings. Documented below.
"0
description" The description of the rule.
"ï

isTerminal
 ÇSpecifies whether a rule is the last to be applied with respect to a finding that matches the rule criteria. Defaults to `false`.
"&
ruleName" The name of the rule.
"∆
	ruleOrder ¥An integer ranging from 1 to 1000 that represents the order in which the rule action is applied to findings. Security Hub applies rules with lower values for this parameter first.
"B

ruleStatus" 0Whether the rule is active after it is created.
"
tagsB2" "
tagsAll2" *öÙ
[
securityhubConfigurationPolicy7aws:securityhub/configurationPolicy:ConfigurationPolicyËÌManages Security Hub configuration policy

> **NOTE:** This resource requires `aws.securityhub.OrganizationConfiguration` to be configured of type `CENTRAL`. More information about Security Hub central configuration and configuration policies can be found in the [How Security Hub configuration policies work](https://docs.aws.amazon.com/securityhub/latest/userguide/configuration-policies-overview.html) documentation.

## Example Usage

### Default standards enabled

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.FindingAggregator("example", {linkingMode: "ALL_REGIONS"});
const exampleOrganizationConfiguration = new aws.securityhub.OrganizationConfiguration("example", {
    autoEnable: false,
    autoEnableStandards: "NONE",
    organizationConfiguration: {
        configurationType: "CENTRAL",
    },
}, {
    dependsOn: [example],
});
const exampleConfigurationPolicy = new aws.securityhub.ConfigurationPolicy("example", {
    name: "Example",
    description: "This is an example configuration policy",
    configurationPolicy: {
        serviceEnabled: true,
        enabledStandardArns: [
            "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
            "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
        ],
        securityControlsConfiguration: {
            disabledControlIdentifiers: [],
        },
    },
}, {
    dependsOn: [exampleOrganizationConfiguration],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.FindingAggregator("example", linking_mode="ALL_REGIONS")
example_organization_configuration = aws.securityhub.OrganizationConfiguration("example",
    auto_enable=False,
    auto_enable_standards="NONE",
    organization_configuration={
        "configuration_type": "CENTRAL",
    },
    opts = pulumi.ResourceOptions(depends_on=[example]))
example_configuration_policy = aws.securityhub.ConfigurationPolicy("example",
    name="Example",
    description="This is an example configuration policy",
    configuration_policy={
        "service_enabled": True,
        "enabled_standard_arns": [
            "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
            "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
        ],
        "security_controls_configuration": {
            "disabled_control_identifiers": [],
        },
    },
    opts = pulumi.ResourceOptions(depends_on=[example_organization_configuration]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.FindingAggregator("example", new()
    {
        LinkingMode = "ALL_REGIONS",
    });

    var exampleOrganizationConfiguration = new Aws.SecurityHub.OrganizationConfiguration("example", new()
    {
        AutoEnable = false,
        AutoEnableStandards = "NONE",
        OrganizationConfigurationDetails = new Aws.SecurityHub.Inputs.OrganizationConfigurationOrganizationConfigurationArgs
        {
            ConfigurationType = "CENTRAL",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

    var exampleConfigurationPolicy = new Aws.SecurityHub.ConfigurationPolicy("example", new()
    {
        Name = "Example",
        Description = "This is an example configuration policy",
        ConfigurationPolicyDetails = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicyArgs
        {
            ServiceEnabled = true,
            EnabledStandardArns = new[]
            {
                "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
                "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
            },
            SecurityControlsConfiguration = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs
            {
                DisabledControlIdentifiers = new() { },
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleOrganizationConfiguration,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewFindingAggregator(ctx, "example", &securityhub.FindingAggregatorArgs{
			LinkingMode: pulumi.String("ALL_REGIONS"),
		})
		if err != nil {
			return err
		}
		exampleOrganizationConfiguration, err := securityhub.NewOrganizationConfiguration(ctx, "example", &securityhub.OrganizationConfigurationArgs{
			AutoEnable:          pulumi.Bool(false),
			AutoEnableStandards: pulumi.String("NONE"),
			OrganizationConfiguration: &securityhub.OrganizationConfigurationOrganizationConfigurationArgs{
				ConfigurationType: pulumi.String("CENTRAL"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
		}))
		if err != nil {
			return err
		}
		_, err = securityhub.NewConfigurationPolicy(ctx, "example", &securityhub.ConfigurationPolicyArgs{
			Name:        pulumi.String("Example"),
			Description: pulumi.String("This is an example configuration policy"),
			ConfigurationPolicy: &securityhub.ConfigurationPolicyConfigurationPolicyArgs{
				ServiceEnabled: pulumi.Bool(true),
				EnabledStandardArns: pulumi.StringArray{
					pulumi.String("arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0"),
					pulumi.String("arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0"),
				},
				SecurityControlsConfiguration: &securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs{
					DisabledControlIdentifiers: pulumi.StringArray{},
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleOrganizationConfiguration,
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
import com.pulumi.aws.securityhub.FindingAggregator;
import com.pulumi.aws.securityhub.FindingAggregatorArgs;
import com.pulumi.aws.securityhub.OrganizationConfiguration;
import com.pulumi.aws.securityhub.OrganizationConfigurationArgs;
import com.pulumi.aws.securityhub.inputs.OrganizationConfigurationOrganizationConfigurationArgs;
import com.pulumi.aws.securityhub.ConfigurationPolicy;
import com.pulumi.aws.securityhub.ConfigurationPolicyArgs;
import com.pulumi.aws.securityhub.inputs.ConfigurationPolicyConfigurationPolicyArgs;
import com.pulumi.aws.securityhub.inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs;
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
        var example = new FindingAggregator("example", FindingAggregatorArgs.builder()
            .linkingMode("ALL_REGIONS")
            .build());

        var exampleOrganizationConfiguration = new OrganizationConfiguration("exampleOrganizationConfiguration", OrganizationConfigurationArgs.builder()
            .autoEnable(false)
            .autoEnableStandards("NONE")
            .organizationConfiguration(OrganizationConfigurationOrganizationConfigurationArgs.builder()
                .configurationType("CENTRAL")
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

        var exampleConfigurationPolicy = new ConfigurationPolicy("exampleConfigurationPolicy", ConfigurationPolicyArgs.builder()
            .name("Example")
            .description("This is an example configuration policy")
            .configurationPolicy(ConfigurationPolicyConfigurationPolicyArgs.builder()
                .serviceEnabled(true)
                .enabledStandardArns(                
                    "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
                    "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0")
                .securityControlsConfiguration(ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs.builder()
                    .disabledControlIdentifiers()
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleOrganizationConfiguration)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:FindingAggregator
    properties:
      linkingMode: ALL_REGIONS
  exampleOrganizationConfiguration:
    type: aws:securityhub:OrganizationConfiguration
    name: example
    properties:
      autoEnable: false
      autoEnableStandards: NONE
      organizationConfiguration:
        configurationType: CENTRAL
    options:
      dependsOn:
        - ${example}
  exampleConfigurationPolicy:
    type: aws:securityhub:ConfigurationPolicy
    name: example
    properties:
      name: Example
      description: This is an example configuration policy
      configurationPolicy:
        serviceEnabled: true
        enabledStandardArns:
          - arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0
          - arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0
        securityControlsConfiguration:
          disabledControlIdentifiers: []
    options:
      dependsOn:
        - ${exampleOrganizationConfiguration}
```
<!--End PulumiCodeChooser -->

### Disabled Policy

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const disabled = new aws.securityhub.ConfigurationPolicy("disabled", {
    name: "Disabled",
    description: "This is an example of disabled configuration policy",
    configurationPolicy: {
        serviceEnabled: false,
    },
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

disabled = aws.securityhub.ConfigurationPolicy("disabled",
    name="Disabled",
    description="This is an example of disabled configuration policy",
    configuration_policy={
        "service_enabled": False,
    },
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var disabled = new Aws.SecurityHub.ConfigurationPolicy("disabled", new()
    {
        Name = "Disabled",
        Description = "This is an example of disabled configuration policy",
        ConfigurationPolicyDetails = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicyArgs
        {
            ServiceEnabled = false,
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securityhub.NewConfigurationPolicy(ctx, "disabled", &securityhub.ConfigurationPolicyArgs{
			Name:        pulumi.String("Disabled"),
			Description: pulumi.String("This is an example of disabled configuration policy"),
			ConfigurationPolicy: &securityhub.ConfigurationPolicyConfigurationPolicyArgs{
				ServiceEnabled: pulumi.Bool(false),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.ConfigurationPolicy;
import com.pulumi.aws.securityhub.ConfigurationPolicyArgs;
import com.pulumi.aws.securityhub.inputs.ConfigurationPolicyConfigurationPolicyArgs;
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
        var disabled = new ConfigurationPolicy("disabled", ConfigurationPolicyArgs.builder()
            .name("Disabled")
            .description("This is an example of disabled configuration policy")
            .configurationPolicy(ConfigurationPolicyConfigurationPolicyArgs.builder()
                .serviceEnabled(false)
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  disabled:
    type: aws:securityhub:ConfigurationPolicy
    properties:
      name: Disabled
      description: This is an example of disabled configuration policy
      configurationPolicy:
        serviceEnabled: false
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

### Custom Control Configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const disabled = new aws.securityhub.ConfigurationPolicy("disabled", {
    name: "Custom Controls",
    description: "This is an example of configuration policy with custom control settings",
    configurationPolicy: {
        serviceEnabled: true,
        enabledStandardArns: [
            "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
            "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
        ],
        securityControlsConfiguration: {
            enabledControlIdentifiers: [
                "APIGateway.1",
                "IAM.7",
            ],
            securityControlCustomParameters: [
                {
                    securityControlId: "APIGateway.1",
                    parameters: [{
                        name: "loggingLevel",
                        valueType: "CUSTOM",
                        "enum": {
                            value: "INFO",
                        },
                    }],
                },
                {
                    securityControlId: "IAM.7",
                    parameters: [
                        {
                            name: "RequireLowercaseCharacters",
                            valueType: "CUSTOM",
                            bool: {
                                value: false,
                            },
                        },
                        {
                            name: "MaxPasswordAge",
                            valueType: "CUSTOM",
                            int: {
                                value: 60,
                            },
                        },
                    ],
                },
            ],
        },
    },
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

disabled = aws.securityhub.ConfigurationPolicy("disabled",
    name="Custom Controls",
    description="This is an example of configuration policy with custom control settings",
    configuration_policy={
        "service_enabled": True,
        "enabled_standard_arns": [
            "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
            "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
        ],
        "security_controls_configuration": {
            "enabled_control_identifiers": [
                "APIGateway.1",
                "IAM.7",
            ],
            "security_control_custom_parameters": [
                {
                    "security_control_id": "APIGateway.1",
                    "parameters": [{
                        "name": "loggingLevel",
                        "value_type": "CUSTOM",
                        "enum": {
                            "value": "INFO",
                        },
                    }],
                },
                {
                    "security_control_id": "IAM.7",
                    "parameters": [
                        {
                            "name": "RequireLowercaseCharacters",
                            "value_type": "CUSTOM",
                            "bool": {
                                "value": False,
                            },
                        },
                        {
                            "name": "MaxPasswordAge",
                            "value_type": "CUSTOM",
                            "int": {
                                "value": 60,
                            },
                        },
                    ],
                },
            ],
        },
    },
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var disabled = new Aws.SecurityHub.ConfigurationPolicy("disabled", new()
    {
        Name = "Custom Controls",
        Description = "This is an example of configuration policy with custom control settings",
        ConfigurationPolicyDetails = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicyArgs
        {
            ServiceEnabled = true,
            EnabledStandardArns = new[]
            {
                "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
                "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
            },
            SecurityControlsConfiguration = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs
            {
                EnabledControlIdentifiers = new[]
                {
                    "APIGateway.1",
                    "IAM.7",
                },
                SecurityControlCustomParameters = new[]
                {
                    new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterArgs
                    {
                        SecurityControlId = "APIGateway.1",
                        Parameters = new[]
                        {
                            new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArgs
                            {
                                Name = "loggingLevel",
                                ValueType = "CUSTOM",
                                Enum = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumArgs
                                {
                                    Value = "INFO",
                                },
                            },
                        },
                    },
                    new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterArgs
                    {
                        SecurityControlId = "IAM.7",
                        Parameters = new[]
                        {
                            new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArgs
                            {
                                Name = "RequireLowercaseCharacters",
                                ValueType = "CUSTOM",
                                Bool = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBoolArgs
                                {
                                    Value = false,
                                },
                            },
                            new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArgs
                            {
                                Name = "MaxPasswordAge",
                                ValueType = "CUSTOM",
                                Int = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntArgs
                                {
                                    Value = 60,
                                },
                            },
                        },
                    },
                },
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securityhub.NewConfigurationPolicy(ctx, "disabled", &securityhub.ConfigurationPolicyArgs{
			Name:        pulumi.String("Custom Controls"),
			Description: pulumi.String("This is an example of configuration policy with custom control settings"),
			ConfigurationPolicy: &securityhub.ConfigurationPolicyConfigurationPolicyArgs{
				ServiceEnabled: pulumi.Bool(true),
				EnabledStandardArns: pulumi.StringArray{
					pulumi.String("arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0"),
					pulumi.String("arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0"),
				},
				SecurityControlsConfiguration: &securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs{
					EnabledControlIdentifiers: pulumi.StringArray{
						pulumi.String("APIGateway.1"),
						pulumi.String("IAM.7"),
					},
					SecurityControlCustomParameters: securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterArray{
						&securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterArgs{
							SecurityControlId: pulumi.String("APIGateway.1"),
							Parameters: securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArray{
								&securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArgs{
									Name:      pulumi.String("loggingLevel"),
									ValueType: pulumi.String("CUSTOM"),
									Enum: &securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumArgs{
										Value: pulumi.String("INFO"),
									},
								},
							},
						},
						&securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterArgs{
							SecurityControlId: pulumi.String("IAM.7"),
							Parameters: securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArray{
								&securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArgs{
									Name:      pulumi.String("RequireLowercaseCharacters"),
									ValueType: pulumi.String("CUSTOM"),
									Bool: &securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBoolArgs{
										Value: pulumi.Bool(false),
									},
								},
								&securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArgs{
									Name:      pulumi.String("MaxPasswordAge"),
									ValueType: pulumi.String("CUSTOM"),
									Int: &securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntArgs{
										Value: pulumi.Int(60),
									},
								},
							},
						},
					},
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.ConfigurationPolicy;
import com.pulumi.aws.securityhub.ConfigurationPolicyArgs;
import com.pulumi.aws.securityhub.inputs.ConfigurationPolicyConfigurationPolicyArgs;
import com.pulumi.aws.securityhub.inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs;
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
        var disabled = new ConfigurationPolicy("disabled", ConfigurationPolicyArgs.builder()
            .name("Custom Controls")
            .description("This is an example of configuration policy with custom control settings")
            .configurationPolicy(ConfigurationPolicyConfigurationPolicyArgs.builder()
                .serviceEnabled(true)
                .enabledStandardArns(                
                    "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
                    "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0")
                .securityControlsConfiguration(ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs.builder()
                    .enabledControlIdentifiers(                    
                        "APIGateway.1",
                        "IAM.7")
                    .securityControlCustomParameters(                    
                        ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterArgs.builder()
                            .securityControlId("APIGateway.1")
                            .parameters(ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArgs.builder()
                                .name("loggingLevel")
                                .valueType("CUSTOM")
                                .enum_(ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumArgs.builder()
                                    .value("INFO")
                                    .build())
                                .build())
                            .build(),
                        ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterArgs.builder()
                            .securityControlId("IAM.7")
                            .parameters(                            
                                ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArgs.builder()
                                    .name("RequireLowercaseCharacters")
                                    .valueType("CUSTOM")
                                    .bool(ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBoolArgs.builder()
                                        .value(false)
                                        .build())
                                    .build(),
                                ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterArgs.builder()
                                    .name("MaxPasswordAge")
                                    .valueType("CUSTOM")
                                    .int_(ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntArgs.builder()
                                        .value(60)
                                        .build())
                                    .build())
                            .build())
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  disabled:
    type: aws:securityhub:ConfigurationPolicy
    properties:
      name: Custom Controls
      description: This is an example of configuration policy with custom control settings
      configurationPolicy:
        serviceEnabled: true
        enabledStandardArns:
          - arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0
          - arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0
        securityControlsConfiguration:
          enabledControlIdentifiers:
            - APIGateway.1
            - IAM.7
          securityControlCustomParameters:
            - securityControlId: APIGateway.1
              parameters:
                - name: loggingLevel
                  valueType: CUSTOM
                  enum:
                    value: INFO
            - securityControlId: IAM.7
              parameters:
                - name: RequireLowercaseCharacters
                  valueType: CUSTOM
                  bool:
                    value: false
                - name: MaxPasswordAge
                  valueType: CUSTOM
                  int:
                    value: 60
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import an existing Security Hub enabled account using the universally unique identifier (UUID) of the policy. For example:

```sh
$ pulumi import aws:securityhub/configurationPolicy:ConfigurationPolicy example "00000000-1111-2222-3333-444444444444"
```
Á
configurationPolicyö:ó
î
securityhub&ConfigurationPolicyConfigurationPolicy]aws:securityhub/ConfigurationPolicyConfigurationPolicy:ConfigurationPolicyConfigurationPolicy3Defines how Security Hub is configured. See below.
B
descriptionB" -The description of the configuration policy.
4
nameB" &The name of the configuration policy.
"	
arn" "Á
configurationPolicyö:ó
î
securityhub&ConfigurationPolicyConfigurationPolicy]aws:securityhub/ConfigurationPolicyConfigurationPolicy:ConfigurationPolicyConfigurationPolicy3Defines how Security Hub is configured. See below.
"B
descriptionB" -The description of the configuration policy.
"2
name" &The name of the configuration policy.
*¢r
|
securityhubConfigurationPolicyAssociationMaws:securityhub/configurationPolicyAssociation:ConfigurationPolicyAssociationﬂmManages Security Hub configuration policy associations.

> **NOTE:** This resource requires `aws.securityhub.OrganizationConfiguration` to be configured with type `CENTRAL`. More information about Security Hub central configuration and configuration policies can be found in the [How Security Hub configuration policies work](https://docs.aws.amazon.com/securityhub/latest/userguide/configuration-policies-overview.html) documentation.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.FindingAggregator("example", {linkingMode: "ALL_REGIONS"});
const exampleOrganizationConfiguration = new aws.securityhub.OrganizationConfiguration("example", {
    autoEnable: false,
    autoEnableStandards: "NONE",
    organizationConfiguration: {
        configurationType: "CENTRAL",
    },
}, {
    dependsOn: [example],
});
const exampleConfigurationPolicy = new aws.securityhub.ConfigurationPolicy("example", {
    name: "Example",
    description: "This is an example configuration policy",
    configurationPolicy: {
        serviceEnabled: true,
        enabledStandardArns: [
            "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
            "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
        ],
        securityControlsConfiguration: {
            disabledControlIdentifiers: [],
        },
    },
}, {
    dependsOn: [exampleOrganizationConfiguration],
});
const accountExample = new aws.securityhub.ConfigurationPolicyAssociation("account_example", {
    targetId: "123456789012",
    policyId: exampleConfigurationPolicy.id,
});
const rootExample = new aws.securityhub.ConfigurationPolicyAssociation("root_example", {
    targetId: "r-abcd",
    policyId: exampleConfigurationPolicy.id,
});
const ouExample = new aws.securityhub.ConfigurationPolicyAssociation("ou_example", {
    targetId: "ou-abcd-12345678",
    policyId: exampleConfigurationPolicy.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.FindingAggregator("example", linking_mode="ALL_REGIONS")
example_organization_configuration = aws.securityhub.OrganizationConfiguration("example",
    auto_enable=False,
    auto_enable_standards="NONE",
    organization_configuration={
        "configuration_type": "CENTRAL",
    },
    opts = pulumi.ResourceOptions(depends_on=[example]))
example_configuration_policy = aws.securityhub.ConfigurationPolicy("example",
    name="Example",
    description="This is an example configuration policy",
    configuration_policy={
        "service_enabled": True,
        "enabled_standard_arns": [
            "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
            "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
        ],
        "security_controls_configuration": {
            "disabled_control_identifiers": [],
        },
    },
    opts = pulumi.ResourceOptions(depends_on=[example_organization_configuration]))
account_example = aws.securityhub.ConfigurationPolicyAssociation("account_example",
    target_id="123456789012",
    policy_id=example_configuration_policy.id)
root_example = aws.securityhub.ConfigurationPolicyAssociation("root_example",
    target_id="r-abcd",
    policy_id=example_configuration_policy.id)
ou_example = aws.securityhub.ConfigurationPolicyAssociation("ou_example",
    target_id="ou-abcd-12345678",
    policy_id=example_configuration_policy.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.FindingAggregator("example", new()
    {
        LinkingMode = "ALL_REGIONS",
    });

    var exampleOrganizationConfiguration = new Aws.SecurityHub.OrganizationConfiguration("example", new()
    {
        AutoEnable = false,
        AutoEnableStandards = "NONE",
        OrganizationConfigurationDetails = new Aws.SecurityHub.Inputs.OrganizationConfigurationOrganizationConfigurationArgs
        {
            ConfigurationType = "CENTRAL",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

    var exampleConfigurationPolicy = new Aws.SecurityHub.ConfigurationPolicy("example", new()
    {
        Name = "Example",
        Description = "This is an example configuration policy",
        ConfigurationPolicyDetails = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicyArgs
        {
            ServiceEnabled = true,
            EnabledStandardArns = new[]
            {
                "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
                "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
            },
            SecurityControlsConfiguration = new Aws.SecurityHub.Inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs
            {
                DisabledControlIdentifiers = new() { },
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleOrganizationConfiguration,
        },
    });

    var accountExample = new Aws.SecurityHub.ConfigurationPolicyAssociation("account_example", new()
    {
        TargetId = "123456789012",
        PolicyId = exampleConfigurationPolicy.Id,
    });

    var rootExample = new Aws.SecurityHub.ConfigurationPolicyAssociation("root_example", new()
    {
        TargetId = "r-abcd",
        PolicyId = exampleConfigurationPolicy.Id,
    });

    var ouExample = new Aws.SecurityHub.ConfigurationPolicyAssociation("ou_example", new()
    {
        TargetId = "ou-abcd-12345678",
        PolicyId = exampleConfigurationPolicy.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewFindingAggregator(ctx, "example", &securityhub.FindingAggregatorArgs{
			LinkingMode: pulumi.String("ALL_REGIONS"),
		})
		if err != nil {
			return err
		}
		exampleOrganizationConfiguration, err := securityhub.NewOrganizationConfiguration(ctx, "example", &securityhub.OrganizationConfigurationArgs{
			AutoEnable:          pulumi.Bool(false),
			AutoEnableStandards: pulumi.String("NONE"),
			OrganizationConfiguration: &securityhub.OrganizationConfigurationOrganizationConfigurationArgs{
				ConfigurationType: pulumi.String("CENTRAL"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
		}))
		if err != nil {
			return err
		}
		exampleConfigurationPolicy, err := securityhub.NewConfigurationPolicy(ctx, "example", &securityhub.ConfigurationPolicyArgs{
			Name:        pulumi.String("Example"),
			Description: pulumi.String("This is an example configuration policy"),
			ConfigurationPolicy: &securityhub.ConfigurationPolicyConfigurationPolicyArgs{
				ServiceEnabled: pulumi.Bool(true),
				EnabledStandardArns: pulumi.StringArray{
					pulumi.String("arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0"),
					pulumi.String("arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0"),
				},
				SecurityControlsConfiguration: &securityhub.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs{
					DisabledControlIdentifiers: pulumi.StringArray{},
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleOrganizationConfiguration,
		}))
		if err != nil {
			return err
		}
		_, err = securityhub.NewConfigurationPolicyAssociation(ctx, "account_example", &securityhub.ConfigurationPolicyAssociationArgs{
			TargetId: pulumi.String("123456789012"),
			PolicyId: exampleConfigurationPolicy.ID(),
		})
		if err != nil {
			return err
		}
		_, err = securityhub.NewConfigurationPolicyAssociation(ctx, "root_example", &securityhub.ConfigurationPolicyAssociationArgs{
			TargetId: pulumi.String("r-abcd"),
			PolicyId: exampleConfigurationPolicy.ID(),
		})
		if err != nil {
			return err
		}
		_, err = securityhub.NewConfigurationPolicyAssociation(ctx, "ou_example", &securityhub.ConfigurationPolicyAssociationArgs{
			TargetId: pulumi.String("ou-abcd-12345678"),
			PolicyId: exampleConfigurationPolicy.ID(),
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
import com.pulumi.aws.securityhub.FindingAggregator;
import com.pulumi.aws.securityhub.FindingAggregatorArgs;
import com.pulumi.aws.securityhub.OrganizationConfiguration;
import com.pulumi.aws.securityhub.OrganizationConfigurationArgs;
import com.pulumi.aws.securityhub.inputs.OrganizationConfigurationOrganizationConfigurationArgs;
import com.pulumi.aws.securityhub.ConfigurationPolicy;
import com.pulumi.aws.securityhub.ConfigurationPolicyArgs;
import com.pulumi.aws.securityhub.inputs.ConfigurationPolicyConfigurationPolicyArgs;
import com.pulumi.aws.securityhub.inputs.ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs;
import com.pulumi.aws.securityhub.ConfigurationPolicyAssociation;
import com.pulumi.aws.securityhub.ConfigurationPolicyAssociationArgs;
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
        var example = new FindingAggregator("example", FindingAggregatorArgs.builder()
            .linkingMode("ALL_REGIONS")
            .build());

        var exampleOrganizationConfiguration = new OrganizationConfiguration("exampleOrganizationConfiguration", OrganizationConfigurationArgs.builder()
            .autoEnable(false)
            .autoEnableStandards("NONE")
            .organizationConfiguration(OrganizationConfigurationOrganizationConfigurationArgs.builder()
                .configurationType("CENTRAL")
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

        var exampleConfigurationPolicy = new ConfigurationPolicy("exampleConfigurationPolicy", ConfigurationPolicyArgs.builder()
            .name("Example")
            .description("This is an example configuration policy")
            .configurationPolicy(ConfigurationPolicyConfigurationPolicyArgs.builder()
                .serviceEnabled(true)
                .enabledStandardArns(                
                    "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
                    "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0")
                .securityControlsConfiguration(ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationArgs.builder()
                    .disabledControlIdentifiers()
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleOrganizationConfiguration)
                .build());

        var accountExample = new ConfigurationPolicyAssociation("accountExample", ConfigurationPolicyAssociationArgs.builder()
            .targetId("123456789012")
            .policyId(exampleConfigurationPolicy.id())
            .build());

        var rootExample = new ConfigurationPolicyAssociation("rootExample", ConfigurationPolicyAssociationArgs.builder()
            .targetId("r-abcd")
            .policyId(exampleConfigurationPolicy.id())
            .build());

        var ouExample = new ConfigurationPolicyAssociation("ouExample", ConfigurationPolicyAssociationArgs.builder()
            .targetId("ou-abcd-12345678")
            .policyId(exampleConfigurationPolicy.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:FindingAggregator
    properties:
      linkingMode: ALL_REGIONS
  exampleOrganizationConfiguration:
    type: aws:securityhub:OrganizationConfiguration
    name: example
    properties:
      autoEnable: false
      autoEnableStandards: NONE
      organizationConfiguration:
        configurationType: CENTRAL
    options:
      dependsOn:
        - ${example}
  exampleConfigurationPolicy:
    type: aws:securityhub:ConfigurationPolicy
    name: example
    properties:
      name: Example
      description: This is an example configuration policy
      configurationPolicy:
        serviceEnabled: true
        enabledStandardArns:
          - arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0
          - arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0
        securityControlsConfiguration:
          disabledControlIdentifiers: []
    options:
      dependsOn:
        - ${exampleOrganizationConfiguration}
  accountExample:
    type: aws:securityhub:ConfigurationPolicyAssociation
    name: account_example
    properties:
      targetId: '123456789012'
      policyId: ${exampleConfigurationPolicy.id}
  rootExample:
    type: aws:securityhub:ConfigurationPolicyAssociation
    name: root_example
    properties:
      targetId: r-abcd
      policyId: ${exampleConfigurationPolicy.id}
  ouExample:
    type: aws:securityhub:ConfigurationPolicyAssociation
    name: ou_example
    properties:
      targetId: ou-abcd-12345678
      policyId: ${exampleConfigurationPolicy.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import an existing Security Hub enabled account using the target id. For example:

```sh
$ pulumi import aws:securityhub/configurationPolicyAssociation:ConfigurationPolicyAssociation example_account_association 123456789012
```
V
policyId" FThe universally unique identifier (UUID) of the configuration policy.
Ü
targetId" vThe identifier of the target account, organizational unit, or the root to associate with the specified configuration.
"V
policyId" FThe universally unique identifier (UUID) of the configuration policy.
"Ü
targetId" vThe identifier of the target account, organizational unit, or the root to associate with the specified configuration.
*é\
U
securityhubFindingAggregator3aws:securityhub/findingAggregator:FindingAggregator‹RManages a Security Hub finding aggregator. Security Hub needs to be enabled in a region in order for the aggregator to pull through findings.

## Example Usage

### All Regions Usage

The following example will enable the aggregator for every region.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleFindingAggregator = new aws.securityhub.FindingAggregator("example", {linkingMode: "ALL_REGIONS"}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_finding_aggregator = aws.securityhub.FindingAggregator("example", linking_mode="ALL_REGIONS",
opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleFindingAggregator = new Aws.SecurityHub.FindingAggregator("example", new()
    {
        LinkingMode = "ALL_REGIONS",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewFindingAggregator(ctx, "example", &securityhub.FindingAggregatorArgs{
			LinkingMode: pulumi.String("ALL_REGIONS"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.FindingAggregator;
import com.pulumi.aws.securityhub.FindingAggregatorArgs;
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
        var example = new Account("example");

        var exampleFindingAggregator = new FindingAggregator("exampleFindingAggregator", FindingAggregatorArgs.builder()
            .linkingMode("ALL_REGIONS")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleFindingAggregator:
    type: aws:securityhub:FindingAggregator
    name: example
    properties:
      linkingMode: ALL_REGIONS
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

### All Regions Except Specified Regions Usage

The following example will enable the aggregator for every region except those specified in `specified_regions`.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleFindingAggregator = new aws.securityhub.FindingAggregator("example", {
    linkingMode: "ALL_REGIONS_EXCEPT_SPECIFIED",
    specifiedRegions: [
        "eu-west-1",
        "eu-west-2",
    ],
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_finding_aggregator = aws.securityhub.FindingAggregator("example",
    linking_mode="ALL_REGIONS_EXCEPT_SPECIFIED",
    specified_regions=[
        "eu-west-1",
        "eu-west-2",
    ],
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleFindingAggregator = new Aws.SecurityHub.FindingAggregator("example", new()
    {
        LinkingMode = "ALL_REGIONS_EXCEPT_SPECIFIED",
        SpecifiedRegions = new[]
        {
            "eu-west-1",
            "eu-west-2",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewFindingAggregator(ctx, "example", &securityhub.FindingAggregatorArgs{
			LinkingMode: pulumi.String("ALL_REGIONS_EXCEPT_SPECIFIED"),
			SpecifiedRegions: pulumi.StringArray{
				pulumi.String("eu-west-1"),
				pulumi.String("eu-west-2"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.FindingAggregator;
import com.pulumi.aws.securityhub.FindingAggregatorArgs;
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
        var example = new Account("example");

        var exampleFindingAggregator = new FindingAggregator("exampleFindingAggregator", FindingAggregatorArgs.builder()
            .linkingMode("ALL_REGIONS_EXCEPT_SPECIFIED")
            .specifiedRegions(            
                "eu-west-1",
                "eu-west-2")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleFindingAggregator:
    type: aws:securityhub:FindingAggregator
    name: example
    properties:
      linkingMode: ALL_REGIONS_EXCEPT_SPECIFIED
      specifiedRegions:
        - eu-west-1
        - eu-west-2
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

### Specified Regions Usage

The following example will enable the aggregator for every region specified in `specified_regions`.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleFindingAggregator = new aws.securityhub.FindingAggregator("example", {
    linkingMode: "SPECIFIED_REGIONS",
    specifiedRegions: [
        "eu-west-1",
        "eu-west-2",
    ],
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_finding_aggregator = aws.securityhub.FindingAggregator("example",
    linking_mode="SPECIFIED_REGIONS",
    specified_regions=[
        "eu-west-1",
        "eu-west-2",
    ],
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleFindingAggregator = new Aws.SecurityHub.FindingAggregator("example", new()
    {
        LinkingMode = "SPECIFIED_REGIONS",
        SpecifiedRegions = new[]
        {
            "eu-west-1",
            "eu-west-2",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewFindingAggregator(ctx, "example", &securityhub.FindingAggregatorArgs{
			LinkingMode: pulumi.String("SPECIFIED_REGIONS"),
			SpecifiedRegions: pulumi.StringArray{
				pulumi.String("eu-west-1"),
				pulumi.String("eu-west-2"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.FindingAggregator;
import com.pulumi.aws.securityhub.FindingAggregatorArgs;
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
        var example = new Account("example");

        var exampleFindingAggregator = new FindingAggregator("exampleFindingAggregator", FindingAggregatorArgs.builder()
            .linkingMode("SPECIFIED_REGIONS")
            .specifiedRegions(            
                "eu-west-1",
                "eu-west-2")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleFindingAggregator:
    type: aws:securityhub:FindingAggregator
    name: example
    properties:
      linkingMode: SPECIFIED_REGIONS
      specifiedRegions:
        - eu-west-1
        - eu-west-2
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import an existing Security Hub finding aggregator using the `arn`. For example:

```sh
$ pulumi import aws:securityhub/findingAggregator:FindingAggregator example arn:aws:securityhub:eu-west-1:123456789098:finding-aggregator/abcd1234-abcd-1234-1234-abcdef123456
```
Ü
linkingMode" ÚIndicates whether to aggregate findings from all of the available Regions or from a specified list. The options are `ALL_REGIONS`, `ALL_REGIONS_EXCEPT_SPECIFIED` or `SPECIFIED_REGIONS`. When `ALL_REGIONS` or `ALL_REGIONS_EXCEPT_SPECIFIED` are used, Security Hub will automatically aggregate findings from new Regions as Security Hub supports them and you opt into them.
†
specifiedRegionsB*" ÉList of regions to include or exclude (required if `linking_mode` is set to `ALL_REGIONS_EXCEPT_SPECIFIED` or `SPECIFIED_REGIONS`)
"Ü
linkingMode" ÚIndicates whether to aggregate findings from all of the available Regions or from a specified list. The options are `ALL_REGIONS`, `ALL_REGIONS_EXCEPT_SPECIFIED` or `SPECIFIED_REGIONS`. When `ALL_REGIONS` or `ALL_REGIONS_EXCEPT_SPECIFIED` are used, Security Hub will automatically aggregate findings from new Regions as Security Hub supports them and you opt into them.
"†
specifiedRegionsB*" ÉList of regions to include or exclude (required if `linking_mode` is set to `ALL_REGIONS_EXCEPT_SPECIFIED` or `SPECIFIED_REGIONS`)
*Âº
7
securityhubInsightaws:securityhub/insight:InsightÉ≥Provides a Security Hub custom insight resource. See the [Managing custom insights section](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-custom-insights.html) of the AWS User Guide for more information.

## Example Usage

### Filter by AWS account ID

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleInsight = new aws.securityhub.Insight("example", {
    filters: {
        awsAccountIds: [
            {
                comparison: "EQUALS",
                value: "1234567890",
            },
            {
                comparison: "EQUALS",
                value: "09876543210",
            },
        ],
    },
    groupByAttribute: "AwsAccountId",
    name: "example-insight",
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_insight = aws.securityhub.Insight("example",
    filters={
        "aws_account_ids": [
            {
                "comparison": "EQUALS",
                "value": "1234567890",
            },
            {
                "comparison": "EQUALS",
                "value": "09876543210",
            },
        ],
    },
    group_by_attribute="AwsAccountId",
    name="example-insight",
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleInsight = new Aws.SecurityHub.Insight("example", new()
    {
        Filters = new Aws.SecurityHub.Inputs.InsightFiltersArgs
        {
            AwsAccountIds = new[]
            {
                new Aws.SecurityHub.Inputs.InsightFiltersAwsAccountIdArgs
                {
                    Comparison = "EQUALS",
                    Value = "1234567890",
                },
                new Aws.SecurityHub.Inputs.InsightFiltersAwsAccountIdArgs
                {
                    Comparison = "EQUALS",
                    Value = "09876543210",
                },
            },
        },
        GroupByAttribute = "AwsAccountId",
        Name = "example-insight",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewInsight(ctx, "example", &securityhub.InsightArgs{
			Filters: &securityhub.InsightFiltersArgs{
				AwsAccountIds: securityhub.InsightFiltersAwsAccountIdArray{
					&securityhub.InsightFiltersAwsAccountIdArgs{
						Comparison: pulumi.String("EQUALS"),
						Value:      pulumi.String("1234567890"),
					},
					&securityhub.InsightFiltersAwsAccountIdArgs{
						Comparison: pulumi.String("EQUALS"),
						Value:      pulumi.String("09876543210"),
					},
				},
			},
			GroupByAttribute: pulumi.String("AwsAccountId"),
			Name:             pulumi.String("example-insight"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.Insight;
import com.pulumi.aws.securityhub.InsightArgs;
import com.pulumi.aws.securityhub.inputs.InsightFiltersArgs;
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
        var example = new Account("example");

        var exampleInsight = new Insight("exampleInsight", InsightArgs.builder()
            .filters(InsightFiltersArgs.builder()
                .awsAccountIds(                
                    InsightFiltersAwsAccountIdArgs.builder()
                        .comparison("EQUALS")
                        .value("1234567890")
                        .build(),
                    InsightFiltersAwsAccountIdArgs.builder()
                        .comparison("EQUALS")
                        .value("09876543210")
                        .build())
                .build())
            .groupByAttribute("AwsAccountId")
            .name("example-insight")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleInsight:
    type: aws:securityhub:Insight
    name: example
    properties:
      filters:
        awsAccountIds:
          - comparison: EQUALS
            value: '1234567890'
          - comparison: EQUALS
            value: '09876543210'
      groupByAttribute: AwsAccountId
      name: example-insight
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

### Filter by date range

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleInsight = new aws.securityhub.Insight("example", {
    filters: {
        createdAts: [{
            dateRange: {
                unit: "DAYS",
                value: 5,
            },
        }],
    },
    groupByAttribute: "CreatedAt",
    name: "example-insight",
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_insight = aws.securityhub.Insight("example",
    filters={
        "created_ats": [{
            "date_range": {
                "unit": "DAYS",
                "value": 5,
            },
        }],
    },
    group_by_attribute="CreatedAt",
    name="example-insight",
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleInsight = new Aws.SecurityHub.Insight("example", new()
    {
        Filters = new Aws.SecurityHub.Inputs.InsightFiltersArgs
        {
            CreatedAts = new[]
            {
                new Aws.SecurityHub.Inputs.InsightFiltersCreatedAtArgs
                {
                    DateRange = new Aws.SecurityHub.Inputs.InsightFiltersCreatedAtDateRangeArgs
                    {
                        Unit = "DAYS",
                        Value = 5,
                    },
                },
            },
        },
        GroupByAttribute = "CreatedAt",
        Name = "example-insight",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewInsight(ctx, "example", &securityhub.InsightArgs{
			Filters: &securityhub.InsightFiltersArgs{
				CreatedAts: securityhub.InsightFiltersCreatedAtArray{
					&securityhub.InsightFiltersCreatedAtArgs{
						DateRange: &securityhub.InsightFiltersCreatedAtDateRangeArgs{
							Unit:  pulumi.String("DAYS"),
							Value: pulumi.Int(5),
						},
					},
				},
			},
			GroupByAttribute: pulumi.String("CreatedAt"),
			Name:             pulumi.String("example-insight"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.Insight;
import com.pulumi.aws.securityhub.InsightArgs;
import com.pulumi.aws.securityhub.inputs.InsightFiltersArgs;
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
        var example = new Account("example");

        var exampleInsight = new Insight("exampleInsight", InsightArgs.builder()
            .filters(InsightFiltersArgs.builder()
                .createdAts(InsightFiltersCreatedAtArgs.builder()
                    .dateRange(InsightFiltersCreatedAtDateRangeArgs.builder()
                        .unit("DAYS")
                        .value(5)
                        .build())
                    .build())
                .build())
            .groupByAttribute("CreatedAt")
            .name("example-insight")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleInsight:
    type: aws:securityhub:Insight
    name: example
    properties:
      filters:
        createdAts:
          - dateRange:
              unit: DAYS
              value: 5
      groupByAttribute: CreatedAt
      name: example-insight
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

### Filter by destination IPv4 address

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleInsight = new aws.securityhub.Insight("example", {
    filters: {
        networkDestinationIpv4s: [{
            cidr: "10.0.0.0/16",
        }],
    },
    groupByAttribute: "NetworkDestinationIpV4",
    name: "example-insight",
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_insight = aws.securityhub.Insight("example",
    filters={
        "network_destination_ipv4s": [{
            "cidr": "10.0.0.0/16",
        }],
    },
    group_by_attribute="NetworkDestinationIpV4",
    name="example-insight",
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleInsight = new Aws.SecurityHub.Insight("example", new()
    {
        Filters = new Aws.SecurityHub.Inputs.InsightFiltersArgs
        {
            NetworkDestinationIpv4s = new[]
            {
                new Aws.SecurityHub.Inputs.InsightFiltersNetworkDestinationIpv4Args
                {
                    Cidr = "10.0.0.0/16",
                },
            },
        },
        GroupByAttribute = "NetworkDestinationIpV4",
        Name = "example-insight",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewInsight(ctx, "example", &securityhub.InsightArgs{
			Filters: &securityhub.InsightFiltersArgs{
				NetworkDestinationIpv4s: securityhub.InsightFiltersNetworkDestinationIpv4Array{
					&securityhub.InsightFiltersNetworkDestinationIpv4Args{
						Cidr: pulumi.String("10.0.0.0/16"),
					},
				},
			},
			GroupByAttribute: pulumi.String("NetworkDestinationIpV4"),
			Name:             pulumi.String("example-insight"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.Insight;
import com.pulumi.aws.securityhub.InsightArgs;
import com.pulumi.aws.securityhub.inputs.InsightFiltersArgs;
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
        var example = new Account("example");

        var exampleInsight = new Insight("exampleInsight", InsightArgs.builder()
            .filters(InsightFiltersArgs.builder()
                .networkDestinationIpv4s(InsightFiltersNetworkDestinationIpv4Args.builder()
                    .cidr("10.0.0.0/16")
                    .build())
                .build())
            .groupByAttribute("NetworkDestinationIpV4")
            .name("example-insight")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleInsight:
    type: aws:securityhub:Insight
    name: example
    properties:
      filters:
        networkDestinationIpv4s:
          - cidr: 10.0.0.0/16
      groupByAttribute: NetworkDestinationIpV4
      name: example-insight
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

### Filter by finding's confidence

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleInsight = new aws.securityhub.Insight("example", {
    filters: {
        confidences: [{
            gte: "80",
        }],
    },
    groupByAttribute: "Confidence",
    name: "example-insight",
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_insight = aws.securityhub.Insight("example",
    filters={
        "confidences": [{
            "gte": "80",
        }],
    },
    group_by_attribute="Confidence",
    name="example-insight",
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleInsight = new Aws.SecurityHub.Insight("example", new()
    {
        Filters = new Aws.SecurityHub.Inputs.InsightFiltersArgs
        {
            Confidences = new[]
            {
                new Aws.SecurityHub.Inputs.InsightFiltersConfidenceArgs
                {
                    Gte = "80",
                },
            },
        },
        GroupByAttribute = "Confidence",
        Name = "example-insight",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewInsight(ctx, "example", &securityhub.InsightArgs{
			Filters: &securityhub.InsightFiltersArgs{
				Confidences: securityhub.InsightFiltersConfidenceArray{
					&securityhub.InsightFiltersConfidenceArgs{
						Gte: pulumi.String("80"),
					},
				},
			},
			GroupByAttribute: pulumi.String("Confidence"),
			Name:             pulumi.String("example-insight"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.Insight;
import com.pulumi.aws.securityhub.InsightArgs;
import com.pulumi.aws.securityhub.inputs.InsightFiltersArgs;
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
        var example = new Account("example");

        var exampleInsight = new Insight("exampleInsight", InsightArgs.builder()
            .filters(InsightFiltersArgs.builder()
                .confidences(InsightFiltersConfidenceArgs.builder()
                    .gte("80")
                    .build())
                .build())
            .groupByAttribute("Confidence")
            .name("example-insight")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleInsight:
    type: aws:securityhub:Insight
    name: example
    properties:
      filters:
        confidences:
          - gte: '80'
      groupByAttribute: Confidence
      name: example-insight
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

### Filter by resource tags

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleInsight = new aws.securityhub.Insight("example", {
    filters: {
        resourceTags: [{
            comparison: "EQUALS",
            key: "Environment",
            value: "Production",
        }],
    },
    groupByAttribute: "ResourceTags",
    name: "example-insight",
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_insight = aws.securityhub.Insight("example",
    filters={
        "resource_tags": [{
            "comparison": "EQUALS",
            "key": "Environment",
            "value": "Production",
        }],
    },
    group_by_attribute="ResourceTags",
    name="example-insight",
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleInsight = new Aws.SecurityHub.Insight("example", new()
    {
        Filters = new Aws.SecurityHub.Inputs.InsightFiltersArgs
        {
            ResourceTags = new[]
            {
                new Aws.SecurityHub.Inputs.InsightFiltersResourceTagArgs
                {
                    Comparison = "EQUALS",
                    Key = "Environment",
                    Value = "Production",
                },
            },
        },
        GroupByAttribute = "ResourceTags",
        Name = "example-insight",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewInsight(ctx, "example", &securityhub.InsightArgs{
			Filters: &securityhub.InsightFiltersArgs{
				ResourceTags: securityhub.InsightFiltersResourceTagArray{
					&securityhub.InsightFiltersResourceTagArgs{
						Comparison: pulumi.String("EQUALS"),
						Key:        pulumi.String("Environment"),
						Value:      pulumi.String("Production"),
					},
				},
			},
			GroupByAttribute: pulumi.String("ResourceTags"),
			Name:             pulumi.String("example-insight"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.Insight;
import com.pulumi.aws.securityhub.InsightArgs;
import com.pulumi.aws.securityhub.inputs.InsightFiltersArgs;
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
        var example = new Account("example");

        var exampleInsight = new Insight("exampleInsight", InsightArgs.builder()
            .filters(InsightFiltersArgs.builder()
                .resourceTags(InsightFiltersResourceTagArgs.builder()
                    .comparison("EQUALS")
                    .key("Environment")
                    .value("Production")
                    .build())
                .build())
            .groupByAttribute("ResourceTags")
            .name("example-insight")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleInsight:
    type: aws:securityhub:Insight
    name: example
    properties:
      filters:
        resourceTags:
          - comparison: EQUALS
            key: Environment
            value: Production
      groupByAttribute: ResourceTags
      name: example-insight
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Hub insights using the ARN. For example:

```sh
$ pulumi import aws:securityhub/insight:Insight example arn:aws:securityhub:us-west-2:1234567890:insight/1234567890/custom/91299ed7-abd0-4e44-a858-d0b15e37141a
```
—
filtersP:N
L
securityhubInsightFilters-aws:securityhub/InsightFilters:InsightFiltersÛA configuration block including one or more (up to 10 distinct) attributes used to filter the findings included in the insight. The insight only includes findings that match criteria defined in the filters. See filters below for more details.
º
groupByAttribute" £The attribute used to group the findings for the insight e.g., if an insight is grouped by `ResourceId`, then the insight produces a list of resource identifiers.
.
nameB"  The name of the custom insight.
"
arn" ARN of the insight.
"—
filtersP:N
L
securityhubInsightFilters-aws:securityhub/InsightFilters:InsightFiltersÛA configuration block including one or more (up to 10 distinct) attributes used to filter the findings included in the insight. The insight only includes findings that match criteria defined in the filters. See filters below for more details.
"º
groupByAttribute" £The attribute used to group the findings for the insight e.g., if an insight is grouped by `ResourceId`, then the insight produces a list of resource identifiers.
",
name"  The name of the custom insight.
*Ñ(
L
securityhubInviteAccepter-aws:securityhub/inviteAccepter:InviteAccepterµ%> **Note:** AWS accounts can only be associated with a single Security Hub master account. Destroying this resource will disassociate the member account from the master account.

Accepts a Security Hub invitation.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleMember = new aws.securityhub.Member("example", {
    accountId: "123456789012",
    email: "example@example.com",
    invite: true,
});
const invitee = new aws.securityhub.Account("invitee", {});
const inviteeInviteAccepter = new aws.securityhub.InviteAccepter("invitee", {masterId: exampleMember.masterId}, {
    dependsOn: [invitee],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_member = aws.securityhub.Member("example",
    account_id="123456789012",
    email="example@example.com",
    invite=True)
invitee = aws.securityhub.Account("invitee")
invitee_invite_accepter = aws.securityhub.InviteAccepter("invitee", master_id=example_member.master_id,
opts = pulumi.ResourceOptions(depends_on=[invitee]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleMember = new Aws.SecurityHub.Member("example", new()
    {
        AccountId = "123456789012",
        Email = "example@example.com",
        Invite = true,
    });

    var invitee = new Aws.SecurityHub.Account("invitee");

    var inviteeInviteAccepter = new Aws.SecurityHub.InviteAccepter("invitee", new()
    {
        MasterId = exampleMember.MasterId,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            invitee,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		exampleMember, err := securityhub.NewMember(ctx, "example", &securityhub.MemberArgs{
			AccountId: pulumi.String("123456789012"),
			Email:     pulumi.String("example@example.com"),
			Invite:    pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		invitee, err := securityhub.NewAccount(ctx, "invitee", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewInviteAccepter(ctx, "invitee", &securityhub.InviteAccepterArgs{
			MasterId: exampleMember.MasterId,
		}, pulumi.DependsOn([]pulumi.Resource{
			invitee,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.Member;
import com.pulumi.aws.securityhub.MemberArgs;
import com.pulumi.aws.securityhub.InviteAccepter;
import com.pulumi.aws.securityhub.InviteAccepterArgs;
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
        var example = new Account("example");

        var exampleMember = new Member("exampleMember", MemberArgs.builder()
            .accountId("123456789012")
            .email("example@example.com")
            .invite(true)
            .build());

        var invitee = new Account("invitee");

        var inviteeInviteAccepter = new InviteAccepter("inviteeInviteAccepter", InviteAccepterArgs.builder()
            .masterId(exampleMember.masterId())
            .build(), CustomResourceOptions.builder()
                .dependsOn(invitee)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleMember:
    type: aws:securityhub:Member
    name: example
    properties:
      accountId: '123456789012'
      email: example@example.com
      invite: true
  invitee:
    type: aws:securityhub:Account
  inviteeInviteAccepter:
    type: aws:securityhub:InviteAccepter
    name: invitee
    properties:
      masterId: ${exampleMember.masterId}
    options:
      dependsOn:
        - ${invitee}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Hub invite acceptance using the account ID. For example:

```sh
$ pulumi import aws:securityhub/inviteAccepter:InviteAccepter example 123456789012
```
e
masterId" UThe account ID of the master Security Hub account whose invitation you're accepting.
".
invitationId" The ID of the invitation.
"e
masterId" UThe account ID of the master Security Hub account whose invitation you're accepting.
*∫
4
securityhubMemberaws:securityhub/member:Member—Provides a Security Hub member resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const exampleMember = new aws.securityhub.Member("example", {
    accountId: "123456789012",
    email: "example@example.com",
    invite: true,
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
example_member = aws.securityhub.Member("example",
    account_id="123456789012",
    email="example@example.com",
    invite=True,
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var exampleMember = new Aws.SecurityHub.Member("example", new()
    {
        AccountId = "123456789012",
        Email = "example@example.com",
        Invite = true,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewMember(ctx, "example", &securityhub.MemberArgs{
			AccountId: pulumi.String("123456789012"),
			Email:     pulumi.String("example@example.com"),
			Invite:    pulumi.Bool(true),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.Member;
import com.pulumi.aws.securityhub.MemberArgs;
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
        var example = new Account("example");

        var exampleMember = new Member("exampleMember", MemberArgs.builder()
            .accountId("123456789012")
            .email("example@example.com")
            .invite(true)
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleMember:
    type: aws:securityhub:Member
    name: example
    properties:
      accountId: '123456789012'
      email: example@example.com
      invite: true
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Hub members using their account ID. For example:

```sh
$ pulumi import aws:securityhub/member:Member example 123456789012
```
3
	accountId" "The ID of the member AWS account.
4
emailB" %The email of the member AWS account.
h
inviteB
 XBoolean whether to invite the account to Security Hub as a member. Defaults to `false`.
"3
	accountId" "The ID of the member AWS account.
"4
emailB" %The email of the member AWS account.
"h
inviteB
 XBoolean whether to invite the account to Security Hub as a member. Defaults to `false`.
"?
masterId" /The ID of the master Security Hub AWS account.
"C
memberStatus" /The status of the member account relationship.
*√4
j
securityhubOrganizationAdminAccountAaws:securityhub/organizationAdminAccount:OrganizationAdminAccount‡1Manages a Security Hub administrator account for an organization. The AWS account utilizing this resource must be an Organizations primary account. More information about Organizations support in Security Hub can be found in the [Security Hub User Guide](https://docs.aws.amazon.com/securityhub/latest/userguide/designate-orgs-admin-account.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.organizations.Organization("example", {
    awsServiceAccessPrincipals: ["securityhub.amazonaws.com"],
    featureSet: "ALL",
});
const exampleAccount = new aws.securityhub.Account("example", {});
const exampleOrganizationAdminAccount = new aws.securityhub.OrganizationAdminAccount("example", {adminAccountId: "123456789012"}, {
    dependsOn: [example],
});
// Auto enable security hub in organization member accounts
const exampleOrganizationConfiguration = new aws.securityhub.OrganizationConfiguration("example", {autoEnable: true});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.organizations.Organization("example",
    aws_service_access_principals=["securityhub.amazonaws.com"],
    feature_set="ALL")
example_account = aws.securityhub.Account("example")
example_organization_admin_account = aws.securityhub.OrganizationAdminAccount("example", admin_account_id="123456789012",
opts = pulumi.ResourceOptions(depends_on=[example]))
# Auto enable security hub in organization member accounts
example_organization_configuration = aws.securityhub.OrganizationConfiguration("example", auto_enable=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Organizations.Organization("example", new()
    {
        AwsServiceAccessPrincipals = new[]
        {
            "securityhub.amazonaws.com",
        },
        FeatureSet = "ALL",
    });

    var exampleAccount = new Aws.SecurityHub.Account("example");

    var exampleOrganizationAdminAccount = new Aws.SecurityHub.OrganizationAdminAccount("example", new()
    {
        AdminAccountId = "123456789012",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

    // Auto enable security hub in organization member accounts
    var exampleOrganizationConfiguration = new Aws.SecurityHub.OrganizationConfiguration("example", new()
    {
        AutoEnable = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/organizations"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := organizations.NewOrganization(ctx, "example", &organizations.OrganizationArgs{
			AwsServiceAccessPrincipals: pulumi.StringArray{
				pulumi.String("securityhub.amazonaws.com"),
			},
			FeatureSet: pulumi.String("ALL"),
		})
		if err != nil {
			return err
		}
		_, err = securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewOrganizationAdminAccount(ctx, "example", &securityhub.OrganizationAdminAccountArgs{
			AdminAccountId: pulumi.String("123456789012"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
		}))
		if err != nil {
			return err
		}
		// Auto enable security hub in organization member accounts
		_, err = securityhub.NewOrganizationConfiguration(ctx, "example", &securityhub.OrganizationConfigurationArgs{
			AutoEnable: pulumi.Bool(true),
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
import com.pulumi.aws.organizations.Organization;
import com.pulumi.aws.organizations.OrganizationArgs;
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.OrganizationAdminAccount;
import com.pulumi.aws.securityhub.OrganizationAdminAccountArgs;
import com.pulumi.aws.securityhub.OrganizationConfiguration;
import com.pulumi.aws.securityhub.OrganizationConfigurationArgs;
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
        var example = new Organization("example", OrganizationArgs.builder()
            .awsServiceAccessPrincipals("securityhub.amazonaws.com")
            .featureSet("ALL")
            .build());

        var exampleAccount = new Account("exampleAccount");

        var exampleOrganizationAdminAccount = new OrganizationAdminAccount("exampleOrganizationAdminAccount", OrganizationAdminAccountArgs.builder()
            .adminAccountId("123456789012")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

        // Auto enable security hub in organization member accounts
        var exampleOrganizationConfiguration = new OrganizationConfiguration("exampleOrganizationConfiguration", OrganizationConfigurationArgs.builder()
            .autoEnable(true)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:organizations:Organization
    properties:
      awsServiceAccessPrincipals:
        - securityhub.amazonaws.com
      featureSet: ALL
  exampleAccount:
    type: aws:securityhub:Account
    name: example
  exampleOrganizationAdminAccount:
    type: aws:securityhub:OrganizationAdminAccount
    name: example
    properties:
      adminAccountId: '123456789012'
    options:
      dependsOn:
        - ${example}
  # Auto enable security hub in organization member accounts
  exampleOrganizationConfiguration:
    type: aws:securityhub:OrganizationConfiguration
    name: example
    properties:
      autoEnable: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Hub Organization Admin Accounts using the AWS account ID. For example:

```sh
$ pulumi import aws:securityhub/organizationAdminAccount:OrganizationAdminAccount example 123456789012
```
x
adminAccountId" bThe AWS account identifier of the account to designate as the Security Hub administrator account.
"x
adminAccountId" bThe AWS account identifier of the account to designate as the Security Hub administrator account.
*“q
m
securityhubOrganizationConfigurationCaws:securityhub/organizationConfiguration:OrganizationConfiguration≥dManages the Security Hub Organization Configuration.

> **NOTE:** This resource requires an `aws.securityhub.OrganizationAdminAccount` to be configured (not necessarily with Pulumi). More information about managing Security Hub in an organization can be found in the [Managing administrator and member accounts](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-accounts.html) documentation.

> **NOTE:** In order to set the `configuration_type` to `CENTRAL`, the delegated admin must be a member account of the organization and not the management account. Central configuration also requires an `aws.securityhub.FindingAggregator` to be configured.

> **NOTE:** This is an advanced AWS resource. Pulumi will automatically assume management of the Security Hub Organization Configuration without import and perform no actions on removal from the Pulumi program.

> **NOTE:** Deleting this resource resets security hub to a local organization configuration with auto enable false.

## Example Usage

### Local Configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.organizations.Organization("example", {
    awsServiceAccessPrincipals: ["securityhub.amazonaws.com"],
    featureSet: "ALL",
});
const exampleOrganizationAdminAccount = new aws.securityhub.OrganizationAdminAccount("example", {adminAccountId: "123456789012"}, {
    dependsOn: [example],
});
const exampleOrganizationConfiguration = new aws.securityhub.OrganizationConfiguration("example", {autoEnable: true});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.organizations.Organization("example",
    aws_service_access_principals=["securityhub.amazonaws.com"],
    feature_set="ALL")
example_organization_admin_account = aws.securityhub.OrganizationAdminAccount("example", admin_account_id="123456789012",
opts = pulumi.ResourceOptions(depends_on=[example]))
example_organization_configuration = aws.securityhub.OrganizationConfiguration("example", auto_enable=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Organizations.Organization("example", new()
    {
        AwsServiceAccessPrincipals = new[]
        {
            "securityhub.amazonaws.com",
        },
        FeatureSet = "ALL",
    });

    var exampleOrganizationAdminAccount = new Aws.SecurityHub.OrganizationAdminAccount("example", new()
    {
        AdminAccountId = "123456789012",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

    var exampleOrganizationConfiguration = new Aws.SecurityHub.OrganizationConfiguration("example", new()
    {
        AutoEnable = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/organizations"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := organizations.NewOrganization(ctx, "example", &organizations.OrganizationArgs{
			AwsServiceAccessPrincipals: pulumi.StringArray{
				pulumi.String("securityhub.amazonaws.com"),
			},
			FeatureSet: pulumi.String("ALL"),
		})
		if err != nil {
			return err
		}
		_, err = securityhub.NewOrganizationAdminAccount(ctx, "example", &securityhub.OrganizationAdminAccountArgs{
			AdminAccountId: pulumi.String("123456789012"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
		}))
		if err != nil {
			return err
		}
		_, err = securityhub.NewOrganizationConfiguration(ctx, "example", &securityhub.OrganizationConfigurationArgs{
			AutoEnable: pulumi.Bool(true),
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
import com.pulumi.aws.organizations.Organization;
import com.pulumi.aws.organizations.OrganizationArgs;
import com.pulumi.aws.securityhub.OrganizationAdminAccount;
import com.pulumi.aws.securityhub.OrganizationAdminAccountArgs;
import com.pulumi.aws.securityhub.OrganizationConfiguration;
import com.pulumi.aws.securityhub.OrganizationConfigurationArgs;
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
        var example = new Organization("example", OrganizationArgs.builder()
            .awsServiceAccessPrincipals("securityhub.amazonaws.com")
            .featureSet("ALL")
            .build());

        var exampleOrganizationAdminAccount = new OrganizationAdminAccount("exampleOrganizationAdminAccount", OrganizationAdminAccountArgs.builder()
            .adminAccountId("123456789012")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

        var exampleOrganizationConfiguration = new OrganizationConfiguration("exampleOrganizationConfiguration", OrganizationConfigurationArgs.builder()
            .autoEnable(true)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:organizations:Organization
    properties:
      awsServiceAccessPrincipals:
        - securityhub.amazonaws.com
      featureSet: ALL
  exampleOrganizationAdminAccount:
    type: aws:securityhub:OrganizationAdminAccount
    name: example
    properties:
      adminAccountId: '123456789012'
    options:
      dependsOn:
        - ${example}
  exampleOrganizationConfiguration:
    type: aws:securityhub:OrganizationConfiguration
    name: example
    properties:
      autoEnable: true
```
<!--End PulumiCodeChooser -->

### Central Configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.OrganizationAdminAccount("example", {adminAccountId: "123456789012"}, {
    dependsOn: [exampleAwsOrganizationsOrganization],
});
const exampleFindingAggregator = new aws.securityhub.FindingAggregator("example", {linkingMode: "ALL_REGIONS"}, {
    dependsOn: [example],
});
const exampleOrganizationConfiguration = new aws.securityhub.OrganizationConfiguration("example", {
    autoEnable: false,
    autoEnableStandards: "NONE",
    organizationConfiguration: {
        configurationType: "CENTRAL",
    },
}, {
    dependsOn: [exampleFindingAggregator],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.OrganizationAdminAccount("example", admin_account_id="123456789012",
opts = pulumi.ResourceOptions(depends_on=[example_aws_organizations_organization]))
example_finding_aggregator = aws.securityhub.FindingAggregator("example", linking_mode="ALL_REGIONS",
opts = pulumi.ResourceOptions(depends_on=[example]))
example_organization_configuration = aws.securityhub.OrganizationConfiguration("example",
    auto_enable=False,
    auto_enable_standards="NONE",
    organization_configuration={
        "configuration_type": "CENTRAL",
    },
    opts = pulumi.ResourceOptions(depends_on=[example_finding_aggregator]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.OrganizationAdminAccount("example", new()
    {
        AdminAccountId = "123456789012",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsOrganizationsOrganization,
        },
    });

    var exampleFindingAggregator = new Aws.SecurityHub.FindingAggregator("example", new()
    {
        LinkingMode = "ALL_REGIONS",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

    var exampleOrganizationConfiguration = new Aws.SecurityHub.OrganizationConfiguration("example", new()
    {
        AutoEnable = false,
        AutoEnableStandards = "NONE",
        OrganizationConfigurationDetails = new Aws.SecurityHub.Inputs.OrganizationConfigurationOrganizationConfigurationArgs
        {
            ConfigurationType = "CENTRAL",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleFindingAggregator,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewOrganizationAdminAccount(ctx, "example", &securityhub.OrganizationAdminAccountArgs{
			AdminAccountId: pulumi.String("123456789012"),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsOrganizationsOrganization,
		}))
		if err != nil {
			return err
		}
		exampleFindingAggregator, err := securityhub.NewFindingAggregator(ctx, "example", &securityhub.FindingAggregatorArgs{
			LinkingMode: pulumi.String("ALL_REGIONS"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
		}))
		if err != nil {
			return err
		}
		_, err = securityhub.NewOrganizationConfiguration(ctx, "example", &securityhub.OrganizationConfigurationArgs{
			AutoEnable:          pulumi.Bool(false),
			AutoEnableStandards: pulumi.String("NONE"),
			OrganizationConfiguration: &securityhub.OrganizationConfigurationOrganizationConfigurationArgs{
				ConfigurationType: pulumi.String("CENTRAL"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleFindingAggregator,
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
import com.pulumi.aws.securityhub.OrganizationAdminAccount;
import com.pulumi.aws.securityhub.OrganizationAdminAccountArgs;
import com.pulumi.aws.securityhub.FindingAggregator;
import com.pulumi.aws.securityhub.FindingAggregatorArgs;
import com.pulumi.aws.securityhub.OrganizationConfiguration;
import com.pulumi.aws.securityhub.OrganizationConfigurationArgs;
import com.pulumi.aws.securityhub.inputs.OrganizationConfigurationOrganizationConfigurationArgs;
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
        var example = new OrganizationAdminAccount("example", OrganizationAdminAccountArgs.builder()
            .adminAccountId("123456789012")
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsOrganizationsOrganization)
                .build());

        var exampleFindingAggregator = new FindingAggregator("exampleFindingAggregator", FindingAggregatorArgs.builder()
            .linkingMode("ALL_REGIONS")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

        var exampleOrganizationConfiguration = new OrganizationConfiguration("exampleOrganizationConfiguration", OrganizationConfigurationArgs.builder()
            .autoEnable(false)
            .autoEnableStandards("NONE")
            .organizationConfiguration(OrganizationConfigurationOrganizationConfigurationArgs.builder()
                .configurationType("CENTRAL")
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleFindingAggregator)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:OrganizationAdminAccount
    properties:
      adminAccountId: '123456789012'
    options:
      dependsOn:
        - ${exampleAwsOrganizationsOrganization}
  exampleFindingAggregator:
    type: aws:securityhub:FindingAggregator
    name: example
    properties:
      linkingMode: ALL_REGIONS
    options:
      dependsOn:
        - ${example}
  exampleOrganizationConfiguration:
    type: aws:securityhub:OrganizationConfiguration
    name: example
    properties:
      autoEnable: false
      autoEnableStandards: NONE
      organizationConfiguration:
        configurationType: CENTRAL
    options:
      dependsOn:
        - ${exampleFindingAggregator}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import an existing Security Hub enabled account using the AWS account ID. For example:

```sh
$ pulumi import aws:securityhub/organizationConfiguration:OrganizationConfiguration example 123456789012
```
e

autoEnable
 SWhether to automatically enable Security Hub for new accounts in the organization.
˘
autoEnableStandardsB" €Whether to automatically enable Security Hub default standards for new member accounts in the organization. By default, this parameter is equal to `DEFAULT`, and new member accounts are automatically enabled with default Security Hub standards. To opt out of enabling default standards for new member accounts, set this parameter equal to `NONE`.
≥
organizationConfiguration¡Bæ:ª
∏
securityhub2OrganizationConfigurationOrganizationConfigurationuaws:securityhub/OrganizationConfigurationOrganizationConfiguration:OrganizationConfigurationOrganizationConfigurationRProvides information about the way an organization is configured in Security Hub.
"e

autoEnable
 SWhether to automatically enable Security Hub for new accounts in the organization.
"˜
autoEnableStandards" €Whether to automatically enable Security Hub default standards for new member accounts in the organization. By default, this parameter is equal to `DEFAULT`, and new member accounts are automatically enabled with default Security Hub standards. To opt out of enabling default standards for new member accounts, set this parameter equal to `NONE`.
"∞
organizationConfigurationæ:ª
∏
securityhub2OrganizationConfigurationOrganizationConfigurationuaws:securityhub/OrganizationConfigurationOrganizationConfiguration:OrganizationConfigurationOrganizationConfigurationRProvides information about the way an organization is configured in Security Hub.
*ÕT
[
securityhubProductSubscription7aws:securityhub/productSubscription:ProductSubscriptioní"Subscribes to a Security Hub product.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const current = aws.getRegion({});
const exampleProductSubscription = new aws.securityhub.ProductSubscription("example", {productArn: current.then(current => `arn:aws:securityhub:${current.name}:733251395267:product/alertlogic/althreatmanagement`)}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
current = aws.get_region()
example_product_subscription = aws.securityhub.ProductSubscription("example", product_arn=f"arn:aws:securityhub:{current.name}:733251395267:product/alertlogic/althreatmanagement",
opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var current = Aws.GetRegion.Invoke();

    var exampleProductSubscription = new Aws.SecurityHub.ProductSubscription("example", new()
    {
        ProductArn = $"arn:aws:securityhub:{current.Apply(getRegionResult => getRegionResult.Name)}:733251395267:product/alertlogic/althreatmanagement",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewProductSubscription(ctx, "example", &securityhub.ProductSubscriptionArgs{
			ProductArn: pulumi.Sprintf("arn:aws:securityhub:%v:733251395267:product/alertlogic/althreatmanagement", current.Name),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.securityhub.ProductSubscription;
import com.pulumi.aws.securityhub.ProductSubscriptionArgs;
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
        var example = new Account("example");

        final var current = AwsFunctions.getRegion();

        var exampleProductSubscription = new ProductSubscription("exampleProductSubscription", ProductSubscriptionArgs.builder()
            .productArn(String.format("arn:aws:securityhub:%s:733251395267:product/alertlogic/althreatmanagement", current.applyValue(getRegionResult -> getRegionResult.name())))
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  exampleProductSubscription:
    type: aws:securityhub:ProductSubscription
    name: example
    properties:
      productArn: arn:aws:securityhub:${current.name}:733251395267:product/alertlogic/althreatmanagement
    options:
      dependsOn:
        - ${example}
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Hub product subscriptions using `product_arn,arn`. For example:

```sh
$ pulumi import aws:securityhub/productSubscription:ProductSubscription example arn:aws:securityhub:eu-west-1:733251395267:product/alertlogic/althreatmanagement,arn:aws:securityhub:eu-west-1:123456789012:product-subscription/alertlogic/althreatmanagement
```
ú

productArn" âThe ARN of the product that generates findings that you want to import into Security Hub - see below.

Amazon maintains a list of [Product integrations in AWS Security Hub](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-findings-providers.html) that changes over time. Any of the products on the linked [Available AWS service integrations](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-internal-providers.html) or [Available third-party partner product integrations](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-partner-providers.html) can be configured using `aws.securityhub.ProductSubscription`.

Available products can also be listed by running the AWS CLI command `aws securityhub describe-products`.

A subset of currently available products (remember to replace `${var.region}` as appropriate) includes:

* `arn:aws:securityhub:${var.region}::product/aws/guardduty`
* `arn:aws:securityhub:${var.region}::product/aws/inspector`
* `arn:aws:securityhub:${var.region}::product/aws/macie`
* `arn:aws:securityhub:${var.region}::product/alertlogic/althreatmanagement`
* `arn:aws:securityhub:${var.region}::product/armordefense/armoranywhere`
* `arn:aws:securityhub:${var.region}::product/barracuda/cloudsecurityguardian`
* `arn:aws:securityhub:${var.region}::product/checkpoint/cloudguard-iaas`
* `arn:aws:securityhub:${var.region}::product/checkpoint/dome9-arc`
* `arn:aws:securityhub:${var.region}::product/crowdstrike/crowdstrike-falcon`
* `arn:aws:securityhub:${var.region}::product/cyberark/cyberark-pta`
* `arn:aws:securityhub:${var.region}::product/f5networks/f5-advanced-waf`
* `arn:aws:securityhub:${var.region}::product/fortinet/fortigate`
* `arn:aws:securityhub:${var.region}::product/guardicore/aws-infection-monkey`
* `arn:aws:securityhub:${var.region}::product/guardicore/guardicore`
* `arn:aws:securityhub:${var.region}::product/ibm/qradar-siem`
* `arn:aws:securityhub:${var.region}::product/imperva/imperva-attack-analytics`
* `arn:aws:securityhub:${var.region}::product/mcafee-skyhigh/mcafee-mvision-cloud-aws`
* `arn:aws:securityhub:${var.region}::product/paloaltonetworks/redlock`
* `arn:aws:securityhub:${var.region}::product/paloaltonetworks/vm-series`
* `arn:aws:securityhub:${var.region}::product/qualys/qualys-pc`
* `arn:aws:securityhub:${var.region}::product/qualys/qualys-vm`
* `arn:aws:securityhub:${var.region}::product/rapid7/insightvm`
* `arn:aws:securityhub:${var.region}::product/sophos/sophos-server-protection`
* `arn:aws:securityhub:${var.region}::product/splunk/splunk-enterprise`
* `arn:aws:securityhub:${var.region}::product/splunk/splunk-phantom`
* `arn:aws:securityhub:${var.region}::product/sumologicinc/sumologic-mda`
* `arn:aws:securityhub:${var.region}::product/symantec-corp/symantec-cwp`
* `arn:aws:securityhub:${var.region}::product/tenable/tenable-io`
* `arn:aws:securityhub:${var.region}::product/trend-micro/deep-security`
* `arn:aws:securityhub:${var.region}::product/turbot/turbot`
* `arn:aws:securityhub:${var.region}::product/twistlock/twistlock-enterprise`
"ö
arn" éThe ARN of a resource that represents your subscription to the product that generates the findings that you want to import into Security Hub.
"ú

productArn" âThe ARN of the product that generates findings that you want to import into Security Hub - see below.

Amazon maintains a list of [Product integrations in AWS Security Hub](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-findings-providers.html) that changes over time. Any of the products on the linked [Available AWS service integrations](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-internal-providers.html) or [Available third-party partner product integrations](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-partner-providers.html) can be configured using `aws.securityhub.ProductSubscription`.

Available products can also be listed by running the AWS CLI command `aws securityhub describe-products`.

A subset of currently available products (remember to replace `${var.region}` as appropriate) includes:

* `arn:aws:securityhub:${var.region}::product/aws/guardduty`
* `arn:aws:securityhub:${var.region}::product/aws/inspector`
* `arn:aws:securityhub:${var.region}::product/aws/macie`
* `arn:aws:securityhub:${var.region}::product/alertlogic/althreatmanagement`
* `arn:aws:securityhub:${var.region}::product/armordefense/armoranywhere`
* `arn:aws:securityhub:${var.region}::product/barracuda/cloudsecurityguardian`
* `arn:aws:securityhub:${var.region}::product/checkpoint/cloudguard-iaas`
* `arn:aws:securityhub:${var.region}::product/checkpoint/dome9-arc`
* `arn:aws:securityhub:${var.region}::product/crowdstrike/crowdstrike-falcon`
* `arn:aws:securityhub:${var.region}::product/cyberark/cyberark-pta`
* `arn:aws:securityhub:${var.region}::product/f5networks/f5-advanced-waf`
* `arn:aws:securityhub:${var.region}::product/fortinet/fortigate`
* `arn:aws:securityhub:${var.region}::product/guardicore/aws-infection-monkey`
* `arn:aws:securityhub:${var.region}::product/guardicore/guardicore`
* `arn:aws:securityhub:${var.region}::product/ibm/qradar-siem`
* `arn:aws:securityhub:${var.region}::product/imperva/imperva-attack-analytics`
* `arn:aws:securityhub:${var.region}::product/mcafee-skyhigh/mcafee-mvision-cloud-aws`
* `arn:aws:securityhub:${var.region}::product/paloaltonetworks/redlock`
* `arn:aws:securityhub:${var.region}::product/paloaltonetworks/vm-series`
* `arn:aws:securityhub:${var.region}::product/qualys/qualys-pc`
* `arn:aws:securityhub:${var.region}::product/qualys/qualys-vm`
* `arn:aws:securityhub:${var.region}::product/rapid7/insightvm`
* `arn:aws:securityhub:${var.region}::product/sophos/sophos-server-protection`
* `arn:aws:securityhub:${var.region}::product/splunk/splunk-enterprise`
* `arn:aws:securityhub:${var.region}::product/splunk/splunk-phantom`
* `arn:aws:securityhub:${var.region}::product/sumologicinc/sumologic-mda`
* `arn:aws:securityhub:${var.region}::product/symantec-corp/symantec-cwp`
* `arn:aws:securityhub:${var.region}::product/tenable/tenable-io`
* `arn:aws:securityhub:${var.region}::product/trend-micro/deep-security`
* `arn:aws:securityhub:${var.region}::product/turbot/turbot`
* `arn:aws:securityhub:${var.region}::product/twistlock/twistlock-enterprise`
*£F
R
securityhubStandardsControl1aws:securityhub/standardsControl:StandardsControlÃ4Disable/enable Security Hub standards control in the current region.

The `aws.securityhub.StandardsControl` behaves differently from normal resources, in that
Pulumi does not _create_ this resource, but instead "adopts" it
into management. When you _delete_ this resource configuration, Pulumi "abandons" resource as is and just removes it from the state.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const cisAwsFoundationsBenchmark = new aws.securityhub.StandardsSubscription("cis_aws_foundations_benchmark", {standardsArn: "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0"}, {
    dependsOn: [example],
});
const ensureIamPasswordPolicyPreventsPasswordReuse = new aws.securityhub.StandardsControl("ensure_iam_password_policy_prevents_password_reuse", {
    standardsControlArn: "arn:aws:securityhub:us-east-1:111111111111:control/cis-aws-foundations-benchmark/v/1.2.0/1.10",
    controlStatus: "DISABLED",
    disabledReason: "We handle password policies within Okta",
}, {
    dependsOn: [cisAwsFoundationsBenchmark],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
cis_aws_foundations_benchmark = aws.securityhub.StandardsSubscription("cis_aws_foundations_benchmark", standards_arn="arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
opts = pulumi.ResourceOptions(depends_on=[example]))
ensure_iam_password_policy_prevents_password_reuse = aws.securityhub.StandardsControl("ensure_iam_password_policy_prevents_password_reuse",
    standards_control_arn="arn:aws:securityhub:us-east-1:111111111111:control/cis-aws-foundations-benchmark/v/1.2.0/1.10",
    control_status="DISABLED",
    disabled_reason="We handle password policies within Okta",
    opts = pulumi.ResourceOptions(depends_on=[cis_aws_foundations_benchmark]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var cisAwsFoundationsBenchmark = new Aws.SecurityHub.StandardsSubscription("cis_aws_foundations_benchmark", new()
    {
        StandardsArn = "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

    var ensureIamPasswordPolicyPreventsPasswordReuse = new Aws.SecurityHub.StandardsControl("ensure_iam_password_policy_prevents_password_reuse", new()
    {
        StandardsControlArn = "arn:aws:securityhub:us-east-1:111111111111:control/cis-aws-foundations-benchmark/v/1.2.0/1.10",
        ControlStatus = "DISABLED",
        DisabledReason = "We handle password policies within Okta",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            cisAwsFoundationsBenchmark,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		cisAwsFoundationsBenchmark, err := securityhub.NewStandardsSubscription(ctx, "cis_aws_foundations_benchmark", &securityhub.StandardsSubscriptionArgs{
			StandardsArn: pulumi.String("arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
		}))
		if err != nil {
			return err
		}
		_, err = securityhub.NewStandardsControl(ctx, "ensure_iam_password_policy_prevents_password_reuse", &securityhub.StandardsControlArgs{
			StandardsControlArn: pulumi.String("arn:aws:securityhub:us-east-1:111111111111:control/cis-aws-foundations-benchmark/v/1.2.0/1.10"),
			ControlStatus:       pulumi.String("DISABLED"),
			DisabledReason:      pulumi.String("We handle password policies within Okta"),
		}, pulumi.DependsOn([]pulumi.Resource{
			cisAwsFoundationsBenchmark,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.StandardsSubscription;
import com.pulumi.aws.securityhub.StandardsSubscriptionArgs;
import com.pulumi.aws.securityhub.StandardsControl;
import com.pulumi.aws.securityhub.StandardsControlArgs;
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
        var example = new Account("example");

        var cisAwsFoundationsBenchmark = new StandardsSubscription("cisAwsFoundationsBenchmark", StandardsSubscriptionArgs.builder()
            .standardsArn("arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

        var ensureIamPasswordPolicyPreventsPasswordReuse = new StandardsControl("ensureIamPasswordPolicyPreventsPasswordReuse", StandardsControlArgs.builder()
            .standardsControlArn("arn:aws:securityhub:us-east-1:111111111111:control/cis-aws-foundations-benchmark/v/1.2.0/1.10")
            .controlStatus("DISABLED")
            .disabledReason("We handle password policies within Okta")
            .build(), CustomResourceOptions.builder()
                .dependsOn(cisAwsFoundationsBenchmark)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  cisAwsFoundationsBenchmark:
    type: aws:securityhub:StandardsSubscription
    name: cis_aws_foundations_benchmark
    properties:
      standardsArn: arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0
    options:
      dependsOn:
        - ${example}
  ensureIamPasswordPolicyPreventsPasswordReuse:
    type: aws:securityhub:StandardsControl
    name: ensure_iam_password_policy_prevents_password_reuse
    properties:
      standardsControlArn: arn:aws:securityhub:us-east-1:111111111111:control/cis-aws-foundations-benchmark/v/1.2.0/1.10
      controlStatus: DISABLED
      disabledReason: We handle password policies within Okta
    options:
      dependsOn:
        - ${cisAwsFoundationsBenchmark}
```
<!--End PulumiCodeChooser -->
ô
controlStatus" ÉThe control status could be `ENABLED` or `DISABLED`. You have to specify `disabled_reason` argument for `DISABLED` control status.
¡
disabledReasonB" ®A description of the reason why you are disabling a security standard control. If you specify this attribute, `control_status` will be set to `DISABLED` automatically.
î
standardsControlArn" ¯The standards control ARN. See the AWS documentation for how to list existing controls using [`get-enabled-standards`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/securityhub/get-enabled-standards.html) and [`describe-standards-controls`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/securityhub/describe-standards-controls.html).
"B
	controlId" 1The identifier of the security standard control.
"ô
controlStatus" ÉThe control status could be `ENABLED` or `DISABLED`. You have to specify `disabled_reason` argument for `DISABLED` control status.
"|
controlStatusUpdatedAt" ^The date and time that the status of the security standard control was most recently updated.
"y
description" fThe standard control longer description. Provides information about what the control is checking for.
"ø
disabledReason" ®A description of the reason why you are disabling a security standard control. If you specify this attribute, `control_status` will be set to `DISABLED` automatically.
"X
relatedRequirements*" ;The list of requirements that are related to this control.
"p
remediationUrl" ZA link to remediation information for the control in the Security Hub user documentation.
"^
severityRating" HThe severity of findings generated from this security standard control.
"î
standardsControlArn" ¯The standards control ARN. See the AWS documentation for how to list existing controls using [`get-enabled-standards`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/securityhub/get-enabled-standards.html) and [`describe-standards-controls`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/securityhub/describe-standards-controls.html).
")
title" The standard control title.
*Ω5
s
securityhubStandardsControlAssociationGaws:securityhub/standardsControlAssociation:StandardsControlAssociation›+## Example Usage

### Basic usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const cisAwsFoundationsBenchmark = new aws.securityhub.StandardsSubscription("cis_aws_foundations_benchmark", {standardsArn: "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0"}, {
    dependsOn: [example],
});
const cisAwsFoundationsBenchmarkDisableIam1 = new aws.securityhub.StandardsControlAssociation("cis_aws_foundations_benchmark_disable_iam_1", {
    standardsArn: cisAwsFoundationsBenchmark.standardsArn,
    securityControlId: "IAM.1",
    associationStatus: "DISABLED",
    updatedReason: "Not needed",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
cis_aws_foundations_benchmark = aws.securityhub.StandardsSubscription("cis_aws_foundations_benchmark", standards_arn="arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
opts = pulumi.ResourceOptions(depends_on=[example]))
cis_aws_foundations_benchmark_disable_iam1 = aws.securityhub.StandardsControlAssociation("cis_aws_foundations_benchmark_disable_iam_1",
    standards_arn=cis_aws_foundations_benchmark.standards_arn,
    security_control_id="IAM.1",
    association_status="DISABLED",
    updated_reason="Not needed")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var cisAwsFoundationsBenchmark = new Aws.SecurityHub.StandardsSubscription("cis_aws_foundations_benchmark", new()
    {
        StandardsArn = "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

    var cisAwsFoundationsBenchmarkDisableIam1 = new Aws.SecurityHub.StandardsControlAssociation("cis_aws_foundations_benchmark_disable_iam_1", new()
    {
        StandardsArn = cisAwsFoundationsBenchmark.StandardsArn,
        SecurityControlId = "IAM.1",
        AssociationStatus = "DISABLED",
        UpdatedReason = "Not needed",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		cisAwsFoundationsBenchmark, err := securityhub.NewStandardsSubscription(ctx, "cis_aws_foundations_benchmark", &securityhub.StandardsSubscriptionArgs{
			StandardsArn: pulumi.String("arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
		}))
		if err != nil {
			return err
		}
		_, err = securityhub.NewStandardsControlAssociation(ctx, "cis_aws_foundations_benchmark_disable_iam_1", &securityhub.StandardsControlAssociationArgs{
			StandardsArn:      cisAwsFoundationsBenchmark.StandardsArn,
			SecurityControlId: pulumi.String("IAM.1"),
			AssociationStatus: pulumi.String("DISABLED"),
			UpdatedReason:     pulumi.String("Not needed"),
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.StandardsSubscription;
import com.pulumi.aws.securityhub.StandardsSubscriptionArgs;
import com.pulumi.aws.securityhub.StandardsControlAssociation;
import com.pulumi.aws.securityhub.StandardsControlAssociationArgs;
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
        var example = new Account("example");

        var cisAwsFoundationsBenchmark = new StandardsSubscription("cisAwsFoundationsBenchmark", StandardsSubscriptionArgs.builder()
            .standardsArn("arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

        var cisAwsFoundationsBenchmarkDisableIam1 = new StandardsControlAssociation("cisAwsFoundationsBenchmarkDisableIam1", StandardsControlAssociationArgs.builder()
            .standardsArn(cisAwsFoundationsBenchmark.standardsArn())
            .securityControlId("IAM.1")
            .associationStatus("DISABLED")
            .updatedReason("Not needed")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  cisAwsFoundationsBenchmark:
    type: aws:securityhub:StandardsSubscription
    name: cis_aws_foundations_benchmark
    properties:
      standardsArn: arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0
    options:
      dependsOn:
        - ${example}
  cisAwsFoundationsBenchmarkDisableIam1:
    type: aws:securityhub:StandardsControlAssociation
    name: cis_aws_foundations_benchmark_disable_iam_1
    properties:
      standardsArn: ${cisAwsFoundationsBenchmark.standardsArn}
      securityControlId: IAM.1
      associationStatus: DISABLED
      updatedReason: Not needed
```
<!--End PulumiCodeChooser -->

|
associationStatus" cThe desired enablement status of the control in the standard. Valid values: `ENABLED`, `DISABLED`.
t
securityControlId" [The unique identifier for the security control whose enablement status you want to update.
®
standardsArn" ìThe Amazon Resource Name (ARN) of the standard in which you want to update the control's enablement status.

The following arguments are optional:
í
updatedReasonB" {The reason for updating the control's enablement status in the standard. Required when `association_status` is `DISABLED`.
"|
associationStatus" cThe desired enablement status of the control in the standard. Valid values: `ENABLED`, `DISABLED`.
"t
securityControlId" [The unique identifier for the security control whose enablement status you want to update.
"®
standardsArn" ìThe Amazon Resource Name (ARN) of the standard in which you want to update the control's enablement status.

The following arguments are optional:
"í
updatedReasonB" {The reason for updating the control's enablement status in the standard. Required when `association_status` is `DISABLED`.
*°G
a
securityhubStandardsSubscription;aws:securityhub/standardsSubscription:StandardsSubscriptionÛ-Subscribes to a Security Hub standard.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securityhub.Account("example", {});
const current = aws.getRegion({});
const cis = new aws.securityhub.StandardsSubscription("cis", {standardsArn: "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0"}, {
    dependsOn: [example],
});
const pci321 = new aws.securityhub.StandardsSubscription("pci_321", {standardsArn: current.then(current => `arn:aws:securityhub:${current.name}::standards/pci-dss/v/3.2.1`)}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securityhub.Account("example")
current = aws.get_region()
cis = aws.securityhub.StandardsSubscription("cis", standards_arn="arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
opts = pulumi.ResourceOptions(depends_on=[example]))
pci321 = aws.securityhub.StandardsSubscription("pci_321", standards_arn=f"arn:aws:securityhub:{current.name}::standards/pci-dss/v/3.2.1",
opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityHub.Account("example");

    var current = Aws.GetRegion.Invoke();

    var cis = new Aws.SecurityHub.StandardsSubscription("cis", new()
    {
        StandardsArn = "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

    var pci321 = new Aws.SecurityHub.StandardsSubscription("pci_321", new()
    {
        StandardsArn = $"arn:aws:securityhub:{current.Apply(getRegionResult => getRegionResult.Name)}::standards/pci-dss/v/3.2.1",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := securityhub.NewAccount(ctx, "example", nil)
		if err != nil {
			return err
		}
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = securityhub.NewStandardsSubscription(ctx, "cis", &securityhub.StandardsSubscriptionArgs{
			StandardsArn: pulumi.String("arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
		}))
		if err != nil {
			return err
		}
		_, err = securityhub.NewStandardsSubscription(ctx, "pci_321", &securityhub.StandardsSubscriptionArgs{
			StandardsArn: pulumi.Sprintf("arn:aws:securityhub:%v::standards/pci-dss/v/3.2.1", current.Name),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.securityhub.StandardsSubscription;
import com.pulumi.aws.securityhub.StandardsSubscriptionArgs;
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
        var example = new Account("example");

        final var current = AwsFunctions.getRegion();

        var cis = new StandardsSubscription("cis", StandardsSubscriptionArgs.builder()
            .standardsArn("arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

        var pci321 = new StandardsSubscription("pci321", StandardsSubscriptionArgs.builder()
            .standardsArn(String.format("arn:aws:securityhub:%s::standards/pci-dss/v/3.2.1", current.applyValue(getRegionResult -> getRegionResult.name())))
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securityhub:Account
  cis:
    type: aws:securityhub:StandardsSubscription
    properties:
      standardsArn: arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0
    options:
      dependsOn:
        - ${example}
  pci321:
    type: aws:securityhub:StandardsSubscription
    name: pci_321
    properties:
      standardsArn: arn:aws:securityhub:${current.name}::standards/pci-dss/v/3.2.1
    options:
      dependsOn:
        - ${example}
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Hub standards subscriptions using the standards subscription ARN. For example:

```sh
$ pulumi import aws:securityhub/standardsSubscription:StandardsSubscription cis arn:aws:securityhub:eu-west-1:123456789012:subscription/cis-aws-foundations-benchmark/v/1.2.0
```
```sh
$ pulumi import aws:securityhub/standardsSubscription:StandardsSubscription pci_321 arn:aws:securityhub:eu-west-1:123456789012:subscription/pci-dss/v/3.2.1
```
```sh
$ pulumi import aws:securityhub/standardsSubscription:StandardsSubscription nist_800_53_rev_5 arn:aws:securityhub:eu-west-1:123456789012:subscription/nist-800-53/v/5.0.0
```
°
standardsArn" åThe ARN of a standard - see below.

Currently available standards (remember to replace `${var.partition}` and `${var.region}` as appropriate):

| Name                                     | ARN                                                                                                          |
|------------------------------------------|--------------------------------------------------------------------------------------------------------------|
| AWS Foundational Security Best Practices | `arn:${var.partition}:securityhub:${var.region}::standards/aws-foundational-security-best-practices/v/1.0.0` |
| AWS Resource Tagging Standard            | `arn:${var.partition}:securityhub:${var.region}::standards/aws-resource-tagging-standard/v/1.0.0`            |
| CIS AWS Foundations Benchmark v1.2.0     | `arn:${var.partition}:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0`                           |
| CIS AWS Foundations Benchmark v1.4.0     | `arn:${var.partition}:securityhub:${var.region}::standards/cis-aws-foundations-benchmark/v/1.4.0`            |
| CIS AWS Foundations Benchmark v3.0.0     | `arn:${var.partition}:securityhub:${var.region}::standards/cis-aws-foundations-benchmark/v/3.0.0`            |
| NIST SP 800-53 Rev. 5                    | `arn:${var.partition}:securityhub:${var.region}::standards/nist-800-53/v/5.0.0`                              |
| PCI DSS                                  | `arn:${var.partition}:securityhub:${var.region}::standards/pci-dss/v/3.2.1`                                  |
"°
standardsArn" åThe ARN of a standard - see below.

Currently available standards (remember to replace `${var.partition}` and `${var.region}` as appropriate):

| Name                                     | ARN                                                                                                          |
|------------------------------------------|--------------------------------------------------------------------------------------------------------------|
| AWS Foundational Security Best Practices | `arn:${var.partition}:securityhub:${var.region}::standards/aws-foundational-security-best-practices/v/1.0.0` |
| AWS Resource Tagging Standard            | `arn:${var.partition}:securityhub:${var.region}::standards/aws-resource-tagging-standard/v/1.0.0`            |
| CIS AWS Foundations Benchmark v1.2.0     | `arn:${var.partition}:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0`                           |
| CIS AWS Foundations Benchmark v1.4.0     | `arn:${var.partition}:securityhub:${var.region}::standards/cis-aws-foundations-benchmark/v/1.4.0`            |
| CIS AWS Foundations Benchmark v3.0.0     | `arn:${var.partition}:securityhub:${var.region}::standards/cis-aws-foundations-benchmark/v/3.0.0`            |
| NIST SP 800-53 Rev. 5                    | `arn:${var.partition}:securityhub:${var.region}::standards/nist-800-53/v/5.0.0`                              |
| PCI DSS                                  | `arn:${var.partition}:securityhub:${var.region}::standards/pci-dss/v/3.2.1`                                  |
*ú"
H
securitylakeAwsLogSource*aws:securitylake/awsLogSource:AwsLogSource—Resource for managing an Amazon Security Lake AWS Log Source.

> **NOTE:** A single `aws.securitylake.AwsLogSource` should be used to configure a log source across all regions and accounts.

> **NOTE:** The underlying `aws.securitylake.DataLake` must be configured before creating the `aws.securitylake.AwsLogSource`. Use a `depends_on` statement.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securitylake.AwsLogSource("example", {source: {
    accounts: ["123456789012"],
    regions: ["eu-west-1"],
    sourceName: "ROUTE53",
}}, {
    dependsOn: [exampleAwsSecuritylakeDataLake],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securitylake.AwsLogSource("example", source={
    "accounts": ["123456789012"],
    "regions": ["eu-west-1"],
    "source_name": "ROUTE53",
},
opts = pulumi.ResourceOptions(depends_on=[example_aws_securitylake_data_lake]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityLake.AwsLogSource("example", new()
    {
        Source = new Aws.SecurityLake.Inputs.AwsLogSourceSourceArgs
        {
            Accounts = new[]
            {
                "123456789012",
            },
            Regions = new[]
            {
                "eu-west-1",
            },
            SourceName = "ROUTE53",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSecuritylakeDataLake,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securitylake"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securitylake.NewAwsLogSource(ctx, "example", &securitylake.AwsLogSourceArgs{
			Source: &securitylake.AwsLogSourceSourceArgs{
				Accounts: pulumi.StringArray{
					pulumi.String("123456789012"),
				},
				Regions: pulumi.StringArray{
					pulumi.String("eu-west-1"),
				},
				SourceName: pulumi.String("ROUTE53"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSecuritylakeDataLake,
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
import com.pulumi.aws.securitylake.AwsLogSource;
import com.pulumi.aws.securitylake.AwsLogSourceArgs;
import com.pulumi.aws.securitylake.inputs.AwsLogSourceSourceArgs;
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
        var example = new AwsLogSource("example", AwsLogSourceArgs.builder()
            .source(AwsLogSourceSourceArgs.builder()
                .accounts("123456789012")
                .regions("eu-west-1")
                .sourceName("ROUTE53")
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSecuritylakeDataLake)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securitylake:AwsLogSource
    properties:
      source:
        accounts:
          - '123456789012'
        regions:
          - eu-west-1
        sourceName: ROUTE53
    options:
      dependsOn:
        - ${exampleAwsSecuritylakeDataLake}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AWS log sources using the source name. For example:

```sh
$ pulumi import aws:securitylake/awsLogSource:AwsLogSource example ROUTE53
```
º
source`B^:\
Z
securitylakeAwsLogSourceSource6aws:securitylake/AwsLogSourceSource:AwsLogSourceSourcePSpecify the natively-supported AWS service to add as a source in Security Lake.
"º
source`B^:\
Z
securitylakeAwsLogSourceSource6aws:securitylake/AwsLogSourceSource:AwsLogSourceSourcePSpecify the natively-supported AWS service to add as a source in Security Lake.
*è;
Q
securitylakeCustomLogSource0aws:securitylake/customLogSource:CustomLogSourceÓ,Resource for managing an AWS Security Lake Custom Log Source.

> **NOTE:** The underlying `aws.securitylake.DataLake` must be configured before creating the `aws.securitylake.CustomLogSource`. Use a `depends_on` statement.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securitylake.CustomLogSource("example", {
    sourceName: "example-name",
    sourceVersion: "1.0",
    eventClasses: ["FILE_ACTIVITY"],
    configuration: {
        crawlerConfiguration: {
            roleArn: customLog.arn,
        },
        providerIdentity: {
            externalId: "example-id",
            principal: "123456789012",
        },
    },
}, {
    dependsOn: [exampleAwsSecuritylakeDataLake],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securitylake.CustomLogSource("example",
    source_name="example-name",
    source_version="1.0",
    event_classes=["FILE_ACTIVITY"],
    configuration={
        "crawler_configuration": {
            "role_arn": custom_log["arn"],
        },
        "provider_identity": {
            "external_id": "example-id",
            "principal": "123456789012",
        },
    },
    opts = pulumi.ResourceOptions(depends_on=[example_aws_securitylake_data_lake]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityLake.CustomLogSource("example", new()
    {
        SourceName = "example-name",
        SourceVersion = "1.0",
        EventClasses = new[]
        {
            "FILE_ACTIVITY",
        },
        Configuration = new Aws.SecurityLake.Inputs.CustomLogSourceConfigurationArgs
        {
            CrawlerConfiguration = new Aws.SecurityLake.Inputs.CustomLogSourceConfigurationCrawlerConfigurationArgs
            {
                RoleArn = customLog.Arn,
            },
            ProviderIdentity = new Aws.SecurityLake.Inputs.CustomLogSourceConfigurationProviderIdentityArgs
            {
                ExternalId = "example-id",
                Principal = "123456789012",
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSecuritylakeDataLake,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securitylake"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securitylake.NewCustomLogSource(ctx, "example", &securitylake.CustomLogSourceArgs{
			SourceName:    pulumi.String("example-name"),
			SourceVersion: pulumi.String("1.0"),
			EventClasses: pulumi.StringArray{
				pulumi.String("FILE_ACTIVITY"),
			},
			Configuration: &securitylake.CustomLogSourceConfigurationArgs{
				CrawlerConfiguration: &securitylake.CustomLogSourceConfigurationCrawlerConfigurationArgs{
					RoleArn: pulumi.Any(customLog.Arn),
				},
				ProviderIdentity: &securitylake.CustomLogSourceConfigurationProviderIdentityArgs{
					ExternalId: pulumi.String("example-id"),
					Principal:  pulumi.String("123456789012"),
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSecuritylakeDataLake,
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
import com.pulumi.aws.securitylake.CustomLogSource;
import com.pulumi.aws.securitylake.CustomLogSourceArgs;
import com.pulumi.aws.securitylake.inputs.CustomLogSourceConfigurationArgs;
import com.pulumi.aws.securitylake.inputs.CustomLogSourceConfigurationCrawlerConfigurationArgs;
import com.pulumi.aws.securitylake.inputs.CustomLogSourceConfigurationProviderIdentityArgs;
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
        var example = new CustomLogSource("example", CustomLogSourceArgs.builder()
            .sourceName("example-name")
            .sourceVersion("1.0")
            .eventClasses("FILE_ACTIVITY")
            .configuration(CustomLogSourceConfigurationArgs.builder()
                .crawlerConfiguration(CustomLogSourceConfigurationCrawlerConfigurationArgs.builder()
                    .roleArn(customLog.arn())
                    .build())
                .providerIdentity(CustomLogSourceConfigurationProviderIdentityArgs.builder()
                    .externalId("example-id")
                    .principal("123456789012")
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSecuritylakeDataLake)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securitylake:CustomLogSource
    properties:
      sourceName: example-name
      sourceVersion: '1.0'
      eventClasses:
        - FILE_ACTIVITY
      configuration:
        crawlerConfiguration:
          roleArn: ${customLog.arn}
        providerIdentity:
          externalId: example-id
          principal: '123456789012'
    options:
      dependsOn:
        - ${exampleAwsSecuritylakeDataLake}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Custom log sources using the source name. For example:

```sh
$ pulumi import aws:securitylake/customLogSource:CustomLogSource example example-name
```
∆
configuration~B|:z
x
securitylakeCustomLogSourceConfigurationJaws:securitylake/CustomLogSourceConfiguration:CustomLogSourceConfiguration5The configuration for the third-party custom source.
™
eventClassesB*" ëThe Open Cybersecurity Schema Framework (OCSF) event classes which describes the type of data that the custom source will send to Security Lake.
à

sourceName" vSpecify the name for a third-party custom source.
This must be a Regionally unique value.
Has a maximum length of 20.
õ
sourceVersionB" ÉSpecify the source version for the third-party custom source, to limit log collection to a specific version of custom data source.
"±

attributesr*p:n
l
securitylakeCustomLogSourceAttributeBaws:securitylake/CustomLogSourceAttribute:CustomLogSourceAttribute/The attributes of a third-party custom source.
"∆
configuration~B|:z
x
securitylakeCustomLogSourceConfigurationJaws:securitylake/CustomLogSourceConfiguration:CustomLogSourceConfiguration5The configuration for the third-party custom source.
"™
eventClassesB*" ëThe Open Cybersecurity Schema Framework (OCSF) event classes which describes the type of data that the custom source will send to Security Lake.
"ÿ
providerDetailsÅ*:}
{
securitylakeCustomLogSourceProviderDetailLaws:securitylake/CustomLogSourceProviderDetail:CustomLogSourceProviderDetailAThe details of the log provider for a third-party custom source.
"à

sourceName" vSpecify the name for a third-party custom source.
This must be a Regionally unique value.
Has a maximum length of 20.
"ô
sourceVersion" ÉSpecify the source version for the third-party custom source, to limit log collection to a specific version of custom data source.
*€c
<
securitylakeDataLake"aws:securitylake/dataLake:DataLake€UResource for managing an AWS Security Lake Data Lake.

> **NOTE:** The underlying `aws.securitylake.DataLake` must be configured before creating other Security Lake resources. Use a `depends_on` statement.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securitylake.DataLake("example", {
    metaStoreManagerRoleArn: metaStoreManager.arn,
    configuration: {
        region: "eu-west-1",
        encryptionConfigurations: [{
            kmsKeyId: "S3_MANAGED_KEY",
        }],
        lifecycleConfiguration: {
            transitions: [
                {
                    days: 31,
                    storageClass: "STANDARD_IA",
                },
                {
                    days: 80,
                    storageClass: "ONEZONE_IA",
                },
            ],
            expiration: {
                days: 300,
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securitylake.DataLake("example",
    meta_store_manager_role_arn=meta_store_manager["arn"],
    configuration={
        "region": "eu-west-1",
        "encryption_configurations": [{
            "kms_key_id": "S3_MANAGED_KEY",
        }],
        "lifecycle_configuration": {
            "transitions": [
                {
                    "days": 31,
                    "storage_class": "STANDARD_IA",
                },
                {
                    "days": 80,
                    "storage_class": "ONEZONE_IA",
                },
            ],
            "expiration": {
                "days": 300,
            },
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
    var example = new Aws.SecurityLake.DataLake("example", new()
    {
        MetaStoreManagerRoleArn = metaStoreManager.Arn,
        Configuration = new Aws.SecurityLake.Inputs.DataLakeConfigurationArgs
        {
            Region = "eu-west-1",
            EncryptionConfigurations = new[]
            {
                new Aws.SecurityLake.Inputs.DataLakeConfigurationEncryptionConfigurationArgs
                {
                    KmsKeyId = "S3_MANAGED_KEY",
                },
            },
            LifecycleConfiguration = new Aws.SecurityLake.Inputs.DataLakeConfigurationLifecycleConfigurationArgs
            {
                Transitions = new[]
                {
                    new Aws.SecurityLake.Inputs.DataLakeConfigurationLifecycleConfigurationTransitionArgs
                    {
                        Days = 31,
                        StorageClass = "STANDARD_IA",
                    },
                    new Aws.SecurityLake.Inputs.DataLakeConfigurationLifecycleConfigurationTransitionArgs
                    {
                        Days = 80,
                        StorageClass = "ONEZONE_IA",
                    },
                },
                Expiration = new Aws.SecurityLake.Inputs.DataLakeConfigurationLifecycleConfigurationExpirationArgs
                {
                    Days = 300,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securitylake"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securitylake.NewDataLake(ctx, "example", &securitylake.DataLakeArgs{
			MetaStoreManagerRoleArn: pulumi.Any(metaStoreManager.Arn),
			Configuration: &securitylake.DataLakeConfigurationArgs{
				Region: pulumi.String("eu-west-1"),
				EncryptionConfigurations: securitylake.DataLakeConfigurationEncryptionConfigurationArray{
					&securitylake.DataLakeConfigurationEncryptionConfigurationArgs{
						KmsKeyId: pulumi.String("S3_MANAGED_KEY"),
					},
				},
				LifecycleConfiguration: &securitylake.DataLakeConfigurationLifecycleConfigurationArgs{
					Transitions: securitylake.DataLakeConfigurationLifecycleConfigurationTransitionArray{
						&securitylake.DataLakeConfigurationLifecycleConfigurationTransitionArgs{
							Days:         pulumi.Int(31),
							StorageClass: pulumi.String("STANDARD_IA"),
						},
						&securitylake.DataLakeConfigurationLifecycleConfigurationTransitionArgs{
							Days:         pulumi.Int(80),
							StorageClass: pulumi.String("ONEZONE_IA"),
						},
					},
					Expiration: &securitylake.DataLakeConfigurationLifecycleConfigurationExpirationArgs{
						Days: pulumi.Int(300),
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
import com.pulumi.aws.securitylake.DataLake;
import com.pulumi.aws.securitylake.DataLakeArgs;
import com.pulumi.aws.securitylake.inputs.DataLakeConfigurationArgs;
import com.pulumi.aws.securitylake.inputs.DataLakeConfigurationLifecycleConfigurationArgs;
import com.pulumi.aws.securitylake.inputs.DataLakeConfigurationLifecycleConfigurationExpirationArgs;
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
        var example = new DataLake("example", DataLakeArgs.builder()
            .metaStoreManagerRoleArn(metaStoreManager.arn())
            .configuration(DataLakeConfigurationArgs.builder()
                .region("eu-west-1")
                .encryptionConfigurations(DataLakeConfigurationEncryptionConfigurationArgs.builder()
                    .kmsKeyId("S3_MANAGED_KEY")
                    .build())
                .lifecycleConfiguration(DataLakeConfigurationLifecycleConfigurationArgs.builder()
                    .transitions(                    
                        DataLakeConfigurationLifecycleConfigurationTransitionArgs.builder()
                            .days(31)
                            .storageClass("STANDARD_IA")
                            .build(),
                        DataLakeConfigurationLifecycleConfigurationTransitionArgs.builder()
                            .days(80)
                            .storageClass("ONEZONE_IA")
                            .build())
                    .expiration(DataLakeConfigurationLifecycleConfigurationExpirationArgs.builder()
                        .days(300)
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
    type: aws:securitylake:DataLake
    properties:
      metaStoreManagerRoleArn: ${metaStoreManager.arn}
      configuration:
        region: eu-west-1
        encryptionConfigurations:
          - kmsKeyId: S3_MANAGED_KEY
        lifecycleConfiguration:
          transitions:
            - days: 31
              storageClass: STANDARD_IA
            - days: 80
              storageClass: ONEZONE_IA
          expiration:
            days: 300
```
<!--End PulumiCodeChooser -->

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securitylake.DataLake("example", {
    metaStoreManagerRoleArn: metaStoreManager.arn,
    configuration: {
        region: "eu-west-1",
        encryptionConfigurations: [{
            kmsKeyId: "S3_MANAGED_KEY",
        }],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securitylake.DataLake("example",
    meta_store_manager_role_arn=meta_store_manager["arn"],
    configuration={
        "region": "eu-west-1",
        "encryption_configurations": [{
            "kms_key_id": "S3_MANAGED_KEY",
        }],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityLake.DataLake("example", new()
    {
        MetaStoreManagerRoleArn = metaStoreManager.Arn,
        Configuration = new Aws.SecurityLake.Inputs.DataLakeConfigurationArgs
        {
            Region = "eu-west-1",
            EncryptionConfigurations = new[]
            {
                new Aws.SecurityLake.Inputs.DataLakeConfigurationEncryptionConfigurationArgs
                {
                    KmsKeyId = "S3_MANAGED_KEY",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securitylake"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securitylake.NewDataLake(ctx, "example", &securitylake.DataLakeArgs{
			MetaStoreManagerRoleArn: pulumi.Any(metaStoreManager.Arn),
			Configuration: &securitylake.DataLakeConfigurationArgs{
				Region: pulumi.String("eu-west-1"),
				EncryptionConfigurations: securitylake.DataLakeConfigurationEncryptionConfigurationArray{
					&securitylake.DataLakeConfigurationEncryptionConfigurationArgs{
						KmsKeyId: pulumi.String("S3_MANAGED_KEY"),
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
import com.pulumi.aws.securitylake.DataLake;
import com.pulumi.aws.securitylake.DataLakeArgs;
import com.pulumi.aws.securitylake.inputs.DataLakeConfigurationArgs;
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
        var example = new DataLake("example", DataLakeArgs.builder()
            .metaStoreManagerRoleArn(metaStoreManager.arn())
            .configuration(DataLakeConfigurationArgs.builder()
                .region("eu-west-1")
                .encryptionConfigurations(DataLakeConfigurationEncryptionConfigurationArgs.builder()
                    .kmsKeyId("S3_MANAGED_KEY")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securitylake:DataLake
    properties:
      metaStoreManagerRoleArn: ${metaStoreManager.arn}
      configuration:
        region: eu-west-1
        encryptionConfigurations:
          - kmsKeyId: S3_MANAGED_KEY
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Hub standards subscriptions using the standards subscription ARN. For example:

```sh
$ pulumi import aws:securitylake/dataLake:DataLake example arn:aws:securitylake:eu-west-1:123456789012:data-lake/default
```
 
configurationiBg:e
c
securitylakeDataLakeConfiguration<aws:securitylake/DataLakeConfiguration:DataLakeConfigurationNSpecify the Region or Regions that will contribute data to the rollup region.
‡
metaStoreManagerRoleArn" ¿The Amazon Resource Name (ARN) used to create and update the AWS Glue table. This table contains partitions generated by the ingestion and normalization of AWS log sources and custom sources.
«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
f
timeoutsZBX:V
T
securitylakeDataLakeTimeouts2aws:securitylake/DataLakeTimeouts:DataLakeTimeouts"!
arn" ARN of the Data Lake.
" 
configurationiBg:e
c
securitylakeDataLakeConfiguration<aws:securitylake/DataLakeConfiguration:DataLakeConfigurationNSpecify the Region or Regions that will contribute data to the rollup region.
"‡
metaStoreManagerRoleArn" ¿The Amazon Resource Name (ARN) used to create and update the AWS Glue table. This table contains partitions generated by the ingestion and normalization of AWS log sources and custom sources.
"J
s3BucketArn" 7The ARN for the Amazon Security Lake Amazon S3 bucket.
"«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"f
timeoutsZBX:V
T
securitylakeDataLakeTimeouts2aws:securitylake/DataLakeTimeouts:DataLakeTimeouts*ÖA
B
securitylake
Subscriber&aws:securitylake/subscriber:Subscriber†)Resource for managing an AWS Security Lake Subscriber.

> **NOTE:** The underlying `aws.securitylake.DataLake` must be configured before creating the `aws.securitylake.Subscriber`. Use a `depends_on` statement.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securitylake.Subscriber("example", {
    subscriberName: "example-name",
    accessType: "S3",
    source: {
        awsLogSourceResource: {
            sourceName: "ROUTE53",
            sourceVersion: "1.0",
        },
    },
    subscriberIdentity: {
        externalId: "example",
        principal: "1234567890",
    },
}, {
    dependsOn: [exampleAwsSecuritylakeDataLake],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securitylake.Subscriber("example",
    subscriber_name="example-name",
    access_type="S3",
    source={
        "aws_log_source_resource": {
            "source_name": "ROUTE53",
            "source_version": "1.0",
        },
    },
    subscriber_identity={
        "external_id": "example",
        "principal": "1234567890",
    },
    opts = pulumi.ResourceOptions(depends_on=[example_aws_securitylake_data_lake]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityLake.Subscriber("example", new()
    {
        SubscriberName = "example-name",
        AccessType = "S3",
        Source = new Aws.SecurityLake.Inputs.SubscriberSourceArgs
        {
            AwsLogSourceResource = new Aws.SecurityLake.Inputs.SubscriberSourceAwsLogSourceResourceArgs
            {
                SourceName = "ROUTE53",
                SourceVersion = "1.0",
            },
        },
        SubscriberIdentity = new Aws.SecurityLake.Inputs.SubscriberSubscriberIdentityArgs
        {
            ExternalId = "example",
            Principal = "1234567890",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSecuritylakeDataLake,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securitylake"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securitylake.NewSubscriber(ctx, "example", &securitylake.SubscriberArgs{
			SubscriberName: pulumi.String("example-name"),
			AccessType:     pulumi.String("S3"),
			Source: &securitylake.SubscriberSourceArgs{
				AwsLogSourceResource: &securitylake.SubscriberSourceAwsLogSourceResourceArgs{
					SourceName:    pulumi.String("ROUTE53"),
					SourceVersion: pulumi.String("1.0"),
				},
			},
			SubscriberIdentity: &securitylake.SubscriberSubscriberIdentityArgs{
				ExternalId: pulumi.String("example"),
				Principal:  pulumi.String("1234567890"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSecuritylakeDataLake,
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
import com.pulumi.aws.securitylake.Subscriber;
import com.pulumi.aws.securitylake.SubscriberArgs;
import com.pulumi.aws.securitylake.inputs.SubscriberSourceArgs;
import com.pulumi.aws.securitylake.inputs.SubscriberSourceAwsLogSourceResourceArgs;
import com.pulumi.aws.securitylake.inputs.SubscriberSubscriberIdentityArgs;
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
        var example = new Subscriber("example", SubscriberArgs.builder()
            .subscriberName("example-name")
            .accessType("S3")
            .source(SubscriberSourceArgs.builder()
                .awsLogSourceResource(SubscriberSourceAwsLogSourceResourceArgs.builder()
                    .sourceName("ROUTE53")
                    .sourceVersion("1.0")
                    .build())
                .build())
            .subscriberIdentity(SubscriberSubscriberIdentityArgs.builder()
                .externalId("example")
                .principal("1234567890")
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSecuritylakeDataLake)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securitylake:Subscriber
    properties:
      subscriberName: example-name
      accessType: S3
      source:
        awsLogSourceResource:
          sourceName: ROUTE53
          sourceVersion: '1.0'
      subscriberIdentity:
        externalId: example
        principal: '1234567890'
    options:
      dependsOn:
        - ${exampleAwsSecuritylakeDataLake}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Security Lake subscriber using the subscriber ID. For example:

```sh
$ pulumi import aws:securitylake/subscriber:Subscriber example 9f3bfe79-d543-474d-a93c-f3846805d208
```
A

accessTypeB" -The Amazon S3 or Lake Formation access type.
ú
sourceZBX:V
T
securitylakeSubscriberSource2aws:securitylake/SubscriberSource:SubscriberSourceµThe supported AWS services from which logs and events are collected. Security Lake supports log and event collection for natively supported AWS services. See `source` Blocks below.
]
subscriberDescriptionB" >The description for your subscriber account in Security Lake.
Ë
subscriberIdentity~B|:z
x
securitylakeSubscriberSubscriberIdentityJaws:securitylake/SubscriberSubscriberIdentity:SubscriberSubscriberIdentityRThe AWS identity used to access your data. See `subscriber_identity` Block below.
K
subscriberNameB" 3The name of your Security Lake subscriber account.
«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
l
timeouts`B^:\
Z
securitylakeSubscriberTimeouts6aws:securitylake/SubscriberTimeouts:SubscriberTimeouts"?

accessType" -The Amazon S3 or Lake Formation access type.
"!
arn" ARN of the Data Lake.
"◊
resourceShareArn" æThe Amazon Resource Name (ARN) which uniquely defines the AWS RAM resource share. Before accepting the RAM resource share invitation, you can view details related to the RAM resource share.
"9
resourceShareName"  The name of the resource share.
"p
roleArn" aThe ARN of the IAM role to be used by the entity putting logs into your custom source partition.
"J
s3BucketArn" 7The ARN for the Amazon Security Lake Amazon S3 bucket.
"ú
sourceZBX:V
T
securitylakeSubscriberSource2aws:securitylake/SubscriberSource:SubscriberSourceµThe supported AWS services from which logs and events are collected. Security Lake supports log and event collection for natively supported AWS services. See `source` Blocks below.
"]
subscriberDescriptionB" >The description for your subscriber account in Security Lake.
"Z
subscriberEndpoint" @The subscriber endpoint to which exception messages are posted.
"Ë
subscriberIdentity~B|:z
x
securitylakeSubscriberSubscriberIdentityJaws:securitylake/SubscriberSubscriberIdentity:SubscriberSubscriberIdentityRThe AWS identity used to access your data. See `subscriber_identity` Block below.
"K
subscriberNameB" 3The name of your Security Lake subscriber account.
"^
subscriberStatus" FThe subscriber status of the Amazon Security Lake subscriber account.
"«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"l
timeouts`B^:\
Z
securitylakeSubscriberTimeouts6aws:securitylake/SubscriberTimeouts:SubscriberTimeouts*ô>
f
securitylakeSubscriberNotification>aws:securitylake/subscriberNotification:SubscriberNotificationœ6Resource for managing an AWS Security Lake Subscriber Notification.

## Example Usage

### SQS Notification

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securitylake.SubscriberNotification("example", {
    subscriberId: exampleAwsSecuritylakeSubscriber.id,
    configuration: {
        sqsNotificationConfiguration: {},
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securitylake.SubscriberNotification("example",
    subscriber_id=example_aws_securitylake_subscriber["id"],
    configuration={
        "sqs_notification_configuration": {},
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SecurityLake.SubscriberNotification("example", new()
    {
        SubscriberId = exampleAwsSecuritylakeSubscriber.Id,
        Configuration = new Aws.SecurityLake.Inputs.SubscriberNotificationConfigurationArgs
        {
            SqsNotificationConfiguration = null,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securitylake"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securitylake.NewSubscriberNotification(ctx, "example", &securitylake.SubscriberNotificationArgs{
			SubscriberId: pulumi.Any(exampleAwsSecuritylakeSubscriber.Id),
			Configuration: &securitylake.SubscriberNotificationConfigurationArgs{
				SqsNotificationConfiguration: &securitylake.SubscriberNotificationConfigurationSqsNotificationConfigurationArgs{},
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
import com.pulumi.aws.securitylake.SubscriberNotification;
import com.pulumi.aws.securitylake.SubscriberNotificationArgs;
import com.pulumi.aws.securitylake.inputs.SubscriberNotificationConfigurationArgs;
import com.pulumi.aws.securitylake.inputs.SubscriberNotificationConfigurationSqsNotificationConfigurationArgs;
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
        var example = new SubscriberNotification("example", SubscriberNotificationArgs.builder()
            .subscriberId(exampleAwsSecuritylakeSubscriber.id())
            .configuration(SubscriberNotificationConfigurationArgs.builder()
                .sqsNotificationConfiguration()
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securitylake:SubscriberNotification
    properties:
      subscriberId: ${exampleAwsSecuritylakeSubscriber.id}
      configuration:
        sqsNotificationConfiguration: {}
```
<!--End PulumiCodeChooser -->

### HTTPS Notification

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.securitylake.SubscriberNotification("example", {
    subscriberId: exampleAwsSecuritylakeSubscriber.id,
    configuration: {
        httpsNotificationConfiguration: {
            endpoint: test.apiEndpoint,
            targetRoleArn: eventBridge.arn,
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.securitylake.SubscriberNotification("example",
    subscriber_id=example_aws_securitylake_subscriber["id"],
    configuration={
        "https_notification_configuration": {
            "endpoint": test["apiEndpoint"],
            "target_role_arn": event_bridge["arn"],
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
    var example = new Aws.SecurityLake.SubscriberNotification("example", new()
    {
        SubscriberId = exampleAwsSecuritylakeSubscriber.Id,
        Configuration = new Aws.SecurityLake.Inputs.SubscriberNotificationConfigurationArgs
        {
            HttpsNotificationConfiguration = new Aws.SecurityLake.Inputs.SubscriberNotificationConfigurationHttpsNotificationConfigurationArgs
            {
                Endpoint = test.ApiEndpoint,
                TargetRoleArn = eventBridge.Arn,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securitylake"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securitylake.NewSubscriberNotification(ctx, "example", &securitylake.SubscriberNotificationArgs{
			SubscriberId: pulumi.Any(exampleAwsSecuritylakeSubscriber.Id),
			Configuration: &securitylake.SubscriberNotificationConfigurationArgs{
				HttpsNotificationConfiguration: &securitylake.SubscriberNotificationConfigurationHttpsNotificationConfigurationArgs{
					Endpoint:      pulumi.Any(test.ApiEndpoint),
					TargetRoleArn: pulumi.Any(eventBridge.Arn),
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
import com.pulumi.aws.securitylake.SubscriberNotification;
import com.pulumi.aws.securitylake.SubscriberNotificationArgs;
import com.pulumi.aws.securitylake.inputs.SubscriberNotificationConfigurationArgs;
import com.pulumi.aws.securitylake.inputs.SubscriberNotificationConfigurationHttpsNotificationConfigurationArgs;
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
        var example = new SubscriberNotification("example", SubscriberNotificationArgs.builder()
            .subscriberId(exampleAwsSecuritylakeSubscriber.id())
            .configuration(SubscriberNotificationConfigurationArgs.builder()
                .httpsNotificationConfiguration(SubscriberNotificationConfigurationHttpsNotificationConfigurationArgs.builder()
                    .endpoint(test.apiEndpoint())
                    .targetRoleArn(eventBridge.arn())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:securitylake:SubscriberNotification
    properties:
      subscriberId: ${exampleAwsSecuritylakeSubscriber.id}
      configuration:
        httpsNotificationConfiguration:
          endpoint: ${test.apiEndpoint}
          targetRoleArn: ${eventBridge.arn}
```
<!--End PulumiCodeChooser -->
Å
configurationñBì:ê
ç
securitylake#SubscriberNotificationConfigurationXaws:securitylake/SubscriberNotificationConfiguration:SubscriberNotificationConfigurationWSpecify the configuration using which you want to create the subscriber notification..
I
subscriberId" 5The subscriber ID for the notification subscription.
"Å
configurationñBì:ê
ç
securitylake#SubscriberNotificationConfigurationXaws:securitylake/SubscriberNotificationConfiguration:SubscriberNotificationConfigurationWSpecify the configuration using which you want to create the subscriber notification..
"c

endpointId" Q(**Deprecated**) The subscriber endpoint to which exception messages are posted.
"Z
subscriberEndpoint" @The subscriber endpoint to which exception messages are posted.
"I
subscriberId" 5The subscriber ID for the notification subscription.
*ªA
m
serverlessrepositoryCloudFormationStack@aws:serverlessrepository/cloudFormationStack:CloudFormationStackø2Deploys an Application CloudFormation Stack from the Serverless Application Repository.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getPartition({});
const currentGetRegion = aws.getRegion({});
const postgres_rotator = new aws.serverlessrepository.CloudFormationStack("postgres-rotator", {
    name: "postgres-rotator",
    applicationId: "arn:aws:serverlessrepo:us-east-1:297356227824:applications/SecretsManagerRDSPostgreSQLRotationSingleUser",
    capabilities: [
        "CAPABILITY_IAM",
        "CAPABILITY_RESOURCE_POLICY",
    ],
    parameters: {
        functionName: "func-postgres-rotator",
        endpoint: Promise.all([currentGetRegion, current]).then(([currentGetRegion, current]) => `secretsmanager.${currentGetRegion.name}.${current.dnsSuffix}`),
    },
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_partition()
current_get_region = aws.get_region()
postgres_rotator = aws.serverlessrepository.CloudFormationStack("postgres-rotator",
    name="postgres-rotator",
    application_id="arn:aws:serverlessrepo:us-east-1:297356227824:applications/SecretsManagerRDSPostgreSQLRotationSingleUser",
    capabilities=[
        "CAPABILITY_IAM",
        "CAPABILITY_RESOURCE_POLICY",
    ],
    parameters={
        "functionName": "func-postgres-rotator",
        "endpoint": f"secretsmanager.{current_get_region.name}.{current.dns_suffix}",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetPartition.Invoke();

    var currentGetRegion = Aws.GetRegion.Invoke();

    var postgres_rotator = new Aws.ServerlessRepository.CloudFormationStack("postgres-rotator", new()
    {
        Name = "postgres-rotator",
        ApplicationId = "arn:aws:serverlessrepo:us-east-1:297356227824:applications/SecretsManagerRDSPostgreSQLRotationSingleUser",
        Capabilities = new[]
        {
            "CAPABILITY_IAM",
            "CAPABILITY_RESOURCE_POLICY",
        },
        Parameters = 
        {
            { "functionName", "func-postgres-rotator" },
            { "endpoint", Output.Tuple(currentGetRegion, current).Apply(values =>
            {
                var currentGetRegion = values.Item1;
                var current = values.Item2;
                return $"secretsmanager.{currentGetRegion.Apply(getRegionResult => getRegionResult.Name)}.{current.Apply(getPartitionResult => getPartitionResult.DnsSuffix)}";
            }) },
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/serverlessrepository"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
		if err != nil {
			return err
		}
		currentGetRegion, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = serverlessrepository.NewCloudFormationStack(ctx, "postgres-rotator", &serverlessrepository.CloudFormationStackArgs{
			Name:          pulumi.String("postgres-rotator"),
			ApplicationId: pulumi.String("arn:aws:serverlessrepo:us-east-1:297356227824:applications/SecretsManagerRDSPostgreSQLRotationSingleUser"),
			Capabilities: pulumi.StringArray{
				pulumi.String("CAPABILITY_IAM"),
				pulumi.String("CAPABILITY_RESOURCE_POLICY"),
			},
			Parameters: pulumi.StringMap{
				"functionName": pulumi.String("func-postgres-rotator"),
				"endpoint":     pulumi.Sprintf("secretsmanager.%v.%v", currentGetRegion.Name, current.DnsSuffix),
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
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.serverlessrepository.CloudFormationStack;
import com.pulumi.aws.serverlessrepository.CloudFormationStackArgs;
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

        final var currentGetRegion = AwsFunctions.getRegion();

        var postgres_rotator = new CloudFormationStack("postgres-rotator", CloudFormationStackArgs.builder()
            .name("postgres-rotator")
            .applicationId("arn:aws:serverlessrepo:us-east-1:297356227824:applications/SecretsManagerRDSPostgreSQLRotationSingleUser")
            .capabilities(            
                "CAPABILITY_IAM",
                "CAPABILITY_RESOURCE_POLICY")
            .parameters(Map.ofEntries(
                Map.entry("functionName", "func-postgres-rotator"),
                Map.entry("endpoint", String.format("secretsmanager.%s.%s", currentGetRegion.applyValue(getRegionResult -> getRegionResult.name()),current.applyValue(getPartitionResult -> getPartitionResult.dnsSuffix())))
            ))
            .build());

    }
}
```
```yaml
resources:
  postgres-rotator:
    type: aws:serverlessrepository:CloudFormationStack
    properties:
      name: postgres-rotator
      applicationId: arn:aws:serverlessrepo:us-east-1:297356227824:applications/SecretsManagerRDSPostgreSQLRotationSingleUser
      capabilities:
        - CAPABILITY_IAM
        - CAPABILITY_RESOURCE_POLICY
      parameters:
        functionName: func-postgres-rotator
        endpoint: secretsmanager.${currentGetRegion.name}.${current.dnsSuffix}
variables:
  current:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
  currentGetRegion:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Serverless Application Repository Stack using the CloudFormation Stack name (with or without the `serverlessrepo-` prefix) or the CloudFormation Stack ID. For example:

```sh
$ pulumi import aws:serverlessrepository/cloudFormationStack:CloudFormationStack example serverlessrepo-postgres-rotator
```
\
applicationId" GThe ARN of the application from the Serverless Application Repository.
§
capabilities*" çA list of capabilities. Valid values are `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_RESOURCE_POLICY`, or `CAPABILITY_AUTO_EXPAND`
t
nameB" fThe name of the stack to create. The resource deployed in AWS will be prefixed with `serverlessrepo-`
a

parametersB2" KA map of Parameter structures that specify input parameters for the stack.
p
semanticVersionB" WThe version of the application to deploy. If not supplied, deploys the latest version.
’
tagsB2" ƒA list of tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"\
applicationId" GThe ARN of the application from the Serverless Application Repository.
"§
capabilities*" çA list of capabilities. Valid values are `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_RESOURCE_POLICY`, or `CAPABILITY_AUTO_EXPAND`
"r
name" fThe name of the stack to create. The resource deployed in AWS will be prefixed with `serverlessrepo-`
"2
outputs2" !A map of outputs from the stack.
"_

parameters2" KA map of Parameter structures that specify input parameters for the stack.
"n
semanticVersion" WThe version of the application to deploy. If not supplied, deploys the latest version.
"’
tagsB2" ƒA list of tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*“3
j
servicecatalogAppregistryApplication@aws:servicecatalog/appregistryApplication:AppregistryApplicationõ)Resource for managing an AWS Service Catalog AppRegistry Application.

> An AWS Service Catalog AppRegistry Application is displayed in the AWS Console under "MyApplications".

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.AppregistryApplication("example", {name: "example-app"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.AppregistryApplication("example", name="example-app")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.AppregistryApplication("example", new()
    {
        Name = "example-app",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewAppregistryApplication(ctx, "example", &servicecatalog.AppregistryApplicationArgs{
			Name: pulumi.String("example-app"),
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
import com.pulumi.aws.servicecatalog.AppregistryApplication;
import com.pulumi.aws.servicecatalog.AppregistryApplicationArgs;
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
        var example = new AppregistryApplication("example", AppregistryApplicationArgs.builder()
            .name("example-app")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:AppregistryApplication
    properties:
      name: example-app
```
<!--End PulumiCodeChooser -->

### Connecting Resources

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.AppregistryApplication("example", {name: "example-app"});
const bucket = new aws.s3.BucketV2("bucket", {
    bucket: "example-bucket",
    tags: example.applicationTag,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.AppregistryApplication("example", name="example-app")
bucket = aws.s3.BucketV2("bucket",
    bucket="example-bucket",
    tags=example.application_tag)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.AppregistryApplication("example", new()
    {
        Name = "example-app",
    });

    var bucket = new Aws.S3.BucketV2("bucket", new()
    {
        Bucket = "example-bucket",
        Tags = example.ApplicationTag,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := servicecatalog.NewAppregistryApplication(ctx, "example", &servicecatalog.AppregistryApplicationArgs{
			Name: pulumi.String("example-app"),
		})
		if err != nil {
			return err
		}
		_, err = s3.NewBucketV2(ctx, "bucket", &s3.BucketV2Args{
			Bucket: pulumi.String("example-bucket"),
			Tags:   example.ApplicationTag,
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
import com.pulumi.aws.servicecatalog.AppregistryApplication;
import com.pulumi.aws.servicecatalog.AppregistryApplicationArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
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
        var example = new AppregistryApplication("example", AppregistryApplicationArgs.builder()
            .name("example-app")
            .build());

        var bucket = new BucketV2("bucket", BucketV2Args.builder()
            .bucket("example-bucket")
            .tags(example.applicationTag())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:AppregistryApplication
    properties:
      name: example-app
  bucket:
    type: aws:s3:BucketV2
    properties:
      bucket: example-bucket
      tags: ${example.applicationTag}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AWS Service Catalog AppRegistry Application using the `id`. For example:

```sh
$ pulumi import aws:servicecatalog/appregistryApplication:AppregistryApplication example application-id-12345678
```
5
descriptionB"  Description of the application.
|
nameB" nName of the application. The name must be unique within an AWS region.

The following arguments are optional:
“
tagsB2" ¡A map of tags assigned to the Application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"Ì
applicationTag2" ‘A map with a single tag key-value pair used to associate resources with the application. This attribute can be passed directly into the `tags` argument of another resource, or merged into a map of existing tags.
":
arn" /ARN (Amazon Resource Name) of the application.
"5
descriptionB"  Description of the application.
"z
name" nName of the application. The name must be unique within an AWS region.

The following arguments are optional:
"“
tagsB2" ¡A map of tags assigned to the Application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*ª%
s
servicecatalogAppregistryAttributeGroupFaws:servicecatalog/appregistryAttributeGroup:AppregistryAttributeGroupÏResource for managing an AWS Service Catalog AppRegistry Attribute Group.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.AppregistryAttributeGroup("example", {
    name: "example",
    description: "example description",
    attributes: JSON.stringify({
        app: "exampleapp",
        group: "examplegroup",
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.servicecatalog.AppregistryAttributeGroup("example",
    name="example",
    description="example description",
    attributes=json.dumps({
        "app": "exampleapp",
        "group": "examplegroup",
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
    var example = new Aws.ServiceCatalog.AppregistryAttributeGroup("example", new()
    {
        Name = "example",
        Description = "example description",
        Attributes = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["app"] = "exampleapp",
            ["group"] = "examplegroup",
        }),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"app":   "exampleapp",
			"group": "examplegroup",
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = servicecatalog.NewAppregistryAttributeGroup(ctx, "example", &servicecatalog.AppregistryAttributeGroupArgs{
			Name:        pulumi.String("example"),
			Description: pulumi.String("example description"),
			Attributes:  pulumi.String(json0),
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
import com.pulumi.aws.servicecatalog.AppregistryAttributeGroup;
import com.pulumi.aws.servicecatalog.AppregistryAttributeGroupArgs;
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
        var example = new AppregistryAttributeGroup("example", AppregistryAttributeGroupArgs.builder()
            .name("example")
            .description("example description")
            .attributes(serializeJson(
                jsonObject(
                    jsonProperty("app", "exampleapp"),
                    jsonProperty("group", "examplegroup")
                )))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:AppregistryAttributeGroup
    properties:
      name: example
      description: example description
      attributes:
        fn::toJSON:
          app: exampleapp
          group: examplegroup
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Catalog AppRegistry Attribute Group using the `id`. For example:

```sh
$ pulumi import aws:servicecatalog/appregistryAttributeGroup:AppregistryAttributeGroup example 1234567890abcfedhijk09876s
```
é

attributes" |A JSON string of nested key-value pairs that represents the attributes of the group.

The following arguments are optional:
9
descriptionB" $Description of the Attribute Group.
+
nameB" Name of the Attribute Group.
÷
tagsB2" ≈A map of tags assigned to the Attribute Group. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"'
arn" ARN of the Attribute Group.
"é

attributes" |A JSON string of nested key-value pairs that represents the attributes of the group.

The following arguments are optional:
"9
descriptionB" $Description of the Attribute Group.
")
name" Name of the Attribute Group.
"÷
tagsB2" ≈A map of tags assigned to the Attribute Group. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*Ù5
î
servicecatalog$AppregistryAttributeGroupAssociation\aws:servicecatalog/appregistryAttributeGroupAssociation:AppregistryAttributeGroupAssociation–2Resource for managing an AWS Service Catalog AppRegistry Attribute Group Association.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.AppregistryApplication("example", {name: "example-app"});
const exampleAppregistryAttributeGroup = new aws.servicecatalog.AppregistryAttributeGroup("example", {
    name: "example",
    description: "example description",
    attributes: JSON.stringify({
        app: "exampleapp",
        group: "examplegroup",
    }),
});
const exampleAppregistryAttributeGroupAssociation = new aws.servicecatalog.AppregistryAttributeGroupAssociation("example", {
    applicationId: example.id,
    attributeGroupId: exampleAppregistryAttributeGroup.id,
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.servicecatalog.AppregistryApplication("example", name="example-app")
example_appregistry_attribute_group = aws.servicecatalog.AppregistryAttributeGroup("example",
    name="example",
    description="example description",
    attributes=json.dumps({
        "app": "exampleapp",
        "group": "examplegroup",
    }))
example_appregistry_attribute_group_association = aws.servicecatalog.AppregistryAttributeGroupAssociation("example",
    application_id=example.id,
    attribute_group_id=example_appregistry_attribute_group.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.AppregistryApplication("example", new()
    {
        Name = "example-app",
    });

    var exampleAppregistryAttributeGroup = new Aws.ServiceCatalog.AppregistryAttributeGroup("example", new()
    {
        Name = "example",
        Description = "example description",
        Attributes = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["app"] = "exampleapp",
            ["group"] = "examplegroup",
        }),
    });

    var exampleAppregistryAttributeGroupAssociation = new Aws.ServiceCatalog.AppregistryAttributeGroupAssociation("example", new()
    {
        ApplicationId = example.Id,
        AttributeGroupId = exampleAppregistryAttributeGroup.Id,
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := servicecatalog.NewAppregistryApplication(ctx, "example", &servicecatalog.AppregistryApplicationArgs{
			Name: pulumi.String("example-app"),
		})
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"app":   "exampleapp",
			"group": "examplegroup",
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		exampleAppregistryAttributeGroup, err := servicecatalog.NewAppregistryAttributeGroup(ctx, "example", &servicecatalog.AppregistryAttributeGroupArgs{
			Name:        pulumi.String("example"),
			Description: pulumi.String("example description"),
			Attributes:  pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		_, err = servicecatalog.NewAppregistryAttributeGroupAssociation(ctx, "example", &servicecatalog.AppregistryAttributeGroupAssociationArgs{
			ApplicationId:    example.ID(),
			AttributeGroupId: exampleAppregistryAttributeGroup.ID(),
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
import com.pulumi.aws.servicecatalog.AppregistryApplication;
import com.pulumi.aws.servicecatalog.AppregistryApplicationArgs;
import com.pulumi.aws.servicecatalog.AppregistryAttributeGroup;
import com.pulumi.aws.servicecatalog.AppregistryAttributeGroupArgs;
import com.pulumi.aws.servicecatalog.AppregistryAttributeGroupAssociation;
import com.pulumi.aws.servicecatalog.AppregistryAttributeGroupAssociationArgs;
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
        var example = new AppregistryApplication("example", AppregistryApplicationArgs.builder()
            .name("example-app")
            .build());

        var exampleAppregistryAttributeGroup = new AppregistryAttributeGroup("exampleAppregistryAttributeGroup", AppregistryAttributeGroupArgs.builder()
            .name("example")
            .description("example description")
            .attributes(serializeJson(
                jsonObject(
                    jsonProperty("app", "exampleapp"),
                    jsonProperty("group", "examplegroup")
                )))
            .build());

        var exampleAppregistryAttributeGroupAssociation = new AppregistryAttributeGroupAssociation("exampleAppregistryAttributeGroupAssociation", AppregistryAttributeGroupAssociationArgs.builder()
            .applicationId(example.id())
            .attributeGroupId(exampleAppregistryAttributeGroup.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:AppregistryApplication
    properties:
      name: example-app
  exampleAppregistryAttributeGroup:
    type: aws:servicecatalog:AppregistryAttributeGroup
    name: example
    properties:
      name: example
      description: example description
      attributes:
        fn::toJSON:
          app: exampleapp
          group: examplegroup
  exampleAppregistryAttributeGroupAssociation:
    type: aws:servicecatalog:AppregistryAttributeGroupAssociation
    name: example
    properties:
      applicationId: ${example.id}
      attributeGroupId: ${exampleAppregistryAttributeGroup.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Catalog AppRegistry Attribute Group Association using `application_id` and `attribute_group_id` arguments separated by a comma (`,`). For example:

```sh
$ pulumi import aws:servicecatalog/appregistryAttributeGroupAssociation:AppregistryAttributeGroupAssociation example 12456778723424sdffsdfsdq34,12234t3564dsfsdf34asff4ww3
```
,
applicationId" ID of the application.
U
attributeGroupId" =ID of the attribute group to associate with the application.
",
applicationId" ID of the application.
"U
attributeGroupId" =ID of the attribute group to associate with the application.
*÷
s
servicecatalogBudgetResourceAssociationFaws:servicecatalog/budgetResourceAssociation:BudgetResourceAssociation Manages a Service Catalog Budget Resource Association.

> **Tip:** A "resource" is either a Service Catalog portfolio or product.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.BudgetResourceAssociation("example", {
    budgetName: "budget-pjtvyakdlyo3m",
    resourceId: "prod-dnigbtea24ste",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.BudgetResourceAssociation("example",
    budget_name="budget-pjtvyakdlyo3m",
    resource_id="prod-dnigbtea24ste")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.BudgetResourceAssociation("example", new()
    {
        BudgetName = "budget-pjtvyakdlyo3m",
        ResourceId = "prod-dnigbtea24ste",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewBudgetResourceAssociation(ctx, "example", &servicecatalog.BudgetResourceAssociationArgs{
			BudgetName: pulumi.String("budget-pjtvyakdlyo3m"),
			ResourceId: pulumi.String("prod-dnigbtea24ste"),
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
import com.pulumi.aws.servicecatalog.BudgetResourceAssociation;
import com.pulumi.aws.servicecatalog.BudgetResourceAssociationArgs;
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
        var example = new BudgetResourceAssociation("example", BudgetResourceAssociationArgs.builder()
            .budgetName("budget-pjtvyakdlyo3m")
            .resourceId("prod-dnigbtea24ste")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:BudgetResourceAssociation
    properties:
      budgetName: budget-pjtvyakdlyo3m
      resourceId: prod-dnigbtea24ste
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_budget_resource_association` using the budget name and resource ID. For example:

```sh
$ pulumi import aws:servicecatalog/budgetResourceAssociation:BudgetResourceAssociation example budget-pjtvyakdlyo3m:prod-dnigbtea24ste
```


budgetName" Budget name.
'

resourceId" Resource identifier.
"

budgetName" Budget name.
"'

resourceId" Resource identifier.
*Ÿ+
F
servicecatalog
Constraint(aws:servicecatalog/constraint:Constraintí"Manages a Service Catalog Constraint.

> **NOTE:** This resource does not associate a Service Catalog product and portfolio. However, the product and portfolio must be associated (see the `aws.servicecatalog.ProductPortfolioAssociation` resource) prior to creating a constraint or you will receive an error.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.Constraint("example", {
    description: "Back off, man. I'm a scientist.",
    portfolioId: exampleAwsServicecatalogPortfolio.id,
    productId: exampleAwsServicecatalogProduct.id,
    type: "LAUNCH",
    parameters: JSON.stringify({
        RoleArn: "arn:aws:iam::123456789012:role/LaunchRole",
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.servicecatalog.Constraint("example",
    description="Back off, man. I'm a scientist.",
    portfolio_id=example_aws_servicecatalog_portfolio["id"],
    product_id=example_aws_servicecatalog_product["id"],
    type="LAUNCH",
    parameters=json.dumps({
        "RoleArn": "arn:aws:iam::123456789012:role/LaunchRole",
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
    var example = new Aws.ServiceCatalog.Constraint("example", new()
    {
        Description = "Back off, man. I'm a scientist.",
        PortfolioId = exampleAwsServicecatalogPortfolio.Id,
        ProductId = exampleAwsServicecatalogProduct.Id,
        Type = "LAUNCH",
        Parameters = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["RoleArn"] = "arn:aws:iam::123456789012:role/LaunchRole",
        }),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"RoleArn": "arn:aws:iam::123456789012:role/LaunchRole",
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = servicecatalog.NewConstraint(ctx, "example", &servicecatalog.ConstraintArgs{
			Description: pulumi.String("Back off, man. I'm a scientist."),
			PortfolioId: pulumi.Any(exampleAwsServicecatalogPortfolio.Id),
			ProductId:   pulumi.Any(exampleAwsServicecatalogProduct.Id),
			Type:        pulumi.String("LAUNCH"),
			Parameters:  pulumi.String(json0),
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
import com.pulumi.aws.servicecatalog.Constraint;
import com.pulumi.aws.servicecatalog.ConstraintArgs;
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
        var example = new Constraint("example", ConstraintArgs.builder()
            .description("Back off, man. I'm a scientist.")
            .portfolioId(exampleAwsServicecatalogPortfolio.id())
            .productId(exampleAwsServicecatalogProduct.id())
            .type("LAUNCH")
            .parameters(serializeJson(
                jsonObject(
                    jsonProperty("RoleArn", "arn:aws:iam::123456789012:role/LaunchRole")
                )))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:Constraint
    properties:
      description: Back off, man. I'm a scientist.
      portfolioId: ${exampleAwsServicecatalogPortfolio.id}
      productId: ${exampleAwsServicecatalogProduct.id}
      type: LAUNCH
      parameters:
        fn::toJSON:
          RoleArn: arn:aws:iam::123456789012:role/LaunchRole
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_constraint` using the constraint ID. For example:

```sh
$ pulumi import aws:servicecatalog/constraint:Constraint example cons-nmdkb6cgxfcrs
```
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
4
descriptionB" Description of the constraint.
v

parameters" dConstraint parameters in JSON format. The syntax depends on the constraint type. See details below.
)
portfolioId" Portfolio identifier.
%
	productId" Product identifier.
¢
type" ïType of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `RESOURCE_UPDATE`, `STACKSET`, and `TEMPLATE`.

The following arguments are optional:
"}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
"2
description" Description of the constraint.
"&
owner" Owner of the constraint.
"v

parameters" dConstraint parameters in JSON format. The syntax depends on the constraint type. See details below.
")
portfolioId" Portfolio identifier.
"%
	productId" Product identifier.
"
status" "¢
type" ïType of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `RESOURCE_UPDATE`, `STACKSET`, and `TEMPLATE`.

The following arguments are optional:
*ç
a
servicecatalogOrganizationsAccess:aws:servicecatalog/organizationsAccess:OrganizationsAccess≠Manages Service Catalog AWS Organizations Access, a portfolio sharing feature through AWS Organizations. This allows Service Catalog to receive updates on your organization in order to sync your shares with the current structure. This resource will prompt AWS to set `organizations:EnableAWSServiceAccess` on your behalf so that your shares can be in sync with any changes in your AWS Organizations structure.

> **NOTE:** This resource can only be used by the management account in the organization. In other words, a delegated administrator is not authorized to use the resource.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.OrganizationsAccess("example", {enabled: true});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.OrganizationsAccess("example", enabled=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.OrganizationsAccess("example", new()
    {
        Enabled = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewOrganizationsAccess(ctx, "example", &servicecatalog.OrganizationsAccessArgs{
			Enabled: pulumi.Bool(true),
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
import com.pulumi.aws.servicecatalog.OrganizationsAccess;
import com.pulumi.aws.servicecatalog.OrganizationsAccessArgs;
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
        var example = new OrganizationsAccess("example", OrganizationsAccessArgs.builder()
            .enabled("true")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:OrganizationsAccess
    properties:
      enabled: 'true'
```
<!--End PulumiCodeChooser -->
;
enabled
 ,Whether to enable AWS Organizations access.
";
enabled
 ,Whether to enable AWS Organizations access.
*Œ
C
servicecatalog	Portfolio&aws:servicecatalog/portfolio:PortfolioÍProvides a resource to create a Service Catalog Portfolio.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const portfolio = new aws.servicecatalog.Portfolio("portfolio", {
    name: "My App Portfolio",
    description: "List of my organizations apps",
    providerName: "Brett",
});
```
```python
import pulumi
import pulumi_aws as aws

portfolio = aws.servicecatalog.Portfolio("portfolio",
    name="My App Portfolio",
    description="List of my organizations apps",
    provider_name="Brett")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var portfolio = new Aws.ServiceCatalog.Portfolio("portfolio", new()
    {
        Name = "My App Portfolio",
        Description = "List of my organizations apps",
        ProviderName = "Brett",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewPortfolio(ctx, "portfolio", &servicecatalog.PortfolioArgs{
			Name:         pulumi.String("My App Portfolio"),
			Description:  pulumi.String("List of my organizations apps"),
			ProviderName: pulumi.String("Brett"),
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
import com.pulumi.aws.servicecatalog.Portfolio;
import com.pulumi.aws.servicecatalog.PortfolioArgs;
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
        var portfolio = new Portfolio("portfolio", PortfolioArgs.builder()
            .name("My App Portfolio")
            .description("List of my organizations apps")
            .providerName("Brett")
            .build());

    }
}
```
```yaml
resources:
  portfolio:
    type: aws:servicecatalog:Portfolio
    properties:
      name: My App Portfolio
      description: List of my organizations apps
      providerName: Brett
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Catalog Portfolios using the Service Catalog Portfolio `id`. For example:

```sh
$ pulumi import aws:servicecatalog/portfolio:Portfolio testfolio port-12344321
```
2
descriptionB" Description of the portfolio
)
nameB" The name of the portfolio.
O
providerName" ;Name of the person or organization who owns the portfolio.
»
tagsB2" ∑Tags to apply to the connection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"	
arn" "
createdTime" "0
description" Description of the portfolio
"'
name" The name of the portfolio.
"O
providerName" ;Name of the person or organization who owns the portfolio.
"»
tagsB2" ∑Tags to apply to the connection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*ç/
R
servicecatalogPortfolioShare0aws:servicecatalog/portfolioShare:PortfolioShareáManages a Service Catalog Portfolio Share. Shares the specified portfolio with the specified account or organization node. You can share portfolios to an organization, an organizational unit, or a specific account.

If the portfolio share with the specified account or organization node already exists, using this resource to re-create the share will have no effect and will not return an error. You can then use this resource to update the share.

> **NOTE:** Shares to an organization node can only be created by the management account of an organization or by a delegated administrator. If a delegated admin is de-registered, they can no longer create portfolio shares.

> **NOTE:** AWSOrganizationsAccess must be enabled in order to create a portfolio share to an organization node.

> **NOTE:** You can't share a shared resource, including portfolios that contain a shared product.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.PortfolioShare("example", {
    principalId: "012128675309",
    portfolioId: exampleAwsServicecatalogPortfolio.id,
    type: "ACCOUNT",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.PortfolioShare("example",
    principal_id="012128675309",
    portfolio_id=example_aws_servicecatalog_portfolio["id"],
    type="ACCOUNT")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.PortfolioShare("example", new()
    {
        PrincipalId = "012128675309",
        PortfolioId = exampleAwsServicecatalogPortfolio.Id,
        Type = "ACCOUNT",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewPortfolioShare(ctx, "example", &servicecatalog.PortfolioShareArgs{
			PrincipalId: pulumi.String("012128675309"),
			PortfolioId: pulumi.Any(exampleAwsServicecatalogPortfolio.Id),
			Type:        pulumi.String("ACCOUNT"),
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
import com.pulumi.aws.servicecatalog.PortfolioShare;
import com.pulumi.aws.servicecatalog.PortfolioShareArgs;
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
        var example = new PortfolioShare("example", PortfolioShareArgs.builder()
            .principalId("012128675309")
            .portfolioId(exampleAwsServicecatalogPortfolio.id())
            .type("ACCOUNT")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:PortfolioShare
    properties:
      principalId: '012128675309'
      portfolioId: ${exampleAwsServicecatalogPortfolio.id}
      type: ACCOUNT
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_portfolio_share` using the portfolio share ID. For example:

```sh
$ pulumi import aws:servicecatalog/portfolioShare:PortfolioShare example port-12344321:ACCOUNT:123456789012
```
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
)
portfolioId" Portfolio identifier.
≠
principalId" ôIdentifier of the principal with whom you will share the portfolio. Valid values AWS account IDs and ARNs of AWS Organizations and organizational units.
†
sharePrincipalsB
 ÜEnables or disables Principal sharing when creating the portfolio share. If this flag is not provided, principal sharing is disabled.
Ç
shareTagOptionsB
 iWhether to enable sharing of `aws.servicecatalog.TagOption` resources when creating the portfolio share.
û
type" ëType of portfolio share. Valid values are `ACCOUNT` (an external account), `ORGANIZATION` (a share to every account in an organization), `ORGANIZATIONAL_UNIT`, `ORGANIZATION_MEMBER_ACCOUNT` (a share to an account in an organization).

The following arguments are optional:
è
waitForAcceptanceB
 tWhether to wait (up to the timeout) for the share to be accepted. Organizational shares are automatically accepted.
"}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
"¬
accepted
 ±Whether the shared portfolio is imported by the recipient account. If the recipient is organizational, the share is automatically imported, and the field is always set to true.
")
portfolioId" Portfolio identifier.
"≠
principalId" ôIdentifier of the principal with whom you will share the portfolio. Valid values AWS account IDs and ARNs of AWS Organizations and organizational units.
"†
sharePrincipalsB
 ÜEnables or disables Principal sharing when creating the portfolio share. If this flag is not provided, principal sharing is disabled.
"Ç
shareTagOptionsB
 iWhether to enable sharing of `aws.servicecatalog.TagOption` resources when creating the portfolio share.
"û
type" ëType of portfolio share. Valid values are `ACCOUNT` (an external account), `ORGANIZATION` (a share to every account in an organization), `ORGANIZATIONAL_UNIT`, `ORGANIZATION_MEMBER_ACCOUNT` (a share to an account in an organization).

The following arguments are optional:
"è
waitForAcceptanceB
 tWhether to wait (up to the timeout) for the share to be accepted. Organizational shares are automatically accepted.
*‹

servicecatalogPrincipalPortfolioAssociationNaws:servicecatalog/principalPortfolioAssociation:PrincipalPortfolioAssociation¯Manages a Service Catalog Principal Portfolio Association.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.PrincipalPortfolioAssociation("example", {
    portfolioId: "port-68656c6c6f",
    principalArn: "arn:aws:iam::123456789012:user/Eleanor",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.PrincipalPortfolioAssociation("example",
    portfolio_id="port-68656c6c6f",
    principal_arn="arn:aws:iam::123456789012:user/Eleanor")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.PrincipalPortfolioAssociation("example", new()
    {
        PortfolioId = "port-68656c6c6f",
        PrincipalArn = "arn:aws:iam::123456789012:user/Eleanor",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewPrincipalPortfolioAssociation(ctx, "example", &servicecatalog.PrincipalPortfolioAssociationArgs{
			PortfolioId:  pulumi.String("port-68656c6c6f"),
			PrincipalArn: pulumi.String("arn:aws:iam::123456789012:user/Eleanor"),
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
import com.pulumi.aws.servicecatalog.PrincipalPortfolioAssociation;
import com.pulumi.aws.servicecatalog.PrincipalPortfolioAssociationArgs;
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
        var example = new PrincipalPortfolioAssociation("example", PrincipalPortfolioAssociationArgs.builder()
            .portfolioId("port-68656c6c6f")
            .principalArn("arn:aws:iam::123456789012:user/Eleanor")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:PrincipalPortfolioAssociation
    properties:
      portfolioId: port-68656c6c6f
      principalArn: arn:aws:iam::123456789012:user/Eleanor
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_principal_portfolio_association` using `accept_language`, `principal_arn`, `portfolio_id`, and `principal_type` separated by a comma. For example:

```sh
$ pulumi import aws:servicecatalog/principalPortfolioAssociation:PrincipalPortfolioAssociation example en,arn:aws:iam::123456789012:user/Eleanor,port-68656c6c6f,IAM
```
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
)
portfolioId" Portfolio identifier.
J
principalArn" 6Principal ARN.

The following arguments are optional:
∑
principalTypeB" üPrincipal type. Setting this argument empty (e.g., `principal_type = ""`) will result in an error. Valid values are `IAM` and `IAM_PATTERN`. Default is `IAM`.
"}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
")
portfolioId" Portfolio identifier.
"J
principalArn" 6Principal ARN.

The following arguments are optional:
"∑
principalTypeB" üPrincipal type. Setting this argument empty (e.g., `principal_type = ""`) will result in an error. Valid values are `IAM` and `IAM_PATTERN`. Default is `IAM`.
*÷9
=
servicecatalogProduct"aws:servicecatalog/product:Productï!Manages a Service Catalog Product.

> **NOTE:** The user or role that uses this resources must have the `cloudformation:GetTemplate` IAM policy permission. This policy permission is required when using the `template_physical_id` argument.

> A "provisioning artifact" is also referred to as a "version." A "distributor" is also referred to as a "vendor."

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.Product("example", {
    name: "example",
    owner: "example-owner",
    type: "CLOUD_FORMATION_TEMPLATE",
    provisioningArtifactParameters: {
        templateUrl: "https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/temp1.json",
    },
    tags: {
        foo: "bar",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.Product("example",
    name="example",
    owner="example-owner",
    type="CLOUD_FORMATION_TEMPLATE",
    provisioning_artifact_parameters={
        "template_url": "https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/temp1.json",
    },
    tags={
        "foo": "bar",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.Product("example", new()
    {
        Name = "example",
        Owner = "example-owner",
        Type = "CLOUD_FORMATION_TEMPLATE",
        ProvisioningArtifactParameters = new Aws.ServiceCatalog.Inputs.ProductProvisioningArtifactParametersArgs
        {
            TemplateUrl = "https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/temp1.json",
        },
        Tags = 
        {
            { "foo", "bar" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewProduct(ctx, "example", &servicecatalog.ProductArgs{
			Name:  pulumi.String("example"),
			Owner: pulumi.String("example-owner"),
			Type:  pulumi.String("CLOUD_FORMATION_TEMPLATE"),
			ProvisioningArtifactParameters: &servicecatalog.ProductProvisioningArtifactParametersArgs{
				TemplateUrl: pulumi.String("https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/temp1.json"),
			},
			Tags: pulumi.StringMap{
				"foo": pulumi.String("bar"),
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
import com.pulumi.aws.servicecatalog.Product;
import com.pulumi.aws.servicecatalog.ProductArgs;
import com.pulumi.aws.servicecatalog.inputs.ProductProvisioningArtifactParametersArgs;
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
        var example = new Product("example", ProductArgs.builder()
            .name("example")
            .owner("example-owner")
            .type("CLOUD_FORMATION_TEMPLATE")
            .provisioningArtifactParameters(ProductProvisioningArtifactParametersArgs.builder()
                .templateUrl("https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/temp1.json")
                .build())
            .tags(Map.of("foo", "bar"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:Product
    properties:
      name: example
      owner: example-owner
      type: CLOUD_FORMATION_TEMPLATE
      provisioningArtifactParameters:
        templateUrl: https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/temp1.json
      tags:
        foo: bar
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_product` using the product ID. For example:

```sh
$ pulumi import aws:servicecatalog/product:Product example prod-dnigbtea24ste
```
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
1
descriptionB" Description of the product.
@
distributorB" +Distributor (i.e., vendor) of the product.
#
nameB" Name of the product.
#
owner" Owner of the product.
«
provisioningArtifactParametersù:ö
ó
servicecatalog%ProductProvisioningArtifactParameters^aws:servicecatalog/ProductProvisioningArtifactParameters:ProductProvisioningArtifactParametersÑConfiguration block for provisioning artifact (i.e., version) parameters. See `provisioning_artifact_parameters` Block for details.
C
supportDescriptionB" 'Support information about the product.
9
supportEmailB" #Contact email for product support.
5

supportUrlB" !Contact URL for product support.
≈
tagsB2" ¥Tags to apply to the product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
€
type" ŒType of product. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_CreateProduct.html#API_CreateProduct_RequestSyntax) for valid list of values.

The following arguments are optional:
"}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
"
arn" ARN of the product.
"6
createdTime" #Time when the product was created.
"/
description" Description of the product.
">
distributor" +Distributor (i.e., vendor) of the product.
"≤
hasDefaultPath
 õWhether the product has a default path. If the product does not have a default path, call `ListLaunchPaths` to disambiguate between paths.  Otherwise, `ListLaunchPaths` is not required, and the output of ProductViewSummary can be used directly with `DescribeProvisioningParameters`.
"!
name" Name of the product.
"#
owner" Owner of the product.
"«
provisioningArtifactParametersù:ö
ó
servicecatalog%ProductProvisioningArtifactParameters^aws:servicecatalog/ProductProvisioningArtifactParameters:ProductProvisioningArtifactParametersÑConfiguration block for provisioning artifact (i.e., version) parameters. See `provisioning_artifact_parameters` Block for details.
"%
status" Status of the product.
"A
supportDescription" 'Support information about the product.
"7
supportEmail" #Contact email for product support.
"3

supportUrl" !Contact URL for product support.
"≈
tagsB2" ¥Tags to apply to the product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"€
type" ŒType of product. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_CreateProduct.html#API_CreateProduct_RequestSyntax) for valid list of values.

The following arguments are optional:
*ˇ
y
servicecatalogProductPortfolioAssociationJaws:servicecatalog/productPortfolioAssociation:ProductPortfolioAssociationèManages a Service Catalog Product Portfolio Association.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.ProductPortfolioAssociation("example", {
    portfolioId: "port-68656c6c6f",
    productId: "prod-dnigbtea24ste",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.ProductPortfolioAssociation("example",
    portfolio_id="port-68656c6c6f",
    product_id="prod-dnigbtea24ste")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.ProductPortfolioAssociation("example", new()
    {
        PortfolioId = "port-68656c6c6f",
        ProductId = "prod-dnigbtea24ste",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewProductPortfolioAssociation(ctx, "example", &servicecatalog.ProductPortfolioAssociationArgs{
			PortfolioId: pulumi.String("port-68656c6c6f"),
			ProductId:   pulumi.String("prod-dnigbtea24ste"),
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
import com.pulumi.aws.servicecatalog.ProductPortfolioAssociation;
import com.pulumi.aws.servicecatalog.ProductPortfolioAssociationArgs;
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
        var example = new ProductPortfolioAssociation("example", ProductPortfolioAssociationArgs.builder()
            .portfolioId("port-68656c6c6f")
            .productId("prod-dnigbtea24ste")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:ProductPortfolioAssociation
    properties:
      portfolioId: port-68656c6c6f
      productId: prod-dnigbtea24ste
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_product_portfolio_association` using the accept language, portfolio ID, and product ID. For example:

```sh
$ pulumi import aws:servicecatalog/productPortfolioAssociation:ProductPortfolioAssociation example en:port-68656c6c6f:prod-dnigbtea24ste
```
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
)
portfolioId" Portfolio identifier.
L
	productId" ;Product identifier.

The following arguments are optional:
?
sourcePortfolioIdB" $Identifier of the source portfolio.
"}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
")
portfolioId" Portfolio identifier.
"L
	productId" ;Product identifier.

The following arguments are optional:
"?
sourcePortfolioIdB" $Identifier of the source portfolio.
*©^
^
servicecatalogProvisionedProduct8aws:servicecatalog/provisionedProduct:ProvisionedProductÔ%This resource provisions and manages a Service Catalog provisioned product.

A provisioned product is a resourced instance of a product. For example, provisioning a product based on a CloudFormation template launches a CloudFormation stack and its underlying resources.

Like this resource, the `aws_servicecatalog_record` data source also provides information about a provisioned product. Although a Service Catalog record provides some overlapping information with this resource, a record is tied to a provisioned product event, such as provisioning, termination, and updating.

> **Tip:** If you include conflicted keys as tags, AWS will report an error, "Parameter validation failed: Missing required parameter in Tags[N]:Value".

> **Tip:** A "provisioning artifact" is also referred to as a "version." A "distributor" is also referred to as a "vendor."

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.ProvisionedProduct("example", {
    name: "example",
    productName: "Example product",
    provisioningArtifactName: "Example version",
    provisioningParameters: [{
        key: "foo",
        value: "bar",
    }],
    tags: {
        foo: "bar",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.ProvisionedProduct("example",
    name="example",
    product_name="Example product",
    provisioning_artifact_name="Example version",
    provisioning_parameters=[{
        "key": "foo",
        "value": "bar",
    }],
    tags={
        "foo": "bar",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.ProvisionedProduct("example", new()
    {
        Name = "example",
        ProductName = "Example product",
        ProvisioningArtifactName = "Example version",
        ProvisioningParameters = new[]
        {
            new Aws.ServiceCatalog.Inputs.ProvisionedProductProvisioningParameterArgs
            {
                Key = "foo",
                Value = "bar",
            },
        },
        Tags = 
        {
            { "foo", "bar" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewProvisionedProduct(ctx, "example", &servicecatalog.ProvisionedProductArgs{
			Name:                     pulumi.String("example"),
			ProductName:              pulumi.String("Example product"),
			ProvisioningArtifactName: pulumi.String("Example version"),
			ProvisioningParameters: servicecatalog.ProvisionedProductProvisioningParameterArray{
				&servicecatalog.ProvisionedProductProvisioningParameterArgs{
					Key:   pulumi.String("foo"),
					Value: pulumi.String("bar"),
				},
			},
			Tags: pulumi.StringMap{
				"foo": pulumi.String("bar"),
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
import com.pulumi.aws.servicecatalog.ProvisionedProduct;
import com.pulumi.aws.servicecatalog.ProvisionedProductArgs;
import com.pulumi.aws.servicecatalog.inputs.ProvisionedProductProvisioningParameterArgs;
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
        var example = new ProvisionedProduct("example", ProvisionedProductArgs.builder()
            .name("example")
            .productName("Example product")
            .provisioningArtifactName("Example version")
            .provisioningParameters(ProvisionedProductProvisioningParameterArgs.builder()
                .key("foo")
                .value("bar")
                .build())
            .tags(Map.of("foo", "bar"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:ProvisionedProduct
    properties:
      name: example
      productName: Example product
      provisioningArtifactName: Example version
      provisioningParameters:
        - key: foo
          value: bar
      tags:
        foo: bar
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_provisioned_product` using the provisioned product ID. For example:

```sh
$ pulumi import aws:servicecatalog/provisionedProduct:ProvisionedProduct example pp-dnigbtea24ste
```
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
€
ignoreErrorsB
 ƒ_Only applies to deleting._ If set to `true`, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources. The default value is `false`.
d
nameB" VUser-friendly name of the provisioned product.

The following arguments are optional:
s
notificationArnsB*" WPassed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.
±
pathIdB" †Path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use `aws.servicecatalog.getLaunchPaths`. When required, you must provide `path_id` or `path_name`, but not both.
]
pathNameB" KName of the path. You must provide `path_id` or `path_name`, but not both.
â
	productIdB" vProduct identifier. For example, `prod-abcdzk7xy33qa`. You must provide `product_id` or `product_name`, but not both.
i
productNameB" TName of the product. You must provide `product_id` or `product_name`, but not both.
 
provisioningArtifactIdB" ©Identifier of the provisioning artifact. For example, `pa-4abcdjnxjj6ne`. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
•
provisioningArtifactNameB" ÇName of the provisioning artifact. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
Î
provisioningParameters©B¶*£:†
ù
servicecatalog'ProvisionedProductProvisioningParameterbaws:servicecatalog/ProvisionedProductProvisioningParameter:ProvisionedProductProvisioningParameter§Configuration block with parameters specified by the administrator that are required for provisioning the product. See `provisioning_parameters` Block for details.
Ö
retainPhysicalResourcesB
 „_Only applies to deleting._ Whether to delete the Service Catalog provisioned product but leave the CloudFormation stack, stack set, or the underlying resources of the deleted provisioned product. The default value is `false`.
Ä
stackSetProvisioningPreferencesƒB¡:æ
ª
servicecatalog1ProvisionedProductStackSetProvisioningPreferencesvaws:servicecatalog/ProvisionedProductStackSetProvisioningPreferences:ProvisionedProductStackSetProvisioningPreferencesïConfiguration block with information about the provisioning preferences for a stack set. See `stack_set_provisioning_preferences` Block for details.
—
tagsB2" ¿Tags to apply to the provisioned product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
"+
arn"  ARN of the provisioned product.
"p
cloudwatchDashboardNames*" NSet of CloudWatch dashboards that were created when provisioning the product.
"B
createdTime" /Time when the provisioned product was created.
"€
ignoreErrorsB
 ƒ_Only applies to deleting._ If set to `true`, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources. The default value is `false`.
"˜
lastProvisioningRecordId" ÷Record identifier of the last request performed on this provisioned product of the following types: `ProvisionedProduct`, `UpdateProvisionedProduct`, `ExecuteProvisionedProductPlan`, `TerminateProvisionedProduct`.
"a
lastRecordId" MRecord identifier of the last request performed on this provisioned product.
"å
"lastSuccessfulProvisioningRecordId" ·Record identifier of the last successful request performed on this provisioned product of the following types: `ProvisionedProduct`, `UpdateProvisionedProduct`, `ExecuteProvisionedProductPlan`, `TerminateProvisionedProduct`.
"U
launchRoleArn" @ARN of the launch role associated with the provisioned product.
"b
name" VUser-friendly name of the provisioned product.

The following arguments are optional:
"s
notificationArnsB*" WPassed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.
"Ø
outputsv*t:r
p
servicecatalogProvisionedProductOutputDaws:servicecatalog/ProvisionedProductOutput:ProvisionedProductOutput,The set of outputs for the product created.
"Ø
pathId" †Path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use `aws.servicecatalog.getLaunchPaths`. When required, you must provide `path_id` or `path_name`, but not both.
"]
pathNameB" KName of the path. You must provide `path_id` or `path_name`, but not both.
"á
	productId" vProduct identifier. For example, `prod-abcdzk7xy33qa`. You must provide `product_id` or `product_name`, but not both.
"i
productNameB" TName of the product. You must provide `product_id` or `product_name`, but not both.
"»
provisioningArtifactId" ©Identifier of the provisioning artifact. For example, `pa-4abcdjnxjj6ne`. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
"•
provisioningArtifactNameB" ÇName of the provisioning artifact. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
"Î
provisioningParameters©B¶*£:†
ù
servicecatalog'ProvisionedProductProvisioningParameterbaws:servicecatalog/ProvisionedProductProvisioningParameter:ProvisionedProductProvisioningParameter§Configuration block with parameters specified by the administrator that are required for provisioning the product. See `provisioning_parameters` Block for details.
"Ö
retainPhysicalResourcesB
 „_Only applies to deleting._ Whether to delete the Service Catalog provisioned product but leave the CloudFormation stack, stack set, or the underlying resources of the deleted provisioned product. The default value is `false`.
"Ä
stackSetProvisioningPreferencesƒB¡:æ
ª
servicecatalog1ProvisionedProductStackSetProvisioningPreferencesvaws:servicecatalog/ProvisionedProductStackSetProvisioningPreferences:ProvisionedProductStackSetProvisioningPreferencesïConfiguration block with information about the provisioning preferences for a stack set. See `stack_set_provisioning_preferences` Block for details.
"M
status" ?Current status of the provisioned product. See meanings below.
"H
statusMessage" 3Current status message of the provisioned product.
"—
tagsB2" ¿Tags to apply to the provisioned product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"Z
type" NType of provisioned product. Valid values are `CFN_STACK` and `CFN_STACKSET`.
*õ<
d
servicecatalogProvisioningArtifact<aws:servicecatalog/provisioningArtifact:ProvisioningArtifactÊManages a Service Catalog Provisioning Artifact for a specified product.

> A "provisioning artifact" is also referred to as a "version."

> **NOTE:** You cannot create a provisioning artifact for a product that was shared with you.

> **NOTE:** The user or role that use this resource must have the `cloudformation:GetTemplate` IAM policy permission. This policy permission is required when using the `template_physical_id` argument.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.ProvisioningArtifact("example", {
    name: "example",
    productId: exampleAwsServicecatalogProduct.id,
    type: "CLOUD_FORMATION_TEMPLATE",
    templateUrl: `https://${exampleAwsS3Bucket.bucketRegionalDomainName}/${exampleAwsS3Object.key}`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.ProvisioningArtifact("example",
    name="example",
    product_id=example_aws_servicecatalog_product["id"],
    type="CLOUD_FORMATION_TEMPLATE",
    template_url=f"https://{example_aws_s3_bucket['bucketRegionalDomainName']}/{example_aws_s3_object['key']}")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.ProvisioningArtifact("example", new()
    {
        Name = "example",
        ProductId = exampleAwsServicecatalogProduct.Id,
        Type = "CLOUD_FORMATION_TEMPLATE",
        TemplateUrl = $"https://{exampleAwsS3Bucket.BucketRegionalDomainName}/{exampleAwsS3Object.Key}",
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewProvisioningArtifact(ctx, "example", &servicecatalog.ProvisioningArtifactArgs{
			Name:        pulumi.String("example"),
			ProductId:   pulumi.Any(exampleAwsServicecatalogProduct.Id),
			Type:        pulumi.String("CLOUD_FORMATION_TEMPLATE"),
			TemplateUrl: pulumi.Sprintf("https://%v/%v", exampleAwsS3Bucket.BucketRegionalDomainName, exampleAwsS3Object.Key),
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
import com.pulumi.aws.servicecatalog.ProvisioningArtifact;
import com.pulumi.aws.servicecatalog.ProvisioningArtifactArgs;
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
        var example = new ProvisioningArtifact("example", ProvisioningArtifactArgs.builder()
            .name("example")
            .productId(exampleAwsServicecatalogProduct.id())
            .type("CLOUD_FORMATION_TEMPLATE")
            .templateUrl(String.format("https://%s/%s", exampleAwsS3Bucket.bucketRegionalDomainName(),exampleAwsS3Object.key()))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:ProvisioningArtifact
    properties:
      name: example
      productId: ${exampleAwsServicecatalogProduct.id}
      type: CLOUD_FORMATION_TEMPLATE
      templateUrl: https://${exampleAwsS3Bucket.bucketRegionalDomainName}/${exampleAwsS3Object.key}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_provisioning_artifact` using the provisioning artifact ID and product ID separated by a colon. For example:

```sh
$ pulumi import aws:servicecatalog/provisioningArtifact:ProvisioningArtifact example pa-ij2b6lusy6dec:prod-el3an0rma3
```
Å
acceptLanguageB" iLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). The default value is `en`.
Á
activeB
 ÷Whether the product version is active. Inactive provisioning artifacts are invisible to end users. End users cannot launch or update a provisioned product from an inactive provisioning artifact. Default is `true`.
ë
descriptionB" |Description of the provisioning artifact (i.e., version), including how it differs from the previous provisioning artifact.
î
disableTemplateValidationB
 qWhether AWS Service Catalog stops validating the specified provisioning artifact template even if it is invalid.
‚
guidanceB" œInformation set by the administrator to provide guidance to end users about which provisioning artifacts to use. Valid values are `DEFAULT` and `DEPRECATED`. The default is `DEFAULT`. Users are able to make updates to a provisioned product of a deprecated version but cannot launch new provisioned products using a deprecated version.
f
nameB" XName of the provisioning artifact (for example, `v1`, `v2beta`). No spaces are allowed.
,
	productId" Identifier of the product.
í
templatePhysicalIdB" ıTemplate source as the physical ID of the resource that contains the template. Currently only supports CloudFormation stack ARN. Specify the physical ID as `arn:[partition]:cloudformation:[region]:[account ID]:stack/[stack name]/[resource ID]`.
Ä
templateUrlB" kTemplate source as URL of the CloudFormation template in Amazon S3.

The following arguments are optional:
µ
typeB" ¶Type of provisioning artifact. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_ProvisioningArtifactProperties.html) for valid list of values.
"Å
acceptLanguageB" iLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). The default value is `en`.
"Á
activeB
 ÷Whether the product version is active. Inactive provisioning artifacts are invisible to end users. End users cannot launch or update a provisioned product from an inactive provisioning artifact. Default is `true`.
"D
createdTime" 1Time when the provisioning artifact was created.
"è
description" |Description of the provisioning artifact (i.e., version), including how it differs from the previous provisioning artifact.
"î
disableTemplateValidationB
 qWhether AWS Service Catalog stops validating the specified provisioning artifact template even if it is invalid.
"‚
guidanceB" œInformation set by the administrator to provide guidance to end users about which provisioning artifacts to use. Valid values are `DEFAULT` and `DEPRECATED`. The default is `DEFAULT`. Users are able to make updates to a provisioned product of a deprecated version but cannot launch new provisioned products using a deprecated version.
"d
name" XName of the provisioning artifact (for example, `v1`, `v2beta`). No spaces are allowed.
",
	productId" Identifier of the product.
"@
provisioningArtifactId" "Provisioning artifact identifier.
"í
templatePhysicalIdB" ıTemplate source as the physical ID of the resource that contains the template. Currently only supports CloudFormation stack ARN. Specify the physical ID as `arn:[partition]:cloudformation:[region]:[account ID]:stack/[stack name]/[resource ID]`.
"Ä
templateUrlB" kTemplate source as URL of the CloudFormation template in Amazon S3.

The following arguments are optional:
"µ
typeB" ¶Type of provisioning artifact. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_ProvisioningArtifactProperties.html) for valid list of values.
* 
O
servicecatalogServiceAction.aws:servicecatalog/serviceAction:ServiceActionÿManages a Service Catalog self-service action.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.ServiceAction("example", {
    description: "Motor generator unit",
    name: "MGU",
    definition: {
        name: "AWS-RestartEC2Instance",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.ServiceAction("example",
    description="Motor generator unit",
    name="MGU",
    definition={
        "name": "AWS-RestartEC2Instance",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.ServiceAction("example", new()
    {
        Description = "Motor generator unit",
        Name = "MGU",
        Definition = new Aws.ServiceCatalog.Inputs.ServiceActionDefinitionArgs
        {
            Name = "AWS-RestartEC2Instance",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewServiceAction(ctx, "example", &servicecatalog.ServiceActionArgs{
			Description: pulumi.String("Motor generator unit"),
			Name:        pulumi.String("MGU"),
			Definition: &servicecatalog.ServiceActionDefinitionArgs{
				Name: pulumi.String("AWS-RestartEC2Instance"),
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
import com.pulumi.aws.servicecatalog.ServiceAction;
import com.pulumi.aws.servicecatalog.ServiceActionArgs;
import com.pulumi.aws.servicecatalog.inputs.ServiceActionDefinitionArgs;
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
        var example = new ServiceAction("example", ServiceActionArgs.builder()
            .description("Motor generator unit")
            .name("MGU")
            .definition(ServiceActionDefinitionArgs.builder()
                .name("AWS-RestartEC2Instance")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:ServiceAction
    properties:
      description: Motor generator unit
      name: MGU
      definition:
        name: AWS-RestartEC2Instance
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_service_action` using the service action ID. For example:

```sh
$ pulumi import aws:servicecatalog/serviceAction:ServiceAction example act-f1w12eperfslh
```
~
acceptLanguageB" fLanguage code. Valid values are `en` (English), `jp` (Japanese), and `zh` (Chinese). Default is `en`.
≈

definitionq:o
m
servicecatalogServiceActionDefinitionBaws:servicecatalog/ServiceActionDefinition:ServiceActionDefinitionDSelf-service action definition configuration block. Detailed below.
6
descriptionB" !Self-service action description.
O
nameB" ASelf-service action name.

The following arguments are optional:
"~
acceptLanguageB" fLanguage code. Valid values are `en` (English), `jp` (Japanese), and `zh` (Chinese). Default is `en`.
"≈

definitionq:o
m
servicecatalogServiceActionDefinitionBaws:servicecatalog/ServiceActionDefinition:ServiceActionDefinitionDSelf-service action definition configuration block. Detailed below.
"4
description" !Self-service action description.
"M
name" ASelf-service action name.

The following arguments are optional:
*∫
C
servicecatalog	TagOption&aws:servicecatalog/tagOption:TagOptionïManages a Service Catalog Tag Option.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.TagOption("example", {
    key: "nyckel",
    value: "v√§rde",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.TagOption("example",
    key="nyckel",
    value="v√§rde")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.TagOption("example", new()
    {
        Key = "nyckel",
        Value = "v√§rde",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewTagOption(ctx, "example", &servicecatalog.TagOptionArgs{
			Key:   pulumi.String("nyckel"),
			Value: pulumi.String("v√§rde"),
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
import com.pulumi.aws.servicecatalog.TagOption;
import com.pulumi.aws.servicecatalog.TagOptionArgs;
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
        var example = new TagOption("example", TagOptionArgs.builder()
            .key("nyckel")
            .value("v√§rde")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:TagOption
    properties:
      key: nyckel
      value: v√§rde
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_tag_option` using the tag option ID. For example:

```sh
$ pulumi import aws:servicecatalog/tagOption:TagOption example tag-pjtvagohlyo3m
```
A
activeB
 1Whether tag option is active. Default is `true`.

key" Tag option key.
F
value" 9Tag option value.

The following arguments are optional:
"A
activeB
 1Whether tag option is active. Default is `true`.
"
key" Tag option key.
"
owner" "F
value" 9Tag option value.

The following arguments are optional:
*È
|
servicecatalogTagOptionResourceAssociationLaws:servicecatalog/tagOptionResourceAssociation:TagOptionResourceAssociationÎManages a Service Catalog Tag Option Resource Association.

> **Tip:** A "resource" is either a Service Catalog portfolio or product.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicecatalog.TagOptionResourceAssociation("example", {
    resourceId: "prod-dnigbtea24ste",
    tagOptionId: "tag-pjtvyakdlyo3m",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.TagOptionResourceAssociation("example",
    resource_id="prod-dnigbtea24ste",
    tag_option_id="tag-pjtvyakdlyo3m")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceCatalog.TagOptionResourceAssociation("example", new()
    {
        ResourceId = "prod-dnigbtea24ste",
        TagOptionId = "tag-pjtvyakdlyo3m",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.NewTagOptionResourceAssociation(ctx, "example", &servicecatalog.TagOptionResourceAssociationArgs{
			ResourceId:  pulumi.String("prod-dnigbtea24ste"),
			TagOptionId: pulumi.String("tag-pjtvyakdlyo3m"),
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
import com.pulumi.aws.servicecatalog.TagOptionResourceAssociation;
import com.pulumi.aws.servicecatalog.TagOptionResourceAssociationArgs;
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
        var example = new TagOptionResourceAssociation("example", TagOptionResourceAssociationArgs.builder()
            .resourceId("prod-dnigbtea24ste")
            .tagOptionId("tag-pjtvyakdlyo3m")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicecatalog:TagOptionResourceAssociation
    properties:
      resourceId: prod-dnigbtea24ste
      tagOptionId: tag-pjtvyakdlyo3m
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicecatalog_tag_option_resource_association` using the tag option ID and resource ID. For example:

```sh
$ pulumi import aws:servicecatalog/tagOptionResourceAssociation:TagOptionResourceAssociation example tag-pjtvyakdlyo3m:prod-dnigbtea24ste
```
'

resourceId" Resource identifier.
*
tagOptionId" Tag Option identifier.
"(
resourceArn" ARN of the resource.
":
resourceCreatedTime" Creation time of the resource.
"8
resourceDescription" Description of the resource.
"'

resourceId" Resource identifier.
"1
resourceName" Description of the resource.
"*
tagOptionId" Tag Option identifier.
*û
S
servicediscoveryHttpNamespace0aws:servicediscovery/httpNamespace:HttpNamespace## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicediscovery.HttpNamespace("example", {
    name: "development",
    description: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicediscovery.HttpNamespace("example",
    name="development",
    description="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceDiscovery.HttpNamespace("example", new()
    {
        Name = "development",
        Description = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicediscovery.NewHttpNamespace(ctx, "example", &servicediscovery.HttpNamespaceArgs{
			Name:        pulumi.String("development"),
			Description: pulumi.String("example"),
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
import com.pulumi.aws.servicediscovery.HttpNamespace;
import com.pulumi.aws.servicediscovery.HttpNamespaceArgs;
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
        var example = new HttpNamespace("example", HttpNamespaceArgs.builder()
            .name("development")
            .description("example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicediscovery:HttpNamespace
    properties:
      name: development
      description: example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Discovery HTTP Namespace using the namespace ID. For example:

```sh
$ pulumi import aws:servicediscovery/httpNamespace:HttpNamespace example ns-1234567890
```
\
descriptionB" GThe description that you specify for the namespace when you create it.
.
nameB"  The name of the http namespace.
—
tagsB2" ¿A map of tags to assign to the namespace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"U
arn" JThe ARN that Amazon Route 53 assigns to the namespace when you create it.
"\
descriptionB" GThe description that you specify for the namespace when you create it.
"/
httpName" The name of an HTTP namespace.
",
name"  The name of the http namespace.
"—
tagsB2" ¿A map of tags to assign to the namespace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
* r
D
servicediscoveryInstance&aws:servicediscovery/instance:InstanceèlProvides a Service Discovery Instance resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ec2.Vpc("example", {
    cidrBlock: "10.0.0.0/16",
    enableDnsSupport: true,
    enableDnsHostnames: true,
});
const examplePrivateDnsNamespace = new aws.servicediscovery.PrivateDnsNamespace("example", {
    name: "example.domain.local",
    description: "example",
    vpc: example.id,
});
const exampleService = new aws.servicediscovery.Service("example", {
    name: "example",
    dnsConfig: {
        namespaceId: examplePrivateDnsNamespace.id,
        dnsRecords: [{
            ttl: 10,
            type: "A",
        }],
        routingPolicy: "MULTIVALUE",
    },
    healthCheckCustomConfig: {
        failureThreshold: 1,
    },
});
const exampleInstance = new aws.servicediscovery.Instance("example", {
    instanceId: "example-instance-id",
    serviceId: exampleService.id,
    attributes: {
        AWS_INSTANCE_IPV4: "172.18.0.1",
        custom_attribute: "custom",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ec2.Vpc("example",
    cidr_block="10.0.0.0/16",
    enable_dns_support=True,
    enable_dns_hostnames=True)
example_private_dns_namespace = aws.servicediscovery.PrivateDnsNamespace("example",
    name="example.domain.local",
    description="example",
    vpc=example.id)
example_service = aws.servicediscovery.Service("example",
    name="example",
    dns_config={
        "namespace_id": example_private_dns_namespace.id,
        "dns_records": [{
            "ttl": 10,
            "type": "A",
        }],
        "routing_policy": "MULTIVALUE",
    },
    health_check_custom_config={
        "failure_threshold": 1,
    })
example_instance = aws.servicediscovery.Instance("example",
    instance_id="example-instance-id",
    service_id=example_service.id,
    attributes={
        "AWS_INSTANCE_IPV4": "172.18.0.1",
        "custom_attribute": "custom",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ec2.Vpc("example", new()
    {
        CidrBlock = "10.0.0.0/16",
        EnableDnsSupport = true,
        EnableDnsHostnames = true,
    });

    var examplePrivateDnsNamespace = new Aws.ServiceDiscovery.PrivateDnsNamespace("example", new()
    {
        Name = "example.domain.local",
        Description = "example",
        Vpc = example.Id,
    });

    var exampleService = new Aws.ServiceDiscovery.Service("example", new()
    {
        Name = "example",
        DnsConfig = new Aws.ServiceDiscovery.Inputs.ServiceDnsConfigArgs
        {
            NamespaceId = examplePrivateDnsNamespace.Id,
            DnsRecords = new[]
            {
                new Aws.ServiceDiscovery.Inputs.ServiceDnsConfigDnsRecordArgs
                {
                    Ttl = 10,
                    Type = "A",
                },
            },
            RoutingPolicy = "MULTIVALUE",
        },
        HealthCheckCustomConfig = new Aws.ServiceDiscovery.Inputs.ServiceHealthCheckCustomConfigArgs
        {
            FailureThreshold = 1,
        },
    });

    var exampleInstance = new Aws.ServiceDiscovery.Instance("example", new()
    {
        InstanceId = "example-instance-id",
        ServiceId = exampleService.Id,
        Attributes = 
        {
            { "AWS_INSTANCE_IPV4", "172.18.0.1" },
            { "custom_attribute", "custom" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ec2.NewVpc(ctx, "example", &ec2.VpcArgs{
			CidrBlock:          pulumi.String("10.0.0.0/16"),
			EnableDnsSupport:   pulumi.Bool(true),
			EnableDnsHostnames: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		examplePrivateDnsNamespace, err := servicediscovery.NewPrivateDnsNamespace(ctx, "example", &servicediscovery.PrivateDnsNamespaceArgs{
			Name:        pulumi.String("example.domain.local"),
			Description: pulumi.String("example"),
			Vpc:         example.ID(),
		})
		if err != nil {
			return err
		}
		exampleService, err := servicediscovery.NewService(ctx, "example", &servicediscovery.ServiceArgs{
			Name: pulumi.String("example"),
			DnsConfig: &servicediscovery.ServiceDnsConfigArgs{
				NamespaceId: examplePrivateDnsNamespace.ID(),
				DnsRecords: servicediscovery.ServiceDnsConfigDnsRecordArray{
					&servicediscovery.ServiceDnsConfigDnsRecordArgs{
						Ttl:  pulumi.Int(10),
						Type: pulumi.String("A"),
					},
				},
				RoutingPolicy: pulumi.String("MULTIVALUE"),
			},
			HealthCheckCustomConfig: &servicediscovery.ServiceHealthCheckCustomConfigArgs{
				FailureThreshold: pulumi.Int(1),
			},
		})
		if err != nil {
			return err
		}
		_, err = servicediscovery.NewInstance(ctx, "example", &servicediscovery.InstanceArgs{
			InstanceId: pulumi.String("example-instance-id"),
			ServiceId:  exampleService.ID(),
			Attributes: pulumi.StringMap{
				"AWS_INSTANCE_IPV4": pulumi.String("172.18.0.1"),
				"custom_attribute":  pulumi.String("custom"),
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
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.servicediscovery.PrivateDnsNamespace;
import com.pulumi.aws.servicediscovery.PrivateDnsNamespaceArgs;
import com.pulumi.aws.servicediscovery.Service;
import com.pulumi.aws.servicediscovery.ServiceArgs;
import com.pulumi.aws.servicediscovery.inputs.ServiceDnsConfigArgs;
import com.pulumi.aws.servicediscovery.inputs.ServiceHealthCheckCustomConfigArgs;
import com.pulumi.aws.servicediscovery.Instance;
import com.pulumi.aws.servicediscovery.InstanceArgs;
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
        var example = new Vpc("example", VpcArgs.builder()
            .cidrBlock("10.0.0.0/16")
            .enableDnsSupport(true)
            .enableDnsHostnames(true)
            .build());

        var examplePrivateDnsNamespace = new PrivateDnsNamespace("examplePrivateDnsNamespace", PrivateDnsNamespaceArgs.builder()
            .name("example.domain.local")
            .description("example")
            .vpc(example.id())
            .build());

        var exampleService = new Service("exampleService", ServiceArgs.builder()
            .name("example")
            .dnsConfig(ServiceDnsConfigArgs.builder()
                .namespaceId(examplePrivateDnsNamespace.id())
                .dnsRecords(ServiceDnsConfigDnsRecordArgs.builder()
                    .ttl(10)
                    .type("A")
                    .build())
                .routingPolicy("MULTIVALUE")
                .build())
            .healthCheckCustomConfig(ServiceHealthCheckCustomConfigArgs.builder()
                .failureThreshold(1)
                .build())
            .build());

        var exampleInstance = new Instance("exampleInstance", InstanceArgs.builder()
            .instanceId("example-instance-id")
            .serviceId(exampleService.id())
            .attributes(Map.ofEntries(
                Map.entry("AWS_INSTANCE_IPV4", "172.18.0.1"),
                Map.entry("custom_attribute", "custom")
            ))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ec2:Vpc
    properties:
      cidrBlock: 10.0.0.0/16
      enableDnsSupport: true
      enableDnsHostnames: true
  examplePrivateDnsNamespace:
    type: aws:servicediscovery:PrivateDnsNamespace
    name: example
    properties:
      name: example.domain.local
      description: example
      vpc: ${example.id}
  exampleService:
    type: aws:servicediscovery:Service
    name: example
    properties:
      name: example
      dnsConfig:
        namespaceId: ${examplePrivateDnsNamespace.id}
        dnsRecords:
          - ttl: 10
            type: A
        routingPolicy: MULTIVALUE
      healthCheckCustomConfig:
        failureThreshold: 1
  exampleInstance:
    type: aws:servicediscovery:Instance
    name: example
    properties:
      instanceId: example-instance-id
      serviceId: ${exampleService.id}
      attributes:
        AWS_INSTANCE_IPV4: 172.18.0.1
        custom_attribute: custom
```
<!--End PulumiCodeChooser -->

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicediscovery.HttpNamespace("example", {
    name: "example.domain.test",
    description: "example",
});
const exampleService = new aws.servicediscovery.Service("example", {
    name: "example",
    namespaceId: example.id,
});
const exampleInstance = new aws.servicediscovery.Instance("example", {
    instanceId: "example-instance-id",
    serviceId: exampleService.id,
    attributes: {
        AWS_EC2_INSTANCE_ID: "i-0abdg374kd892cj6dl",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicediscovery.HttpNamespace("example",
    name="example.domain.test",
    description="example")
example_service = aws.servicediscovery.Service("example",
    name="example",
    namespace_id=example.id)
example_instance = aws.servicediscovery.Instance("example",
    instance_id="example-instance-id",
    service_id=example_service.id,
    attributes={
        "AWS_EC2_INSTANCE_ID": "i-0abdg374kd892cj6dl",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceDiscovery.HttpNamespace("example", new()
    {
        Name = "example.domain.test",
        Description = "example",
    });

    var exampleService = new Aws.ServiceDiscovery.Service("example", new()
    {
        Name = "example",
        NamespaceId = example.Id,
    });

    var exampleInstance = new Aws.ServiceDiscovery.Instance("example", new()
    {
        InstanceId = "example-instance-id",
        ServiceId = exampleService.Id,
        Attributes = 
        {
            { "AWS_EC2_INSTANCE_ID", "i-0abdg374kd892cj6dl" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := servicediscovery.NewHttpNamespace(ctx, "example", &servicediscovery.HttpNamespaceArgs{
			Name:        pulumi.String("example.domain.test"),
			Description: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleService, err := servicediscovery.NewService(ctx, "example", &servicediscovery.ServiceArgs{
			Name:        pulumi.String("example"),
			NamespaceId: example.ID(),
		})
		if err != nil {
			return err
		}
		_, err = servicediscovery.NewInstance(ctx, "example", &servicediscovery.InstanceArgs{
			InstanceId: pulumi.String("example-instance-id"),
			ServiceId:  exampleService.ID(),
			Attributes: pulumi.StringMap{
				"AWS_EC2_INSTANCE_ID": pulumi.String("i-0abdg374kd892cj6dl"),
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
import com.pulumi.aws.servicediscovery.HttpNamespace;
import com.pulumi.aws.servicediscovery.HttpNamespaceArgs;
import com.pulumi.aws.servicediscovery.Service;
import com.pulumi.aws.servicediscovery.ServiceArgs;
import com.pulumi.aws.servicediscovery.Instance;
import com.pulumi.aws.servicediscovery.InstanceArgs;
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
        var example = new HttpNamespace("example", HttpNamespaceArgs.builder()
            .name("example.domain.test")
            .description("example")
            .build());

        var exampleService = new Service("exampleService", ServiceArgs.builder()
            .name("example")
            .namespaceId(example.id())
            .build());

        var exampleInstance = new Instance("exampleInstance", InstanceArgs.builder()
            .instanceId("example-instance-id")
            .serviceId(exampleService.id())
            .attributes(Map.of("AWS_EC2_INSTANCE_ID", "i-0abdg374kd892cj6dl"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicediscovery:HttpNamespace
    properties:
      name: example.domain.test
      description: example
  exampleService:
    type: aws:servicediscovery:Service
    name: example
    properties:
      name: example
      namespaceId: ${example.id}
  exampleInstance:
    type: aws:servicediscovery:Instance
    name: example
    properties:
      instanceId: example-instance-id
      serviceId: ${exampleService.id}
      attributes:
        AWS_EC2_INSTANCE_ID: i-0abdg374kd892cj6dl
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Discovery Instance using the service ID and instance ID. For example:

```sh
$ pulumi import aws:servicediscovery/instance:Instance example 0123456789/i-0123
```
Ï

attributes2" ◊A map contains the attributes of the instance. Check the [doc](https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html#API_RegisterInstance_RequestSyntax) for the supported attributes and syntax.
2

instanceId"  The ID of the service instance.
T
	serviceId" CThe ID of the service that you want to use to create the instance.
"Ï

attributes2" ◊A map contains the attributes of the instance. Check the [doc](https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html#API_RegisterInstance_RequestSyntax) for the supported attributes and syntax.
"2

instanceId"  The ID of the service instance.
"T
	serviceId" CThe ID of the service that you want to use to create the instance.
*é%
e
servicediscoveryPrivateDnsNamespace<aws:servicediscovery/privateDnsNamespace:PrivateDnsNamespaceåProvides a Service Discovery Private DNS Namespace resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ec2.Vpc("example", {cidrBlock: "10.0.0.0/16"});
const examplePrivateDnsNamespace = new aws.servicediscovery.PrivateDnsNamespace("example", {
    name: "hoge.example.local",
    description: "example",
    vpc: example.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ec2.Vpc("example", cidr_block="10.0.0.0/16")
example_private_dns_namespace = aws.servicediscovery.PrivateDnsNamespace("example",
    name="hoge.example.local",
    description="example",
    vpc=example.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ec2.Vpc("example", new()
    {
        CidrBlock = "10.0.0.0/16",
    });

    var examplePrivateDnsNamespace = new Aws.ServiceDiscovery.PrivateDnsNamespace("example", new()
    {
        Name = "hoge.example.local",
        Description = "example",
        Vpc = example.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ec2.NewVpc(ctx, "example", &ec2.VpcArgs{
			CidrBlock: pulumi.String("10.0.0.0/16"),
		})
		if err != nil {
			return err
		}
		_, err = servicediscovery.NewPrivateDnsNamespace(ctx, "example", &servicediscovery.PrivateDnsNamespaceArgs{
			Name:        pulumi.String("hoge.example.local"),
			Description: pulumi.String("example"),
			Vpc:         example.ID(),
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
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.servicediscovery.PrivateDnsNamespace;
import com.pulumi.aws.servicediscovery.PrivateDnsNamespaceArgs;
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
        var example = new Vpc("example", VpcArgs.builder()
            .cidrBlock("10.0.0.0/16")
            .build());

        var examplePrivateDnsNamespace = new PrivateDnsNamespace("examplePrivateDnsNamespace", PrivateDnsNamespaceArgs.builder()
            .name("hoge.example.local")
            .description("example")
            .vpc(example.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ec2:Vpc
    properties:
      cidrBlock: 10.0.0.0/16
  examplePrivateDnsNamespace:
    type: aws:servicediscovery:PrivateDnsNamespace
    name: example
    properties:
      name: hoge.example.local
      description: example
      vpc: ${example.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Discovery Private DNS Namespace using the namespace ID and VPC ID. For example:

```sh
$ pulumi import aws:servicediscovery/privateDnsNamespace:PrivateDnsNamespace example 0123456789:vpc-123345
```
\
descriptionB" GThe description that you specify for the namespace when you create it.
)
nameB" The name of the namespace.
—
tagsB2" ¿A map of tags to assign to the namespace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
H
vpc" =The ID of VPC that you want to associate the namespace with.
"U
arn" JThe ARN that Amazon Route 53 assigns to the namespace when you create it.
"\
descriptionB" GThe description that you specify for the namespace when you create it.
"g

hostedZone" UThe ID for the hosted zone that Amazon Route 53 creates when you create a namespace.
"'
name" The name of the namespace.
"—
tagsB2" ¿A map of tags to assign to the namespace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"H
vpc" =The ID of VPC that you want to associate the namespace with.
*ı
b
servicediscoveryPublicDnsNamespace:aws:servicediscovery/publicDnsNamespace:PublicDnsNamespaceäProvides a Service Discovery Public DNS Namespace resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicediscovery.PublicDnsNamespace("example", {
    name: "hoge.example.com",
    description: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicediscovery.PublicDnsNamespace("example",
    name="hoge.example.com",
    description="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceDiscovery.PublicDnsNamespace("example", new()
    {
        Name = "hoge.example.com",
        Description = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicediscovery.NewPublicDnsNamespace(ctx, "example", &servicediscovery.PublicDnsNamespaceArgs{
			Name:        pulumi.String("hoge.example.com"),
			Description: pulumi.String("example"),
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
import com.pulumi.aws.servicediscovery.PublicDnsNamespace;
import com.pulumi.aws.servicediscovery.PublicDnsNamespaceArgs;
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
        var example = new PublicDnsNamespace("example", PublicDnsNamespaceArgs.builder()
            .name("hoge.example.com")
            .description("example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicediscovery:PublicDnsNamespace
    properties:
      name: hoge.example.com
      description: example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Discovery Public DNS Namespace using the namespace ID. For example:

```sh
$ pulumi import aws:servicediscovery/publicDnsNamespace:PublicDnsNamespace example 0123456789
```
\
descriptionB" GThe description that you specify for the namespace when you create it.
)
nameB" The name of the namespace.
—
tagsB2" ¿A map of tags to assign to the namespace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"U
arn" JThe ARN that Amazon Route 53 assigns to the namespace when you create it.
"\
descriptionB" GThe description that you specify for the namespace when you create it.
"g

hostedZone" UThe ID for the hosted zone that Amazon Route 53 creates when you create a namespace.
"'
name" The name of the namespace.
"—
tagsB2" ¿A map of tags to assign to the namespace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*Ω~
A
servicediscoveryService$aws:servicediscovery/service:Service‰aProvides a Service Discovery Service resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ec2.Vpc("example", {
    cidrBlock: "10.0.0.0/16",
    enableDnsSupport: true,
    enableDnsHostnames: true,
});
const examplePrivateDnsNamespace = new aws.servicediscovery.PrivateDnsNamespace("example", {
    name: "example.mydomain.local",
    description: "example",
    vpc: example.id,
});
const exampleService = new aws.servicediscovery.Service("example", {
    name: "example",
    dnsConfig: {
        namespaceId: examplePrivateDnsNamespace.id,
        dnsRecords: [{
            ttl: 10,
            type: "A",
        }],
        routingPolicy: "MULTIVALUE",
    },
    healthCheckCustomConfig: {
        failureThreshold: 1,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ec2.Vpc("example",
    cidr_block="10.0.0.0/16",
    enable_dns_support=True,
    enable_dns_hostnames=True)
example_private_dns_namespace = aws.servicediscovery.PrivateDnsNamespace("example",
    name="example.mydomain.local",
    description="example",
    vpc=example.id)
example_service = aws.servicediscovery.Service("example",
    name="example",
    dns_config={
        "namespace_id": example_private_dns_namespace.id,
        "dns_records": [{
            "ttl": 10,
            "type": "A",
        }],
        "routing_policy": "MULTIVALUE",
    },
    health_check_custom_config={
        "failure_threshold": 1,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ec2.Vpc("example", new()
    {
        CidrBlock = "10.0.0.0/16",
        EnableDnsSupport = true,
        EnableDnsHostnames = true,
    });

    var examplePrivateDnsNamespace = new Aws.ServiceDiscovery.PrivateDnsNamespace("example", new()
    {
        Name = "example.mydomain.local",
        Description = "example",
        Vpc = example.Id,
    });

    var exampleService = new Aws.ServiceDiscovery.Service("example", new()
    {
        Name = "example",
        DnsConfig = new Aws.ServiceDiscovery.Inputs.ServiceDnsConfigArgs
        {
            NamespaceId = examplePrivateDnsNamespace.Id,
            DnsRecords = new[]
            {
                new Aws.ServiceDiscovery.Inputs.ServiceDnsConfigDnsRecordArgs
                {
                    Ttl = 10,
                    Type = "A",
                },
            },
            RoutingPolicy = "MULTIVALUE",
        },
        HealthCheckCustomConfig = new Aws.ServiceDiscovery.Inputs.ServiceHealthCheckCustomConfigArgs
        {
            FailureThreshold = 1,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ec2.NewVpc(ctx, "example", &ec2.VpcArgs{
			CidrBlock:          pulumi.String("10.0.0.0/16"),
			EnableDnsSupport:   pulumi.Bool(true),
			EnableDnsHostnames: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		examplePrivateDnsNamespace, err := servicediscovery.NewPrivateDnsNamespace(ctx, "example", &servicediscovery.PrivateDnsNamespaceArgs{
			Name:        pulumi.String("example.mydomain.local"),
			Description: pulumi.String("example"),
			Vpc:         example.ID(),
		})
		if err != nil {
			return err
		}
		_, err = servicediscovery.NewService(ctx, "example", &servicediscovery.ServiceArgs{
			Name: pulumi.String("example"),
			DnsConfig: &servicediscovery.ServiceDnsConfigArgs{
				NamespaceId: examplePrivateDnsNamespace.ID(),
				DnsRecords: servicediscovery.ServiceDnsConfigDnsRecordArray{
					&servicediscovery.ServiceDnsConfigDnsRecordArgs{
						Ttl:  pulumi.Int(10),
						Type: pulumi.String("A"),
					},
				},
				RoutingPolicy: pulumi.String("MULTIVALUE"),
			},
			HealthCheckCustomConfig: &servicediscovery.ServiceHealthCheckCustomConfigArgs{
				FailureThreshold: pulumi.Int(1),
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
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.servicediscovery.PrivateDnsNamespace;
import com.pulumi.aws.servicediscovery.PrivateDnsNamespaceArgs;
import com.pulumi.aws.servicediscovery.Service;
import com.pulumi.aws.servicediscovery.ServiceArgs;
import com.pulumi.aws.servicediscovery.inputs.ServiceDnsConfigArgs;
import com.pulumi.aws.servicediscovery.inputs.ServiceHealthCheckCustomConfigArgs;
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
        var example = new Vpc("example", VpcArgs.builder()
            .cidrBlock("10.0.0.0/16")
            .enableDnsSupport(true)
            .enableDnsHostnames(true)
            .build());

        var examplePrivateDnsNamespace = new PrivateDnsNamespace("examplePrivateDnsNamespace", PrivateDnsNamespaceArgs.builder()
            .name("example.mydomain.local")
            .description("example")
            .vpc(example.id())
            .build());

        var exampleService = new Service("exampleService", ServiceArgs.builder()
            .name("example")
            .dnsConfig(ServiceDnsConfigArgs.builder()
                .namespaceId(examplePrivateDnsNamespace.id())
                .dnsRecords(ServiceDnsConfigDnsRecordArgs.builder()
                    .ttl(10)
                    .type("A")
                    .build())
                .routingPolicy("MULTIVALUE")
                .build())
            .healthCheckCustomConfig(ServiceHealthCheckCustomConfigArgs.builder()
                .failureThreshold(1)
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ec2:Vpc
    properties:
      cidrBlock: 10.0.0.0/16
      enableDnsSupport: true
      enableDnsHostnames: true
  examplePrivateDnsNamespace:
    type: aws:servicediscovery:PrivateDnsNamespace
    name: example
    properties:
      name: example.mydomain.local
      description: example
      vpc: ${example.id}
  exampleService:
    type: aws:servicediscovery:Service
    name: example
    properties:
      name: example
      dnsConfig:
        namespaceId: ${examplePrivateDnsNamespace.id}
        dnsRecords:
          - ttl: 10
            type: A
        routingPolicy: MULTIVALUE
      healthCheckCustomConfig:
        failureThreshold: 1
```
<!--End PulumiCodeChooser -->

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicediscovery.PublicDnsNamespace("example", {
    name: "example.mydomain.com",
    description: "example",
});
const exampleService = new aws.servicediscovery.Service("example", {
    name: "example",
    dnsConfig: {
        namespaceId: example.id,
        dnsRecords: [{
            ttl: 10,
            type: "A",
        }],
    },
    healthCheckConfig: {
        failureThreshold: 10,
        resourcePath: "path",
        type: "HTTP",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicediscovery.PublicDnsNamespace("example",
    name="example.mydomain.com",
    description="example")
example_service = aws.servicediscovery.Service("example",
    name="example",
    dns_config={
        "namespace_id": example.id,
        "dns_records": [{
            "ttl": 10,
            "type": "A",
        }],
    },
    health_check_config={
        "failure_threshold": 10,
        "resource_path": "path",
        "type": "HTTP",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceDiscovery.PublicDnsNamespace("example", new()
    {
        Name = "example.mydomain.com",
        Description = "example",
    });

    var exampleService = new Aws.ServiceDiscovery.Service("example", new()
    {
        Name = "example",
        DnsConfig = new Aws.ServiceDiscovery.Inputs.ServiceDnsConfigArgs
        {
            NamespaceId = example.Id,
            DnsRecords = new[]
            {
                new Aws.ServiceDiscovery.Inputs.ServiceDnsConfigDnsRecordArgs
                {
                    Ttl = 10,
                    Type = "A",
                },
            },
        },
        HealthCheckConfig = new Aws.ServiceDiscovery.Inputs.ServiceHealthCheckConfigArgs
        {
            FailureThreshold = 10,
            ResourcePath = "path",
            Type = "HTTP",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := servicediscovery.NewPublicDnsNamespace(ctx, "example", &servicediscovery.PublicDnsNamespaceArgs{
			Name:        pulumi.String("example.mydomain.com"),
			Description: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		_, err = servicediscovery.NewService(ctx, "example", &servicediscovery.ServiceArgs{
			Name: pulumi.String("example"),
			DnsConfig: &servicediscovery.ServiceDnsConfigArgs{
				NamespaceId: example.ID(),
				DnsRecords: servicediscovery.ServiceDnsConfigDnsRecordArray{
					&servicediscovery.ServiceDnsConfigDnsRecordArgs{
						Ttl:  pulumi.Int(10),
						Type: pulumi.String("A"),
					},
				},
			},
			HealthCheckConfig: &servicediscovery.ServiceHealthCheckConfigArgs{
				FailureThreshold: pulumi.Int(10),
				ResourcePath:     pulumi.String("path"),
				Type:             pulumi.String("HTTP"),
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
import com.pulumi.aws.servicediscovery.PublicDnsNamespace;
import com.pulumi.aws.servicediscovery.PublicDnsNamespaceArgs;
import com.pulumi.aws.servicediscovery.Service;
import com.pulumi.aws.servicediscovery.ServiceArgs;
import com.pulumi.aws.servicediscovery.inputs.ServiceDnsConfigArgs;
import com.pulumi.aws.servicediscovery.inputs.ServiceHealthCheckConfigArgs;
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
        var example = new PublicDnsNamespace("example", PublicDnsNamespaceArgs.builder()
            .name("example.mydomain.com")
            .description("example")
            .build());

        var exampleService = new Service("exampleService", ServiceArgs.builder()
            .name("example")
            .dnsConfig(ServiceDnsConfigArgs.builder()
                .namespaceId(example.id())
                .dnsRecords(ServiceDnsConfigDnsRecordArgs.builder()
                    .ttl(10)
                    .type("A")
                    .build())
                .build())
            .healthCheckConfig(ServiceHealthCheckConfigArgs.builder()
                .failureThreshold(10)
                .resourcePath("path")
                .type("HTTP")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicediscovery:PublicDnsNamespace
    properties:
      name: example.mydomain.com
      description: example
  exampleService:
    type: aws:servicediscovery:Service
    name: example
    properties:
      name: example
      dnsConfig:
        namespaceId: ${example.id}
        dnsRecords:
          - ttl: 10
            type: A
      healthCheckConfig:
        failureThreshold: 10
        resourcePath: path
        type: HTTP
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Discovery Service using the service ID. For example:

```sh
$ pulumi import aws:servicediscovery/service:Service example 0123456789
```
5
descriptionB"  The description of the service.
•
	dnsConfigbB`:^
\
servicediscoveryServiceDnsConfig6aws:servicediscovery/ServiceDnsConfig:ServiceDnsConfig≥A complex type that contains information about the resource record sets that you want Amazon Route 53 to create when you register an instance. See `dns_config` Block for details.
œ
forceDestroyB
 ∏A boolean that indicates all instances should be deleted from the service so that the service can be destroyed without error. These instances are not recoverable. Defaults to `false`.
£
healthCheckConfigzBx:v
t
servicediscoveryServiceHealthCheckConfigFaws:servicediscovery/ServiceHealthCheckConfig:ServiceHealthCheckConfigëA complex type that contains settings for an optional health check. Only for Public DNS namespaces. See `health_check_config` Block for details.
¶
healthCheckCustomConfigèBå:â
Ü
servicediscoveryServiceHealthCheckCustomConfigRaws:servicediscovery/ServiceHealthCheckCustomConfig:ServiceHealthCheckCustomConfigyA complex type that contains settings for ECS managed health checks. See `health_check_custom_config` Block for details.
'
nameB" The name of the service.
Y
namespaceIdB" DThe ID of the namespace that you want to use to create the service.
œ
tagsB2" æA map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
€
typeB" ÃIf present, specifies that the service instances are only discoverable using the `DiscoverInstances` API operation. No DNS records is registered for the service instances. The only valid value is `HTTP`.
"#
arn" The ARN of the service.
"5
descriptionB"  The description of the service.
"•
	dnsConfigbB`:^
\
servicediscoveryServiceDnsConfig6aws:servicediscovery/ServiceDnsConfig:ServiceDnsConfig≥A complex type that contains information about the resource record sets that you want Amazon Route 53 to create when you register an instance. See `dns_config` Block for details.
"œ
forceDestroyB
 ∏A boolean that indicates all instances should be deleted from the service so that the service can be destroyed without error. These instances are not recoverable. Defaults to `false`.
"£
healthCheckConfigzBx:v
t
servicediscoveryServiceHealthCheckConfigFaws:servicediscovery/ServiceHealthCheckConfig:ServiceHealthCheckConfigëA complex type that contains settings for an optional health check. Only for Public DNS namespaces. See `health_check_config` Block for details.
"¶
healthCheckCustomConfigèBå:â
Ü
servicediscoveryServiceHealthCheckCustomConfigRaws:servicediscovery/ServiceHealthCheckCustomConfig:ServiceHealthCheckCustomConfigyA complex type that contains settings for ECS managed health checks. See `health_check_custom_config` Block for details.
"%
name" The name of the service.
"W
namespaceId" DThe ID of the namespace that you want to use to create the service.
"œ
tagsB2" æA map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"Ÿ
type" ÃIf present, specifies that the service instances are only discoverable using the `DiscoverInstances` API operation. No DNS records is registered for the service instances. The only valid value is `HTTP`.
*ñ'
J
servicequotasServiceQuota+aws:servicequotas/serviceQuota:ServiceQuota»Manages an individual Service Quota.

> **NOTE:** Global quotas apply to all AWS regions, but can only be accessed in `us-east-1` in the Commercial partition or `us-gov-west-1` in the GovCloud partition. In other regions, the AWS API will return the error `The request failed because the specified service does not exist.`

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicequotas.ServiceQuota("example", {
    quotaCode: "L-F678F1CE",
    serviceCode: "vpc",
    value: 75,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicequotas.ServiceQuota("example",
    quota_code="L-F678F1CE",
    service_code="vpc",
    value=75)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceQuotas.ServiceQuota("example", new()
    {
        QuotaCode = "L-F678F1CE",
        ServiceCode = "vpc",
        Value = 75,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicequotas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicequotas.NewServiceQuota(ctx, "example", &servicequotas.ServiceQuotaArgs{
			QuotaCode:   pulumi.String("L-F678F1CE"),
			ServiceCode: pulumi.String("vpc"),
			Value:       pulumi.Float64(75),
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
import com.pulumi.aws.servicequotas.ServiceQuota;
import com.pulumi.aws.servicequotas.ServiceQuotaArgs;
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
        var example = new ServiceQuota("example", ServiceQuotaArgs.builder()
            .quotaCode("L-F678F1CE")
            .serviceCode("vpc")
            .value(75)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicequotas:ServiceQuota
    properties:
      quotaCode: L-F678F1CE
      serviceCode: vpc
      value: 75
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_servicequotas_service_quota` using the service code and quota code, separated by a front slash (`/`). For example:

~> __NOTE:__ This resource does not require explicit import and will assume management of an existing service quota on Pulumi resource creation.

```sh
$ pulumi import aws:servicequotas/serviceQuota:ServiceQuota example vpc/L-F678F1CE
```
à
	quotaCode" ˆCode of the service quota to track. For example: `L-F678F1CE`. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html).
Ò
serviceCode" ›Code of the service to track. For example: `vpc`. Available values can be found with the [AWS CLI service-quotas list-services command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-services.html).
í
value ÑFloat specifying the desired value for the service quota. If the desired value is higher than the current value, a quota increase request is submitted. When a known request is submitted and pending, the value reflects the desired value of the pending request.
">

adjustable
 ,Whether the service quota can be increased.
"<
arn" 1Amazon Resource Name (ARN) of the service quota.
"8
defaultValue $Default value of the service quota.
"à
	quotaCode" ˆCode of the service quota to track. For example: `L-F678F1CE`. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html).
"$
	quotaName" Name of the quota.
"
	requestId" "
requestStatus" "Ò
serviceCode" ›Code of the service to track. For example: `vpc`. Available values can be found with the [AWS CLI service-quotas list-services command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-services.html).
"(
serviceName" Name of the service.
"¶
usageMetricsq*o:m
k
servicequotasServiceQuotaUsageMetricAaws:servicequotas/ServiceQuotaUsageMetric:ServiceQuotaUsageMetric#Information about the measurement.
"í
value ÑFloat specifying the desired value for the service quota. If the desired value is higher than the current value, a quota increase request is submitted. When a known request is submitted and pending, the value reflects the desired value of the pending request.
*ò
>
servicequotasTemplate#aws:servicequotas/template:Template¶Resource for managing an AWS Service Quotas Template.

> Only the management account of an organization can alter Service Quota templates, and this must be done from the `us-east-1` region.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicequotas.Template("example", {
    region: "us-east-1",
    quotaCode: "L-2ACBD22F",
    serviceCode: "lambda",
    value: 80,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicequotas.Template("example",
    region="us-east-1",
    quota_code="L-2ACBD22F",
    service_code="lambda",
    value=80)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceQuotas.Template("example", new()
    {
        Region = "us-east-1",
        QuotaCode = "L-2ACBD22F",
        ServiceCode = "lambda",
        Value = 80,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicequotas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicequotas.NewTemplate(ctx, "example", &servicequotas.TemplateArgs{
			Region:      pulumi.String("us-east-1"),
			QuotaCode:   pulumi.String("L-2ACBD22F"),
			ServiceCode: pulumi.String("lambda"),
			Value:       pulumi.Float64(80),
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
import com.pulumi.aws.servicequotas.Template;
import com.pulumi.aws.servicequotas.TemplateArgs;
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
        var example = new Template("example", TemplateArgs.builder()
            .region("us-east-1")
            .quotaCode("L-2ACBD22F")
            .serviceCode("lambda")
            .value("80")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:servicequotas:Template
    properties:
      region: us-east-1
      quotaCode: L-2ACBD22F
      serviceCode: lambda
      value: '80'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Quotas Template using the `id`. For example:

```sh
$ pulumi import aws:servicequotas/template:Template example us-east-1,L-2ACBD22F,lambda
```
Ñ
	quotaCode" sQuota identifier. To find the quota code for a specific quota, use the aws.servicequotas.ServiceQuota data source.
8
region" *AWS Region to which the template applies.
å
serviceCode" yService identifier. To find the service code value for an AWS service, use the aws.servicequotas.getService data source.
5
value (The new, increased value for the quota.
":
globalQuota
 'Indicates whether the quota is global.
"Ñ
	quotaCode" sQuota identifier. To find the quota code for a specific quota, use the aws.servicequotas.ServiceQuota data source.
"
	quotaName" Quota name.
"8
region" *AWS Region to which the template applies.
"å
serviceCode" yService identifier. To find the service code value for an AWS service, use the aws.servicequotas.getService data source.
"!
serviceName" Service name.
"!
unit" Unit of measurement.
"5
value (The new, increased value for the quota.
*ù
_
servicequotasTemplateAssociation9aws:servicequotas/templateAssociation:TemplateAssociationºResource for managing an AWS Service Quotas Template Association.

> Only the management account of an organization can associate Service Quota templates, and this must be done from the `us-east-1` region.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.servicequotas.TemplateAssociation("example", {});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicequotas.TemplateAssociation("example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.ServiceQuotas.TemplateAssociation("example");

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicequotas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicequotas.NewTemplateAssociation(ctx, "example", nil)
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
import com.pulumi.aws.servicequotas.TemplateAssociation;
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
        var example = new TemplateAssociation("example");

    }
}
```
```yaml
resources:
  example:
    type: aws:servicequotas:TemplateAssociation
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Service Quotas Template Association using the `id`. For example:

```sh
$ pulumi import aws:servicequotas/templateAssociation:TemplateAssociation example 123456789012
```

skipDestroyB
 "
skipDestroyB
 "–
status" ¡Association status. Creating this resource will result in an `ASSOCIATED` status, and quota increase requests in the template are automatically applied to new AWS accounts in the organization.
*©
N
sesActiveReceiptRuleSet1aws:ses/activeReceiptRuleSet:ActiveReceiptRuleSetœProvides a resource to designate the active SES receipt rule set

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = new aws.ses.ActiveReceiptRuleSet("main", {ruleSetName: "primary-rules"});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.ses.ActiveReceiptRuleSet("main", rule_set_name="primary-rules")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = new Aws.Ses.ActiveReceiptRuleSet("main", new()
    {
        RuleSetName = "primary-rules",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewActiveReceiptRuleSet(ctx, "main", &ses.ActiveReceiptRuleSetArgs{
			RuleSetName: pulumi.String("primary-rules"),
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
import com.pulumi.aws.ses.ActiveReceiptRuleSet;
import com.pulumi.aws.ses.ActiveReceiptRuleSetArgs;
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
        var main = new ActiveReceiptRuleSet("main", ActiveReceiptRuleSetArgs.builder()
            .ruleSetName("primary-rules")
            .build());

    }
}
```
```yaml
resources:
  main:
    type: aws:ses:ActiveReceiptRuleSet
    properties:
      ruleSetName: primary-rules
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import active SES receipt rule sets using the rule set name. For example:

```sh
$ pulumi import aws:ses/activeReceiptRuleSet:ActiveReceiptRuleSet my_rule_set my_rule_set_name
```
,
ruleSetName" The name of the rule set
")
arn" The SES receipt rule set ARN.
",
ruleSetName" The name of the rule set
*˘H
B
sesConfigurationSet)aws:ses/configurationSet:ConfigurationSet¨8Provides an SES configuration set resource.

## Example Usage

### Basic Example

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.ses.ConfigurationSet("test", {name: "some-configuration-set-test"});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.ses.ConfigurationSet("test", name="some-configuration-set-test")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Ses.ConfigurationSet("test", new()
    {
        Name = "some-configuration-set-test",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewConfigurationSet(ctx, "test", &ses.ConfigurationSetArgs{
			Name: pulumi.String("some-configuration-set-test"),
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
import com.pulumi.aws.ses.ConfigurationSet;
import com.pulumi.aws.ses.ConfigurationSetArgs;
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
        var test = new ConfigurationSet("test", ConfigurationSetArgs.builder()
            .name("some-configuration-set-test")
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:ses:ConfigurationSet
    properties:
      name: some-configuration-set-test
```
<!--End PulumiCodeChooser -->

### Require TLS Connections

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.ses.ConfigurationSet("test", {
    name: "some-configuration-set-test",
    deliveryOptions: {
        tlsPolicy: "Require",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.ses.ConfigurationSet("test",
    name="some-configuration-set-test",
    delivery_options={
        "tls_policy": "Require",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Ses.ConfigurationSet("test", new()
    {
        Name = "some-configuration-set-test",
        DeliveryOptions = new Aws.Ses.Inputs.ConfigurationSetDeliveryOptionsArgs
        {
            TlsPolicy = "Require",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewConfigurationSet(ctx, "test", &ses.ConfigurationSetArgs{
			Name: pulumi.String("some-configuration-set-test"),
			DeliveryOptions: &ses.ConfigurationSetDeliveryOptionsArgs{
				TlsPolicy: pulumi.String("Require"),
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
import com.pulumi.aws.ses.ConfigurationSet;
import com.pulumi.aws.ses.ConfigurationSetArgs;
import com.pulumi.aws.ses.inputs.ConfigurationSetDeliveryOptionsArgs;
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
        var test = new ConfigurationSet("test", ConfigurationSetArgs.builder()
            .name("some-configuration-set-test")
            .deliveryOptions(ConfigurationSetDeliveryOptionsArgs.builder()
                .tlsPolicy("Require")
                .build())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:ses:ConfigurationSet
    properties:
      name: some-configuration-set-test
      deliveryOptions:
        tlsPolicy: Require
```
<!--End PulumiCodeChooser -->

### Tracking Options

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.ses.ConfigurationSet("test", {
    name: "some-configuration-set-test",
    trackingOptions: {
        customRedirectDomain: "sub.example.com",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.ses.ConfigurationSet("test",
    name="some-configuration-set-test",
    tracking_options={
        "custom_redirect_domain": "sub.example.com",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Ses.ConfigurationSet("test", new()
    {
        Name = "some-configuration-set-test",
        TrackingOptions = new Aws.Ses.Inputs.ConfigurationSetTrackingOptionsArgs
        {
            CustomRedirectDomain = "sub.example.com",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewConfigurationSet(ctx, "test", &ses.ConfigurationSetArgs{
			Name: pulumi.String("some-configuration-set-test"),
			TrackingOptions: &ses.ConfigurationSetTrackingOptionsArgs{
				CustomRedirectDomain: pulumi.String("sub.example.com"),
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
import com.pulumi.aws.ses.ConfigurationSet;
import com.pulumi.aws.ses.ConfigurationSetArgs;
import com.pulumi.aws.ses.inputs.ConfigurationSetTrackingOptionsArgs;
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
        var test = new ConfigurationSet("test", ConfigurationSetArgs.builder()
            .name("some-configuration-set-test")
            .trackingOptions(ConfigurationSetTrackingOptionsArgs.builder()
                .customRedirectDomain("sub.example.com")
                .build())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:ses:ConfigurationSet
    properties:
      name: some-configuration-set-test
      trackingOptions:
        customRedirectDomain: sub.example.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SES Configuration Sets using their `name`. For example:

```sh
$ pulumi import aws:ses/configurationSet:ConfigurationSet test some-configuration-set-test
```
ﬁ
deliveryOptionsuBs:q
o
sesConfigurationSetDeliveryOptionsGaws:ses/ConfigurationSetDeliveryOptions:ConfigurationSetDeliveryOptionsTWhether messages that use the configuration set are required to use TLS. See below.
R
nameB" DName of the configuration set.

The following argument is optional:
Õ
reputationMetricsEnabledB
 ™Whether or not Amazon SES publishes reputation metrics for the configuration set, such as bounce and complaint rates, to Amazon CloudWatch. The default value is `false`.
}
sendingEnabledB
 eWhether email sending is enabled or disabled for the configuration set. The default value is `true`.
ñ
trackingOptionsuBs:q
o
sesConfigurationSetTrackingOptionsGaws:ses/ConfigurationSetTrackingOptions:ConfigurationSetTrackingOptionsãDomain that is used to redirect email recipients to an Amazon SES-operated domain. See below. **NOTE:** This functionality is best effort.
"&
arn" SES configuration set ARN.
"ﬁ
deliveryOptionsuBs:q
o
sesConfigurationSetDeliveryOptionsGaws:ses/ConfigurationSetDeliveryOptions:ConfigurationSetDeliveryOptionsTWhether messages that use the configuration set are required to use TLS. See below.
"£
lastFreshStart" åDate and time at which the reputation metrics for the configuration set were last reset. Resetting these metrics is known as a fresh start.
"P
name" DName of the configuration set.

The following argument is optional:
"Õ
reputationMetricsEnabledB
 ™Whether or not Amazon SES publishes reputation metrics for the configuration set, such as bounce and complaint rates, to Amazon CloudWatch. The default value is `false`.
"}
sendingEnabledB
 eWhether email sending is enabled or disabled for the configuration set. The default value is `true`.
"ñ
trackingOptionsuBs:q
o
sesConfigurationSetTrackingOptionsGaws:ses/ConfigurationSetTrackingOptions:ConfigurationSetTrackingOptionsãDomain that is used to redirect email recipients to an Amazon SES-operated domain. See below. **NOTE:** This functionality is best effort.
*ü4
0
ses
DomainDkimaws:ses/domainDkim:DomainDkim«/Provides an SES domain DKIM generation resource.

Domain ownership needs to be confirmed first using ses_domain_identity Resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ses.DomainIdentity("example", {domain: "example.com"});
const exampleDomainDkim = new aws.ses.DomainDkim("example", {domain: example.domain});
const exampleAmazonsesDkimRecord: aws.route53.Record[] = [];
for (const range = {value: 0}; range.value < 3; range.value++) {
    exampleAmazonsesDkimRecord.push(new aws.route53.Record(`example_amazonses_dkim_record-${range.value}`, {
        zoneId: "ABCDEFGHIJ123",
        name: exampleDomainDkim.dkimTokens.apply(dkimTokens => `${dkimTokens[range.value]}._domainkey`),
        type: aws.route53.RecordType.CNAME,
        ttl: 600,
        records: [exampleDomainDkim.dkimTokens.apply(dkimTokens => `${dkimTokens[range.value]}.dkim.amazonses.com`)],
    }));
}
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ses.DomainIdentity("example", domain="example.com")
example_domain_dkim = aws.ses.DomainDkim("example", domain=example.domain)
example_amazonses_dkim_record = []
for range in [{"value": i} for i in range(0, 3)]:
    example_amazonses_dkim_record.append(aws.route53.Record(f"example_amazonses_dkim_record-{range['value']}",
        zone_id="ABCDEFGHIJ123",
        name=example_domain_dkim.dkim_tokens.apply(lambda dkim_tokens: f"{dkim_tokens[range['value']]}._domainkey"),
        type=aws.route53.RecordType.CNAME,
        ttl=600,
        records=[example_domain_dkim.dkim_tokens.apply(lambda dkim_tokens: f"{dkim_tokens[range['value']]}.dkim.amazonses.com")]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ses.DomainIdentity("example", new()
    {
        Domain = "example.com",
    });

    var exampleDomainDkim = new Aws.Ses.DomainDkim("example", new()
    {
        Domain = example.Domain,
    });

    var exampleAmazonsesDkimRecord = new List<Aws.Route53.Record>();
    for (var rangeIndex = 0; rangeIndex < 3; rangeIndex++)
    {
        var range = new { Value = rangeIndex };
        exampleAmazonsesDkimRecord.Add(new Aws.Route53.Record($"example_amazonses_dkim_record-{range.Value}", new()
        {
            ZoneId = "ABCDEFGHIJ123",
            Name = exampleDomainDkim.DkimTokens.Apply(dkimTokens => $"{dkimTokens[range.Value]}._domainkey"),
            Type = Aws.Route53.RecordType.CNAME,
            Ttl = 600,
            Records = new[]
            {
                exampleDomainDkim.DkimTokens.Apply(dkimTokens => $"{dkimTokens[range.Value]}.dkim.amazonses.com"),
            },
        }));
    }
});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/route53"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ses.NewDomainIdentity(ctx, "example", &ses.DomainIdentityArgs{
			Domain: pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		exampleDomainDkim, err := ses.NewDomainDkim(ctx, "example", &ses.DomainDkimArgs{
			Domain: example.Domain,
		})
		if err != nil {
			return err
		}
		var exampleAmazonsesDkimRecord []*route53.Record
		for index := 0; index < 3; index++ {
			key0 := index
			val0 := index
			__res, err := route53.NewRecord(ctx, fmt.Sprintf("example_amazonses_dkim_record-%v", key0), &route53.RecordArgs{
				ZoneId: pulumi.String("ABCDEFGHIJ123"),
				Name: exampleDomainDkim.DkimTokens.ApplyT(func(dkimTokens []string) (string, error) {
					return fmt.Sprintf("%v._domainkey", dkimTokens[val0]), nil
				}).(pulumi.StringOutput),
				Type: pulumi.String(route53.RecordTypeCNAME),
				Ttl:  pulumi.Int(600),
				Records: pulumi.StringArray{
					exampleDomainDkim.DkimTokens.ApplyT(func(dkimTokens []string) (string, error) {
						return fmt.Sprintf("%v.dkim.amazonses.com", dkimTokens[val0]), nil
					}).(pulumi.StringOutput),
				},
			})
			if err != nil {
				return err
			}
			exampleAmazonsesDkimRecord = append(exampleAmazonsesDkimRecord, __res)
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
import com.pulumi.aws.ses.DomainIdentity;
import com.pulumi.aws.ses.DomainIdentityArgs;
import com.pulumi.aws.ses.DomainDkim;
import com.pulumi.aws.ses.DomainDkimArgs;
import com.pulumi.aws.route53.Record;
import com.pulumi.aws.route53.RecordArgs;
import com.pulumi.codegen.internal.KeyedValue;
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
        var example = new DomainIdentity("example", DomainIdentityArgs.builder()
            .domain("example.com")
            .build());

        var exampleDomainDkim = new DomainDkim("exampleDomainDkim", DomainDkimArgs.builder()
            .domain(example.domain())
            .build());

        for (var i = 0; i < 3; i++) {
            new Record("exampleAmazonsesDkimRecord-" + i, RecordArgs.builder()
                .zoneId("ABCDEFGHIJ123")
                .name(exampleDomainDkim.dkimTokens().applyValue(dkimTokens -> String.format("%s._domainkey", dkimTokens[range.value()])))
                .type("CNAME")
                .ttl("600")
                .records(exampleDomainDkim.dkimTokens().applyValue(dkimTokens -> String.format("%s.dkim.amazonses.com", dkimTokens[range.value()])))
                .build());

        
}
    }
}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import DKIM tokens using the `domain` attribute. For example:

```sh
$ pulumi import aws:ses/domainDkim:DomainDkim example example.com
```
@
domain" 2Verified domain name to generate DKIM tokens for.
"ú

dkimTokens*" áDKIM tokens generated by SES.
These tokens should be used to create CNAME records used to verify SES Easy DKIM.
See below for an example of how this might be achieved
when the domain is hosted in Route 53 and managed by this provider.
Find out more about verifying domains in Amazon SES
in the [AWS SES docs](http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim-dns-records.html).
"@
domain" 2Verified domain name to generate DKIM tokens for.
*ê3
<
sesDomainIdentity%aws:ses/domainIdentity:DomainIdentity•-Provides an SES domain identity resource

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ses.DomainIdentity("example", {domain: "example.com"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ses.DomainIdentity("example", domain="example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ses.DomainIdentity("example", new()
    {
        Domain = "example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewDomainIdentity(ctx, "example", &ses.DomainIdentityArgs{
			Domain: pulumi.String("example.com"),
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
import com.pulumi.aws.ses.DomainIdentity;
import com.pulumi.aws.ses.DomainIdentityArgs;
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
        var example = new DomainIdentity("example", DomainIdentityArgs.builder()
            .domain("example.com")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ses:DomainIdentity
    properties:
      domain: example.com
```
<!--End PulumiCodeChooser -->

### With Route53 Record

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ses.DomainIdentity("example", {domain: "example.com"});
const exampleAmazonsesVerificationRecord = new aws.route53.Record("example_amazonses_verification_record", {
    zoneId: "ABCDEFGHIJ123",
    name: "_amazonses.example.com",
    type: aws.route53.RecordType.TXT,
    ttl: 600,
    records: [example.verificationToken],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ses.DomainIdentity("example", domain="example.com")
example_amazonses_verification_record = aws.route53.Record("example_amazonses_verification_record",
    zone_id="ABCDEFGHIJ123",
    name="_amazonses.example.com",
    type=aws.route53.RecordType.TXT,
    ttl=600,
    records=[example.verification_token])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ses.DomainIdentity("example", new()
    {
        Domain = "example.com",
    });

    var exampleAmazonsesVerificationRecord = new Aws.Route53.Record("example_amazonses_verification_record", new()
    {
        ZoneId = "ABCDEFGHIJ123",
        Name = "_amazonses.example.com",
        Type = Aws.Route53.RecordType.TXT,
        Ttl = 600,
        Records = new[]
        {
            example.VerificationToken,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/route53"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ses.NewDomainIdentity(ctx, "example", &ses.DomainIdentityArgs{
			Domain: pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		_, err = route53.NewRecord(ctx, "example_amazonses_verification_record", &route53.RecordArgs{
			ZoneId: pulumi.String("ABCDEFGHIJ123"),
			Name:   pulumi.String("_amazonses.example.com"),
			Type:   pulumi.String(route53.RecordTypeTXT),
			Ttl:    pulumi.Int(600),
			Records: pulumi.StringArray{
				example.VerificationToken,
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
import com.pulumi.aws.ses.DomainIdentity;
import com.pulumi.aws.ses.DomainIdentityArgs;
import com.pulumi.aws.route53.Record;
import com.pulumi.aws.route53.RecordArgs;
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
        var example = new DomainIdentity("example", DomainIdentityArgs.builder()
            .domain("example.com")
            .build());

        var exampleAmazonsesVerificationRecord = new Record("exampleAmazonsesVerificationRecord", RecordArgs.builder()
            .zoneId("ABCDEFGHIJ123")
            .name("_amazonses.example.com")
            .type("TXT")
            .ttl("600")
            .records(example.verificationToken())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ses:DomainIdentity
    properties:
      domain: example.com
  exampleAmazonsesVerificationRecord:
    type: aws:route53:Record
    name: example_amazonses_verification_record
    properties:
      zoneId: ABCDEFGHIJ123
      name: _amazonses.example.com
      type: TXT
      ttl: '600'
      records:
        - ${example.verificationToken}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SES domain identities using the domain name. For example:

```sh
$ pulumi import aws:ses/domainIdentity:DomainIdentity example example.com
```
/
domain" !The domain name to assign to SES
"+
arn"  The ARN of the domain identity.
"/
domain" !The domain name to assign to SES
"ò
verificationToken" ˛A code which when added to the domain as a TXT record will signal to SES that the owner of the domain has authorized SES to act on their behalf. The domain identity will be in state "verification pending" until this is done. See the With Route53 Record example for how this might be achieved when the domain is hosted in Route 53 and managed by this provider.  Find out more about verifying domains in Amazon SES in the [AWS SES docs](http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-domains.html).
*≥2
`
sesDomainIdentityVerification=aws:ses/domainIdentityVerification:DomainIdentityVerificationï0Represents a successful verification of an SES domain identity.

Most commonly, this resource is used together with `aws.route53.Record` and
`aws.ses.DomainIdentity` to request an SES domain identity,
deploy the required DNS verification records, and wait for verification to complete.

> **WARNING:** This resource implements a part of the verification workflow. It does not represent a real-world entity in AWS, therefore changing or deleting this resource on its own has no immediate effect.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ses.DomainIdentity("example", {domain: "example.com"});
const exampleAmazonsesVerificationRecord = new aws.route53.Record("example_amazonses_verification_record", {
    zoneId: exampleAwsRoute53Zone.zoneId,
    name: pulumi.interpolate`_amazonses.${example.id}`,
    type: aws.route53.RecordType.TXT,
    ttl: 600,
    records: [example.verificationToken],
});
const exampleVerification = new aws.ses.DomainIdentityVerification("example_verification", {domain: example.id}, {
    dependsOn: [exampleAmazonsesVerificationRecord],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ses.DomainIdentity("example", domain="example.com")
example_amazonses_verification_record = aws.route53.Record("example_amazonses_verification_record",
    zone_id=example_aws_route53_zone["zoneId"],
    name=example.id.apply(lambda id: f"_amazonses.{id}"),
    type=aws.route53.RecordType.TXT,
    ttl=600,
    records=[example.verification_token])
example_verification = aws.ses.DomainIdentityVerification("example_verification", domain=example.id,
opts = pulumi.ResourceOptions(depends_on=[example_amazonses_verification_record]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ses.DomainIdentity("example", new()
    {
        Domain = "example.com",
    });

    var exampleAmazonsesVerificationRecord = new Aws.Route53.Record("example_amazonses_verification_record", new()
    {
        ZoneId = exampleAwsRoute53Zone.ZoneId,
        Name = example.Id.Apply(id => $"_amazonses.{id}"),
        Type = Aws.Route53.RecordType.TXT,
        Ttl = 600,
        Records = new[]
        {
            example.VerificationToken,
        },
    });

    var exampleVerification = new Aws.Ses.DomainIdentityVerification("example_verification", new()
    {
        Domain = example.Id,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAmazonsesVerificationRecord,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/route53"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ses.NewDomainIdentity(ctx, "example", &ses.DomainIdentityArgs{
			Domain: pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		exampleAmazonsesVerificationRecord, err := route53.NewRecord(ctx, "example_amazonses_verification_record", &route53.RecordArgs{
			ZoneId: pulumi.Any(exampleAwsRoute53Zone.ZoneId),
			Name: example.ID().ApplyT(func(id string) (string, error) {
				return fmt.Sprintf("_amazonses.%v", id), nil
			}).(pulumi.StringOutput),
			Type: pulumi.String(route53.RecordTypeTXT),
			Ttl:  pulumi.Int(600),
			Records: pulumi.StringArray{
				example.VerificationToken,
			},
		})
		if err != nil {
			return err
		}
		_, err = ses.NewDomainIdentityVerification(ctx, "example_verification", &ses.DomainIdentityVerificationArgs{
			Domain: example.ID(),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAmazonsesVerificationRecord,
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
import com.pulumi.aws.ses.DomainIdentity;
import com.pulumi.aws.ses.DomainIdentityArgs;
import com.pulumi.aws.route53.Record;
import com.pulumi.aws.route53.RecordArgs;
import com.pulumi.aws.ses.DomainIdentityVerification;
import com.pulumi.aws.ses.DomainIdentityVerificationArgs;
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
        var example = new DomainIdentity("example", DomainIdentityArgs.builder()
            .domain("example.com")
            .build());

        var exampleAmazonsesVerificationRecord = new Record("exampleAmazonsesVerificationRecord", RecordArgs.builder()
            .zoneId(exampleAwsRoute53Zone.zoneId())
            .name(example.id().applyValue(id -> String.format("_amazonses.%s", id)))
            .type("TXT")
            .ttl("600")
            .records(example.verificationToken())
            .build());

        var exampleVerification = new DomainIdentityVerification("exampleVerification", DomainIdentityVerificationArgs.builder()
            .domain(example.id())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAmazonsesVerificationRecord)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ses:DomainIdentity
    properties:
      domain: example.com
  exampleAmazonsesVerificationRecord:
    type: aws:route53:Record
    name: example_amazonses_verification_record
    properties:
      zoneId: ${exampleAwsRoute53Zone.zoneId}
      name: _amazonses.${example.id}
      type: TXT
      ttl: '600'
      records:
        - ${example.verificationToken}
  exampleVerification:
    type: aws:ses:DomainIdentityVerification
    name: example_verification
    properties:
      domain: ${example.id}
    options:
      dependsOn:
        - ${exampleAmazonsesVerificationRecord}
```
<!--End PulumiCodeChooser -->
D
domain" 6The domain name of the SES domain identity to verify.
"+
arn"  The ARN of the domain identity.
"D
domain" 6The domain name of the SES domain identity to verify.
*∑
9
sesEmailIdentity#aws:ses/emailIdentity:EmailIdentityÁProvides an SES email identity resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ses.EmailIdentity("example", {email: "email@example.com"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ses.EmailIdentity("example", email="email@example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ses.EmailIdentity("example", new()
    {
        Email = "email@example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewEmailIdentity(ctx, "example", &ses.EmailIdentityArgs{
			Email: pulumi.String("email@example.com"),
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
import com.pulumi.aws.ses.EmailIdentity;
import com.pulumi.aws.ses.EmailIdentityArgs;
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
            .email("email@example.com")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ses:EmailIdentity
    properties:
      email: email@example.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SES email identities using the email address. For example:

```sh
$ pulumi import aws:ses/emailIdentity:EmailIdentity example email@example.com
```
1
email" $The email address to assign to SES.
"*
arn" The ARN of the email identity.
"1
email" $The email address to assign to SES.
*Òn
B
sesEventDestination)aws:ses/eventDestination:EventDestination–]Provides an SES event destination

## Example Usage

### CloudWatch Destination

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const cloudwatch = new aws.ses.EventDestination("cloudwatch", {
    name: "event-destination-cloudwatch",
    configurationSetName: example.name,
    enabled: true,
    matchingTypes: [
        "bounce",
        "send",
    ],
    cloudwatchDestinations: [{
        defaultValue: "default",
        dimensionName: "dimension",
        valueSource: "emailHeader",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

cloudwatch = aws.ses.EventDestination("cloudwatch",
    name="event-destination-cloudwatch",
    configuration_set_name=example["name"],
    enabled=True,
    matching_types=[
        "bounce",
        "send",
    ],
    cloudwatch_destinations=[{
        "default_value": "default",
        "dimension_name": "dimension",
        "value_source": "emailHeader",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var cloudwatch = new Aws.Ses.EventDestination("cloudwatch", new()
    {
        Name = "event-destination-cloudwatch",
        ConfigurationSetName = example.Name,
        Enabled = true,
        MatchingTypes = new[]
        {
            "bounce",
            "send",
        },
        CloudwatchDestinations = new[]
        {
            new Aws.Ses.Inputs.EventDestinationCloudwatchDestinationArgs
            {
                DefaultValue = "default",
                DimensionName = "dimension",
                ValueSource = "emailHeader",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewEventDestination(ctx, "cloudwatch", &ses.EventDestinationArgs{
			Name:                 pulumi.String("event-destination-cloudwatch"),
			ConfigurationSetName: pulumi.Any(example.Name),
			Enabled:              pulumi.Bool(true),
			MatchingTypes: pulumi.StringArray{
				pulumi.String("bounce"),
				pulumi.String("send"),
			},
			CloudwatchDestinations: ses.EventDestinationCloudwatchDestinationArray{
				&ses.EventDestinationCloudwatchDestinationArgs{
					DefaultValue:  pulumi.String("default"),
					DimensionName: pulumi.String("dimension"),
					ValueSource:   pulumi.String("emailHeader"),
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
import com.pulumi.aws.ses.EventDestination;
import com.pulumi.aws.ses.EventDestinationArgs;
import com.pulumi.aws.ses.inputs.EventDestinationCloudwatchDestinationArgs;
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
        var cloudwatch = new EventDestination("cloudwatch", EventDestinationArgs.builder()
            .name("event-destination-cloudwatch")
            .configurationSetName(example.name())
            .enabled(true)
            .matchingTypes(            
                "bounce",
                "send")
            .cloudwatchDestinations(EventDestinationCloudwatchDestinationArgs.builder()
                .defaultValue("default")
                .dimensionName("dimension")
                .valueSource("emailHeader")
                .build())
            .build());

    }
}
```
```yaml
resources:
  cloudwatch:
    type: aws:ses:EventDestination
    properties:
      name: event-destination-cloudwatch
      configurationSetName: ${example.name}
      enabled: true
      matchingTypes:
        - bounce
        - send
      cloudwatchDestinations:
        - defaultValue: default
          dimensionName: dimension
          valueSource: emailHeader
```
<!--End PulumiCodeChooser -->

### Kinesis Destination

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const kinesis = new aws.ses.EventDestination("kinesis", {
    name: "event-destination-kinesis",
    configurationSetName: exampleAwsSesConfigurationSet.name,
    enabled: true,
    matchingTypes: [
        "bounce",
        "send",
    ],
    kinesisDestination: {
        streamArn: exampleAwsKinesisFirehoseDeliveryStream.arn,
        roleArn: example.arn,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

kinesis = aws.ses.EventDestination("kinesis",
    name="event-destination-kinesis",
    configuration_set_name=example_aws_ses_configuration_set["name"],
    enabled=True,
    matching_types=[
        "bounce",
        "send",
    ],
    kinesis_destination={
        "stream_arn": example_aws_kinesis_firehose_delivery_stream["arn"],
        "role_arn": example["arn"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var kinesis = new Aws.Ses.EventDestination("kinesis", new()
    {
        Name = "event-destination-kinesis",
        ConfigurationSetName = exampleAwsSesConfigurationSet.Name,
        Enabled = true,
        MatchingTypes = new[]
        {
            "bounce",
            "send",
        },
        KinesisDestination = new Aws.Ses.Inputs.EventDestinationKinesisDestinationArgs
        {
            StreamArn = exampleAwsKinesisFirehoseDeliveryStream.Arn,
            RoleArn = example.Arn,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewEventDestination(ctx, "kinesis", &ses.EventDestinationArgs{
			Name:                 pulumi.String("event-destination-kinesis"),
			ConfigurationSetName: pulumi.Any(exampleAwsSesConfigurationSet.Name),
			Enabled:              pulumi.Bool(true),
			MatchingTypes: pulumi.StringArray{
				pulumi.String("bounce"),
				pulumi.String("send"),
			},
			KinesisDestination: &ses.EventDestinationKinesisDestinationArgs{
				StreamArn: pulumi.Any(exampleAwsKinesisFirehoseDeliveryStream.Arn),
				RoleArn:   pulumi.Any(example.Arn),
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
import com.pulumi.aws.ses.EventDestination;
import com.pulumi.aws.ses.EventDestinationArgs;
import com.pulumi.aws.ses.inputs.EventDestinationKinesisDestinationArgs;
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
        var kinesis = new EventDestination("kinesis", EventDestinationArgs.builder()
            .name("event-destination-kinesis")
            .configurationSetName(exampleAwsSesConfigurationSet.name())
            .enabled(true)
            .matchingTypes(            
                "bounce",
                "send")
            .kinesisDestination(EventDestinationKinesisDestinationArgs.builder()
                .streamArn(exampleAwsKinesisFirehoseDeliveryStream.arn())
                .roleArn(example.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  kinesis:
    type: aws:ses:EventDestination
    properties:
      name: event-destination-kinesis
      configurationSetName: ${exampleAwsSesConfigurationSet.name}
      enabled: true
      matchingTypes:
        - bounce
        - send
      kinesisDestination:
        streamArn: ${exampleAwsKinesisFirehoseDeliveryStream.arn}
        roleArn: ${example.arn}
```
<!--End PulumiCodeChooser -->

### SNS Destination

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const sns = new aws.ses.EventDestination("sns", {
    name: "event-destination-sns",
    configurationSetName: exampleAwsSesConfigurationSet.name,
    enabled: true,
    matchingTypes: [
        "bounce",
        "send",
    ],
    snsDestination: {
        topicArn: example.arn,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

sns = aws.ses.EventDestination("sns",
    name="event-destination-sns",
    configuration_set_name=example_aws_ses_configuration_set["name"],
    enabled=True,
    matching_types=[
        "bounce",
        "send",
    ],
    sns_destination={
        "topic_arn": example["arn"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var sns = new Aws.Ses.EventDestination("sns", new()
    {
        Name = "event-destination-sns",
        ConfigurationSetName = exampleAwsSesConfigurationSet.Name,
        Enabled = true,
        MatchingTypes = new[]
        {
            "bounce",
            "send",
        },
        SnsDestination = new Aws.Ses.Inputs.EventDestinationSnsDestinationArgs
        {
            TopicArn = example.Arn,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewEventDestination(ctx, "sns", &ses.EventDestinationArgs{
			Name:                 pulumi.String("event-destination-sns"),
			ConfigurationSetName: pulumi.Any(exampleAwsSesConfigurationSet.Name),
			Enabled:              pulumi.Bool(true),
			MatchingTypes: pulumi.StringArray{
				pulumi.String("bounce"),
				pulumi.String("send"),
			},
			SnsDestination: &ses.EventDestinationSnsDestinationArgs{
				TopicArn: pulumi.Any(example.Arn),
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
import com.pulumi.aws.ses.EventDestination;
import com.pulumi.aws.ses.EventDestinationArgs;
import com.pulumi.aws.ses.inputs.EventDestinationSnsDestinationArgs;
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
        var sns = new EventDestination("sns", EventDestinationArgs.builder()
            .name("event-destination-sns")
            .configurationSetName(exampleAwsSesConfigurationSet.name())
            .enabled(true)
            .matchingTypes(            
                "bounce",
                "send")
            .snsDestination(EventDestinationSnsDestinationArgs.builder()
                .topicArn(example.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  sns:
    type: aws:ses:EventDestination
    properties:
      name: event-destination-sns
      configurationSetName: ${exampleAwsSesConfigurationSet.name}
      enabled: true
      matchingTypes:
        - bounce
        - send
      snsDestination:
        topicArn: ${example.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SES event destinations using `configuration_set_name` together with the event destination's `name`. For example:

```sh
$ pulumi import aws:ses/eventDestination:EventDestination sns some-configuration-set-test/event-destination-sns
```
–
cloudwatchDestinationsçBä*á:Ñ
Å
ses%EventDestinationCloudwatchDestinationSaws:ses/EventDestinationCloudwatchDestination:EventDestinationCloudwatchDestination&CloudWatch destination for the events
>
configurationSetName" "The name of the configuration set
@
enabledB
 /If true, the event destination will be enabled
»
kinesisDestination~B|:z
x
ses"EventDestinationKinesisDestinationMaws:ses/EventDestinationKinesisDestination:EventDestinationKinesisDestination2Send the events to a kinesis firehose destination
≠
matchingTypes*" ïA list of matching types. May be any of `"send"`, `"reject"`, `"bounce"`, `"complaint"`, `"delivery"`, `"open"`, `"click"`, or `"renderingFailure"`.
0
nameB" "The name of the event destination
ì
snsDestinationrBp:n
l
sesEventDestinationSnsDestinationEaws:ses/EventDestinationSnsDestination:EventDestinationSnsDestinationåSend the events to an SNS Topic destination

> **NOTE:** You can specify `"cloudwatch_destination"` or `"kinesis_destination"` but not both
"*
arn" The SES event destination ARN.
"–
cloudwatchDestinationsçBä*á:Ñ
Å
ses%EventDestinationCloudwatchDestinationSaws:ses/EventDestinationCloudwatchDestination:EventDestinationCloudwatchDestination&CloudWatch destination for the events
">
configurationSetName" "The name of the configuration set
"@
enabledB
 /If true, the event destination will be enabled
"»
kinesisDestination~B|:z
x
ses"EventDestinationKinesisDestinationMaws:ses/EventDestinationKinesisDestination:EventDestinationKinesisDestination2Send the events to a kinesis firehose destination
"≠
matchingTypes*" ïA list of matching types. May be any of `"send"`, `"reject"`, `"bounce"`, `"complaint"`, `"delivery"`, `"open"`, `"click"`, or `"renderingFailure"`.
".
name" "The name of the event destination
"ì
snsDestinationrBp:n
l
sesEventDestinationSnsDestinationEaws:ses/EventDestinationSnsDestination:EventDestinationSnsDestinationåSend the events to an SNS Topic destination

> **NOTE:** You can specify `"cloudwatch_destination"` or `"kinesis_destination"` but not both
*Û!
]
sesIdentityNotificationTopic;aws:ses/identityNotificationTopic:IdentityNotificationTopic”Resource for managing SES Identity Notification Topics

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.ses.IdentityNotificationTopic("test", {
    topicArn: exampleAwsSnsTopic.arn,
    notificationType: "Bounce",
    identity: example.domain,
    includeOriginalHeaders: true,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.ses.IdentityNotificationTopic("test",
    topic_arn=example_aws_sns_topic["arn"],
    notification_type="Bounce",
    identity=example["domain"],
    include_original_headers=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Ses.IdentityNotificationTopic("test", new()
    {
        TopicArn = exampleAwsSnsTopic.Arn,
        NotificationType = "Bounce",
        Identity = example.Domain,
        IncludeOriginalHeaders = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewIdentityNotificationTopic(ctx, "test", &ses.IdentityNotificationTopicArgs{
			TopicArn:               pulumi.Any(exampleAwsSnsTopic.Arn),
			NotificationType:       pulumi.String("Bounce"),
			Identity:               pulumi.Any(example.Domain),
			IncludeOriginalHeaders: pulumi.Bool(true),
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
import com.pulumi.aws.ses.IdentityNotificationTopic;
import com.pulumi.aws.ses.IdentityNotificationTopicArgs;
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
        var test = new IdentityNotificationTopic("test", IdentityNotificationTopicArgs.builder()
            .topicArn(exampleAwsSnsTopic.arn())
            .notificationType("Bounce")
            .identity(example.domain())
            .includeOriginalHeaders(true)
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:ses:IdentityNotificationTopic
    properties:
      topicArn: ${exampleAwsSnsTopic.arn}
      notificationType: Bounce
      identity: ${example.domain}
      includeOriginalHeaders: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Identity Notification Topics using the ID of the record. The ID is made up as `IDENTITY|TYPE` where `IDENTITY` is the SES Identity and `TYPE` is the Notification Type. For example:

```sh
$ pulumi import aws:ses/identityNotificationTopic:IdentityNotificationTopic test 'example.com|Bounce'
```
§
identity" ìThe identity for which the Amazon SNS topic will be set. You can specify an identity by using its name or by using its Amazon Resource Name (ARN).
â
includeOriginalHeadersB
 iWhether SES should include original email headers in SNS notifications of this type. `false` by default.
†
notificationType" áThe type of notifications that will be published to the specified Amazon SNS topic. Valid Values: `Bounce`, `Complaint` or `Delivery`.
Ü
topicArnB" tThe Amazon Resource Name (ARN) of the Amazon SNS topic. Can be set to `""` (an empty string) to disable publishing.
"§
identity" ìThe identity for which the Amazon SNS topic will be set. You can specify an identity by using its name or by using its Amazon Resource Name (ARN).
"â
includeOriginalHeadersB
 iWhether SES should include original email headers in SNS notifications of this type. `false` by default.
"†
notificationType" áThe type of notifications that will be published to the specified Amazon SNS topic. Valid Values: `Bounce`, `Complaint` or `Delivery`.
"Ü
topicArnB" tThe Amazon Resource Name (ARN) of the Amazon SNS topic. Can be set to `""` (an empty string) to disable publishing.
*”:
<
sesIdentityPolicy%aws:ses/identityPolicy:IdentityPolicy‚7Manages a SES Identity Policy. More information about SES Sending Authorization Policies can be found in the [SES Developer Guide](https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleDomainIdentity = new aws.ses.DomainIdentity("example", {domain: "example.com"});
const example = aws.iam.getPolicyDocumentOutput({
    statements: [{
        actions: [
            "SES:SendEmail",
            "SES:SendRawEmail",
        ],
        resources: [exampleDomainIdentity.arn],
        principals: [{
            identifiers: ["*"],
            type: "AWS",
        }],
    }],
});
const exampleIdentityPolicy = new aws.ses.IdentityPolicy("example", {
    identity: exampleDomainIdentity.arn,
    name: "example",
    policy: example.apply(example => example.json),
});
```
```python
import pulumi
import pulumi_aws as aws

example_domain_identity = aws.ses.DomainIdentity("example", domain="example.com")
example = aws.iam.get_policy_document_output(statements=[{
    "actions": [
        "SES:SendEmail",
        "SES:SendRawEmail",
    ],
    "resources": [example_domain_identity.arn],
    "principals": [{
        "identifiers": ["*"],
        "type": "AWS",
    }],
}])
example_identity_policy = aws.ses.IdentityPolicy("example",
    identity=example_domain_identity.arn,
    name="example",
    policy=example.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleDomainIdentity = new Aws.Ses.DomainIdentity("example", new()
    {
        Domain = "example.com",
    });

    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "SES:SendEmail",
                    "SES:SendRawEmail",
                },
                Resources = new[]
                {
                    exampleDomainIdentity.Arn,
                },
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Identifiers = new[]
                        {
                            "*",
                        },
                        Type = "AWS",
                    },
                },
            },
        },
    });

    var exampleIdentityPolicy = new Aws.Ses.IdentityPolicy("example", new()
    {
        Identity = exampleDomainIdentity.Arn,
        Name = "example",
        Policy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleDomainIdentity, err := ses.NewDomainIdentity(ctx, "example", &ses.DomainIdentityArgs{
			Domain: pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		example := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Actions: pulumi.StringArray{
						pulumi.String("SES:SendEmail"),
						pulumi.String("SES:SendRawEmail"),
					},
					Resources: pulumi.StringArray{
						exampleDomainIdentity.Arn,
					},
					Principals: iam.GetPolicyDocumentStatementPrincipalArray{
						&iam.GetPolicyDocumentStatementPrincipalArgs{
							Identifiers: pulumi.StringArray{
								pulumi.String("*"),
							},
							Type: pulumi.String("AWS"),
						},
					},
				},
			},
		}, nil)
		_, err = ses.NewIdentityPolicy(ctx, "example", &ses.IdentityPolicyArgs{
			Identity: exampleDomainIdentity.Arn,
			Name:     pulumi.String("example"),
			Policy: pulumi.String(example.ApplyT(func(example iam.GetPolicyDocumentResult) (*string, error) {
				return &example.Json, nil
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
import com.pulumi.aws.ses.DomainIdentity;
import com.pulumi.aws.ses.DomainIdentityArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.ses.IdentityPolicy;
import com.pulumi.aws.ses.IdentityPolicyArgs;
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
        var exampleDomainIdentity = new DomainIdentity("exampleDomainIdentity", DomainIdentityArgs.builder()
            .domain("example.com")
            .build());

        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions(                
                    "SES:SendEmail",
                    "SES:SendRawEmail")
                .resources(exampleDomainIdentity.arn())
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .identifiers("*")
                    .type("AWS")
                    .build())
                .build())
            .build());

        var exampleIdentityPolicy = new IdentityPolicy("exampleIdentityPolicy", IdentityPolicyArgs.builder()
            .identity(exampleDomainIdentity.arn())
            .name("example")
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(example -> example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  exampleDomainIdentity:
    type: aws:ses:DomainIdentity
    name: example
    properties:
      domain: example.com
  exampleIdentityPolicy:
    type: aws:ses:IdentityPolicy
    name: example
    properties:
      identity: ${exampleDomainIdentity.arn}
      name: example
      policy: ${example.json}
variables:
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - SES:SendEmail
              - SES:SendRawEmail
            resources:
              - ${exampleDomainIdentity.arn}
            principals:
              - identifiers:
                  - '*'
                type: AWS
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SES Identity Policies using the identity and policy name, separated by a pipe character (`|`). For example:

```sh
$ pulumi import aws:ses/identityPolicy:IdentityPolicy example 'example.com|example'
```
H
identity" 8Name or Amazon Resource Name (ARN) of the SES Identity.
"
nameB" Name of the policy.
)
policy" JSON string of the policy.
"H
identity" 8Name or Amazon Resource Name (ARN) of the SES Identity.
" 
name" Name of the policy.
")
policy" JSON string of the policy.
*äd
*
sesMailFromaws:ses/mailFrom:MailFrom◊ZProvides an SES domain MAIL FROM resource.

> **NOTE:** For the MAIL FROM domain to be fully usable, this resource should be paired with the aws.ses.DomainIdentity resource. To validate the MAIL FROM domain, a DNS MX record is required. To pass SPF checks, a DNS TXT record may also be required. See the [Amazon SES MAIL FROM documentation](https://docs.aws.amazon.com/ses/latest/dg/mail-from.html) for more information.

## Example Usage

### Domain Identity MAIL FROM

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// Example SES Domain Identity
const exampleDomainIdentity = new aws.ses.DomainIdentity("example", {domain: "example.com"});
const example = new aws.ses.MailFrom("example", {
    domain: exampleDomainIdentity.domain,
    mailFromDomain: pulumi.interpolate`bounce.${exampleDomainIdentity.domain}`,
});
// Example Route53 MX record
const exampleSesDomainMailFromMx = new aws.route53.Record("example_ses_domain_mail_from_mx", {
    zoneId: exampleAwsRoute53Zone.id,
    name: example.mailFromDomain,
    type: aws.route53.RecordType.MX,
    ttl: 600,
    records: ["10 feedback-smtp.us-east-1.amazonses.com"],
});
// Example Route53 TXT record for SPF
const exampleSesDomainMailFromTxt = new aws.route53.Record("example_ses_domain_mail_from_txt", {
    zoneId: exampleAwsRoute53Zone.id,
    name: example.mailFromDomain,
    type: aws.route53.RecordType.TXT,
    ttl: 600,
    records: ["v=spf1 include:amazonses.com ~all"],
});
```
```python
import pulumi
import pulumi_aws as aws

# Example SES Domain Identity
example_domain_identity = aws.ses.DomainIdentity("example", domain="example.com")
example = aws.ses.MailFrom("example",
    domain=example_domain_identity.domain,
    mail_from_domain=example_domain_identity.domain.apply(lambda domain: f"bounce.{domain}"))
# Example Route53 MX record
example_ses_domain_mail_from_mx = aws.route53.Record("example_ses_domain_mail_from_mx",
    zone_id=example_aws_route53_zone["id"],
    name=example.mail_from_domain,
    type=aws.route53.RecordType.MX,
    ttl=600,
    records=["10 feedback-smtp.us-east-1.amazonses.com"])
# Example Route53 TXT record for SPF
example_ses_domain_mail_from_txt = aws.route53.Record("example_ses_domain_mail_from_txt",
    zone_id=example_aws_route53_zone["id"],
    name=example.mail_from_domain,
    type=aws.route53.RecordType.TXT,
    ttl=600,
    records=["v=spf1 include:amazonses.com ~all"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // Example SES Domain Identity
    var exampleDomainIdentity = new Aws.Ses.DomainIdentity("example", new()
    {
        Domain = "example.com",
    });

    var example = new Aws.Ses.MailFrom("example", new()
    {
        Domain = exampleDomainIdentity.Domain,
        MailFromDomain = exampleDomainIdentity.Domain.Apply(domain => $"bounce.{domain}"),
    });

    // Example Route53 MX record
    var exampleSesDomainMailFromMx = new Aws.Route53.Record("example_ses_domain_mail_from_mx", new()
    {
        ZoneId = exampleAwsRoute53Zone.Id,
        Name = example.MailFromDomain,
        Type = Aws.Route53.RecordType.MX,
        Ttl = 600,
        Records = new[]
        {
            "10 feedback-smtp.us-east-1.amazonses.com",
        },
    });

    // Example Route53 TXT record for SPF
    var exampleSesDomainMailFromTxt = new Aws.Route53.Record("example_ses_domain_mail_from_txt", new()
    {
        ZoneId = exampleAwsRoute53Zone.Id,
        Name = example.MailFromDomain,
        Type = Aws.Route53.RecordType.TXT,
        Ttl = 600,
        Records = new[]
        {
            "v=spf1 include:amazonses.com ~all",
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/route53"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// Example SES Domain Identity
		exampleDomainIdentity, err := ses.NewDomainIdentity(ctx, "example", &ses.DomainIdentityArgs{
			Domain: pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		example, err := ses.NewMailFrom(ctx, "example", &ses.MailFromArgs{
			Domain: exampleDomainIdentity.Domain,
			MailFromDomain: exampleDomainIdentity.Domain.ApplyT(func(domain string) (string, error) {
				return fmt.Sprintf("bounce.%v", domain), nil
			}).(pulumi.StringOutput),
		})
		if err != nil {
			return err
		}
		// Example Route53 MX record
		_, err = route53.NewRecord(ctx, "example_ses_domain_mail_from_mx", &route53.RecordArgs{
			ZoneId: pulumi.Any(exampleAwsRoute53Zone.Id),
			Name:   example.MailFromDomain,
			Type:   pulumi.String(route53.RecordTypeMX),
			Ttl:    pulumi.Int(600),
			Records: pulumi.StringArray{
				pulumi.String("10 feedback-smtp.us-east-1.amazonses.com"),
			},
		})
		if err != nil {
			return err
		}
		// Example Route53 TXT record for SPF
		_, err = route53.NewRecord(ctx, "example_ses_domain_mail_from_txt", &route53.RecordArgs{
			ZoneId: pulumi.Any(exampleAwsRoute53Zone.Id),
			Name:   example.MailFromDomain,
			Type:   pulumi.String(route53.RecordTypeTXT),
			Ttl:    pulumi.Int(600),
			Records: pulumi.StringArray{
				pulumi.String("v=spf1 include:amazonses.com ~all"),
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
import com.pulumi.aws.ses.DomainIdentity;
import com.pulumi.aws.ses.DomainIdentityArgs;
import com.pulumi.aws.ses.MailFrom;
import com.pulumi.aws.ses.MailFromArgs;
import com.pulumi.aws.route53.Record;
import com.pulumi.aws.route53.RecordArgs;
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
        // Example SES Domain Identity
        var exampleDomainIdentity = new DomainIdentity("exampleDomainIdentity", DomainIdentityArgs.builder()
            .domain("example.com")
            .build());

        var example = new MailFrom("example", MailFromArgs.builder()
            .domain(exampleDomainIdentity.domain())
            .mailFromDomain(exampleDomainIdentity.domain().applyValue(domain -> String.format("bounce.%s", domain)))
            .build());

        // Example Route53 MX record
        var exampleSesDomainMailFromMx = new Record("exampleSesDomainMailFromMx", RecordArgs.builder()
            .zoneId(exampleAwsRoute53Zone.id())
            .name(example.mailFromDomain())
            .type("MX")
            .ttl("600")
            .records("10 feedback-smtp.us-east-1.amazonses.com")
            .build());

        // Example Route53 TXT record for SPF
        var exampleSesDomainMailFromTxt = new Record("exampleSesDomainMailFromTxt", RecordArgs.builder()
            .zoneId(exampleAwsRoute53Zone.id())
            .name(example.mailFromDomain())
            .type("TXT")
            .ttl("600")
            .records("v=spf1 include:amazonses.com ~all")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ses:MailFrom
    properties:
      domain: ${exampleDomainIdentity.domain}
      mailFromDomain: bounce.${exampleDomainIdentity.domain}
  # Example SES Domain Identity
  exampleDomainIdentity:
    type: aws:ses:DomainIdentity
    name: example
    properties:
      domain: example.com
  # Example Route53 MX record
  exampleSesDomainMailFromMx:
    type: aws:route53:Record
    name: example_ses_domain_mail_from_mx
    properties:
      zoneId: ${exampleAwsRoute53Zone.id}
      name: ${example.mailFromDomain}
      type: MX
      ttl: '600'
      records: # Change to the region in which `aws_ses_domain_identity.example` is created
        - 10 feedback-smtp.us-east-1.amazonses.com
  # Example Route53 TXT record for SPF
  exampleSesDomainMailFromTxt:
    type: aws:route53:Record
    name: example_ses_domain_mail_from_txt
    properties:
      zoneId: ${exampleAwsRoute53Zone.id}
      name: ${example.mailFromDomain}
      type: TXT
      ttl: '600'
      records:
        - v=spf1 include:amazonses.com ~all
```
<!--End PulumiCodeChooser -->

### Email Identity MAIL FROM

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// Example SES Email Identity
const example = new aws.ses.EmailIdentity("example", {email: "user@example.com"});
const exampleMailFrom = new aws.ses.MailFrom("example", {
    domain: example.email,
    mailFromDomain: "mail.example.com",
});
```
```python
import pulumi
import pulumi_aws as aws

# Example SES Email Identity
example = aws.ses.EmailIdentity("example", email="user@example.com")
example_mail_from = aws.ses.MailFrom("example",
    domain=example.email,
    mail_from_domain="mail.example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // Example SES Email Identity
    var example = new Aws.Ses.EmailIdentity("example", new()
    {
        Email = "user@example.com",
    });

    var exampleMailFrom = new Aws.Ses.MailFrom("example", new()
    {
        Domain = example.Email,
        MailFromDomain = "mail.example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// Example SES Email Identity
		example, err := ses.NewEmailIdentity(ctx, "example", &ses.EmailIdentityArgs{
			Email: pulumi.String("user@example.com"),
		})
		if err != nil {
			return err
		}
		_, err = ses.NewMailFrom(ctx, "example", &ses.MailFromArgs{
			Domain:         example.Email,
			MailFromDomain: pulumi.String("mail.example.com"),
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
import com.pulumi.aws.ses.EmailIdentity;
import com.pulumi.aws.ses.EmailIdentityArgs;
import com.pulumi.aws.ses.MailFrom;
import com.pulumi.aws.ses.MailFromArgs;
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
        // Example SES Email Identity
        var example = new EmailIdentity("example", EmailIdentityArgs.builder()
            .email("user@example.com")
            .build());

        var exampleMailFrom = new MailFrom("exampleMailFrom", MailFromArgs.builder()
            .domain(example.email())
            .mailFromDomain("mail.example.com")
            .build());

    }
}
```
```yaml
resources:
  # Example SES Email Identity
  example:
    type: aws:ses:EmailIdentity
    properties:
      email: user@example.com
  exampleMailFrom:
    type: aws:ses:MailFrom
    name: example
    properties:
      domain: ${example.email}
      mailFromDomain: mail.example.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import MAIL FROM domain using the `domain` attribute. For example:

```sh
$ pulumi import aws:ses/mailFrom:MailFrom example example.com
```
≈
behaviorOnMxFailureB" ßThe action that you want Amazon SES to take if it cannot successfully read the required MX record when you send an email. Defaults to `UseDefaultValue`. See the [SES API documentation](https://docs.aws.amazon.com/ses/latest/APIReference/API_SetIdentityMailFromDomain.html) for more information.
R
domain" DVerified domain name or email identity to generate DKIM tokens for.
£
mailFromDomain" åSubdomain (of above domain) which is to be used as MAIL FROM address (Required for DMARC validation)

The following arguments are optional:
"≈
behaviorOnMxFailureB" ßThe action that you want Amazon SES to take if it cannot successfully read the required MX record when you send an email. Defaults to `UseDefaultValue`. See the [SES API documentation](https://docs.aws.amazon.com/ses/latest/APIReference/API_SetIdentityMailFromDomain.html) for more information.
"R
domain" DVerified domain name or email identity to generate DKIM tokens for.
"£
mailFromDomain" åSubdomain (of above domain) which is to be used as MAIL FROM address (Required for DMARC validation)

The following arguments are optional:
*â
9
sesReceiptFilter#aws:ses/receiptFilter:ReceiptFilterÑProvides an SES receipt filter resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const filter = new aws.ses.ReceiptFilter("filter", {
    name: "block-spammer",
    cidr: "10.10.10.10",
    policy: "Block",
});
```
```python
import pulumi
import pulumi_aws as aws

filter = aws.ses.ReceiptFilter("filter",
    name="block-spammer",
    cidr="10.10.10.10",
    policy="Block")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var filter = new Aws.Ses.ReceiptFilter("filter", new()
    {
        Name = "block-spammer",
        Cidr = "10.10.10.10",
        Policy = "Block",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewReceiptFilter(ctx, "filter", &ses.ReceiptFilterArgs{
			Name:   pulumi.String("block-spammer"),
			Cidr:   pulumi.String("10.10.10.10"),
			Policy: pulumi.String("Block"),
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
import com.pulumi.aws.ses.ReceiptFilter;
import com.pulumi.aws.ses.ReceiptFilterArgs;
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
        var filter = new ReceiptFilter("filter", ReceiptFilterArgs.builder()
            .name("block-spammer")
            .cidr("10.10.10.10")
            .policy("Block")
            .build());

    }
}
```
```yaml
resources:
  filter:
    type: aws:ses:ReceiptFilter
    properties:
      name: block-spammer
      cidr: 10.10.10.10
      policy: Block
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SES Receipt Filter using their `name`. For example:

```sh
$ pulumi import aws:ses/receiptFilter:ReceiptFilter test some-filter
```
H
cidr" <The IP address or address range to filter, in CIDR notation
%
nameB" The name of the filter

policy" Block or Allow
"'
arn" The SES receipt filter ARN.
"H
cidr" <The IP address or address range to filter, in CIDR notation
"#
name" The name of the filter
"
policy" Block or Allow
*¬@
3
sesReceiptRuleaws:ses/receiptRule:ReceiptRuleı'Provides an SES receipt rule resource

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// Add a header to the email and store it in S3
const store = new aws.ses.ReceiptRule("store", {
    name: "store",
    ruleSetName: "default-rule-set",
    recipients: ["karen@example.com"],
    enabled: true,
    scanEnabled: true,
    addHeaderActions: [{
        headerName: "Custom-Header",
        headerValue: "Added by SES",
        position: 1,
    }],
    s3Actions: [{
        bucketName: "emails",
        position: 2,
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

# Add a header to the email and store it in S3
store = aws.ses.ReceiptRule("store",
    name="store",
    rule_set_name="default-rule-set",
    recipients=["karen@example.com"],
    enabled=True,
    scan_enabled=True,
    add_header_actions=[{
        "header_name": "Custom-Header",
        "header_value": "Added by SES",
        "position": 1,
    }],
    s3_actions=[{
        "bucket_name": "emails",
        "position": 2,
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // Add a header to the email and store it in S3
    var store = new Aws.Ses.ReceiptRule("store", new()
    {
        Name = "store",
        RuleSetName = "default-rule-set",
        Recipients = new[]
        {
            "karen@example.com",
        },
        Enabled = true,
        ScanEnabled = true,
        AddHeaderActions = new[]
        {
            new Aws.Ses.Inputs.ReceiptRuleAddHeaderActionArgs
            {
                HeaderName = "Custom-Header",
                HeaderValue = "Added by SES",
                Position = 1,
            },
        },
        S3Actions = new[]
        {
            new Aws.Ses.Inputs.ReceiptRuleS3ActionArgs
            {
                BucketName = "emails",
                Position = 2,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// Add a header to the email and store it in S3
		_, err := ses.NewReceiptRule(ctx, "store", &ses.ReceiptRuleArgs{
			Name:        pulumi.String("store"),
			RuleSetName: pulumi.String("default-rule-set"),
			Recipients: pulumi.StringArray{
				pulumi.String("karen@example.com"),
			},
			Enabled:     pulumi.Bool(true),
			ScanEnabled: pulumi.Bool(true),
			AddHeaderActions: ses.ReceiptRuleAddHeaderActionArray{
				&ses.ReceiptRuleAddHeaderActionArgs{
					HeaderName:  pulumi.String("Custom-Header"),
					HeaderValue: pulumi.String("Added by SES"),
					Position:    pulumi.Int(1),
				},
			},
			S3Actions: ses.ReceiptRuleS3ActionArray{
				&ses.ReceiptRuleS3ActionArgs{
					BucketName: pulumi.String("emails"),
					Position:   pulumi.Int(2),
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
import com.pulumi.aws.ses.ReceiptRule;
import com.pulumi.aws.ses.ReceiptRuleArgs;
import com.pulumi.aws.ses.inputs.ReceiptRuleAddHeaderActionArgs;
import com.pulumi.aws.ses.inputs.ReceiptRuleS3ActionArgs;
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
        // Add a header to the email and store it in S3
        var store = new ReceiptRule("store", ReceiptRuleArgs.builder()
            .name("store")
            .ruleSetName("default-rule-set")
            .recipients("karen@example.com")
            .enabled(true)
            .scanEnabled(true)
            .addHeaderActions(ReceiptRuleAddHeaderActionArgs.builder()
                .headerName("Custom-Header")
                .headerValue("Added by SES")
                .position(1)
                .build())
            .s3Actions(ReceiptRuleS3ActionArgs.builder()
                .bucketName("emails")
                .position(2)
                .build())
            .build());

    }
}
```
```yaml
resources:
  # Add a header to the email and store it in S3
  store:
    type: aws:ses:ReceiptRule
    properties:
      name: store
      ruleSetName: default-rule-set
      recipients:
        - karen@example.com
      enabled: true
      scanEnabled: true
      addHeaderActions:
        - headerName: Custom-Header
          headerValue: Added by SES
          position: 1
      s3Actions:
        - bucketName: emails
          position: 2
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SES receipt rules using the ruleset name and rule name separated by `:`. For example:

```sh
$ pulumi import aws:ses/receiptRule:ReceiptRule my_rule my_rule_set:my_rule
```
¥
addHeaderActionshBf*d:b
`
sesReceiptRuleAddHeaderAction=aws:ses/ReceiptRuleAddHeaderAction:ReceiptRuleAddHeaderAction6A list of Add Header Action blocks. Documented below.
=
afterB" .The name of the rule to place this rule after
§
bounceActions_B]*[:Y
W
sesReceiptRuleBounceAction7aws:ses/ReceiptRuleBounceAction:ReceiptRuleBounceAction2A list of Bounce Action blocks. Documented below.
3
enabledB
 "If true, the rule will be enabled
§
lambdaActions_B]*[:Y
W
sesReceiptRuleLambdaAction7aws:ses/ReceiptRuleLambdaAction:ReceiptRuleLambdaAction2A list of Lambda Action blocks. Documented below.
#
nameB" The name of the rule
0

recipientsB*" A list of email addresses
,
ruleSetName" The name of the rule set
ê
	s3ActionsSBQ*O:M
K
sesReceiptRuleS3Action/aws:ses/ReceiptRuleS3Action:ReceiptRuleS3Action.A list of S3 Action blocks. Documented below.
S
scanEnabledB
 >If true, incoming emails will be scanned for spam and viruses
ï

snsActionsVBT*R:P
N
sesReceiptRuleSnsAction1aws:ses/ReceiptRuleSnsAction:ReceiptRuleSnsAction/A list of SNS Action blocks. Documented below.
ö
stopActionsYBW*U:S
Q
sesReceiptRuleStopAction3aws:ses/ReceiptRuleStopAction:ReceiptRuleStopAction0A list of Stop Action blocks. Documented below.
+
	tlsPolicyB" `Require` or `Optional`
Æ
workmailActionseBc*a:_
]
sesReceiptRuleWorkmailAction;aws:ses/ReceiptRuleWorkmailAction:ReceiptRuleWorkmailAction4A list of WorkMail Action blocks. Documented below.
"¥
addHeaderActionshBf*d:b
`
sesReceiptRuleAddHeaderAction=aws:ses/ReceiptRuleAddHeaderAction:ReceiptRuleAddHeaderAction6A list of Add Header Action blocks. Documented below.
"=
afterB" .The name of the rule to place this rule after
"%
arn" The SES receipt rule ARN.
"§
bounceActions_B]*[:Y
W
sesReceiptRuleBounceAction7aws:ses/ReceiptRuleBounceAction:ReceiptRuleBounceAction2A list of Bounce Action blocks. Documented below.
"3
enabledB
 "If true, the rule will be enabled
"§
lambdaActions_B]*[:Y
W
sesReceiptRuleLambdaAction7aws:ses/ReceiptRuleLambdaAction:ReceiptRuleLambdaAction2A list of Lambda Action blocks. Documented below.
"!
name" The name of the rule
"0

recipientsB*" A list of email addresses
",
ruleSetName" The name of the rule set
"ê
	s3ActionsSBQ*O:M
K
sesReceiptRuleS3Action/aws:ses/ReceiptRuleS3Action:ReceiptRuleS3Action.A list of S3 Action blocks. Documented below.
"S
scanEnabledB
 >If true, incoming emails will be scanned for spam and viruses
"ï

snsActionsVBT*R:P
N
sesReceiptRuleSnsAction1aws:ses/ReceiptRuleSnsAction:ReceiptRuleSnsAction/A list of SNS Action blocks. Documented below.
"ö
stopActionsYBW*U:S
Q
sesReceiptRuleStopAction3aws:ses/ReceiptRuleStopAction:ReceiptRuleStopAction0A list of Stop Action blocks. Documented below.
")
	tlsPolicy" `Require` or `Optional`
"Æ
workmailActionseBc*a:_
]
sesReceiptRuleWorkmailAction;aws:ses/ReceiptRuleWorkmailAction:ReceiptRuleWorkmailAction4A list of WorkMail Action blocks. Documented below.
*®
<
sesReceiptRuleSet%aws:ses/receiptRuleSet:ReceiptRuleSetÍProvides an SES receipt rule set resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = new aws.ses.ReceiptRuleSet("main", {ruleSetName: "primary-rules"});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.ses.ReceiptRuleSet("main", rule_set_name="primary-rules")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = new Aws.Ses.ReceiptRuleSet("main", new()
    {
        RuleSetName = "primary-rules",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewReceiptRuleSet(ctx, "main", &ses.ReceiptRuleSetArgs{
			RuleSetName: pulumi.String("primary-rules"),
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
import com.pulumi.aws.ses.ReceiptRuleSet;
import com.pulumi.aws.ses.ReceiptRuleSetArgs;
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
        var main = new ReceiptRuleSet("main", ReceiptRuleSetArgs.builder()
            .ruleSetName("primary-rules")
            .build());

    }
}
```
```yaml
resources:
  main:
    type: aws:ses:ReceiptRuleSet
    properties:
      ruleSetName: primary-rules
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SES receipt rule sets using the rule set name. For example:

```sh
$ pulumi import aws:ses/receiptRuleSet:ReceiptRuleSet my_rule_set my_rule_set_name
```
)
ruleSetName" Name of the rule set.
"%
arn" SES receipt rule set ARN.
")
ruleSetName" Name of the rule set.
*‘!
*
sesTemplateaws:ses/template:Template‘Provides a resource to create a SES template.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const myTemplate = new aws.ses.Template("MyTemplate", {
    name: "MyTemplate",
    subject: "Greetings, {{name}}!",
    html: "<h1>Hello {{name}},</h1><p>Your favorite animal is {{favoriteanimal}}.</p>",
    text: `Hello {{name}},\x0d
Your favorite animal is {{favoriteanimal}}.`,
});
```
```python
import pulumi
import pulumi_aws as aws

my_template = aws.ses.Template("MyTemplate",
    name="MyTemplate",
    subject="Greetings, {{name}}!",
    html="<h1>Hello {{name}},</h1><p>Your favorite animal is {{favoriteanimal}}.</p>",
    text="""Hello {{name}},\x0d
Your favorite animal is {{favoriteanimal}}.""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var myTemplate = new Aws.Ses.Template("MyTemplate", new()
    {
        Name = "MyTemplate",
        Subject = "Greetings, {{name}}!",
        Html = "<h1>Hello {{name}},</h1><p>Your favorite animal is {{favoriteanimal}}.</p>",
        Text = @"Hello {{name}},
Your favorite animal is {{favoriteanimal}}.",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.NewTemplate(ctx, "MyTemplate", &ses.TemplateArgs{
			Name:    pulumi.String("MyTemplate"),
			Subject: pulumi.String("Greetings, {{name}}!"),
			Html:    pulumi.String("<h1>Hello {{name}},</h1><p>Your favorite animal is {{favoriteanimal}}.</p>"),
			Text:    pulumi.String("Hello {{name}},\nYour favorite animal is {{favoriteanimal}}."),
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
import com.pulumi.aws.ses.Template;
import com.pulumi.aws.ses.TemplateArgs;
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
        var myTemplate = new Template("myTemplate", TemplateArgs.builder()
            .name("MyTemplate")
            .subject("Greetings, {{name}}!")
            .html("<h1>Hello {{name}},</h1><p>Your favorite animal is {{favoriteanimal}}.</p>")
            .text("""
Hello {{name}},
Your favorite animal is {{favoriteanimal}}.            """)
            .build());

    }
}
```
```yaml
resources:
  myTemplate:
    type: aws:ses:Template
    name: MyTemplate
    properties:
      name: MyTemplate
      subject: Greetings, {{name}}!
      html: <h1>Hello {{name}},</h1><p>Your favorite animal is {{favoriteanimal}}.</p>
      text: "Hello {{name}},\r\nYour favorite animal is {{favoriteanimal}}."
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SES templates using the template name. For example:

```sh
$ pulumi import aws:ses/template:Template MyTemplate MyTemplate
```
s
htmlB" eThe HTML body of the email. Must be less than 500KB in size, including both the text and HTML parts.
v
nameB" hThe name of the template. Cannot exceed 64 characters. You will refer to this name when you send email.
0
subjectB" The subject line of the email.
≥
textB" §The email body that will be visible to recipients whose email clients do not display HTML. Must be less than 500KB in size, including both the text and HTML parts.
"'
arn" The ARN of the SES template
"s
htmlB" eThe HTML body of the email. Must be less than 500KB in size, including both the text and HTML parts.
"t
name" hThe name of the template. Cannot exceed 64 characters. You will refer to this name when you send email.
"0
subjectB" The subject line of the email.
"≥
textB" §The email body that will be visible to recipients whose email clients do not display HTML. Must be less than 500KB in size, including both the text and HTML parts.
2¯
!getArnaws:index/getArn:getArnÛParses an ARN into its constituent parts.

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

idB" "û
account" éThe [ID](https://docs.aws.amazon.com/general/latest/gr/acct-identifiers.html) of the AWS account that owns the resource, without the hyphens.
"	
arn" "
id" "4
	partition" #Partition that the resource is in.
"í
region" ÉRegion the resource resides in.
Note that the ARNs for some resources do not require a region, so this component might be omitted.
"˛
resource" ÌContent of this part of the ARN varies by service.
It often includes an indicator of the type of resource‚Äîfor example, an IAM user or Amazon RDS database ‚Äîfollowed by a slash (/) or a colon (:), followed by the resource name itself.
"≥
service" £The [service namespace](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces) that identifies the AWS product.
2Â[
HgetAvailabilityZone1aws:index/getAvailabilityZone:getAvailabilityZone¢J`aws.getAvailabilityZone` provides details about a specific availability zone (AZ)
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
Ä
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
•
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
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter"ß
	groupName" ïFor Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "Ÿ

nameSuffix" ∆Part of the AZ name that appears after the region name, uniquely identifying the AZ within its region.
For Availability Zones this is usually a single letter, for example `a` for the `us-west-2a` zone.
For Local and Wavelength Zones this is a longer string, for example `wl1-sfo-wlz-1` for the `us-west-2-wl1-sfo-wlz-1` zone.
"Y
networkBorderGroup" ?The name of the location from which the address is advertised.
"√
optInStatus" ØFor Availability Zones, this always has the value of `opt-in-not-required`. For Local Zones, this is the opt in status. The possible values are `opted-in` and `not-opted-in`.
"á
parentZoneId" sID of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"ã
parentZoneName" uName of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"±
region" ¢Region where the selected availability zone resides. This is always the region selected on the provider, since this data source searches only within that region.
"
state" "
zoneId" "c
zoneType" SType of zone. Values are `availability-zone`, `local-zone`, and `wavelength-zone`.
2Ã\
KgetAvailabilityZones3aws:index/getAvailabilityZones:getAvailabilityZones˙PThe Availability Zones data source allows access to the list of AWS
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
Ä
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
D
excludeNamesB*" ,List of Availability Zone names to exclude.
D
excludeZoneIdsB*" *List of Availability Zone IDs to exclude.
®
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter6Configuration block(s) for filtering. Detailed below.
∂
stateB" ¶Allows to filter list of Availability Zones based on their
current state. Can be either `"available"`, `"information"`, `"impaired"` or
`"unavailable"`. By default the list includes a complete set of Availability Zones
to which the underlying AWS account has access, regardless of their state.
"
allAvailabilityZonesB
 "
excludeNamesB*" "
excludeZoneIdsB*" "p
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter"÷

groupNames*" ¡A set of the Availability Zone Group names. For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"M
names*" >List of the Availability Zone names available to the account.
"
stateB" "M
zoneIds*" <List of the Availability Zone IDs available to the account.
2Û[
WgetBillingServiceAccount;aws:index/getBillingServiceAccount:getBillingServiceAccount˙YUse this data source to get the Account ID of the [AWS Billing and Cost Management Service Account](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-getting-started.html#step-2) for the purpose of permitting in S3 bucket policy.

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
2û
BgetCallerIdentity-aws:index/getCallerIdentity:getCallerIdentity◊Use this data source to get the access to the effective Account ID, User ID, and ARN in
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
2Ç
9getDefaultTags'aws:index/getDefaultTags:getDefaultTagsÚUse this data source to get the default tags configured on the provider.

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
2”;
0getIpRanges!aws:index/getIpRanges:getIpRangesç1Use this data source to get the IP ranges of various AWS products and services. For more information about the contents of this data source and required JSON syntax if referencing a custom URL, see the [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html).

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

idB" ¥
regionsB*" †Filter IP ranges by regions (or include all regions, if
omitted). Valid items are `global` (for `cloudfront`) as well as all AWS regions
(e.g., `eu-central-1`)
Ê
services*" ”Filter IP ranges by services. Valid items are `amazon`
(for amazon.com), `amazon_connect`, `api_gateway`, `cloud9`, `cloudfront`,
`codebuild`, `dynamodb`, `ec2`, `ec2_instance_connect`, `globalaccelerator`,
`route53`, `route53_healthchecks`, `s3` and `workspaces_gateways`. See the
[`service` attribute][2] documentation for other possible values.

> **NOTE:** If the specified combination of regions and services does not yield any
CIDR blocks, this call will fail.
Ë
urlB" ⁄Custom URL for source JSON file. Syntax must match [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html). Defaults to `https://ip-ranges.amazonaws.com/ip-ranges.json`.
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
urlB" 2Å"
3getPartition#aws:index/getPartition:getPartition˚Use this data source to lookup information about the current AWS partition in
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
"å
	dnsSuffix" {Base DNS domain name for the current partition (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
"f
id" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"m
	partition" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"|
reverseDnsPrefix" dPrefix of service names (e.g., `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
2œ
*	getRegionaws:index/getRegion:getRegionÊ`aws.getRegion` provides details about a specific AWS region.

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
2Û4
-
getRegionsaws:index/getRegions:getRegionsﬂ/Provides information about AWS Regions. Can be used to filter regions i.e., by Opt-In status or only regions enabled for current account. To get details like endpoint and description of each region the data source can be combined with the `aws.getRegion` data source.

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
é
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
2Ω9
-
getServiceaws:index/getService:getServiceÃ0Use this data source to compose and decompose AWS service DNS names.

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
ó
dnsNameB" ÖDNS name of the service (_e.g.,_ `rds.us-east-1.amazonaws.com`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.


idB" O
regionB" ?Region of the service (_e.g.,_ `us-west-2`, `ap-northeast-1`).
•
reverseDnsNameB" åReverse DNS name of the service (_e.g.,_ `com.amazonaws.us-west-2.s3`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.
~
reverseDnsPrefixB" dPrefix of the service (_e.g.,_ `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
ù
	serviceIdB" âService endpoint ID (_e.g.,_ `s3`, `rds`, `ec2`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required. A service's endpoint ID can be found in the [_AWS General Reference_](https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html).
"
dnsName" "
id" "
	partition" "
region" "
reverseDnsName" "
reverseDnsPrefix" "
	serviceId" "Ü
	supported
 uWhether the service is supported in the region's partition. New services may not be listed immediately as supported.
2 
HgetServicePrincipal1aws:index/getServicePrincipal:getServicePrincipalâUse this data source to create a Service Principal Name for a service in a given region. Service Principal Names should always end in the standard global format: `{servicename}.amazonaws.com`. However, in some AWS partitions, AWS may expect a different format.

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
" 
id" øIdentifier of the current Service Principal (compound of service, region and suffix). (e.g. `logs.us-east-1.amazonaws.com`in AWS Commercial, `logs.cn-north-1.amazonaws.com.cn` in AWS China).
"y
name" mService Principal Name (e.g., `logs.amazonaws.com` in AWS Commercial, `logs.amazonaws.com.cn` in AWS China).
"w
region" iRegion identifier of the generated SPN (e.g., `us-east-1` in AWS Commercial, `cn-north-1` in AWS China).
"
serviceName" "l
suffix" ^Suffix of the SPN (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
2á
[
secretsmanagergetRandomPassword6aws:secretsmanager/getRandomPassword:getRandomPassword∫Generate a random password.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.secretsmanager.getRandomPassword({
    passwordLength: 50,
    excludeNumbers: true,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.secretsmanager.get_random_password(password_length=50,
    exclude_numbers=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.SecretsManager.GetRandomPassword.Invoke(new()
    {
        PasswordLength = 50,
        ExcludeNumbers = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.GetRandomPassword(ctx, &secretsmanager.GetRandomPasswordArgs{
			PasswordLength: pulumi.IntRef(50),
			ExcludeNumbers: pulumi.BoolRef(true),
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
import com.pulumi.aws.secretsmanager.SecretsmanagerFunctions;
import com.pulumi.aws.secretsmanager.inputs.GetRandomPasswordArgs;
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
        final var test = SecretsmanagerFunctions.getRandomPassword(GetRandomPasswordArgs.builder()
            .passwordLength(50)
            .excludeNumbers(true)
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:secretsmanager:getRandomPassword
      arguments:
        passwordLength: 50
        excludeNumbers: true
```
<!--End PulumiCodeChooser -->
Y
excludeCharactersB" >String of the characters that you don't want in the password.
\
excludeLowercaseB
 BSpecifies whether to exclude lowercase letters from the password.
P
excludeNumbersB
 8Specifies whether to exclude numbers from the password.
∏
excludePunctuationB
 õSpecifies whether to exclude the following punctuation characters from the password: ``! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~ .``
\
excludeUppercaseB
 BSpecifies whether to exclude uppercase letters from the password.
H
includeSpaceB
 2Specifies whether to include the space character.
0
passwordLengthB Length of the password.
à
requireEachIncludedTypeB
 gSpecifies whether to include at least one upper and lowercase letter, one number, and one punctuation.
"
excludeCharactersB" "
excludeLowercaseB
 "
excludeNumbersB
 "
excludePunctuationB
 "
excludeUppercaseB
 "E
id" ;The provider-assigned unique ID for this managed resource.
"
includeSpaceB
 "
passwordLengthB "'
randomPassword" Random password.
"
requireEachIncludedTypeB
 2È%
C
secretsmanager	getSecret&aws:secretsmanager/getSecret:getSecret™ Retrieve metadata information about a Secrets Manager secret. To retrieve a secret value, see the `aws.secretsmanager.SecretVersion` data source.

## Example Usage

### ARN

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const by-arn = aws.secretsmanager.getSecret({
    arn: "arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456",
});
```
```python
import pulumi
import pulumi_aws as aws

by_arn = aws.secretsmanager.get_secret(arn="arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var by_arn = Aws.SecretsManager.GetSecret.Invoke(new()
    {
        Arn = "arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.LookupSecret(ctx, &secretsmanager.LookupSecretArgs{
			Arn: pulumi.StringRef("arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456"),
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
import com.pulumi.aws.secretsmanager.SecretsmanagerFunctions;
import com.pulumi.aws.secretsmanager.inputs.GetSecretArgs;
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
        final var by-arn = SecretsmanagerFunctions.getSecret(GetSecretArgs.builder()
            .arn("arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456")
            .build());

    }
}
```
```yaml
variables:
  by-arn:
    fn::invoke:
      function: aws:secretsmanager:getSecret
      arguments:
        arn: arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456
```
<!--End PulumiCodeChooser -->

### Name

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const by-name = aws.secretsmanager.getSecret({
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

by_name = aws.secretsmanager.get_secret(name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var by_name = Aws.SecretsManager.GetSecret.Invoke(new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.LookupSecret(ctx, &secretsmanager.LookupSecretArgs{
			Name: pulumi.StringRef("example"),
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
import com.pulumi.aws.secretsmanager.SecretsmanagerFunctions;
import com.pulumi.aws.secretsmanager.inputs.GetSecretArgs;
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
        final var by-name = SecretsmanagerFunctions.getSecret(GetSecretArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  by-name:
    fn::invoke:
      function: aws:secretsmanager:getSecret
      arguments:
        name: example
```
<!--End PulumiCodeChooser -->
,
arnB" ARN of the secret to retrieve.
.
nameB"  Name of the secret to retrieve.
$
tagsB2" Tags of the secret.
"
arn" ARN of the secret.
"6
createdDate" #Created date of the secret in UTC.
".
description" Description of the secret.
"E
id" ;The provider-assigned unique ID for this managed resource.
"c
kmsKeyId" SKey Management Service (KMS) Customer Master Key (CMK) associated with the secret.
"?
lastChangedDate" (Last updated date of the secret in UTC.
"

name" "L
policy" >Resource-based policy document that's attached to the secret.
""
tags2" Tags of the secret.
2ö
[
secretsmanagergetSecretRotation6aws:secretsmanager/getSecretRotation:getSecretRotationñRetrieve information about a Secrets Manager secret rotation. To retrieve secret metadata, see the `aws.secretsmanager.Secret` data source. To retrieve a secret value, see the `aws.secretsmanager.SecretVersion` data source.

## Example Usage

### Retrieve Secret Rotation Configuration

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.secretsmanager.getSecretRotation({
    secretId: exampleAwsSecretsmanagerSecret.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.secretsmanager.get_secret_rotation(secret_id=example_aws_secretsmanager_secret["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SecretsManager.GetSecretRotation.Invoke(new()
    {
        SecretId = exampleAwsSecretsmanagerSecret.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.LookupSecretRotation(ctx, &secretsmanager.LookupSecretRotationArgs{
			SecretId: exampleAwsSecretsmanagerSecret.Id,
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
import com.pulumi.aws.secretsmanager.SecretsmanagerFunctions;
import com.pulumi.aws.secretsmanager.inputs.GetSecretRotationArgs;
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
        final var example = SecretsmanagerFunctions.getSecretRotation(GetSecretRotationArgs.builder()
            .secretId(exampleAwsSecretsmanagerSecret.id())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:secretsmanager:getSecretRotation
      arguments:
        secretId: ${exampleAwsSecretsmanagerSecret.id}
```
<!--End PulumiCodeChooser -->
õ
secretId" äSpecifies the secret containing the version that you want to retrieve. You can specify either the ARN or the friendly name of the secret.
"E
id" ;The provider-assigned unique ID for this managed resource.
"*
rotationEnabled
 ARN of the secret.
"v
rotationLambdaArn" ]Decrypted part of the protected secret information that was originally provided as a string.
"à
rotationRulesá*Ñ:Å

secretsmanagergetSecretRotationRotationRuleNaws:secretsmanager/getSecretRotationRotationRule:getSecretRotationRotationRulemDecrypted part of the protected secret information that was originally provided as a binary. Base64 encoded.
"
secretId" 2Í*
X
secretsmanagergetSecretVersion4aws:secretsmanager/getSecretVersion:getSecretVersion»"Retrieve information about a Secrets Manager secret version, including its secret value. To retrieve secret metadata, see the `aws.secretsmanager.Secret` data source.

## Example Usage

### Retrieve Current Secret Version

By default, this data sources retrieves information based on the `AWSCURRENT` staging label.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const secret-version = aws.secretsmanager.getSecretVersion({
    secretId: example.id,
});
```
```python
import pulumi
import pulumi_aws as aws

secret_version = aws.secretsmanager.get_secret_version(secret_id=example["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var secret_version = Aws.SecretsManager.GetSecretVersion.Invoke(new()
    {
        SecretId = example.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.LookupSecretVersion(ctx, &secretsmanager.LookupSecretVersionArgs{
			SecretId: example.Id,
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
import com.pulumi.aws.secretsmanager.SecretsmanagerFunctions;
import com.pulumi.aws.secretsmanager.inputs.GetSecretVersionArgs;
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
        final var secret-version = SecretsmanagerFunctions.getSecretVersion(GetSecretVersionArgs.builder()
            .secretId(example.id())
            .build());

    }
}
```
```yaml
variables:
  secret-version:
    fn::invoke:
      function: aws:secretsmanager:getSecretVersion
      arguments:
        secretId: ${example.id}
```
<!--End PulumiCodeChooser -->

### Retrieve Specific Secret Version

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const by-version-stage = aws.secretsmanager.getSecretVersion({
    secretId: example.id,
    versionStage: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

by_version_stage = aws.secretsmanager.get_secret_version(secret_id=example["id"],
    version_stage="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var by_version_stage = Aws.SecretsManager.GetSecretVersion.Invoke(new()
    {
        SecretId = example.Id,
        VersionStage = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.LookupSecretVersion(ctx, &secretsmanager.LookupSecretVersionArgs{
			SecretId:     example.Id,
			VersionStage: pulumi.StringRef("example"),
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
import com.pulumi.aws.secretsmanager.SecretsmanagerFunctions;
import com.pulumi.aws.secretsmanager.inputs.GetSecretVersionArgs;
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
        final var by-version-stage = SecretsmanagerFunctions.getSecretVersion(GetSecretVersionArgs.builder()
            .secretId(example.id())
            .versionStage("example")
            .build());

    }
}
```
```yaml
variables:
  by-version-stage:
    fn::invoke:
      function: aws:secretsmanager:getSecretVersion
      arguments:
        secretId: ${example.id}
        versionStage: example
```
<!--End PulumiCodeChooser -->

õ
secretId" äSpecifies the secret containing the version that you want to retrieve. You can specify either the ARN or the friendly name of the secret.
Ü
	versionIdB" sSpecifies the unique identifier of the version of the secret that you want to retrieve. Overrides `version_stage`.
ï
versionStageB" Specifies the secret version that you want to retrieve by the staging label attached to the version. Defaults to `AWSCURRENT`.
"
arn" ARN of the secret.
"6
createdDate" #Created date of the secret in UTC.
"E
id" ;The provider-assigned unique ID for this managed resource.
"q
secretBinary" ]Decrypted part of the protected secret information that was originally provided as a binary.
"
secretId" "q
secretString" ]Decrypted part of the protected secret information that was originally provided as a string.
"B
	versionId" 1Unique identifier of this version of the secret.
"
versionStageB" "
versionStages*" 2«
[
secretsmanagergetSecretVersions6aws:secretsmanager/getSecretVersions:getSecretVersions‡
includeDeprecatedB
 ƒIf true, all deprecated secret versions are included in the response.
If false, no deprecated secret versions are included in the response. If no value is specified, the default value is `false`.
õ
secretId" äSpecifies the secret containing the version that you want to retrieve. You can specify either the ARN or the friendly name of the secret.
"
arn" ARN of the secret.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
includeDeprecatedB
 "

name" "
secretId" "»
versionsv*t:r
p
secretsmanagergetSecretVersionsVersionDaws:secretsmanager/getSecretVersionsVersion:getSecretVersionsVersionDList of the versions of the secret. Attributes are specified below.
2£
F
secretsmanager
getSecrets(aws:secretsmanager/getSecrets:getSecretsÙUse this data source to get the ARNs and names of Secrets Manager secrets matching the specified criteria.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.secretsmanager.getSecrets({
    filters: [{
        name: "name",
        values: ["example"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.secretsmanager.get_secrets(filters=[{
    "name": "name",
    "values": ["example"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SecretsManager.GetSecrets.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.SecretsManager.Inputs.GetSecretsFilterInputArgs
            {
                Name = "name",
                Values = new[]
                {
                    "example",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/secretsmanager"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := secretsmanager.GetSecrets(ctx, &secretsmanager.GetSecretsArgs{
			Filters: []secretsmanager.GetSecretsFilter{
				{
					Name: "name",
					Values: []string{
						"example",
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
import com.pulumi.aws.secretsmanager.SecretsmanagerFunctions;
import com.pulumi.aws.secretsmanager.inputs.GetSecretsArgs;
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
        final var example = SecretsmanagerFunctions.getSecrets(GetSecretsArgs.builder()
            .filters(GetSecretsFilterArgs.builder()
                .name("name")
                .values("example")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:secretsmanager:getSecrets
      arguments:
        filters:
          - name: name
            values:
              - example
```
<!--End PulumiCodeChooser -->
£
filters`B^*\:Z
X
secretsmanagergetSecretsFilter4aws:secretsmanager/getSecretsFilter:getSecretsFilter6Configuration block(s) for filtering. Detailed below.
"B
arns*" 4Set of ARNs of the matched Secrets Manager secrets.
"k
filters`B^*\:Z
X
secretsmanagergetSecretsFilter4aws:secretsmanager/getSecretsFilter:getSecretsFilter"E
id" ;The provider-assigned unique ID for this managed resource.
"D
names*" 5Set of names of the matched Secrets Manager secrets.
2Â

securityhubgetStandardsControlAssociationsOaws:securityhub/getStandardsControlAssociations:getStandardsControlAssociationsÏData source for managing an AWS Security Hub Standards Control Associations.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testAccount = new aws.securityhub.Account("test", {});
const test = aws.securityhub.getStandardsControlAssociations({
    securityControlId: "IAM.1",
});
```
```python
import pulumi
import pulumi_aws as aws

test_account = aws.securityhub.Account("test")
test = aws.securityhub.get_standards_control_associations(security_control_id="IAM.1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testAccount = new Aws.SecurityHub.Account("test");

    var test = Aws.SecurityHub.GetStandardsControlAssociations.Invoke(new()
    {
        SecurityControlId = "IAM.1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/securityhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := securityhub.NewAccount(ctx, "test", nil)
		if err != nil {
			return err
		}
		_, err = securityhub.GetStandardsControlAssociations(ctx, &securityhub.GetStandardsControlAssociationsArgs{
			SecurityControlId: "IAM.1",
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
import com.pulumi.aws.securityhub.Account;
import com.pulumi.aws.securityhub.SecurityhubFunctions;
import com.pulumi.aws.securityhub.inputs.GetStandardsControlAssociationsArgs;
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
        var testAccount = new Account("testAccount");

        final var test = SecurityhubFunctions.getStandardsControlAssociations(GetStandardsControlAssociationsArgs.builder()
            .securityControlId("IAM.1")
            .build());

    }
}
```
```yaml
resources:
  testAccount:
    type: aws:securityhub:Account
    name: test
variables:
  test:
    fn::invoke:
      function: aws:securityhub:getStandardsControlAssociations
      arguments:
        securityControlId: IAM.1
```
<!--End PulumiCodeChooser -->
ë
securityControlId" xThe identifier of the control (identified with `SecurityControlId`, `SecurityControlArn`, or a mix of both parameters).
"
id" "5
securityControlId" ID of the security control.
"ù
standardsControlAssociations⁄*◊:‘
—
securityhub:getStandardsControlAssociationsStandardsControlAssociationÖaws:securityhub/getStandardsControlAssociationsStandardsControlAssociation:getStandardsControlAssociationsStandardsControlAssociationüA list that provides the status and other details for each security control that applies to each enabled standard.
See `standards_control_associations` below.
2®,
^
serverlessrepositorygetApplication6aws:serverlessrepository/getApplication:getApplication—&Use this data source to get information about an AWS Serverless Application Repository application. For example, this can be used to determine the required `capabilities` for an application.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.serverlessrepository.getApplication({
    applicationId: "arn:aws:serverlessrepo:us-east-1:123456789012:applications/ExampleApplication",
});
const exampleCloudFormationStack = new aws.serverlessrepository.CloudFormationStack("example", {
    name: "Example",
    applicationId: example.then(example => example.applicationId),
    semanticVersion: example.then(example => example.semanticVersion),
    capabilities: example.then(example => example.requiredCapabilities),
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.serverlessrepository.get_application(application_id="arn:aws:serverlessrepo:us-east-1:123456789012:applications/ExampleApplication")
example_cloud_formation_stack = aws.serverlessrepository.CloudFormationStack("example",
    name="Example",
    application_id=example.application_id,
    semantic_version=example.semantic_version,
    capabilities=example.required_capabilities)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServerlessRepository.GetApplication.Invoke(new()
    {
        ApplicationId = "arn:aws:serverlessrepo:us-east-1:123456789012:applications/ExampleApplication",
    });

    var exampleCloudFormationStack = new Aws.ServerlessRepository.CloudFormationStack("example", new()
    {
        Name = "Example",
        ApplicationId = example.Apply(getApplicationResult => getApplicationResult.ApplicationId),
        SemanticVersion = example.Apply(getApplicationResult => getApplicationResult.SemanticVersion),
        Capabilities = example.Apply(getApplicationResult => getApplicationResult.RequiredCapabilities),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/serverlessrepository"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := serverlessrepository.GetApplication(ctx, &serverlessrepository.GetApplicationArgs{
			ApplicationId: "arn:aws:serverlessrepo:us-east-1:123456789012:applications/ExampleApplication",
		}, nil)
		if err != nil {
			return err
		}
		_, err = serverlessrepository.NewCloudFormationStack(ctx, "example", &serverlessrepository.CloudFormationStackArgs{
			Name:            pulumi.String("Example"),
			ApplicationId:   pulumi.String(example.ApplicationId),
			SemanticVersion: pulumi.String(example.SemanticVersion),
			Capabilities:    interface{}(example.RequiredCapabilities),
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
import com.pulumi.aws.serverlessrepository.ServerlessrepositoryFunctions;
import com.pulumi.aws.serverlessrepository.inputs.GetApplicationArgs;
import com.pulumi.aws.serverlessrepository.CloudFormationStack;
import com.pulumi.aws.serverlessrepository.CloudFormationStackArgs;
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
        final var example = ServerlessrepositoryFunctions.getApplication(GetApplicationArgs.builder()
            .applicationId("arn:aws:serverlessrepo:us-east-1:123456789012:applications/ExampleApplication")
            .build());

        var exampleCloudFormationStack = new CloudFormationStack("exampleCloudFormationStack", CloudFormationStackArgs.builder()
            .name("Example")
            .applicationId(example.applyValue(getApplicationResult -> getApplicationResult.applicationId()))
            .semanticVersion(example.applyValue(getApplicationResult -> getApplicationResult.semanticVersion()))
            .capabilities(example.applyValue(getApplicationResult -> getApplicationResult.requiredCapabilities()))
            .build());

    }
}
```
```yaml
resources:
  exampleCloudFormationStack:
    type: aws:serverlessrepository:CloudFormationStack
    name: example
    properties:
      name: Example
      applicationId: ${example.applicationId}
      semanticVersion: ${example.semanticVersion}
      capabilities: ${example.requiredCapabilities}
variables:
  example:
    fn::invoke:
      function: aws:serverlessrepository:getApplication
      arguments:
        applicationId: arn:aws:serverlessrepo:us-east-1:123456789012:applications/ExampleApplication
```
<!--End PulumiCodeChooser -->
-
applicationId" ARN of the application.
i
semanticVersionB" PRequested version of the application. By default, retrieves the latest version.
"-
applicationId" ARN of the application.
"E
id" ;The provider-assigned unique ID for this managed resource.
"%
name" Name of the application.
"r
requiredCapabilities*" TA list of capabilities describing the permissions needed to deploy the application.
"
semanticVersion" "Q
sourceCodeUrl" <URL pointing to the source code of the application version.
"]
templateUrl" JURL pointing to the Cloud Formation template for the application version.
2§
s
servicecataloggetAppregistryApplicationFaws:servicecatalog/getAppregistryApplication:getAppregistryApplication†Data source for managing an AWS Service Catalog AppRegistry Application.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicecatalog.getAppregistryApplication({
    id: "application-1234",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.get_appregistry_application(id="application-1234")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceCatalog.GetAppregistryApplication.Invoke(new()
    {
        Id = "application-1234",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.LookupAppregistryApplication(ctx, &servicecatalog.LookupAppregistryApplicationArgs{
			Id: "application-1234",
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
import com.pulumi.aws.servicecatalog.ServicecatalogFunctions;
import com.pulumi.aws.servicecatalog.inputs.GetAppregistryApplicationArgs;
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
        final var example = ServicecatalogFunctions.getAppregistryApplication(GetAppregistryApplicationArgs.builder()
            .id("application-1234")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicecatalog:getAppregistryApplication
      arguments:
        id: application-1234
```
<!--End PulumiCodeChooser -->
"
id" Application identifier.
"q
applicationTag2" YA map with a single tag key-value pair used to associate resources with the application.
":
arn" /ARN (Amazon Resource Name) of the application.
"3
description"  Description of the application.
"
id" "%
name" Name of the application.
"–
tags2" ¡A map of tags assigned to the Application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
2º
|
servicecataloggetAppregistryAttributeGroupLaws:servicecatalog/getAppregistryAttributeGroup:getAppregistryAttributeGroupàData source for managing an AWS Service Catalog AppRegistry Attribute Group.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicecatalog.getAppregistryAttributeGroup({
    name: "example_attribute_group",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.get_appregistry_attribute_group(name="example_attribute_group")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceCatalog.GetAppregistryAttributeGroup.Invoke(new()
    {
        Name = "example_attribute_group",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.LookupAppregistryAttributeGroup(ctx, &servicecatalog.LookupAppregistryAttributeGroupArgs{
			Name: pulumi.StringRef("example_attribute_group"),
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
import com.pulumi.aws.servicecatalog.ServicecatalogFunctions;
import com.pulumi.aws.servicecatalog.inputs.GetAppregistryAttributeGroupArgs;
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
        final var example = ServicecatalogFunctions.getAppregistryAttributeGroup(GetAppregistryAttributeGroupArgs.builder()
            .name("example_attribute_group")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicecatalog:getAppregistryAttributeGroup
      arguments:
        name: example_attribute_group
```
<!--End PulumiCodeChooser -->
1
arnB" $ARN of the Attribute Group to find.
/
idB" #ID of the Attribute Group to find.
3
nameB" %Name of the Attribute Group to find.
"	
arn" "g

attributes" UA JSON string of nested key-value pairs that represents the attributes of the group.
"7
description" $Description of the Attribute Group.
"
id" "

name" "‘
tags2" ≈A map of tags assigned to the Attribute Group. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
2Ö
†
servicecatalog(getAppregistryAttributeGroupAssociationsdaws:servicecatalog/getAppregistryAttributeGroupAssociations:getAppregistryAttributeGroupAssociations°Data source for managing AWS Service Catalog AppRegistry Attribute Group Associations.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicecatalog.getAppregistryAttributeGroupAssociations({
    id: "12456778723424sdffsdfsdq34,12234t3564dsfsdf34asff4ww3",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.get_appregistry_attribute_group_associations(id="12456778723424sdffsdfsdq34,12234t3564dsfsdf34asff4ww3")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceCatalog.GetAppregistryAttributeGroupAssociations.Invoke(new()
    {
        Id = "12456778723424sdffsdfsdq34,12234t3564dsfsdf34asff4ww3",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.GetAppregistryAttributeGroupAssociations(ctx, &servicecatalog.GetAppregistryAttributeGroupAssociationsArgs{
			Id: pulumi.StringRef("12456778723424sdffsdfsdq34,12234t3564dsfsdf34asff4ww3"),
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
import com.pulumi.aws.servicecatalog.ServicecatalogFunctions;
import com.pulumi.aws.servicecatalog.inputs.GetAppregistryAttributeGroupAssociationsArgs;
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
        final var example = ServicecatalogFunctions.getAppregistryAttributeGroupAssociations(GetAppregistryAttributeGroupAssociationsArgs.builder()
            .id("12456778723424sdffsdfsdq34,12234t3564dsfsdf34asff4ww3")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicecatalog:getAppregistryAttributeGroupAssociations
      arguments:
        id: 12456778723424sdffsdfsdq34,12234t3564dsfsdf34asff4ww3
```
<!--End PulumiCodeChooser -->
L
idB" @ID of the application to which attribute groups are associated.
w
nameB" iName of the application to which attribute groups are associated.

The following arguments are optional:
"[
attributeGroupIds*" @Set of attribute group IDs this application is associated with.
"

idB" "
nameB" 2—
O
servicecataloggetConstraint.aws:servicecatalog/getConstraint:getConstraint’Provides information on a Service Catalog Constraint.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicecatalog.getConstraint({
    acceptLanguage: "en",
    id: "cons-hrvy0335",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.get_constraint(accept_language="en",
    id="cons-hrvy0335")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceCatalog.GetConstraint.Invoke(new()
    {
        AcceptLanguage = "en",
        Id = "cons-hrvy0335",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.LookupConstraint(ctx, &servicecatalog.LookupConstraintArgs{
			AcceptLanguage: pulumi.StringRef("en"),
			Id:             "cons-hrvy0335",
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
import com.pulumi.aws.servicecatalog.ServicecatalogFunctions;
import com.pulumi.aws.servicecatalog.inputs.GetConstraintArgs;
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
        final var example = ServicecatalogFunctions.getConstraint(GetConstraintArgs.builder()
            .acceptLanguage("en")
            .id("cons-hrvy0335")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicecatalog:getConstraint
      arguments:
        acceptLanguage: en
        id: cons-hrvy0335
```
<!--End PulumiCodeChooser -->
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
4
descriptionB" Description of the constraint.
H
id" >Constraint identifier.

The following arguments are optional:
"
acceptLanguageB" "2
description" Description of the constraint.
"
id" "&
owner" Owner of the constraint.
"8

parameters" &Constraint parameters in JSON format.
")
portfolioId" Portfolio identifier.
"%
	productId" Product identifier.
"!
status" Constraint status.
"z
type" nType of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `RESOURCE_UPDATE`, `STACKSET`, and `TEMPLATE`.
2∆
R
servicecataloggetLaunchPaths0aws:servicecatalog/getLaunchPaths:getLaunchPathsÚLists the paths to the specified product. A path is how the user has access to a specified product, and is necessary when provisioning a product. A path also determines the constraints put on the product.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicecatalog.getLaunchPaths({
    productId: "prod-yakog5pdriver",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.get_launch_paths(product_id="prod-yakog5pdriver")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceCatalog.GetLaunchPaths.Invoke(new()
    {
        ProductId = "prod-yakog5pdriver",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.GetLaunchPaths(ctx, &servicecatalog.GetLaunchPathsArgs{
			ProductId: "prod-yakog5pdriver",
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
import com.pulumi.aws.servicecatalog.ServicecatalogFunctions;
import com.pulumi.aws.servicecatalog.inputs.GetLaunchPathsArgs;
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
        final var example = ServicecatalogFunctions.getLaunchPaths(GetLaunchPathsArgs.builder()
            .productId("prod-yakog5pdriver")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicecatalog:getLaunchPaths
      arguments:
        productId: prod-yakog5pdriver
```
<!--End PulumiCodeChooser -->
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
L
	productId" ;Product identifier.

The following arguments are optional:
"
acceptLanguageB" "E
id" ;The provider-assigned unique ID for this managed resource.
"
	productId" "Ω
	summariesm*k:i
g
servicecataloggetLaunchPathsSummary>aws:servicecatalog/getLaunchPathsSummary:getLaunchPathsSummaryABlock with information about the launch path. See details below.
2ã
L
servicecataloggetPortfolio,aws:servicecatalog/getPortfolio:getPortfolio˛Provides information for a Service Catalog Portfolio.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const portfolio = aws.servicecatalog.getPortfolio({
    id: "port-07052002",
});
```
```python
import pulumi
import pulumi_aws as aws

portfolio = aws.servicecatalog.get_portfolio(id="port-07052002")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var portfolio = Aws.ServiceCatalog.GetPortfolio.Invoke(new()
    {
        Id = "port-07052002",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.LookupPortfolio(ctx, &servicecatalog.LookupPortfolioArgs{
			Id: "port-07052002",
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
import com.pulumi.aws.servicecatalog.ServicecatalogFunctions;
import com.pulumi.aws.servicecatalog.inputs.GetPortfolioArgs;
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
        final var portfolio = ServicecatalogFunctions.getPortfolio(GetPortfolioArgs.builder()
            .id("port-07052002")
            .build());

    }
}
```
```yaml
variables:
  portfolio:
    fn::invoke:
      function: aws:servicecatalog:getPortfolio
      arguments:
        id: port-07052002
```
<!--End PulumiCodeChooser -->
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
G
id" =Portfolio identifier.

The following arguments are optional:
/
tagsB2" Tags applied to the portfolio.
"
acceptLanguageB" "
arn" Portfolio ARN.
"3
createdTime"  Time the portfolio was created.
"0
description" Description of the portfolio
"
id" "
name" Portfolio name.
"O
providerName" ;Name of the person or organization who owns the portfolio.
"-
tags2" Tags applied to the portfolio.
2ì
m
servicecataloggetPortfolioConstraintsBaws:servicecatalog/getPortfolioConstraints:getPortfolioConstraints©Provides information on Service Catalog Portfolio Constraints.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicecatalog.getPortfolioConstraints({
    portfolioId: "port-3lli3b3an",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.get_portfolio_constraints(portfolio_id="port-3lli3b3an")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceCatalog.GetPortfolioConstraints.Invoke(new()
    {
        PortfolioId = "port-3lli3b3an",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.GetPortfolioConstraints(ctx, &servicecatalog.GetPortfolioConstraintsArgs{
			PortfolioId: "port-3lli3b3an",
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
import com.pulumi.aws.servicecatalog.ServicecatalogFunctions;
import com.pulumi.aws.servicecatalog.inputs.GetPortfolioConstraintsArgs;
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
        final var example = ServicecatalogFunctions.getPortfolioConstraints(GetPortfolioConstraintsArgs.builder()
            .portfolioId("port-3lli3b3an")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicecatalog:getPortfolioConstraints
      arguments:
        portfolioId: port-3lli3b3an
```
<!--End PulumiCodeChooser -->
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
P
portfolioId" =Portfolio identifier.

The following arguments are optional:
'
	productIdB" Product identifier.
"
acceptLanguageB" "”
detailsá*Ñ:Å

servicecataloggetPortfolioConstraintsDetailNaws:servicecatalog/getPortfolioConstraintsDetail:getPortfolioConstraintsDetail>List of information about the constraints. See details below.
"E
id" ;The provider-assigned unique ID for this managed resource.
"•
portfolioId" ëIdentifier of the portfolio the product resides in. The constraint applies only to the instance of the product that lives within this portfolio.
"û
	productIdB" äIdentifier of the product the constraint applies to. A constraint applies to a specific instance of a product within a certain portfolio.
2˛
F
servicecatalog
getProduct(aws:servicecatalog/getProduct:getProductüUse this data source to retrieve information about a Service Catalog product.

> **NOTE:** A "provisioning artifact" is also known as a "version," and a "distributor" is also known as a "vendor."

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicecatalog.getProduct({
    id: "prod-dnigbtea24ste",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.get_product(id="prod-dnigbtea24ste")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceCatalog.GetProduct.Invoke(new()
    {
        Id = "prod-dnigbtea24ste",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.LookupProduct(ctx, &servicecatalog.LookupProductArgs{
			Id: "prod-dnigbtea24ste",
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
import com.pulumi.aws.servicecatalog.ServicecatalogFunctions;
import com.pulumi.aws.servicecatalog.inputs.GetProductArgs;
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
        final var example = ServicecatalogFunctions.getProduct(GetProductArgs.builder()
            .id("prod-dnigbtea24ste")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicecatalog:getProduct
      arguments:
        id: prod-dnigbtea24ste
```
<!--End PulumiCodeChooser -->
Ñ
acceptLanguageB" lLanguage code. Valid values are `en` (English), `jp` (Japanese), `zh` (Chinese). The default value is `en`.
D
id" :ID of the product.

The following arguments are optional:
-
tagsB2" Tags applied to the product.
"
acceptLanguageB" "
arn" ARN of the product.
"6
createdTime" #Time when the product was created.
"/
description" Description of the product.
"*
distributor" Vendor of the product.
">
hasDefaultPath
 (Whether the product has a default path.
"
id" "!
name" Name of the product.
"#
owner" Owner of the product.
"%
status" Status of the product.
"U
supportDescription" ;Field that provides support information about the product.
"7
supportEmail" #Contact email for product support.
"3

supportUrl" !Contact URL for product support.
"+
tags2" Tags applied to the product.
"
type" Type of product.
2§
p
servicecataloggetProvisioningArtifactsDaws:servicecatalog/getProvisioningArtifacts:getProvisioningArtifactsªLists the provisioning artifacts for the specified product.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicecatalog.getProvisioningArtifacts({
    productId: "prod-yakog5pdriver",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicecatalog.get_provisioning_artifacts(product_id="prod-yakog5pdriver")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceCatalog.GetProvisioningArtifacts.Invoke(new()
    {
        ProductId = "prod-yakog5pdriver",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicecatalog"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicecatalog.GetProvisioningArtifacts(ctx, &servicecatalog.GetProvisioningArtifactsArgs{
			ProductId: "prod-yakog5pdriver",
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
import com.pulumi.aws.servicecatalog.ServicecatalogFunctions;
import com.pulumi.aws.servicecatalog.inputs.GetProvisioningArtifactsArgs;
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
        final var example = ServicecatalogFunctions.getProvisioningArtifacts(GetProvisioningArtifactsArgs.builder()
            .productId("prod-yakog5pdriver")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicecatalog:getProvisioningArtifacts
      arguments:
        productId: prod-yakog5pdriver
```
<!--End PulumiCodeChooser -->
}
acceptLanguageB" eLanguage code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
L
	productId" ;Product identifier.

The following arguments are optional:
"
acceptLanguageB" "E
id" ;The provider-assigned unique ID for this managed resource.
"
	productId" "¥
provisioningArtifactDetails«*ƒ:¡
æ
servicecatalog2getProvisioningArtifactsProvisioningArtifactDetailxaws:servicecatalog/getProvisioningArtifactsProvisioningArtifactDetail:getProvisioningArtifactsProvisioningArtifactDetailKList with information about the provisioning artifacts. See details below.
2‹
Y
servicediscoverygetDnsNamespace4aws:servicediscovery/getDnsNamespace:getDnsNamespaceäRetrieves information about a Service Discovery private or public DNS namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.servicediscovery.getDnsNamespace({
    name: "example.service.local",
    type: "DNS_PRIVATE",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.servicediscovery.get_dns_namespace(name="example.service.local",
    type="DNS_PRIVATE")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.ServiceDiscovery.GetDnsNamespace.Invoke(new()
    {
        Name = "example.service.local",
        Type = "DNS_PRIVATE",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicediscovery.GetDnsNamespace(ctx, &servicediscovery.GetDnsNamespaceArgs{
			Name: "example.service.local",
			Type: "DNS_PRIVATE",
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
import com.pulumi.aws.servicediscovery.ServicediscoveryFunctions;
import com.pulumi.aws.servicediscovery.inputs.GetDnsNamespaceArgs;
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
        final var test = ServicediscoveryFunctions.getDnsNamespace(GetDnsNamespaceArgs.builder()
            .name("example.service.local")
            .type("DNS_PRIVATE")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:servicediscovery:getDnsNamespace
      arguments:
        name: example.service.local
        type: DNS_PRIVATE
```
<!--End PulumiCodeChooser -->
#
name" Name of the namespace.
.
tagsB2" Map of tags for the resource.
U
type" IType of the namespace. Allowed values are `DNS_PUBLIC` or `DNS_PRIVATE`.
"!
arn" ARN of the namespace.
"1
description" Description of the namespace.
"c

hostedZone" QID for the hosted zone that Amazon Route 53 creates when you create a namespace.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" ",
tags2" Map of tags for the resource.
"

type" 2å
\
servicediscoverygetHttpNamespace6aws:servicediscovery/getHttpNamespace:getHttpNamespace¯## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicediscovery.getHttpNamespace({
    name: "development",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicediscovery.get_http_namespace(name="development")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceDiscovery.GetHttpNamespace.Invoke(new()
    {
        Name = "development",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicediscovery.LookupHttpNamespace(ctx, &servicediscovery.LookupHttpNamespaceArgs{
			Name: "development",
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
import com.pulumi.aws.servicediscovery.ServicediscoveryFunctions;
import com.pulumi.aws.servicediscovery.inputs.GetHttpNamespaceArgs;
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
        final var example = ServicediscoveryFunctions.getHttpNamespace(GetHttpNamespaceArgs.builder()
            .name("development")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicediscovery:getHttpNamespace
      arguments:
        name: development
```
<!--End PulumiCodeChooser -->
(
name" Name of the http namespace.
.
tagsB2" Map of tags for the resource.
"Q
arn" FARN that Amazon Route 53 assigns to the namespace when you create it.
"V
description" CDescription that you specify for the namespace when you create it.
"+
httpName" Name of an HTTP namespace.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" ",
tags2" Map of tags for the resource.
2„ 
J
servicediscovery
getService*aws:servicediscovery/getService:getService“Retrieves information about a Service Discovery Service.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.servicediscovery.getService({
    name: "example",
    namespaceId: "NAMESPACE_ID_VALUE",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.servicediscovery.get_service(name="example",
    namespace_id="NAMESPACE_ID_VALUE")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.ServiceDiscovery.GetService.Invoke(new()
    {
        Name = "example",
        NamespaceId = "NAMESPACE_ID_VALUE",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicediscovery"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicediscovery.LookupService(ctx, &servicediscovery.LookupServiceArgs{
			Name:        "example",
			NamespaceId: "NAMESPACE_ID_VALUE",
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
import com.pulumi.aws.servicediscovery.ServicediscoveryFunctions;
import com.pulumi.aws.servicediscovery.inputs.GetServiceArgs;
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
        final var test = ServicediscoveryFunctions.getService(GetServiceArgs.builder()
            .name("example")
            .namespaceId("NAMESPACE_ID_VALUE")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:servicediscovery:getService
      arguments:
        name: example
        namespaceId: NAMESPACE_ID_VALUE
```
<!--End PulumiCodeChooser -->
!
name" Name of the service.
D
namespaceId" 1ID of the namespace that the service belongs to.
Õ
tagsB2" ºMap of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
õ
tagsAllB2" á(**Deprecated**) Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"
arn" ARN of the service.
"/
description" Description of the service.
"≠

dnsConfigsk*i:g
e
servicediscoverygetServiceDnsConfig<aws:servicediscovery/getServiceDnsConfig:getServiceDnsConfig±Complex type that contains information about the resource record sets that you want Amazon Route 53 to create when you register an instance. See `dns_config` Block for details.
"≠
healthCheckConfigsÑ*Å:
}
servicediscoverygetServiceHealthCheckConfigLaws:servicediscovery/getServiceHealthCheckConfig:getServiceHealthCheckConfigèComplex type that contains settings for an optional health check. Only for Public DNS namespaces. See `health_check_config` Block for details.
"∞
healthCheckCustomConfigsò*ï:í
è
servicediscovery!getServiceHealthCheckCustomConfigXaws:servicediscovery/getServiceHealthCheckCustomConfig:getServiceHealthCheckCustomConfigyA complex type that contains settings for ECS managed health checks. See `health_check_custom_config` Block for details.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "E
namespaceId" 2ID of the namespace to use for DNS configuration.
"Õ
tagsB2" ºMap of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"ô
tagsAll2" á(**Deprecated**) Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
2ä
D
servicequotas
getService'aws:servicequotas/getService:getServiceÕRetrieve information about a Service Quotas Service.

> **NOTE:** Global quotas apply to all AWS regions, but can only be accessed in `us-east-1` in the Commercial partition or `us-gov-west-1` in the GovCloud partition. In other regions, the AWS API will return the error `The request failed because the specified service does not exist.`

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicequotas.getService({
    serviceName: "Amazon Virtual Private Cloud (Amazon VPC)",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicequotas.get_service(service_name="Amazon Virtual Private Cloud (Amazon VPC)")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceQuotas.GetService.Invoke(new()
    {
        ServiceName = "Amazon Virtual Private Cloud (Amazon VPC)",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicequotas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicequotas.GetService(ctx, &servicequotas.GetServiceArgs{
			ServiceName: "Amazon Virtual Private Cloud (Amazon VPC)",
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
import com.pulumi.aws.servicequotas.ServicequotasFunctions;
import com.pulumi.aws.servicequotas.inputs.GetServiceArgs;
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
        final var example = ServicequotasFunctions.getService(GetServiceArgs.builder()
            .serviceName("Amazon Virtual Private Cloud (Amazon VPC)")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicequotas:getService
      arguments:
        serviceName: Amazon Virtual Private Cloud (Amazon VPC)
```
<!--End PulumiCodeChooser -->
Ì
serviceName" ŸService name to lookup within Service Quotas. Available values can be found with the [AWS CLI service-quotas list-services command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-services.html).
"E
id" ;The provider-assigned unique ID for this managed resource.
"(
serviceCode" Code of the service.
"
serviceName" 2˜'
S
servicequotasgetServiceQuota1aws:servicequotas/getServiceQuota:getServiceQuota®Retrieve information about a Service Quota.

> **NOTE:** Global quotas apply to all AWS regions, but can only be accessed in `us-east-1` in the Commercial partition or `us-gov-west-1` in the GovCloud partition. In other regions, the AWS API will return the error `The request failed because the specified service does not exist.`

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const byQuotaCode = aws.servicequotas.getServiceQuota({
    quotaCode: "L-F678F1CE",
    serviceCode: "vpc",
});
const byQuotaName = aws.servicequotas.getServiceQuota({
    quotaName: "VPCs per Region",
    serviceCode: "vpc",
});
```
```python
import pulumi
import pulumi_aws as aws

by_quota_code = aws.servicequotas.get_service_quota(quota_code="L-F678F1CE",
    service_code="vpc")
by_quota_name = aws.servicequotas.get_service_quota(quota_name="VPCs per Region",
    service_code="vpc")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var byQuotaCode = Aws.ServiceQuotas.GetServiceQuota.Invoke(new()
    {
        QuotaCode = "L-F678F1CE",
        ServiceCode = "vpc",
    });

    var byQuotaName = Aws.ServiceQuotas.GetServiceQuota.Invoke(new()
    {
        QuotaName = "VPCs per Region",
        ServiceCode = "vpc",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicequotas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicequotas.LookupServiceQuota(ctx, &servicequotas.LookupServiceQuotaArgs{
			QuotaCode:   pulumi.StringRef("L-F678F1CE"),
			ServiceCode: "vpc",
		}, nil)
		if err != nil {
			return err
		}
		_, err = servicequotas.LookupServiceQuota(ctx, &servicequotas.LookupServiceQuotaArgs{
			QuotaName:   pulumi.StringRef("VPCs per Region"),
			ServiceCode: "vpc",
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
import com.pulumi.aws.servicequotas.ServicequotasFunctions;
import com.pulumi.aws.servicequotas.inputs.GetServiceQuotaArgs;
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
        final var byQuotaCode = ServicequotasFunctions.getServiceQuota(GetServiceQuotaArgs.builder()
            .quotaCode("L-F678F1CE")
            .serviceCode("vpc")
            .build());

        final var byQuotaName = ServicequotasFunctions.getServiceQuota(GetServiceQuotaArgs.builder()
            .quotaName("VPCs per Region")
            .serviceCode("vpc")
            .build());

    }
}
```
```yaml
variables:
  byQuotaCode:
    fn::invoke:
      function: aws:servicequotas:getServiceQuota
      arguments:
        quotaCode: L-F678F1CE
        serviceCode: vpc
  byQuotaName:
    fn::invoke:
      function: aws:servicequotas:getServiceQuota
      arguments:
        quotaName: VPCs per Region
        serviceCode: vpc
```
<!--End PulumiCodeChooser -->
Á
	quotaCodeB" ”Quota code within the service. When configured, the data source directly looks up the service quota. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html). One of `quota_code` or `quota_name` must be specified.
á
	quotaNameB" ÛQuota name within the service. When configured, the data source searches through all service quotas to find the matching quota name. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html). One of `quota_name` or `quota_code` must be specified.
â
serviceCode" ıService code for the quota. Available values can be found with the `aws.servicequotas.getService` data source or [AWS CLI service-quotas list-services command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-services.html).
";

adjustable
 )Whether the service quota is adjustable.
"%
arn" ARN of the service quota.
"8
defaultValue $Default value of the service quota.
"L
globalQuota
 9Whether the service quota is global for the AWS account.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
	quotaCode" "
	quotaName" "
serviceCode" "(
serviceName" Name of the service.
"Ø
usageMetricsz*x:v
t
servicequotasgetServiceQuotaUsageMetricGaws:servicequotas/getServiceQuotaUsageMetric:getServiceQuotaUsageMetric#Information about the measurement.
"1
value $Current value of the service quota.
2÷
J
servicequotasgetTemplates+aws:servicequotas/getTemplates:getTemplates¯Data source for managing an AWS Service Quotas Templates.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.servicequotas.getTemplates({
    region: "us-east-1",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.servicequotas.get_templates(region="us-east-1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.ServiceQuotas.GetTemplates.Invoke(new()
    {
        Region = "us-east-1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/servicequotas"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := servicequotas.GetTemplates(ctx, &servicequotas.GetTemplatesArgs{
			Region: "us-east-1",
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
import com.pulumi.aws.servicequotas.ServicequotasFunctions;
import com.pulumi.aws.servicequotas.inputs.GetTemplatesArgs;
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
        final var example = ServicequotasFunctions.getTemplates(GetTemplatesArgs.builder()
            .region("us-east-1")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:servicequotas:getTemplates
      arguments:
        region: us-east-1
```
<!--End PulumiCodeChooser -->
=
region" /AWS Region to which the quota increases apply.
√
	templatesjBh*f:d
b
servicequotasgetTemplatesTemplate;aws:servicequotas/getTemplatesTemplate:getTemplatesTemplateJA list of quota increase templates for specified region. See `templates`.
"
id" "8
region" *AWS Region to which the template applies.
"√
	templatesjBh*f:d
b
servicequotasgetTemplatesTemplate;aws:servicequotas/getTemplatesTemplate:getTemplatesTemplateJA list of quota increase templates for specified region. See `templates`.
2Ü
W
sesgetActiveReceiptRuleSet7aws:ses/getActiveReceiptRuleSet:getActiveReceiptRuleSetíRetrieve the active SES receipt rule set

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = aws.ses.getActiveReceiptRuleSet({});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.ses.get_active_receipt_rule_set()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = Aws.Ses.GetActiveReceiptRuleSet.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.LookupActiveReceiptRuleSet(ctx, map[string]interface{}{}, nil)
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
import com.pulumi.aws.ses.SesFunctions;
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
        final var main = SesFunctions.getActiveReceiptRuleSet();

    }
}
```
```yaml
variables:
  main:
    fn::invoke:
      function: aws:ses:getActiveReceiptRuleSet
      arguments: {}
```
<!--End PulumiCodeChooser -->
"%
arn" SES receipt rule set ARN.
"E
id" ;The provider-assigned unique ID for this managed resource.
"(
ruleSetName" Name of the rule set
2Ã
E
sesgetDomainIdentity+aws:ses/getDomainIdentity:getDomainIdentity†Retrieve the SES domain identity

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ses.getDomainIdentity({
    domain: "example.com",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ses.get_domain_identity(domain="example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Ses.GetDomainIdentity.Invoke(new()
    {
        Domain = "example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.LookupDomainIdentity(ctx, &ses.LookupDomainIdentityArgs{
			Domain: "example.com",
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
import com.pulumi.aws.ses.SesFunctions;
import com.pulumi.aws.ses.inputs.GetDomainIdentityArgs;
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
        final var example = SesFunctions.getDomainIdentity(GetDomainIdentityArgs.builder()
            .domain("example.com")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ses:getDomainIdentity
      arguments:
        domain: example.com
```
<!--End PulumiCodeChooser -->
!
domain" Name of the domain
"'
arn" ARN of the domain identity.
"!
domain" Name of the domain
"E
id" ;The provider-assigned unique ID for this managed resource.
"©
verificationToken" èCode which when added to the domain as a TXT record will signal to SES that the owner of the domain has authorized SES to act on their behalf.
2ø
B
sesgetEmailIdentity)aws:ses/getEmailIdentity:getEmailIdentity«Retrieve the active SES email identity

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ses.getEmailIdentity({
    email: "awesome@example.com",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ses.get_email_identity(email="awesome@example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Ses.GetEmailIdentity.Invoke(new()
    {
        Email = "awesome@example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ses"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ses.LookupEmailIdentity(ctx, &ses.LookupEmailIdentityArgs{
			Email: "awesome@example.com",
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
import com.pulumi.aws.ses.SesFunctions;
import com.pulumi.aws.ses.inputs.GetEmailIdentityArgs;
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
        final var example = SesFunctions.getEmailIdentity(GetEmailIdentityArgs.builder()
            .email("awesome@example.com")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ses:getEmailIdentity
      arguments:
        email: awesome@example.com
```
<!--End PulumiCodeChooser -->

email" Email identity.
"*
arn" The ARN of the email identity.
"
email" Email identity.
"E
id" ;The provider-assigned unique ID for this managed resource.
:ª
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter‹
Ÿ“
name" ≈Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
Å
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:æ
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter‹
Ÿ“
name" ≈Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
Å
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:Ω
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter˘
ˆp
name" dName of the filter field. Valid values can be found in the [describe-regions AWS CLI Reference][1].
Å
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:Ë
l
	schedulerScheduleFlexibleTimeWindowCaws:scheduler/ScheduleFlexibleTimeWindow:ScheduleFlexibleTimeWindow˜
Ù
maximumWindowInMinutesB _Maximum time window during which a schedule can be invoked. Ranges from `1` to `1440` minutes.
q
mode" eDetermines whether the schedule is invoked within a flexible time window. One of: `OFF`, `FLEXIBLE`.
:ø
H
	schedulerScheduleTarget+aws:scheduler/ScheduleTarget:ScheduleTargetÚ
Ôó
arn" ãARN of the target of this schedule, such as a SQS queue or ECS cluster. For universal targets, this is a [Service ARN specific to the target service](https://docs.aws.amazon.com/scheduler/latest/UserGuide/managing-targets-universal.html#supported-universal-targets).
ì
deadLetterConfig~B|:z
x
	schedulerScheduleTargetDeadLetterConfigKaws:scheduler/ScheduleTargetDeadLetterConfig:ScheduleTargetDeadLetterConfig˛Information about an Amazon SQS queue that EventBridge Scheduler uses as a dead-letter queue for your schedule. If specified, EventBridge Scheduler delivers failed events that could not be successfully delivered to a target to the queue. Detailed below.
©
ecsParametersuBs:q
o
	schedulerScheduleTargetEcsParametersEaws:scheduler/ScheduleTargetEcsParameters:ScheduleTargetEcsParameters†Templated target type for the Amazon ECS [`RunTask`](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html) API operation. Detailed below.
‘
eventbridgeParametersêBç:ä
á
	scheduler#ScheduleTargetEventbridgeParametersUaws:scheduler/ScheduleTargetEventbridgeParameters:ScheduleTargetEventbridgeParametersßTemplated target type for the EventBridge [`PutEvents`](https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_PutEvents.html) API operation. Detailed below.
∫
inputB" ™Text, or well-formed JSON, passed to the target. Read more in [Universal target](https://docs.aws.amazon.com/scheduler/latest/UserGuide/managing-targets-universal.html).
¿
kinesisParametersÅB:}
{
	schedulerScheduleTargetKinesisParametersMaws:scheduler/ScheduleTargetKinesisParameters:ScheduleTargetKinesisParameters¶Templated target type for the Amazon Kinesis [`PutRecord`](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecord.html) API operation. Detailed below.
Ω
retryPolicyoBm:k
i
	schedulerScheduleTargetRetryPolicyAaws:scheduler/ScheduleTargetRetryPolicy:ScheduleTargetRetryPolicy=Information about the retry policy settings. Detailed below.
©
roleArn" ôARN of the IAM role that EventBridge Scheduler will use for this target when the schedule is invoked. Read more in [Set up the execution role](https://docs.aws.amazon.com/scheduler/latest/UserGuide/setting-up.html#setting-up-execution-role).

The following arguments are optional:
â
sagemakerPipelineParameters¢Bü:ú
ô
	scheduler)ScheduleTargetSagemakerPipelineParametersaaws:scheduler/ScheduleTargetSagemakerPipelineParameters:ScheduleTargetSagemakerPipelineParametersƒTemplated target type for the Amazon SageMaker [`StartPipelineExecution`](https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_StartPipelineExecution.html) API operation. Detailed below.
¡
sqsParametersuBs:q
o
	schedulerScheduleTargetSqsParametersEaws:scheduler/ScheduleTargetSqsParameters:ScheduleTargetSqsParameters∏The templated target type for the Amazon SQS [`SendMessage`](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SendMessage.html) API operation. Detailed below.
:ÿ
x
	schedulerScheduleTargetDeadLetterConfigKaws:scheduler/ScheduleTargetDeadLetterConfig:ScheduleTargetDeadLetterConfig\
ZX
arn" MARN of the SQS queue specified as the destination for the dead-letter queue.
:º
o
	schedulerScheduleTargetEcsParametersEaws:scheduler/ScheduleTargetEcsParameters:ScheduleTargetEcsParameters»
≈∞
capacityProviderStrategies√B¿*Ω:∫
∑
	scheduler3ScheduleTargetEcsParametersCapacityProviderStrategyuaws:scheduler/ScheduleTargetEcsParametersCapacityProviderStrategy:ScheduleTargetEcsParametersCapacityProviderStrategyLUp to `6` capacity provider strategies to use for the task. Detailed below.
ì
enableEcsManagedTagsB
 ÙSpecifies whether to enable Amazon ECS managed tags for the task. For more information, see [Tagging Your Amazon ECS Resources](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html) in the Amazon ECS Developer Guide.
}
enableExecuteCommandB
 _Specifies whether to enable the execute command functionality for the containers in this task.
Q
groupB" BSpecifies an ECS task group for the task. At most 255 characters.
‚

launchTypeB" ÕSpecifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. One of: `EC2`, `FARGATE`, `EXTERNAL`.
ì
networkConfiguration¥B±:Æ
´
	scheduler/ScheduleTargetEcsParametersNetworkConfigurationmaws:scheduler/ScheduleTargetEcsParametersNetworkConfiguration:ScheduleTargetEcsParametersNetworkConfigurationDConfigures the networking associated with the task. Detailed below.
ú
placementConstraints¥B±*Æ:´
®
	scheduler.ScheduleTargetEcsParametersPlacementConstraintkaws:scheduler/ScheduleTargetEcsParametersPlacementConstraint:ScheduleTargetEcsParametersPlacementConstraintMA set of up to 10 placement constraints to use for the task. Detailed below.
ˇ
placementStrategiesÆB´*®:•
¢
	scheduler,ScheduleTargetEcsParametersPlacementStrategygaws:scheduler/ScheduleTargetEcsParametersPlacementStrategy:ScheduleTargetEcsParametersPlacementStrategy7A set of up to 5 placement strategies. Detailed below.
ë
platformVersionB" xSpecifies the platform version for the task. Specify only the numeric portion of the platform version, such as `1.1.0`.
Ä
propagateTagsB" iSpecifies whether to propagate the tags from the task definition to the task. One of: `TASK_DEFINITION`.
7
referenceIdB" "Reference ID to use for the task.
ˇ
tagsB2" ÓThe metadata that you apply to the task. Each tag consists of a key and an optional value. For more information, see [`RunTask`](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html) in the Amazon ECS API Reference.
U
	taskCountB BThe number of tasks to create. Ranges from `1` (default) to `10`.
c
taskDefinitionArn" JARN of the task definition to use.

The following arguments are optional:
:‡
∑
	scheduler3ScheduleTargetEcsParametersCapacityProviderStrategyuaws:scheduler/ScheduleTargetEcsParametersCapacityProviderStrategy:ScheduleTargetEcsParametersCapacityProviderStrategy£
†”
baseB ƒHow many tasks, at a minimum, to run on the specified capacity provider. Only one capacity provider in a capacity provider strategy can have a base defined. Ranges from `0` (default) to `100000`.
=
capacityProvider" %Short name of the capacity provider.
à
weightB ˜Designates the relative percentage of the total number of tasks launched that should use the specified capacity provider. The weight value is taken into consideration after the base value, if defined, is satisfied. Ranges from from `0` to `1000`.
:–
´
	scheduler/ScheduleTargetEcsParametersNetworkConfigurationmaws:scheduler/ScheduleTargetEcsParametersNetworkConfiguration:ScheduleTargetEcsParametersNetworkConfigurationü
úí
assignPublicIpB
 ˘Specifies whether the task's elastic network interface receives a public IP address. This attribute is a boolean type, where `true` maps to `ENABLED` and `false` to `DISABLED`. You can specify `true` only when the `launch_type` is set to `FARGATE`.
è
securityGroupsB*" uSet of 1 to 5 Security Group ID-s to be associated with the task. These security groups must all be in the same VPC.
s
subnets*" bSet of 1 to 16 subnets to be associated with the task. These subnets must all be in the same VPC.
:’
®
	scheduler.ScheduleTargetEcsParametersPlacementConstraintkaws:scheduler/ScheduleTargetEcsParametersPlacementConstraint:ScheduleTargetEcsParametersPlacementConstraintß
§”

expressionB" æA cluster query language expression to apply to the constraint. You cannot specify an expression if the constraint type is `distinctInstance`. For more information, see [Cluster query language](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html) in the Amazon ECS Developer Guide.
L
type" @The type of constraint. One of: `distinctInstance`, `memberOf`.
:ƒ
¢
	scheduler,ScheduleTargetEcsParametersPlacementStrategygaws:scheduler/ScheduleTargetEcsParametersPlacementStrategy:ScheduleTargetEcsParametersPlacementStrategyú
ôB
fieldB" 3The field to apply the placement strategy against.
S
type" GThe type of placement strategy. One of: `random`, `spread`, `binpack`.
:™
á
	scheduler#ScheduleTargetEventbridgeParametersUaws:scheduler/ScheduleTargetEventbridgeParameters:ScheduleTargetEventbridgeParametersù
ös

detailType" aFree-form string used to decide what fields to expect in the event detail. Up to 128 characters.
#
source" Source of the event.
:Ò
{
	schedulerScheduleTargetKinesisParametersMaws:scheduler/ScheduleTargetKinesisParameters:ScheduleTargetKinesisParametersr
pn
partitionKey" ZSpecifies the shard to which EventBridge Scheduler sends the event. Up to 256 characters.
:ç
i
	schedulerScheduleTargetRetryPolicyAaws:scheduler/ScheduleTargetRetryPolicy:ScheduleTargetRetryPolicyü
úë
maximumEventAgeInSecondsB oMaximum amount of time, in seconds, to continue to make retry attempts. Ranges from `60` to `86400` (default).
Ö
maximumRetryAttemptsB gMaximum number of retry attempts to make before the request fails. Ranges from `0` to `185` (default).
:ê
ô
	scheduler)ScheduleTargetSagemakerPipelineParametersaaws:scheduler/ScheduleTargetSagemakerPipelineParameters:ScheduleTargetSagemakerPipelineParametersÒ
ÓÎ
pipelineParametersŸB÷*”:–
Õ
	scheduler:ScheduleTargetSagemakerPipelineParametersPipelineParameterÉaws:scheduler/ScheduleTargetSagemakerPipelineParametersPipelineParameter:ScheduleTargetSagemakerPipelineParametersPipelineParameterySet of up to 200 parameter names and values to use when executing the SageMaker Model Building Pipeline. Detailed below.
:é
Õ
	scheduler:ScheduleTargetSagemakerPipelineParametersPipelineParameterÉaws:scheduler/ScheduleTargetSagemakerPipelineParametersPipelineParameter:ScheduleTargetSagemakerPipelineParametersPipelineParameterª
∏Y
name" MName of parameter to start execution of a SageMaker Model Building Pipeline.
[
value" NValue of parameter to start execution of a SageMaker Model Building Pipeline.
:ª
o
	schedulerScheduleTargetSqsParametersEaws:scheduler/ScheduleTargetSqsParameters:ScheduleTargetSqsParametersH
FD
messageGroupIdB" ,FIFO message group ID to use as the target.
:µ
O
secretsmanagerSecretReplica.aws:secretsmanager/SecretReplica:SecretReplica·
ﬁñ
kmsKeyIdB" ÉARN, Key ID, or Alias of the AWS KMS key within the region secret is replicated to. If one is not specified, then Secrets Manager defaults to using the AWS account's default KMS key (`aws/secretsmanager`) in the region or creates one for use if non-existent.
P
lastAccessedDateB" 6Date that you last accessed the secret in the Region.
1
region" #Region for replicating the secret.
C
statusB" 3Status can be `InProgress`, `Failed`, or `InSync`.
y
statusMessageB" bMessage such as `Replication succeeded` or `Secret with this name already exists in this region`.
:Ù
y
secretsmanagerSecretRotationRotationRulesJaws:secretsmanager/SecretRotationRotationRules:SecretRotationRotationRulesˆ
Û¡
automaticallyAfterDaysB †Specifies the number of days between automatic scheduled rotations of the secret. Either `automatically_after_days` or `schedule_expression` must be specified.
i
durationB" WThe length of the rotation window in hours. For example, `3h` for a three hour window.
¡
scheduleExpressionB" §A `cron()` or `rate()` expression that defines the schedule for rotating your secret. Either `automatically_after_days` or `schedule_expression` must be specified.
:Õ

secretsmanagergetSecretRotationRotationRuleNaws:secretsmanager/getSecretRotationRotationRule:getSecretRotationRotationRuleJ
H
automaticallyAfterDays 
duration" 
scheduleExpression" :¿
p
secretsmanagergetSecretVersionsVersionDaws:secretsmanager/getSecretVersionsVersion:getSecretVersionsVersionÀ
»
createdTime" P
lastAccessedDate" 8Date that this version of the secret was last accessed.
J
	versionId" 9Unique version identifier of this version of the secret.

versionStages*" :±
X
secretsmanagergetSecretsFilter4aws:secretsmanager/getSecretsFilter:getSecretsFilter‘
— 
name" ΩName of the filter field. Valid values can be found in the [Secrets Manager ListSecrets API Reference](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_ListSecrets.html).
Å
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:∆
^
securityhubAutomationRuleAction9aws:securityhub/AutomationRuleAction:AutomationRuleAction„
‡•
findingFieldsUpdate†Bù:ö
ó
securityhub'AutomationRuleActionFindingFieldsUpdate_aws:securityhub/AutomationRuleActionFindingFieldsUpdate:AutomationRuleActionFindingFieldsUpdatekA block that specifies that the automation rule action is an update to a finding field.  Documented below.
µ
typeB" ¶Specifies that the rule action should update the `Types` finding field. The `Types` finding field classifies findings in the format of namespace/category/classifier.
:Â
ó
securityhub'AutomationRuleActionFindingFieldsUpdate_aws:securityhub/AutomationRuleActionFindingFieldsUpdate:AutomationRuleActionFindingFieldsUpdate»
≈Q

confidenceB =The rule action updates the `Confidence` field of a finding.
S
criticalityB >The rule action updates the `Criticality` field of a finding.
Ò
note¨B©:¶
£
securityhub+AutomationRuleActionFindingFieldsUpdateNotegaws:securityhub/AutomationRuleActionFindingFieldsUpdateNote:AutomationRuleActionFindingFieldsUpdateNote:A resource block that updates the note. Documented below.
Õ
relatedFindingsÕB *«:ƒ
¡
securityhub5AutomationRuleActionFindingFieldsUpdateRelatedFinding{aws:securityhub/AutomationRuleActionFindingFieldsUpdateRelatedFinding:AutomationRuleActionFindingFieldsUpdateRelatedFindingjA resource block that the rule action updates the `RelatedFindings` field of a finding. Documented below.
¢
severity∏Bµ:≤
Ø
securityhub/AutomationRuleActionFindingFieldsUpdateSeverityoaws:securityhub/AutomationRuleActionFindingFieldsUpdateSeverity:AutomationRuleActionFindingFieldsUpdateSeverity[A resource block that updates to the severity information for a finding. Documented below.
I
typesB*" 8The rule action updates the `Types` field of a finding.
a
userDefinedFieldsB2" DThe rule action updates the `UserDefinedFields` field of a finding.
…
verificationStateB" ≠The rule action updates the `VerificationState` field of a finding. The allowed values are the following `UNKNOWN`, `TRUE_POSITIVE`, `FALSE_POSITIVE` and `BENIGN_POSITIVE`.
∑
workflow∏Bµ:≤
Ø
securityhub/AutomationRuleActionFindingFieldsUpdateWorkflowoaws:securityhub/AutomationRuleActionFindingFieldsUpdateWorkflow:AutomationRuleActionFindingFieldsUpdateWorkflowpA resource block that is used to update information about the investigation into the finding. Documented below.
:á
£
securityhub+AutomationRuleActionFindingFieldsUpdateNotegaws:securityhub/AutomationRuleActionFindingFieldsUpdateNote:AutomationRuleActionFindingFieldsUpdateNote_
]#
text" The updated note text.
6
	updatedBy" %The principal that updated the note.
:€
¡
securityhub5AutomationRuleActionFindingFieldsUpdateRelatedFinding{aws:securityhub/AutomationRuleActionFindingFieldsUpdateRelatedFinding:AutomationRuleActionFindingFieldsUpdateRelatedFindingî
ëB
id" 8The product-generated identifier for a related finding.
K

productArn" 9The ARN of the product that generated a related finding.
:À
Ø
securityhub/AutomationRuleActionFindingFieldsUpdateSeverityoaws:securityhub/AutomationRuleActionFindingFieldsUpdateSeverity:AutomationRuleActionFindingFieldsUpdateSeverityñ
ìë
labelB" ÅThe severity value of the finding. The allowed values are the following `INFORMATIONAL`, `LOW`, `MEDIUM`, `HIGH` and `CRITICAL`.
}
productB lThe native severity as defined by the AWS service or integrated partner product that generated the finding.
:”
Ø
securityhub/AutomationRuleActionFindingFieldsUpdateWorkflowoaws:securityhub/AutomationRuleActionFindingFieldsUpdateWorkflow:AutomationRuleActionFindingFieldsUpdateWorkflowû
õò
statusB" áThe status of the investigation into the finding. The allowed values are the following `NEW`, `NOTIFIED`, `RESOLVED` and `SUPPRESSED`.
:øT
d
securityhubAutomationRuleCriteria=aws:securityhub/AutomationRuleCriteria:AutomationRuleCriteria÷S
”SÔ
awsAccountIdsîBë*é:ã
à
securityhub"AutomationRuleCriteriaAwsAccountIdUaws:securityhub/AutomationRuleCriteriaAwsAccountId:AutomationRuleCriteriaAwsAccountIdGThe AWS account ID in which a finding was generated. Documented below.
Ä
awsAccountNamesöBó*î:ë
é
securityhub$AutomationRuleCriteriaAwsAccountNameYaws:securityhub/AutomationRuleCriteriaAwsAccountName:AutomationRuleCriteriaAwsAccountNamePThe name of the AWS account in which a finding was generated. Documented below.
´
companyNamesëBé*ã:à
Ö
securityhub!AutomationRuleCriteriaCompanyNameSaws:securityhub/AutomationRuleCriteriaCompanyName:AutomationRuleCriteriaCompanyNameÜThe name of the company for the product that generated the finding. For control-based findings, the company is AWS. Documented below.
…
 complianceAssociatedStandardsIdsÕB *«:ƒ
¡
securityhub5AutomationRuleCriteriaComplianceAssociatedStandardsId{aws:securityhub/AutomationRuleCriteriaComplianceAssociatedStandardsId:AutomationRuleCriteriaComplianceAssociatedStandardsIdUThe unique identifier of a standard in which a control is enabled. Documented below.
Ê
complianceSecurityControlIds¡Bæ*ª:∏
µ
securityhub1AutomationRuleCriteriaComplianceSecurityControlIdsaws:securityhub/AutomationRuleCriteriaComplianceSecurityControlId:AutomationRuleCriteriaComplianceSecurityControlIdÅThe security control ID for which a finding was generated. Security control IDs are the same across standards. Documented below.
©
complianceStatuses†Bù*ö:ó
î
securityhub&AutomationRuleCriteriaComplianceStatus]aws:securityhub/AutomationRuleCriteriaComplianceStatus:AutomationRuleCriteriaComplianceStatuspThe result of a security check. This field is only used for findings generated from controls. Documented below.
º
confidenceséBã*à:Ö
Ç
securityhub AutomationRuleCriteriaConfidenceQaws:securityhub/AutomationRuleCriteriaConfidence:AutomationRuleCriteriaConfidenceõThe likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. `Confidence` is scored on a 0‚Äì100 basis using a ratio scale. A value of `0` means 0 percent confidence, and a value of `100` means 100 percent confidence. Documented below.
Ó

createdAtsäBá*Ñ:Å

securityhubAutomationRuleCriteriaCreatedAtOaws:securityhub/AutomationRuleCriteriaCreatedAt:AutomationRuleCriteriaCreatedAtSA timestamp that indicates when this finding record was created. Documented below.
ï
criticalitiesëBé*ã:à
Ö
securityhub!AutomationRuleCriteriaCriticalitySaws:securityhub/AutomationRuleCriteriaCriticality:AutomationRuleCriteriaCriticalitypThe level of importance that is assigned to the resources that are associated with a finding. Documented below.
œ
descriptionsëBé*ã:à
Ö
securityhub!AutomationRuleCriteriaDescriptionSaws:securityhub/AutomationRuleCriteriaDescription:AutomationRuleCriteriaDescription+A finding's description. Documented below.
œ
firstObservedAtsùBö*ó:î
ë
securityhub%AutomationRuleCriteriaFirstObservedAt[aws:securityhub/AutomationRuleCriteriaFirstObservedAt:AutomationRuleCriteriaFirstObservedAtöA timestamp that indicates when the potential security issue captured by a finding was first observed by the security findings product. Documented below.
É
generatorIdsëBé*ã:à
Ö
securityhub!AutomationRuleCriteriaGeneratorIdSaws:securityhub/AutomationRuleCriteriaGeneratorId:AutomationRuleCriteriaGeneratorId_The identifier for the solution-specific component that generated a finding. Documented below.
º
idsrBp*n:l
j
securityhubAutomationRuleCriteriaIdAaws:securityhub/AutomationRuleCriteriaId:AutomationRuleCriteriaIdAThe product-specific identifier for a finding. Documented below.
”
lastObservedAtsöBó*î:ë
é
securityhub$AutomationRuleCriteriaLastObservedAtYaws:securityhub/AutomationRuleCriteriaLastObservedAt:AutomationRuleCriteriaLastObservedAt¢A timestamp that indicates when the potential security issue captured by a finding was most recently observed by the security findings product. Documented below.
„
	noteTextsÜBÉ*Ä:~
|
securityhubAutomationRuleCriteriaNoteTextMaws:securityhub/AutomationRuleCriteriaNoteText:AutomationRuleCriteriaNoteTextMThe text of a user-defined note that's added to a finding. Documented below.
Í
noteUpdatedAtsóBî*ë:é
ã
securityhub#AutomationRuleCriteriaNoteUpdatedAtWaws:securityhub/AutomationRuleCriteriaNoteUpdatedAt:AutomationRuleCriteriaNoteUpdatedAt>The timestamp of when the note was updated. Documented below.
‚
noteUpdatedBiesóBî*ë:é
ã
securityhub#AutomationRuleCriteriaNoteUpdatedByWaws:securityhub/AutomationRuleCriteriaNoteUpdatedBy:AutomationRuleCriteriaNoteUpdatedBy5The principal that created a note. Documented below.
ï
productArnséBã*à:Ö
Ç
securityhub AutomationRuleCriteriaProductArnQaws:securityhub/AutomationRuleCriteriaProductArn:AutomationRuleCriteriaProductArnuThe Amazon Resource Name (ARN) for a third-party product that generated a finding in Security Hub. Documented below.
≤
productNamesëBé*ã:à
Ö
securityhub!AutomationRuleCriteriaProductNameSaws:securityhub/AutomationRuleCriteriaProductName:AutomationRuleCriteriaProductNameçProvides the name of the product that generated the finding. For control-based findings, the product name is Security Hub. Documented below.
ﬂ
recordStatesëBé*ã:à
Ö
securityhub!AutomationRuleCriteriaRecordStateSaws:securityhub/AutomationRuleCriteriaRecordState:AutomationRuleCriteriaRecordState;Provides the current state of a finding. Documented below.
á
relatedFindingsIds£B†*ù:ö
ó
securityhub'AutomationRuleCriteriaRelatedFindingsId_aws:securityhub/AutomationRuleCriteriaRelatedFindingsId:AutomationRuleCriteriaRelatedFindingsIdKThe product-generated identifier for a related finding.  Documented below.
®
relatedFindingsProductArnsªB∏*µ:≤
Ø
securityhub/AutomationRuleCriteriaRelatedFindingsProductArnoaws:securityhub/AutomationRuleCriteriaRelatedFindingsProductArn:AutomationRuleCriteriaRelatedFindingsProductArnLThe ARN for the product that generated a related finding. Documented below.
≤
resourceApplicationArns≤BØ*¨:©
¶
securityhub,AutomationRuleCriteriaResourceApplicationArniaws:securityhub/AutomationRuleCriteriaResourceApplicationArn:AutomationRuleCriteriaResourceApplicationArnbThe Amazon Resource Name (ARN) of the application that is related to a finding. Documented below.
†
resourceApplicationNamesµB≤*Ø:¨
©
securityhub-AutomationRuleCriteriaResourceApplicationNamekaws:securityhub/AutomationRuleCriteriaResourceApplicationName:AutomationRuleCriteriaResourceApplicationNameLThe name of the application that is related to a finding. Documented below.
¢
resourceDetailsOthers¨B©*¶:£
†
securityhub*AutomationRuleCriteriaResourceDetailsOthereaws:securityhub/AutomationRuleCriteriaResourceDetailsOther:AutomationRuleCriteriaResourceDetailsOtherZCustom fields and values about the resource that a finding pertains to. Documented below.
Ç
resourceIdséBã*à:Ö
Ç
securityhub AutomationRuleCriteriaResourceIdQaws:securityhub/AutomationRuleCriteriaResourceId:AutomationRuleCriteriaResourceId·The identifier for the given resource type. For AWS resources that are identified by Amazon Resource Names (ARNs), this is the ARN. For AWS resources that lack ARNs, this is the identifier as defined by the AWS service that created the resource. For non-AWS resources, this is a unique identifier that is associated with the resource. Documented below.

resourcePartitions£B†*ù:ö
ó
securityhub'AutomationRuleCriteriaResourcePartition_aws:securityhub/AutomationRuleCriteriaResourcePartition:AutomationRuleCriteriaResourcePartition≥The partition in which the resource that the finding pertains to is located. A partition is a group of AWS Regions. Each AWS account is scoped to one partition. Documented below.
ã
resourceRegionsöBó*î:ë
é
securityhub$AutomationRuleCriteriaResourceRegionYaws:securityhub/AutomationRuleCriteriaResourceRegion:AutomationRuleCriteriaResourceRegion[The AWS Region where the resource that a finding pertains to is located. Documented below.
ã
resourceTagsëBé*ã:à
Ö
securityhub!AutomationRuleCriteriaResourceTagSaws:securityhub/AutomationRuleCriteriaResourceTag:AutomationRuleCriteriaResourceTaggA list of AWS tags associated with a resource at the time the finding was processed. Documented below.
Ì
resourceTypesîBë*é:ã
à
securityhub"AutomationRuleCriteriaResourceTypeUaws:securityhub/AutomationRuleCriteriaResourceType:AutomationRuleCriteriaResourceTypeEThe type of resource that the finding pertains to. Documented below.
·
severityLabelsóBî*ë:é
ã
securityhub#AutomationRuleCriteriaSeverityLabelWaws:securityhub/AutomationRuleCriteriaSeverityLabel:AutomationRuleCriteriaSeverityLabel5The severity value of the finding. Documented below.
É

sourceUrlsäBá*Ñ:Å

securityhubAutomationRuleCriteriaSourceUrlOaws:securityhub/AutomationRuleCriteriaSourceUrl:AutomationRuleCriteriaSourceUrlhProvides a URL that links to a page about the current finding in the finding product. Documented below.
¨
titles{By*w:u
s
securityhubAutomationRuleCriteriaTitleGaws:securityhub/AutomationRuleCriteriaTitle:AutomationRuleCriteriaTitle%A finding's title. Documented below.
˜
typesxBv*t:r
p
securityhubAutomationRuleCriteriaTypeEaws:securityhub/AutomationRuleCriteriaType:AutomationRuleCriteriaTypetOne or more finding types in the format of namespace/category/classifier that classify a finding. Documented below.
˚

updatedAtsäBá*Ñ:Å

securityhubAutomationRuleCriteriaUpdatedAtOaws:securityhub/AutomationRuleCriteriaUpdatedAt:AutomationRuleCriteriaUpdatedAt`A timestamp that indicates when the finding record was most recently updated. Documented below.
ë
userDefinedFields†Bù*ö:ó
î
securityhub&AutomationRuleCriteriaUserDefinedField]aws:securityhub/AutomationRuleCriteriaUserDefinedField:AutomationRuleCriteriaUserDefinedFieldYA list of user-defined name and value string pairs added to a finding. Documented below.
Ú
verificationStates£B†*ù:ö
ó
securityhub'AutomationRuleCriteriaVerificationState_aws:securityhub/AutomationRuleCriteriaVerificationState:AutomationRuleCriteriaVerificationState6Provides the veracity of a finding. Documented below.
é
workflowStatusesöBó*î:ë
é
securityhub$AutomationRuleCriteriaWorkflowStatusYaws:securityhub/AutomationRuleCriteriaWorkflowStatus:AutomationRuleCriteriaWorkflowStatus]Provides information about the status of the investigation into a finding. Documented below.
:Æ
à
securityhub"AutomationRuleCriteriaAwsAccountIdUaws:securityhub/AutomationRuleCriteriaAwsAccountId:AutomationRuleCriteriaAwsAccountId!


comparison" 
value" :¥
é
securityhub$AutomationRuleCriteriaAwsAccountNameYaws:securityhub/AutomationRuleCriteriaAwsAccountName:AutomationRuleCriteriaAwsAccountName!


comparison" 
value" :´
Ö
securityhub!AutomationRuleCriteriaCompanyNameSaws:securityhub/AutomationRuleCriteriaCompanyName:AutomationRuleCriteriaCompanyName!


comparison" 
value" :Á
¡
securityhub5AutomationRuleCriteriaComplianceAssociatedStandardsId{aws:securityhub/AutomationRuleCriteriaComplianceAssociatedStandardsId:AutomationRuleCriteriaComplianceAssociatedStandardsId!


comparison" 
value" :€
µ
securityhub1AutomationRuleCriteriaComplianceSecurityControlIdsaws:securityhub/AutomationRuleCriteriaComplianceSecurityControlId:AutomationRuleCriteriaComplianceSecurityControlId!


comparison" 
value" :∫
î
securityhub&AutomationRuleCriteriaComplianceStatus]aws:securityhub/AutomationRuleCriteriaComplianceStatus:AutomationRuleCriteriaComplianceStatus!


comparison" 
value" :ú
Ç
securityhub AutomationRuleCriteriaConfidenceQaws:securityhub/AutomationRuleCriteriaConfidence:AutomationRuleCriteriaConfidenceî
ëu
eqB iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.


gtB Ä
gteB sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.


ltB }
lteB pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:Ú

securityhubAutomationRuleCriteriaCreatedAtOaws:securityhub/AutomationRuleCriteriaCreatedAt:AutomationRuleCriteriaCreatedAtÓ
Îó
	dateRange£B†:ù
ö
securityhub(AutomationRuleCriteriaCreatedAtDateRangeaaws:securityhub/AutomationRuleCriteriaCreatedAtDateRange:AutomationRuleCriteriaCreatedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:Ω
ö
securityhub(AutomationRuleCriteriaCreatedAtDateRangeaaws:securityhub/AutomationRuleCriteriaCreatedAtDateRange:AutomationRuleCriteriaCreatedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:ü
Ö
securityhub!AutomationRuleCriteriaCriticalitySaws:securityhub/AutomationRuleCriteriaCriticality:AutomationRuleCriteriaCriticalityî
ëu
eqB iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.


gtB Ä
gteB sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.


ltB }
lteB pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:´
Ö
securityhub!AutomationRuleCriteriaDescriptionSaws:securityhub/AutomationRuleCriteriaDescription:AutomationRuleCriteriaDescription!


comparison" 
value" :ó
ë
securityhub%AutomationRuleCriteriaFirstObservedAt[aws:securityhub/AutomationRuleCriteriaFirstObservedAt:AutomationRuleCriteriaFirstObservedAtÄ
˝©
	dateRangeµB≤:Ø
¨
securityhub.AutomationRuleCriteriaFirstObservedAtDateRangemaws:securityhub/AutomationRuleCriteriaFirstObservedAtDateRange:AutomationRuleCriteriaFirstObservedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:œ
¨
securityhub.AutomationRuleCriteriaFirstObservedAtDateRangemaws:securityhub/AutomationRuleCriteriaFirstObservedAtDateRange:AutomationRuleCriteriaFirstObservedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:´
Ö
securityhub!AutomationRuleCriteriaGeneratorIdSaws:securityhub/AutomationRuleCriteriaGeneratorId:AutomationRuleCriteriaGeneratorId!


comparison" 
value" :è
j
securityhubAutomationRuleCriteriaIdAaws:securityhub/AutomationRuleCriteriaId:AutomationRuleCriteriaId!


comparison" 
value" :ë
é
securityhub$AutomationRuleCriteriaLastObservedAtYaws:securityhub/AutomationRuleCriteriaLastObservedAt:AutomationRuleCriteriaLastObservedAt˝
˙¶
	dateRange≤BØ:¨
©
securityhub-AutomationRuleCriteriaLastObservedAtDateRangekaws:securityhub/AutomationRuleCriteriaLastObservedAtDateRange:AutomationRuleCriteriaLastObservedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:Ã
©
securityhub-AutomationRuleCriteriaLastObservedAtDateRangekaws:securityhub/AutomationRuleCriteriaLastObservedAtDateRange:AutomationRuleCriteriaLastObservedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:°
|
securityhubAutomationRuleCriteriaNoteTextMaws:securityhub/AutomationRuleCriteriaNoteText:AutomationRuleCriteriaNoteText!


comparison" 
value" :ã
ã
securityhub#AutomationRuleCriteriaNoteUpdatedAtWaws:securityhub/AutomationRuleCriteriaNoteUpdatedAt:AutomationRuleCriteriaNoteUpdatedAt˙
˜£
	dateRangeØB¨:©
¶
securityhub,AutomationRuleCriteriaNoteUpdatedAtDateRangeiaws:securityhub/AutomationRuleCriteriaNoteUpdatedAtDateRange:AutomationRuleCriteriaNoteUpdatedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:…
¶
securityhub,AutomationRuleCriteriaNoteUpdatedAtDateRangeiaws:securityhub/AutomationRuleCriteriaNoteUpdatedAtDateRange:AutomationRuleCriteriaNoteUpdatedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:±
ã
securityhub#AutomationRuleCriteriaNoteUpdatedByWaws:securityhub/AutomationRuleCriteriaNoteUpdatedBy:AutomationRuleCriteriaNoteUpdatedBy!


comparison" 
value" :®
Ç
securityhub AutomationRuleCriteriaProductArnQaws:securityhub/AutomationRuleCriteriaProductArn:AutomationRuleCriteriaProductArn!


comparison" 
value" :´
Ö
securityhub!AutomationRuleCriteriaProductNameSaws:securityhub/AutomationRuleCriteriaProductName:AutomationRuleCriteriaProductName!


comparison" 
value" :´
Ö
securityhub!AutomationRuleCriteriaRecordStateSaws:securityhub/AutomationRuleCriteriaRecordState:AutomationRuleCriteriaRecordState!


comparison" 
value" :Ω
ó
securityhub'AutomationRuleCriteriaRelatedFindingsId_aws:securityhub/AutomationRuleCriteriaRelatedFindingsId:AutomationRuleCriteriaRelatedFindingsId!


comparison" 
value" :’
Ø
securityhub/AutomationRuleCriteriaRelatedFindingsProductArnoaws:securityhub/AutomationRuleCriteriaRelatedFindingsProductArn:AutomationRuleCriteriaRelatedFindingsProductArn!


comparison" 
value" :Ã
¶
securityhub,AutomationRuleCriteriaResourceApplicationArniaws:securityhub/AutomationRuleCriteriaResourceApplicationArn:AutomationRuleCriteriaResourceApplicationArn!


comparison" 
value" :œ
©
securityhub-AutomationRuleCriteriaResourceApplicationNamekaws:securityhub/AutomationRuleCriteriaResourceApplicationName:AutomationRuleCriteriaResourceApplicationName!


comparison" 
value" :Ó
†
securityhub*AutomationRuleCriteriaResourceDetailsOthereaws:securityhub/AutomationRuleCriteriaResourceDetailsOther:AutomationRuleCriteriaResourceDetailsOtherI
G

comparison" &
key" The key of the map filter.

value" :®
Ç
securityhub AutomationRuleCriteriaResourceIdQaws:securityhub/AutomationRuleCriteriaResourceId:AutomationRuleCriteriaResourceId!


comparison" 
value" :Ω
ó
securityhub'AutomationRuleCriteriaResourcePartition_aws:securityhub/AutomationRuleCriteriaResourcePartition:AutomationRuleCriteriaResourcePartition!


comparison" 
value" :¥
é
securityhub$AutomationRuleCriteriaResourceRegionYaws:securityhub/AutomationRuleCriteriaResourceRegion:AutomationRuleCriteriaResourceRegion!


comparison" 
value" :”
Ö
securityhub!AutomationRuleCriteriaResourceTagSaws:securityhub/AutomationRuleCriteriaResourceTag:AutomationRuleCriteriaResourceTagI
G

comparison" &
key" The key of the map filter.

value" :Æ
à
securityhub"AutomationRuleCriteriaResourceTypeUaws:securityhub/AutomationRuleCriteriaResourceType:AutomationRuleCriteriaResourceType!


comparison" 
value" :±
ã
securityhub#AutomationRuleCriteriaSeverityLabelWaws:securityhub/AutomationRuleCriteriaSeverityLabel:AutomationRuleCriteriaSeverityLabel!


comparison" 
value" :§

securityhubAutomationRuleCriteriaSourceUrlOaws:securityhub/AutomationRuleCriteriaSourceUrl:AutomationRuleCriteriaSourceUrl!


comparison" 
value" :ò
s
securityhubAutomationRuleCriteriaTitleGaws:securityhub/AutomationRuleCriteriaTitle:AutomationRuleCriteriaTitle!


comparison" 
value" :ï
p
securityhubAutomationRuleCriteriaTypeEaws:securityhub/AutomationRuleCriteriaType:AutomationRuleCriteriaType!


comparison" 
value" :Ú

securityhubAutomationRuleCriteriaUpdatedAtOaws:securityhub/AutomationRuleCriteriaUpdatedAt:AutomationRuleCriteriaUpdatedAtÓ
Îó
	dateRange£B†:ù
ö
securityhub(AutomationRuleCriteriaUpdatedAtDateRangeaaws:securityhub/AutomationRuleCriteriaUpdatedAtDateRange:AutomationRuleCriteriaUpdatedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:Ω
ö
securityhub(AutomationRuleCriteriaUpdatedAtDateRangeaaws:securityhub/AutomationRuleCriteriaUpdatedAtDateRange:AutomationRuleCriteriaUpdatedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:‚
î
securityhub&AutomationRuleCriteriaUserDefinedField]aws:securityhub/AutomationRuleCriteriaUserDefinedField:AutomationRuleCriteriaUserDefinedFieldI
G

comparison" &
key" The key of the map filter.

value" :Ω
ó
securityhub'AutomationRuleCriteriaVerificationState_aws:securityhub/AutomationRuleCriteriaVerificationState:AutomationRuleCriteriaVerificationState!


comparison" 
value" :¥
é
securityhub$AutomationRuleCriteriaWorkflowStatusYaws:securityhub/AutomationRuleCriteriaWorkflowStatus:AutomationRuleCriteriaWorkflowStatus!


comparison" 
value" :ƒ
î
securityhub&ConfigurationPolicyConfigurationPolicy]aws:securityhub/ConfigurationPolicyConfigurationPolicy:ConfigurationPolicyConfigurationPolicy™
ßÆ
enabledStandardArnsB*" éA list that defines which security standards are enabled in the configuration policy. It must be defined if `service_enabled` is set to true.
¢
securityControlsConfigurationıBÚ:Ô
Ï
securityhubCConfigurationPolicyConfigurationPolicySecurityControlsConfigurationóaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationàDefines which security controls are enabled in the configuration policy and any customizations to parameters affecting them. See below.
O
serviceEnabled
 9Indicates whether Security Hub is enabled in the policy.
:∑

Ï
securityhubCConfigurationPolicyConfigurationPolicySecurityControlsConfigurationóaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration:ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration≈
¬ã
disabledControlIdentifiersB*" ‰A list of security controls that are disabled in the configuration policy Security Hub enables all other controls (including newly released controls) other than the listed controls. Conflicts with `enabled_control_identifiers`.
å
enabledControlIdentifiersB*" ÊA list of security controls that are enabled in the configuration policy. Security Hub disables all other controls (including newly released controls) other than the listed controls. Conflicts with `disabled_control_identifiers`.
¢
securityControlCustomParameters“Bœ*Ã:…
∆
securityhubaConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter”aws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter©A list of control parameter customizations that are included in a configuration policy. Include multiple blocks to define multiple control custom parameters. See below.
:µ
∆
securityhubaConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter”aws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterÈ
ÊŸ

parametersÍ*Á:‰
·
securityhubjConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterÂaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter^An object that specifies parameter values for a control in a configuration policy. See below.
á
securityControlId" nThe ID of the security control. For more information see the [Security Hub controls reference] documentation.
:∂"
·
securityhubjConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterÂaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterœ
Ã∆
boolˆBÛ:
Ì
securityhubnConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBoolÌaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBool:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBoolEThe bool `value` for a Boolean-typed Security Hub Control Parameter.
Œ
double¸B˘:ˆ
Û
securityhubpConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterDoubleÒaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterDouble:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterDoubleEThe float `value` for a Double-typed Security Hub Control Parameter.
≈
enumˆBÛ:
Ì
securityhubnConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumÌaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnum:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumDThe string `value` for a Enum-typed Security Hub Control Parameter.
ﬁ
enumListÇBˇ:¸
˘
securityhubrConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumListıaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumList:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumListMThe string list `value` for a EnumList-typed Security Hub Control Parameter.
Ω
intÛB:Ì
Í
securityhubmConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntÎaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterInt:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterInt@The int `value` for a Int-typed Security Hub Control Parameter.
÷
intListˇB¸:˘
ˆ
securityhubqConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntListÛaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntList:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntListIThe int list `value` for a IntList-typed Security Hub Control Parameter.
}
name" qThe name of the control parameter. For more information see the [Security Hub controls reference] documentation.
œ
string¸B˘:ˆ
Û
securityhubpConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringÒaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterString:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringFThe string `value` for a String-typed Security Hub Control Parameter.
Ë

stringListàBÖ:Ç
ˇ
securityhubtConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringList˘aws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringList:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringListOThe string list `value` for a StringList-typed Security Hub Control Parameter.
±
	valueType" üIdentifies whether a control parameter uses a custom user-defined value or subscribes to the default Security Hub behavior. Valid values: `DEFAULT`, `CUSTOM`.
:Å
Ì
securityhubnConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBoolÌaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBool:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBool

value
 :á
Û
securityhubpConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterDoubleÒaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterDouble:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterDouble

value :Å
Ì
securityhubnConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumÌaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnum:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnum

value" :ê
˘
securityhubrConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumListıaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumList:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnumList

values*" :˛
Í
securityhubmConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntÎaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterInt:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterInt

value :ç
ˆ
securityhubqConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntListÛaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntList:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterIntList

values* :á
Û
securityhubpConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringÒaws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterString:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterString

value" :ñ
ˇ
securityhubtConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringList˘aws:securityhub/ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringList:ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterStringList

values*" :Ê 
L
securityhubInsightFilters-aws:securityhub/InsightFilters:InsightFiltersî 
ê ‰
awsAccountIdsxBv*t:r
p
securityhubInsightFiltersAwsAccountIdEaws:securityhub/InsightFiltersAwsAccountId:InsightFiltersAwsAccountIdYAWS account ID that a finding is generated in. See String_Filter below for more details.
ò
companyNamesuBs*q:o
m
securityhubInsightFiltersCompanyNameCaws:securityhub/InsightFiltersCompanyName:InsightFiltersCompanyNameêThe name of the findings provider (company) that owns the solution (product) that generates findings. See String_Filter below for more details.
í
complianceStatusesÜBÉ*Ä:~
|
securityhubInsightFiltersComplianceStatusMaws:securityhub/InsightFiltersComplianceStatus:InsightFiltersComplianceStatusÚExclusive to findings that are generated as the result of a check run against a specific rule in a supported standard, such as CIS AWS Foundations. Contains security standard-related finding details. See String Filter below for more details.
“
confidencesrBp*n:l
j
securityhubInsightFiltersConfidenceAaws:securityhub/InsightFiltersConfidence:InsightFiltersConfidenceŒA finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence. See Number Filter below for more details.
π

createdAtsoBm*k:i
g
securityhubInsightFiltersCreatedAt?aws:securityhub/InsightFiltersCreatedAt:InsightFiltersCreatedAtπAn ISO8601-formatted timestamp that indicates when the security-findings provider captured the potential security issue that a finding captured. See Date Filter below for more details.
à
criticalitiesuBs*q:o
m
securityhubInsightFiltersCriticalityCaws:securityhub/InsightFiltersCriticality:InsightFiltersCriticalityˇThe level of importance assigned to the resources associated with the finding. A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources. See Number Filter below for more details.
 
descriptionsuBs*q:o
m
securityhubInsightFiltersDescriptionCaws:securityhub/InsightFiltersDescription:InsightFiltersDescriptionCA finding's description. See String Filter below for more details.
 
 findingProviderFieldsConfidencesµB≤*Ø:¨
©
securityhub-InsightFiltersFindingProviderFieldsConfidencekaws:securityhub/InsightFiltersFindingProviderFieldsConfidence:InsightFiltersFindingProviderFieldsConfidenceÌThe finding provider value for the finding confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence. See Number Filter below for more details.
Å
"findingProviderFieldsCriticalities∏Bµ*≤:Ø
¨
securityhub.InsightFiltersFindingProviderFieldsCriticalitymaws:securityhub/InsightFiltersFindingProviderFieldsCriticality:InsightFiltersFindingProviderFieldsCriticalityüThe finding provider value for the level of importance assigned to the resources associated with the findings. A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources. See Number Filter below for more details.
˚
'findingProviderFieldsRelatedFindingsIds B«*ƒ:¡
æ
securityhub4InsightFiltersFindingProviderFieldsRelatedFindingsIdyaws:securityhub/InsightFiltersFindingProviderFieldsRelatedFindingsId:InsightFiltersFindingProviderFieldsRelatedFindingsIdÇThe finding identifier of a related finding that is identified by the finding provider. See String Filter below for more details.
©
/findingProviderFieldsRelatedFindingsProductArns„B‡*›:⁄
◊
securityhub<InsightFiltersFindingProviderFieldsRelatedFindingsProductArnâaws:securityhub/InsightFiltersFindingProviderFieldsRelatedFindingsProductArn:InsightFiltersFindingProviderFieldsRelatedFindingsProductArnèThe ARN of the solution that generated a related finding that is identified by the finding provider. See String Filter below for more details.
≈
#findingProviderFieldsSeverityLabelsæBª*∏:µ
≤
securityhub0InsightFiltersFindingProviderFieldsSeverityLabelqaws:securityhub/InsightFiltersFindingProviderFieldsSeverityLabel:InsightFiltersFindingProviderFieldsSeverityLabel]The finding provider value for the severity label. See String Filter below for more details.
÷
&findingProviderFieldsSeverityOriginals«Bƒ*¡:æ
ª
securityhub3InsightFiltersFindingProviderFieldsSeverityOriginalwaws:securityhub/InsightFiltersFindingProviderFieldsSeverityOriginal:InsightFiltersFindingProviderFieldsSeverityOriginalbThe finding provider's original value for the severity. See String Filter below for more details.
õ
findingProviderFieldsTypes£B†*ù:ö
ó
securityhub'InsightFiltersFindingProviderFieldsType_aws:securityhub/InsightFiltersFindingProviderFieldsType:InsightFiltersFindingProviderFieldsType÷One or more finding types that the finding provider assigned to the finding. Uses the format of `namespace/category/classifier` that classify a finding. Valid namespace values include: `Software and Configuration Checks`, `TTPs`, `Effects`, `Unusual Behaviors`, and `Sensitive Data Identifications`. See String Filter below for more details.
ÿ
firstObservedAtsÅB*}:{
y
securityhubInsightFiltersFirstObservedAtKaws:securityhub/InsightFiltersFirstObservedAt:InsightFiltersFirstObservedAtøAn ISO8601-formatted timestamp that indicates when the security-findings provider first observed the potential security issue that a finding captured. See Date Filter below for more details.
ö
generatorIdsuBs*q:o
m
securityhubInsightFiltersGeneratorIdCaws:securityhub/InsightFiltersGeneratorId:InsightFiltersGeneratorIdíThe identifier for the solution-specific component (a discrete unit of logic) that generated a finding. See String Filter below for more details.
œ
idsZBX*V:T
R
securityhubInsightFiltersId1aws:securityhub/InsightFiltersId:InsightFiltersIdlThe security findings provider-specific identifier for a finding. See String Filter below for more details.
ª
keywordsiBg*e:c
a
securityhubInsightFiltersKeyword;aws:securityhub/InsightFiltersKeyword:InsightFiltersKeywordDA keyword for a finding. See Keyword Filter below for more details.
€
lastObservedAts~B|*z:x
v
securityhubInsightFiltersLastObservedAtIaws:securityhub/InsightFiltersLastObservedAt:InsightFiltersLastObservedAt«An ISO8601-formatted timestamp that indicates when the security-findings provider most recently observed the potential security issue that a finding captured. See Date Filter below for more details.
‹
malwareNamesuBs*q:o
m
securityhubInsightFiltersMalwareNameCaws:securityhub/InsightFiltersMalwareName:InsightFiltersMalwareNameUThe name of the malware that was observed. See String Filter below for more details.
Á
malwarePathsuBs*q:o
m
securityhubInsightFiltersMalwarePathCaws:securityhub/InsightFiltersMalwarePath:InsightFiltersMalwarePath`The filesystem path of the malware that was observed. See String Filter below for more details.
·
malwareStatesxBv*t:r
p
securityhubInsightFiltersMalwareStateEaws:securityhub/InsightFiltersMalwareState:InsightFiltersMalwareStateVThe state of the malware that was observed. See String Filter below for more details.
‹
malwareTypesuBs*q:o
m
securityhubInsightFiltersMalwareTypeCaws:securityhub/InsightFiltersMalwareType:InsightFiltersMalwareTypeUThe type of the malware that was observed. See String Filter below for more details.
±
networkDestinationDomains†Bù*ö:ó
î
securityhub&InsightFiltersNetworkDestinationDomain]aws:securityhub/InsightFiltersNetworkDestinationDomain:InsightFiltersNetworkDestinationDomainqThe destination domain of network-related information about a finding. See String Filter below for more details.
´
networkDestinationIpv4söBó*î:ë
é
securityhub$InsightFiltersNetworkDestinationIpv4Yaws:securityhub/InsightFiltersNetworkDestinationIpv4:InsightFiltersNetworkDestinationIpv4sThe destination IPv4 address of network-related information about a finding. See Ip Filter below for more details.
´
networkDestinationIpv6söBó*î:ë
é
securityhub$InsightFiltersNetworkDestinationIpv6Yaws:securityhub/InsightFiltersNetworkDestinationIpv6:InsightFiltersNetworkDestinationIpv6sThe destination IPv6 address of network-related information about a finding. See Ip Filter below for more details.
ß
networkDestinationPortsöBó*î:ë
é
securityhub$InsightFiltersNetworkDestinationPortYaws:securityhub/InsightFiltersNetworkDestinationPort:InsightFiltersNetworkDestinationPortoThe destination port of network-related information about a finding. See Number Filter below for more details.
é
networkDirectionsÜBÉ*Ä:~
|
securityhubInsightFiltersNetworkDirectionMaws:securityhub/InsightFiltersNetworkDirection:InsightFiltersNetworkDirectionpIndicates the direction of network traffic associated with a finding. See String Filter below for more details.
ˇ
networkProtocolsÅB*}:{
y
securityhubInsightFiltersNetworkProtocolKaws:securityhub/InsightFiltersNetworkProtocol:InsightFiltersNetworkProtocolgThe protocol of network-related information about a finding. See String Filter below for more details.
ò
networkSourceDomainsëBé*ã:à
Ö
securityhub!InsightFiltersNetworkSourceDomainSaws:securityhub/InsightFiltersNetworkSourceDomain:InsightFiltersNetworkSourceDomainlThe source domain of network-related information about a finding. See String Filter below for more details.
ë
networkSourceIpv4säBá*Ñ:Å

securityhubInsightFiltersNetworkSourceIpv4Oaws:securityhub/InsightFiltersNetworkSourceIpv4:InsightFiltersNetworkSourceIpv4nThe source IPv4 address of network-related information about a finding. See Ip Filter below for more details.
ë
networkSourceIpv6säBá*Ñ:Å

securityhubInsightFiltersNetworkSourceIpv6Oaws:securityhub/InsightFiltersNetworkSourceIpv6:InsightFiltersNetworkSourceIpv6nThe source IPv6 address of network-related information about a finding. See Ip Filter below for more details.
ß
networkSourceMacsÜBÉ*Ä:~
|
securityhubInsightFiltersNetworkSourceMacMaws:securityhub/InsightFiltersNetworkSourceMac:InsightFiltersNetworkSourceMacàThe source media access control (MAC) address of network-related information about a finding. See String Filter below for more details.
ç
networkSourcePortsäBá*Ñ:Å

securityhubInsightFiltersNetworkSourcePortOaws:securityhub/InsightFiltersNetworkSourcePort:InsightFiltersNetworkSourcePortjThe source port of network-related information about a finding. See Number Filter below for more details.
π
	noteTextslBj*h:f
d
securityhubInsightFiltersNoteText=aws:securityhub/InsightFiltersNoteText:InsightFiltersNoteText>The text of a note. See String Filter below for more details.
„
noteUpdatedAts{By*w:u
s
securityhubInsightFiltersNoteUpdatedAtGaws:securityhub/InsightFiltersNoteUpdatedAt:InsightFiltersNoteUpdatedAtTThe timestamp of when the note was updated. See Date Filter below for more details.
›
noteUpdatedBies{By*w:u
s
securityhubInsightFiltersNoteUpdatedByGaws:securityhub/InsightFiltersNoteUpdatedBy:InsightFiltersNoteUpdatedByMThe principal that created a note. See String Filter below for more details.
¯
processLaunchedAtsäBá*Ñ:Å

securityhubInsightFiltersProcessLaunchedAtOaws:securityhub/InsightFiltersProcessLaunchedAt:InsightFiltersProcessLaunchedAtUThe date/time that the process was launched. See Date Filter below for more details.
 
processNamesuBs*q:o
m
securityhubInsightFiltersProcessNameCaws:securityhub/InsightFiltersProcessName:InsightFiltersProcessNameCThe name of the process. See String Filter below for more details.
ﬂ
processParentPidsÜBÉ*Ä:~
|
securityhubInsightFiltersProcessParentPidMaws:securityhub/InsightFiltersProcessParentPid:InsightFiltersProcessParentPidAThe parent process ID. See Number Filter below for more details.
’
processPathsuBs*q:o
m
securityhubInsightFiltersProcessPathCaws:securityhub/InsightFiltersProcessPath:InsightFiltersProcessPathNThe path to the process executable. See String Filter below for more details.
Ω
processPidsrBp*n:l
j
securityhubInsightFiltersProcessPidAaws:securityhub/InsightFiltersProcessPid:InsightFiltersProcessPid:The process ID. See Number Filter below for more details.
É
processTerminatedAtsëBé*ã:à
Ö
securityhub!InsightFiltersProcessTerminatedAtSaws:securityhub/InsightFiltersProcessTerminatedAt:InsightFiltersProcessTerminatedAtWThe date/time that the process was terminated. See Date Filter below for more details.
˛
productArnsrBp*n:l
j
securityhubInsightFiltersProductArnAaws:securityhub/InsightFiltersProductArn:InsightFiltersProductArn˙The ARN generated by Security Hub that uniquely identifies a third-party company (security findings provider) after this provider's product (solution that generates findings) is registered with Security Hub. See String Filter below for more details.
œ
productFieldsxBv*t:r
p
securityhubInsightFiltersProductFieldEaws:securityhub/InsightFiltersProductField:InsightFiltersProductField√A data type where security-findings providers can include additional solution-specific details that aren't part of the defined `AwsSecurityFinding` format. See Map Filter below for more details.
Ì
productNamesuBs*q:o
m
securityhubInsightFiltersProductNameCaws:securityhub/InsightFiltersProductName:InsightFiltersProductNamefThe name of the solution (product) that generates findings. See String Filter below for more details.
õ
recommendationTextséBã*à:Ö
Ç
securityhub InsightFiltersRecommendationTextQaws:securityhub/InsightFiltersRecommendationText:InsightFiltersRecommendationTextsThe recommendation of what to do about the issue described in a finding. See String Filter below for more details.
€
recordStatesuBs*q:o
m
securityhubInsightFiltersRecordStateCaws:securityhub/InsightFiltersRecordState:InsightFiltersRecordStateTThe updated record state for the finding. See String Filter below for more details.
Ü
relatedFindingsIdsäBá*Ñ:Å

securityhubInsightFiltersRelatedFindingsIdOaws:securityhub/InsightFiltersRelatedFindingsId:InsightFiltersRelatedFindingsIdcThe solution-generated identifier for a related finding. See String Filter below for more details.
®
relatedFindingsProductArns£B†*ù:ö
ó
securityhub'InsightFiltersRelatedFindingsProductArn_aws:securityhub/InsightFiltersRelatedFindingsProductArn:InsightFiltersRelatedFindingsProductArndThe ARN of the solution that generated a related finding. See String Filter below for more details.
‹
,resourceAwsEc2InstanceIamInstanceProfileArns⁄B◊*‘:—
Œ
securityhub9InsightFiltersResourceAwsEc2InstanceIamInstanceProfileArnÉaws:securityhub/InsightFiltersResourceAwsEc2InstanceIamInstanceProfileArn:InsightFiltersResourceAwsEc2InstanceIamInstanceProfileArnOThe IAM profile ARN of the instance. See String Filter below for more details.
±
resourceAwsEc2InstanceImageIdsØB¨*©:¶
£
securityhub+InsightFiltersResourceAwsEc2InstanceImageIdgaws:securityhub/InsightFiltersResourceAwsEc2InstanceImageId:InsightFiltersResourceAwsEc2InstanceImageId]The Amazon Machine Image (AMI) ID of the instance. See String Filter below for more details.
º
#resourceAwsEc2InstanceIpv4AddressesªB∏*µ:≤
Ø
securityhub/InsightFiltersResourceAwsEc2InstanceIpv4Addressoaws:securityhub/InsightFiltersResourceAwsEc2InstanceIpv4Address:InsightFiltersResourceAwsEc2InstanceIpv4AddressWThe IPv4 addresses associated with the instance. See Ip Filter below for more details.
º
#resourceAwsEc2InstanceIpv6AddressesªB∏*µ:≤
Ø
securityhub/InsightFiltersResourceAwsEc2InstanceIpv6Addressoaws:securityhub/InsightFiltersResourceAwsEc2InstanceIpv6Address:InsightFiltersResourceAwsEc2InstanceIpv6AddressWThe IPv6 addresses associated with the instance. See Ip Filter below for more details.
©
resourceAwsEc2InstanceKeyNamesØB¨*©:¶
£
securityhub+InsightFiltersResourceAwsEc2InstanceKeyNamegaws:securityhub/InsightFiltersResourceAwsEc2InstanceKeyName:InsightFiltersResourceAwsEc2InstanceKeyNameUThe key name associated with the instance. See String Filter below for more details.
µ
!resourceAwsEc2InstanceLaunchedAts∏Bµ*≤:Ø
¨
securityhub.InsightFiltersResourceAwsEc2InstanceLaunchedAtmaws:securityhub/InsightFiltersResourceAwsEc2InstanceLaunchedAt:InsightFiltersResourceAwsEc2InstanceLaunchedAtUThe date and time the instance was launched. See Date Filter below for more details.
¬
resourceAwsEc2InstanceSubnetIds≤BØ*¨:©
¶
securityhub,InsightFiltersResourceAwsEc2InstanceSubnetIdiaws:securityhub/InsightFiltersResourceAwsEc2InstanceSubnetId:InsightFiltersResourceAwsEc2InstanceSubnetIdjThe identifier of the subnet that the instance was launched in. See String Filter below for more details.
ï
resourceAwsEc2InstanceTypes¶B£*†:ù
ö
securityhub(InsightFiltersResourceAwsEc2InstanceTypeaaws:securityhub/InsightFiltersResourceAwsEc2InstanceType:InsightFiltersResourceAwsEc2InstanceTypeMThe instance type of the instance. See String Filter below for more details.
≥
resourceAwsEc2InstanceVpcIds©B¶*£:†
ù
securityhub)InsightFiltersResourceAwsEc2InstanceVpcIdcaws:securityhub/InsightFiltersResourceAwsEc2InstanceVpcId:InsightFiltersResourceAwsEc2InstanceVpcIdgThe identifier of the VPC that the instance was launched in. See String Filter below for more details.
À
!resourceAwsIamAccessKeyCreatedAts∏Bµ*≤:Ø
¨
securityhub.InsightFiltersResourceAwsIamAccessKeyCreatedAtmaws:securityhub/InsightFiltersResourceAwsIamAccessKeyCreatedAt:InsightFiltersResourceAwsIamAccessKeyCreatedAtkThe creation date/time of the IAM access key related to a finding. See Date Filter below for more details.
∂
resourceAwsIamAccessKeyStatusesØB¨*©:¶
£
securityhub+InsightFiltersResourceAwsIamAccessKeyStatusgaws:securityhub/InsightFiltersResourceAwsIamAccessKeyStatus:InsightFiltersResourceAwsIamAccessKeyStatusaThe status of the IAM access key related to a finding. See String Filter below for more details.
»
 resourceAwsIamAccessKeyUserNamesµB≤*Ø:¨
©
securityhub-InsightFiltersResourceAwsIamAccessKeyUserNamekaws:securityhub/InsightFiltersResourceAwsIamAccessKeyUserName:InsightFiltersResourceAwsIamAccessKeyUserNamelThe user associated with the IAM access key related to a finding. See String Filter below for more details.
ß
resourceAwsS3BucketOwnerIds¶B£*†:ù
ö
securityhub(InsightFiltersResourceAwsS3BucketOwnerIdaaws:securityhub/InsightFiltersResourceAwsS3BucketOwnerId:InsightFiltersResourceAwsS3BucketOwnerId_The canonical user ID of the owner of the S3 bucket. See String Filter below for more details.
™
resourceAwsS3BucketOwnerNames¨B©*¶:£
†
securityhub*InsightFiltersResourceAwsS3BucketOwnerNameeaws:securityhub/InsightFiltersResourceAwsS3BucketOwnerName:InsightFiltersResourceAwsS3BucketOwnerNameZThe display name of the owner of the S3 bucket. See String Filter below for more details.
ú
resourceContainerImageIds†Bù*ö:ó
î
securityhub&InsightFiltersResourceContainerImageId]aws:securityhub/InsightFiltersResourceContainerImageId:InsightFiltersResourceContainerImageId\The identifier of the image related to a finding. See String Filter below for more details.
û
resourceContainerImageNames¶B£*†:ù
ö
securityhub(InsightFiltersResourceContainerImageNameaaws:securityhub/InsightFiltersResourceContainerImageName:InsightFiltersResourceContainerImageNameVThe name of the image related to a finding. See String Filter below for more details.
¢
resourceContainerLaunchedAts©B¶*£:†
ù
securityhub)InsightFiltersResourceContainerLaunchedAtcaws:securityhub/InsightFiltersResourceContainerLaunchedAt:InsightFiltersResourceContainerLaunchedAtVThe date/time that the container was started. See Date Filter below for more details.
é
resourceContainerNamesóBî*ë:é
ã
securityhub#InsightFiltersResourceContainerNameWaws:securityhub/InsightFiltersResourceContainerName:InsightFiltersResourceContainerNameZThe name of the container related to a finding. See String Filter below for more details.
∑
resourceDetailsOthersîBë*é:ã
à
securityhub"InsightFiltersResourceDetailsOtherUaws:securityhub/InsightFiltersResourceDetailsOther:InsightFiltersResourceDetailsOtherÜThe details of a resource that doesn't have a specific subfield for the resource type defined. See Map Filter below for more details.
„
resourceIdsrBp*n:l
j
securityhubInsightFiltersResourceIdAaws:securityhub/InsightFiltersResourceId:InsightFiltersResourceId`The canonical identifier for the given resource type. See String Filter below for more details.
é
resourcePartitionsäBá*Ñ:Å

securityhubInsightFiltersResourcePartitionOaws:securityhub/InsightFiltersResourcePartition:InsightFiltersResourcePartitionkThe canonical AWS partition name that the Region is assigned to. See String Filter below for more details.
Ñ
resourceRegions~B|*z:x
v
securityhubInsightFiltersResourceRegionIaws:securityhub/InsightFiltersResourceRegion:InsightFiltersResourceRegionqThe canonical AWS external Region name where this resource is located. See String Filter below for more details.
É
resourceTagsuBs*q:o
m
securityhubInsightFiltersResourceTagCaws:securityhub/InsightFiltersResourceTag:InsightFiltersResourceTag|A list of AWS tags associated with a resource at the time the finding was processed. See Map Filter below for more details.
˜
resourceTypesxBv*t:r
p
securityhubInsightFiltersResourceTypeEaws:securityhub/InsightFiltersResourceType:InsightFiltersResourceTypelSpecifies the type of the resource that details are provided for. See String Filter below for more details.
‹
severityLabels{By*w:u
s
securityhubInsightFiltersSeverityLabelGaws:securityhub/InsightFiltersSeverityLabel:InsightFiltersSeverityLabelMThe label of a finding's severity. See String Filter below for more details.
ç

sourceUrlsoBm*k:i
g
securityhubInsightFiltersSourceUrl?aws:securityhub/InsightFiltersSourceUrl:InsightFiltersSourceUrlçA URL that links to a page about the current finding in the security-findings provider's solution. See String Filter below for more details.
¨
threatIntelIndicatorCategories¨B©*¶:£
†
securityhub*InsightFiltersThreatIntelIndicatorCategoryeaws:securityhub/InsightFiltersThreatIntelIndicatorCategory:InsightFiltersThreatIntelIndicatorCategory[The category of a threat intelligence indicator. See String Filter below for more details.
⁄
#threatIntelIndicatorLastObservedAtsæBª*∏:µ
≤
securityhub0InsightFiltersThreatIntelIndicatorLastObservedAtqaws:securityhub/InsightFiltersThreatIntelIndicatorLastObservedAt:InsightFiltersThreatIntelIndicatorLastObservedAtrThe date/time of the last observation of a threat intelligence indicator. See Date Filter below for more details.
√
threatIntelIndicatorSourceUrlsØB¨*©:¶
£
securityhub+InsightFiltersThreatIntelIndicatorSourceUrlgaws:securityhub/InsightFiltersThreatIntelIndicatorSourceUrl:InsightFiltersThreatIntelIndicatorSourceUrloThe URL for more details from the source of the threat intelligence. See String Filter below for more details.
ô
threatIntelIndicatorSources¶B£*†:ù
ö
securityhub(InsightFiltersThreatIntelIndicatorSourceaaws:securityhub/InsightFiltersThreatIntelIndicatorSource:InsightFiltersThreatIntelIndicatorSourceQThe source of the threat intelligence. See String Filter below for more details.
ó
threatIntelIndicatorTypes†Bù*ö:ó
î
securityhub&InsightFiltersThreatIntelIndicatorType]aws:securityhub/InsightFiltersThreatIntelIndicatorType:InsightFiltersThreatIntelIndicatorTypeWThe type of a threat intelligence indicator. See String Filter below for more details.
ú
threatIntelIndicatorValues£B†*ù:ö
ó
securityhub'InsightFiltersThreatIntelIndicatorValue_aws:securityhub/InsightFiltersThreatIntelIndicatorValue:InsightFiltersThreatIntelIndicatorValueXThe value of a threat intelligence indicator. See String Filter below for more details.
¨
titlescBa*_:]
[
securityhubInsightFiltersTitle7aws:securityhub/InsightFiltersTitle:InsightFiltersTitle=A finding's title. See String Filter below for more details.
Ò
types`B^*\:Z
X
securityhubInsightFiltersType5aws:securityhub/InsightFiltersType:InsightFiltersTypeÖA finding type in the format of `namespace/category/classifier` that classifies a finding. See String Filter below for more details.
õ

updatedAtsoBm*k:i
g
securityhubInsightFiltersUpdatedAt?aws:securityhub/InsightFiltersUpdatedAt:InsightFiltersUpdatedAtõAn ISO8601-formatted timestamp that indicates when the security-findings provider last updated the finding record. See Date Filter below for more details.
ø
userDefinedValuesÜBÉ*Ä:~
|
securityhubInsightFiltersUserDefinedValueMaws:securityhub/InsightFiltersUserDefinedValue:InsightFiltersUserDefinedValue†A list of name/value string pairs associated with the finding. These are custom, user-defined fields added to a finding. See Map Filter below for more details.
Ë
verificationStatesäBá*Ñ:Å

securityhubInsightFiltersVerificationStateOaws:securityhub/InsightFiltersVerificationState:InsightFiltersVerificationStateEThe veracity of a finding. See String Filter below for more details.
˜
workflowStatuses~B|*z:x
v
securityhubInsightFiltersWorkflowStatusIaws:securityhub/InsightFiltersWorkflowStatus:InsightFiltersWorkflowStatuscThe status of the investigation into a finding. See Workflow Status Filter below for more details.
:ï
p
securityhubInsightFiltersAwsAccountIdEaws:securityhub/InsightFiltersAwsAccountId:InsightFiltersAwsAccountId!


comparison" 
value" :í
m
securityhubInsightFiltersCompanyNameCaws:securityhub/InsightFiltersCompanyName:InsightFiltersCompanyName!


comparison" 
value" :°
|
securityhubInsightFiltersComplianceStatusMaws:securityhub/InsightFiltersComplianceStatus:InsightFiltersComplianceStatus!


comparison" 
value" :Î
j
securityhubInsightFiltersConfidenceAaws:securityhub/InsightFiltersConfidence:InsightFiltersConfidence¸
˘u
eqB" iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.
Ä
gteB" sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
}
lteB" pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:¬
g
securityhubInsightFiltersCreatedAt?aws:securityhub/InsightFiltersCreatedAt:InsightFiltersCreatedAt÷
”ˇ
	dateRangeãBà:Ö
Ç
securityhub InsightFiltersCreatedAtDateRangeQaws:securityhub/InsightFiltersCreatedAtDateRange:InsightFiltersCreatedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:•
Ç
securityhub InsightFiltersCreatedAtDateRangeQaws:securityhub/InsightFiltersCreatedAtDateRange:InsightFiltersCreatedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:Ó
m
securityhubInsightFiltersCriticalityCaws:securityhub/InsightFiltersCriticality:InsightFiltersCriticality¸
˘u
eqB" iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.
Ä
gteB" sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
}
lteB" pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:í
m
securityhubInsightFiltersDescriptionCaws:securityhub/InsightFiltersDescription:InsightFiltersDescription!


comparison" 
value" :´
©
securityhub-InsightFiltersFindingProviderFieldsConfidencekaws:securityhub/InsightFiltersFindingProviderFieldsConfidence:InsightFiltersFindingProviderFieldsConfidence¸
˘u
eqB" iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.
Ä
gteB" sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
}
lteB" pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:Æ
¨
securityhub.InsightFiltersFindingProviderFieldsCriticalitymaws:securityhub/InsightFiltersFindingProviderFieldsCriticality:InsightFiltersFindingProviderFieldsCriticality¸
˘u
eqB" iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.
Ä
gteB" sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
}
lteB" pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:‰
æ
securityhub4InsightFiltersFindingProviderFieldsRelatedFindingsIdyaws:securityhub/InsightFiltersFindingProviderFieldsRelatedFindingsId:InsightFiltersFindingProviderFieldsRelatedFindingsId!


comparison" 
value" :˝
◊
securityhub<InsightFiltersFindingProviderFieldsRelatedFindingsProductArnâaws:securityhub/InsightFiltersFindingProviderFieldsRelatedFindingsProductArn:InsightFiltersFindingProviderFieldsRelatedFindingsProductArn!


comparison" 
value" :ÿ
≤
securityhub0InsightFiltersFindingProviderFieldsSeverityLabelqaws:securityhub/InsightFiltersFindingProviderFieldsSeverityLabel:InsightFiltersFindingProviderFieldsSeverityLabel!


comparison" 
value" :·
ª
securityhub3InsightFiltersFindingProviderFieldsSeverityOriginalwaws:securityhub/InsightFiltersFindingProviderFieldsSeverityOriginal:InsightFiltersFindingProviderFieldsSeverityOriginal!


comparison" 
value" :Ω
ó
securityhub'InsightFiltersFindingProviderFieldsType_aws:securityhub/InsightFiltersFindingProviderFieldsType:InsightFiltersFindingProviderFieldsType!


comparison" 
value" :Ê
y
securityhubInsightFiltersFirstObservedAtKaws:securityhub/InsightFiltersFirstObservedAt:InsightFiltersFirstObservedAtË
Âë
	dateRangeùBö:ó
î
securityhub&InsightFiltersFirstObservedAtDateRange]aws:securityhub/InsightFiltersFirstObservedAtDateRange:InsightFiltersFirstObservedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:∑
î
securityhub&InsightFiltersFirstObservedAtDateRange]aws:securityhub/InsightFiltersFirstObservedAtDateRange:InsightFiltersFirstObservedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:í
m
securityhubInsightFiltersGeneratorIdCaws:securityhub/InsightFiltersGeneratorId:InsightFiltersGeneratorId!


comparison" 
value" :w
R
securityhubInsightFiltersId1aws:securityhub/InsightFiltersId:InsightFiltersId!


comparison" 
value" :è
a
securityhubInsightFiltersKeyword;aws:securityhub/InsightFiltersKeyword:InsightFiltersKeyword*
(&
value" A value for the keyword.
:‡
v
securityhubInsightFiltersLastObservedAtIaws:securityhub/InsightFiltersLastObservedAt:InsightFiltersLastObservedAtÂ
‚é
	dateRangeöBó:î
ë
securityhub%InsightFiltersLastObservedAtDateRange[aws:securityhub/InsightFiltersLastObservedAtDateRange:InsightFiltersLastObservedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:¥
ë
securityhub%InsightFiltersLastObservedAtDateRange[aws:securityhub/InsightFiltersLastObservedAtDateRange:InsightFiltersLastObservedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:í
m
securityhubInsightFiltersMalwareNameCaws:securityhub/InsightFiltersMalwareName:InsightFiltersMalwareName!


comparison" 
value" :í
m
securityhubInsightFiltersMalwarePathCaws:securityhub/InsightFiltersMalwarePath:InsightFiltersMalwarePath!


comparison" 
value" :ï
p
securityhubInsightFiltersMalwareStateEaws:securityhub/InsightFiltersMalwareState:InsightFiltersMalwareState!


comparison" 
value" :í
m
securityhubInsightFiltersMalwareTypeCaws:securityhub/InsightFiltersMalwareType:InsightFiltersMalwareType!


comparison" 
value" :∫
î
securityhub&InsightFiltersNetworkDestinationDomain]aws:securityhub/InsightFiltersNetworkDestinationDomain:InsightFiltersNetworkDestinationDomain!


comparison" 
value" :ª
é
securityhub$InsightFiltersNetworkDestinationIpv4Yaws:securityhub/InsightFiltersNetworkDestinationIpv4:InsightFiltersNetworkDestinationIpv4(
&$
cidr" A finding's CIDR value.
:ª
é
securityhub$InsightFiltersNetworkDestinationIpv6Yaws:securityhub/InsightFiltersNetworkDestinationIpv6:InsightFiltersNetworkDestinationIpv6(
&$
cidr" A finding's CIDR value.
:ê
é
securityhub$InsightFiltersNetworkDestinationPortYaws:securityhub/InsightFiltersNetworkDestinationPort:InsightFiltersNetworkDestinationPort¸
˘u
eqB" iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.
Ä
gteB" sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
}
lteB" pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:°
|
securityhubInsightFiltersNetworkDirectionMaws:securityhub/InsightFiltersNetworkDirection:InsightFiltersNetworkDirection!


comparison" 
value" :û
y
securityhubInsightFiltersNetworkProtocolKaws:securityhub/InsightFiltersNetworkProtocol:InsightFiltersNetworkProtocol!


comparison" 
value" :´
Ö
securityhub!InsightFiltersNetworkSourceDomainSaws:securityhub/InsightFiltersNetworkSourceDomain:InsightFiltersNetworkSourceDomain!


comparison" 
value" :´

securityhubInsightFiltersNetworkSourceIpv4Oaws:securityhub/InsightFiltersNetworkSourceIpv4:InsightFiltersNetworkSourceIpv4(
&$
cidr" A finding's CIDR value.
:´

securityhubInsightFiltersNetworkSourceIpv6Oaws:securityhub/InsightFiltersNetworkSourceIpv6:InsightFiltersNetworkSourceIpv6(
&$
cidr" A finding's CIDR value.
:°
|
securityhubInsightFiltersNetworkSourceMacMaws:securityhub/InsightFiltersNetworkSourceMac:InsightFiltersNetworkSourceMac!


comparison" 
value" :Ä

securityhubInsightFiltersNetworkSourcePortOaws:securityhub/InsightFiltersNetworkSourcePort:InsightFiltersNetworkSourcePort¸
˘u
eqB" iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.
Ä
gteB" sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
}
lteB" pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:â
d
securityhubInsightFiltersNoteText=aws:securityhub/InsightFiltersNoteText:InsightFiltersNoteText!


comparison" 
value" :⁄
s
securityhubInsightFiltersNoteUpdatedAtGaws:securityhub/InsightFiltersNoteUpdatedAt:InsightFiltersNoteUpdatedAt‚
ﬂã
	dateRangeóBî:ë
é
securityhub$InsightFiltersNoteUpdatedAtDateRangeYaws:securityhub/InsightFiltersNoteUpdatedAtDateRange:InsightFiltersNoteUpdatedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:±
é
securityhub$InsightFiltersNoteUpdatedAtDateRangeYaws:securityhub/InsightFiltersNoteUpdatedAtDateRange:InsightFiltersNoteUpdatedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:ò
s
securityhubInsightFiltersNoteUpdatedByGaws:securityhub/InsightFiltersNoteUpdatedBy:InsightFiltersNoteUpdatedBy!


comparison" 
value" :Ú

securityhubInsightFiltersProcessLaunchedAtOaws:securityhub/InsightFiltersProcessLaunchedAt:InsightFiltersProcessLaunchedAtÓ
Îó
	dateRange£B†:ù
ö
securityhub(InsightFiltersProcessLaunchedAtDateRangeaaws:securityhub/InsightFiltersProcessLaunchedAtDateRange:InsightFiltersProcessLaunchedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:Ω
ö
securityhub(InsightFiltersProcessLaunchedAtDateRangeaaws:securityhub/InsightFiltersProcessLaunchedAtDateRange:InsightFiltersProcessLaunchedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:í
m
securityhubInsightFiltersProcessNameCaws:securityhub/InsightFiltersProcessName:InsightFiltersProcessName!


comparison" 
value" :˝
|
securityhubInsightFiltersProcessParentPidMaws:securityhub/InsightFiltersProcessParentPid:InsightFiltersProcessParentPid¸
˘u
eqB" iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.
Ä
gteB" sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
}
lteB" pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:í
m
securityhubInsightFiltersProcessPathCaws:securityhub/InsightFiltersProcessPath:InsightFiltersProcessPath!


comparison" 
value" :Î
j
securityhubInsightFiltersProcessPidAaws:securityhub/InsightFiltersProcessPid:InsightFiltersProcessPid¸
˘u
eqB" iThe equal-to condition to be applied to a single field when querying for findings, provided as a String.
Ä
gteB" sThe greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
}
lteB" pThe less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
:ˇ
Ö
securityhub!InsightFiltersProcessTerminatedAtSaws:securityhub/InsightFiltersProcessTerminatedAt:InsightFiltersProcessTerminatedAtÙ
Òù
	dateRange©B¶:£
†
securityhub*InsightFiltersProcessTerminatedAtDateRangeeaws:securityhub/InsightFiltersProcessTerminatedAtDateRange:InsightFiltersProcessTerminatedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:√
†
securityhub*InsightFiltersProcessTerminatedAtDateRangeeaws:securityhub/InsightFiltersProcessTerminatedAtDateRange:InsightFiltersProcessTerminatedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:è
j
securityhubInsightFiltersProductArnAaws:securityhub/InsightFiltersProductArn:InsightFiltersProductArn!


comparison" 
value" :¡
p
securityhubInsightFiltersProductFieldEaws:securityhub/InsightFiltersProductField:InsightFiltersProductFieldÃ
…

comparison" ß
key" õThe key of the map filter. For example, for `ResourceTags`, `Key` identifies the name of the tag. For `UserDefinedFields`, `Key` is the name of the field.

value" :í
m
securityhubInsightFiltersProductNameCaws:securityhub/InsightFiltersProductName:InsightFiltersProductName!


comparison" 
value" :®
Ç
securityhub InsightFiltersRecommendationTextQaws:securityhub/InsightFiltersRecommendationText:InsightFiltersRecommendationText!


comparison" 
value" :í
m
securityhubInsightFiltersRecordStateCaws:securityhub/InsightFiltersRecordState:InsightFiltersRecordState!


comparison" 
value" :§

securityhubInsightFiltersRelatedFindingsIdOaws:securityhub/InsightFiltersRelatedFindingsId:InsightFiltersRelatedFindingsId!


comparison" 
value" :Ω
ó
securityhub'InsightFiltersRelatedFindingsProductArn_aws:securityhub/InsightFiltersRelatedFindingsProductArn:InsightFiltersRelatedFindingsProductArn!


comparison" 
value" :Ù
Œ
securityhub9InsightFiltersResourceAwsEc2InstanceIamInstanceProfileArnÉaws:securityhub/InsightFiltersResourceAwsEc2InstanceIamInstanceProfileArn:InsightFiltersResourceAwsEc2InstanceIamInstanceProfileArn!


comparison" 
value" :…
£
securityhub+InsightFiltersResourceAwsEc2InstanceImageIdgaws:securityhub/InsightFiltersResourceAwsEc2InstanceImageId:InsightFiltersResourceAwsEc2InstanceImageId!


comparison" 
value" :‹
Ø
securityhub/InsightFiltersResourceAwsEc2InstanceIpv4Addressoaws:securityhub/InsightFiltersResourceAwsEc2InstanceIpv4Address:InsightFiltersResourceAwsEc2InstanceIpv4Address(
&$
cidr" A finding's CIDR value.
:‹
Ø
securityhub/InsightFiltersResourceAwsEc2InstanceIpv6Addressoaws:securityhub/InsightFiltersResourceAwsEc2InstanceIpv6Address:InsightFiltersResourceAwsEc2InstanceIpv6Address(
&$
cidr" A finding's CIDR value.
:…
£
securityhub+InsightFiltersResourceAwsEc2InstanceKeyNamegaws:securityhub/InsightFiltersResourceAwsEc2InstanceKeyName:InsightFiltersResourceAwsEc2InstanceKeyName!


comparison" 
value" :Õ
¨
securityhub.InsightFiltersResourceAwsEc2InstanceLaunchedAtmaws:securityhub/InsightFiltersResourceAwsEc2InstanceLaunchedAt:InsightFiltersResourceAwsEc2InstanceLaunchedAtõ
òƒ
	dateRange–BÕ: 
«
securityhub7InsightFiltersResourceAwsEc2InstanceLaunchedAtDateRangeaws:securityhub/InsightFiltersResourceAwsEc2InstanceLaunchedAtDateRange:InsightFiltersResourceAwsEc2InstanceLaunchedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:Í
«
securityhub7InsightFiltersResourceAwsEc2InstanceLaunchedAtDateRangeaws:securityhub/InsightFiltersResourceAwsEc2InstanceLaunchedAtDateRange:InsightFiltersResourceAwsEc2InstanceLaunchedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:Ã
¶
securityhub,InsightFiltersResourceAwsEc2InstanceSubnetIdiaws:securityhub/InsightFiltersResourceAwsEc2InstanceSubnetId:InsightFiltersResourceAwsEc2InstanceSubnetId!


comparison" 
value" :¿
ö
securityhub(InsightFiltersResourceAwsEc2InstanceTypeaaws:securityhub/InsightFiltersResourceAwsEc2InstanceType:InsightFiltersResourceAwsEc2InstanceType!


comparison" 
value" :√
ù
securityhub)InsightFiltersResourceAwsEc2InstanceVpcIdcaws:securityhub/InsightFiltersResourceAwsEc2InstanceVpcId:InsightFiltersResourceAwsEc2InstanceVpcId!


comparison" 
value" :Õ
¨
securityhub.InsightFiltersResourceAwsIamAccessKeyCreatedAtmaws:securityhub/InsightFiltersResourceAwsIamAccessKeyCreatedAt:InsightFiltersResourceAwsIamAccessKeyCreatedAtõ
òƒ
	dateRange–BÕ: 
«
securityhub7InsightFiltersResourceAwsIamAccessKeyCreatedAtDateRangeaws:securityhub/InsightFiltersResourceAwsIamAccessKeyCreatedAtDateRange:InsightFiltersResourceAwsIamAccessKeyCreatedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:Í
«
securityhub7InsightFiltersResourceAwsIamAccessKeyCreatedAtDateRangeaws:securityhub/InsightFiltersResourceAwsIamAccessKeyCreatedAtDateRange:InsightFiltersResourceAwsIamAccessKeyCreatedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:…
£
securityhub+InsightFiltersResourceAwsIamAccessKeyStatusgaws:securityhub/InsightFiltersResourceAwsIamAccessKeyStatus:InsightFiltersResourceAwsIamAccessKeyStatus!


comparison" 
value" :œ
©
securityhub-InsightFiltersResourceAwsIamAccessKeyUserNamekaws:securityhub/InsightFiltersResourceAwsIamAccessKeyUserName:InsightFiltersResourceAwsIamAccessKeyUserName!


comparison" 
value" :¿
ö
securityhub(InsightFiltersResourceAwsS3BucketOwnerIdaaws:securityhub/InsightFiltersResourceAwsS3BucketOwnerId:InsightFiltersResourceAwsS3BucketOwnerId!


comparison" 
value" :∆
†
securityhub*InsightFiltersResourceAwsS3BucketOwnerNameeaws:securityhub/InsightFiltersResourceAwsS3BucketOwnerName:InsightFiltersResourceAwsS3BucketOwnerName!


comparison" 
value" :∫
î
securityhub&InsightFiltersResourceContainerImageId]aws:securityhub/InsightFiltersResourceContainerImageId:InsightFiltersResourceContainerImageId!


comparison" 
value" :¿
ö
securityhub(InsightFiltersResourceContainerImageNameaaws:securityhub/InsightFiltersResourceContainerImageName:InsightFiltersResourceContainerImageName!


comparison" 
value" :Ø
ù
securityhub)InsightFiltersResourceContainerLaunchedAtcaws:securityhub/InsightFiltersResourceContainerLaunchedAt:InsightFiltersResourceContainerLaunchedAtå
âµ
	dateRange¡Bæ:ª
∏
securityhub2InsightFiltersResourceContainerLaunchedAtDateRangeuaws:securityhub/InsightFiltersResourceContainerLaunchedAtDateRange:InsightFiltersResourceContainerLaunchedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:€
∏
securityhub2InsightFiltersResourceContainerLaunchedAtDateRangeuaws:securityhub/InsightFiltersResourceContainerLaunchedAtDateRange:InsightFiltersResourceContainerLaunchedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:±
ã
securityhub#InsightFiltersResourceContainerNameWaws:securityhub/InsightFiltersResourceContainerName:InsightFiltersResourceContainerName!


comparison" 
value" :⁄
à
securityhub"InsightFiltersResourceDetailsOtherUaws:securityhub/InsightFiltersResourceDetailsOther:InsightFiltersResourceDetailsOtherÃ
…

comparison" ß
key" õThe key of the map filter. For example, for `ResourceTags`, `Key` identifies the name of the tag. For `UserDefinedFields`, `Key` is the name of the field.

value" :è
j
securityhubInsightFiltersResourceIdAaws:securityhub/InsightFiltersResourceId:InsightFiltersResourceId!


comparison" 
value" :§

securityhubInsightFiltersResourcePartitionOaws:securityhub/InsightFiltersResourcePartition:InsightFiltersResourcePartition!


comparison" 
value" :õ
v
securityhubInsightFiltersResourceRegionIaws:securityhub/InsightFiltersResourceRegion:InsightFiltersResourceRegion!


comparison" 
value" :æ
m
securityhubInsightFiltersResourceTagCaws:securityhub/InsightFiltersResourceTag:InsightFiltersResourceTagÃ
…

comparison" ß
key" õThe key of the map filter. For example, for `ResourceTags`, `Key` identifies the name of the tag. For `UserDefinedFields`, `Key` is the name of the field.

value" :ï
p
securityhubInsightFiltersResourceTypeEaws:securityhub/InsightFiltersResourceType:InsightFiltersResourceType!


comparison" 
value" :ò
s
securityhubInsightFiltersSeverityLabelGaws:securityhub/InsightFiltersSeverityLabel:InsightFiltersSeverityLabel!


comparison" 
value" :å
g
securityhubInsightFiltersSourceUrl?aws:securityhub/InsightFiltersSourceUrl:InsightFiltersSourceUrl!


comparison" 
value" :∆
†
securityhub*InsightFiltersThreatIntelIndicatorCategoryeaws:securityhub/InsightFiltersThreatIntelIndicatorCategory:InsightFiltersThreatIntelIndicatorCategory!


comparison" 
value" :⁄
≤
securityhub0InsightFiltersThreatIntelIndicatorLastObservedAtqaws:securityhub/InsightFiltersThreatIntelIndicatorLastObservedAt:InsightFiltersThreatIntelIndicatorLastObservedAt¢
üÀ
	dateRange◊B‘:—
Œ
securityhub9InsightFiltersThreatIntelIndicatorLastObservedAtDateRangeÉaws:securityhub/InsightFiltersThreatIntelIndicatorLastObservedAtDateRange:InsightFiltersThreatIntelIndicatorLastObservedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:Ò
Œ
securityhub9InsightFiltersThreatIntelIndicatorLastObservedAtDateRangeÉaws:securityhub/InsightFiltersThreatIntelIndicatorLastObservedAtDateRange:InsightFiltersThreatIntelIndicatorLastObservedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:¿
ö
securityhub(InsightFiltersThreatIntelIndicatorSourceaaws:securityhub/InsightFiltersThreatIntelIndicatorSource:InsightFiltersThreatIntelIndicatorSource!


comparison" 
value" :…
£
securityhub+InsightFiltersThreatIntelIndicatorSourceUrlgaws:securityhub/InsightFiltersThreatIntelIndicatorSourceUrl:InsightFiltersThreatIntelIndicatorSourceUrl!


comparison" 
value" :∫
î
securityhub&InsightFiltersThreatIntelIndicatorType]aws:securityhub/InsightFiltersThreatIntelIndicatorType:InsightFiltersThreatIntelIndicatorType!


comparison" 
value" :Ω
ó
securityhub'InsightFiltersThreatIntelIndicatorValue_aws:securityhub/InsightFiltersThreatIntelIndicatorValue:InsightFiltersThreatIntelIndicatorValue!


comparison" 
value" :Ä
[
securityhubInsightFiltersTitle7aws:securityhub/InsightFiltersTitle:InsightFiltersTitle!


comparison" 
value" :}
X
securityhubInsightFiltersType5aws:securityhub/InsightFiltersType:InsightFiltersType!


comparison" 
value" :¬
g
securityhubInsightFiltersUpdatedAt?aws:securityhub/InsightFiltersUpdatedAt:InsightFiltersUpdatedAt÷
”ˇ
	dateRangeãBà:Ö
Ç
securityhub InsightFiltersUpdatedAtDateRangeQaws:securityhub/InsightFiltersUpdatedAtDateRange:InsightFiltersUpdatedAtDateRangedA configuration block of the date range for the date filter. See date_range below for more details.
f
endB" YAn end date for the date filter. Required with `start` if `date_range` is not specified.
g
startB" XA start date for the date filter. Required with `end` if `date_range` is not specified.
:•
Ç
securityhub InsightFiltersUpdatedAtDateRangeQaws:securityhub/InsightFiltersUpdatedAtDateRange:InsightFiltersUpdatedAtDateRangeù
öI
unit" =A date range unit for the date filter. Valid values: `DAYS`.
M
value @A date range value for the date filter, provided as an Integer.
:Õ
|
securityhubInsightFiltersUserDefinedValueMaws:securityhub/InsightFiltersUserDefinedValue:InsightFiltersUserDefinedValueÃ
…

comparison" ß
key" õThe key of the map filter. For example, for `ResourceTags`, `Key` identifies the name of the tag. For `UserDefinedFields`, `Key` is the name of the field.

value" :§

securityhubInsightFiltersVerificationStateOaws:securityhub/InsightFiltersVerificationState:InsightFiltersVerificationState!


comparison" 
value" :õ
v
securityhubInsightFiltersWorkflowStatusIaws:securityhub/InsightFiltersWorkflowStatus:InsightFiltersWorkflowStatus!


comparison" 
value" :„
∏
securityhub2OrganizationConfigurationOrganizationConfigurationuaws:securityhub/OrganizationConfigurationOrganizationConfiguration:OrganizationConfigurationOrganizationConfiguration•
¢ü
configurationType" ÖIndicates whether the organization uses local or central configuration. If using central configuration, `auto_enable` must be set to `false` and `auto_enable_standards` set to `NONE`. More information can be found in the [documentation for central configuration](https://docs.aws.amazon.com/securityhub/latest/userguide/central-configuration-intro.html). Valid values: `LOCAL`, `CENTRAL`.
:◊
—
securityhub:getStandardsControlAssociationsStandardsControlAssociationÖaws:securityhub/getStandardsControlAssociationsStandardsControlAssociation:getStandardsControlAssociationsStandardsControlAssociationÄ
˝P
associationStatus" 7Enablement status of a control in a specific standard.
r
relatedRequirements*" UList of underlying requirements in the compliance framework related to the standard.
7
securityControlArn" ARN of the security control.
ë
securityControlId" xThe identifier of the control (identified with `SecurityControlId`, `SecurityControlArn`, or a mix of both parameters).
)
standardsArn" ARN of the standard.
@
standardsControlDescription" Description of the standard.
4
standardsControlTitle" Title of the standard.
c
	updatedAt" RLast time that a control's enablement status in a specified standard was updated.
`
updatedReason" KReason for updating a control's enablement status in a specified standard.
:ø
Z
securitylakeAwsLogSourceSource6aws:securitylake/AwsLogSourceSource:AwsLogSourceSource‡
›§
accountsB*" èSpecify the AWS account information where you want to enable Security Lake.
If not specified, uses all accounts included in the Security Lake.
M
regions*" <Specify the Regions where you want to enable Security Lake.
—

sourceName" æThe name for a AWS source. This must be a Regionally unique value. Valid values: `ROUTE53`, `VPC_FLOW`, `SH_FINDINGS`, `CLOUD_TRAIL_MGMT`, `LAMBDA_EXECUTION`, `S3_DATA`, `EKS_AUDIT`, `WAF`.
ê
sourceVersionB" yThe version for a AWS source.
If not specified, the version will be the default.
This must be a Regionally unique value.
:´
l
securitylakeCustomLogSourceAttributeBaws:securitylake/CustomLogSourceAttribute:CustomLogSourceAttribute∫
∑3

crawlerArn" !The ARN of the AWS Glue crawler.
O
databaseArn" <The ARN of the AWS Glue database where results are written.
/
tableArn" The ARN of the AWS Glue table.
:¥
x
securitylakeCustomLogSourceConfigurationJaws:securitylake/CustomLogSourceConfiguration:CustomLogSourceConfiguration∑
¥¢
crawlerConfigurationΩB∫:∑
¥
securitylake0CustomLogSourceConfigurationCrawlerConfigurationraws:securitylake/CustomLogSourceConfigurationCrawlerConfiguration:CustomLogSourceConfigurationCrawlerConfigurationJThe configuration for the Glue Crawler for the third-party custom source.
å
providerIdentity±BÆ:´
®
securitylake,CustomLogSourceConfigurationProviderIdentityjaws:securitylake/CustomLogSourceConfigurationProviderIdentity:CustomLogSourceConfigurationProviderIdentityDThe identity of the log provider for the third-party custom source.
:«
¥
securitylake0CustomLogSourceConfigurationCrawlerConfigurationraws:securitylake/CustomLogSourceConfigurationCrawlerConfiguration:CustomLogSourceConfigurationCrawlerConfigurationç
äá
roleArn" xThe Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role to be used by the AWS Glue crawler.
:ø
®
securitylake,CustomLogSourceConfigurationProviderIdentityjaws:securitylake/CustomLogSourceConfigurationProviderIdentity:CustomLogSourceConfigurationProviderIdentityë
é]

externalId" KThe external ID used to estalish trust relationship with the AWS identity.
-
	principal" The AWS identity principal.
:–
{
securitylakeCustomLogSourceProviderDetailLaws:securitylake/CustomLogSourceProviderDetail:CustomLogSourceProviderDetail–
ÕY
location" IThe location of the partition in the Amazon S3 bucket for Security Lake.
p
roleArn" aThe ARN of the IAM role to be used by the entity putting logs into your custom source partition.
:Á
c
securitylakeDataLakeConfiguration<aws:securitylake/DataLakeConfiguration:DataLakeConfigurationˇ
¸è
encryptionConfigurations¥B±*Æ:´
®
securitylake,DataLakeConfigurationEncryptionConfigurationjaws:securitylake/DataLakeConfigurationEncryptionConfiguration:DataLakeConfigurationEncryptionConfiguration<Provides encryption details of Amazon Security Lake object.
Ü
lifecycleConfigurationÆB´:®
•
securitylake+DataLakeConfigurationLifecycleConfigurationhaws:securitylake/DataLakeConfigurationLifecycleConfiguration:DataLakeConfigurationLifecycleConfiguration;Provides lifecycle details of Amazon Security Lake object.
L
region" >The AWS Regions where Security Lake is automatically enabled.
ê
replicationConfiguration¥B±:Æ
´
securitylake-DataLakeConfigurationReplicationConfigurationlaws:securitylake/DataLakeConfigurationReplicationConfiguration:DataLakeConfigurationReplicationConfiguration=Provides replication details of Amazon Security Lake object.
:†
®
securitylake,DataLakeConfigurationEncryptionConfigurationjaws:securitylake/DataLakeConfigurationEncryptionConfiguration:DataLakeConfigurationEncryptionConfigurations
qo
kmsKeyId" _The id of KMS encryption key used by Amazon Security Lake to encrypt the Security Lake object.
:¸
•
securitylake+DataLakeConfigurationLifecycleConfigurationhaws:securitylake/DataLakeConfigurationLifecycleConfiguration:DataLakeConfigurationLifecycleConfiguration—
Œû

expirationÃB…:∆
√
securitylake5DataLakeConfigurationLifecycleConfigurationExpiration|aws:securitylake/DataLakeConfigurationLifecycleConfigurationExpiration:DataLakeConfigurationLifecycleConfigurationExpirationAProvides data expiration details of Amazon Security Lake object.
™
transitionsœBÃ*…:∆
√
securitylake5DataLakeConfigurationLifecycleConfigurationTransition|aws:securitylake/DataLakeConfigurationLifecycleConfigurationTransition:DataLakeConfigurationLifecycleConfigurationTransitionIProvides data storage transition details of Amazon Security Lake object.
:ƒ
√
securitylake5DataLakeConfigurationLifecycleConfigurationExpiration|aws:securitylake/DataLakeConfigurationLifecycleConfigurationExpiration:DataLakeConfigurationLifecycleConfigurationExpiration|
zx
daysB jNumber of days before data transition to a different S3 Storage Class in the Amazon Security Lake object.
:Â
√
securitylake5DataLakeConfigurationLifecycleConfigurationTransition|aws:securitylake/DataLakeConfigurationLifecycleConfigurationTransition:DataLakeConfigurationLifecycleConfigurationTransitionú
ôx
daysB jNumber of days before data transition to a different S3 Storage Class in the Amazon Security Lake object.
ú
storageClassB" ÖThe range of storage classes that you can choose from based on the data access, resiliency, and cost requirements of your workloads.
:Ã
´
securitylake-DataLakeConfigurationReplicationConfigurationlaws:securitylake/DataLakeConfigurationReplicationConfiguration:DataLakeConfigurationReplicationConfigurationõ
òÆ
regionsB*" öReplication enables automatic, asynchronous copying of objects across Amazon S3 buckets. Amazon S3 buckets that are configured for object replication can be owned by the same AWS account or by different accounts. You can replicate objects to a single destination bucket or to multiple destination buckets. The destination buckets can be in different AWS Regions or within the same Region as the source bucket.
‰
roleArnB" “Replication settings for the Amazon S3 buckets. This parameter uses the AWS Identity and Access Management (IAM) role you created that is managed by Security Lake, to ensure the replication setting is correct.
:õ
T
securitylakeDataLakeTimeouts2aws:securitylake/DataLakeTimeouts:DataLakeTimeouts¬
øÁ
createB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
Ë
deleteB" ◊A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
Á
updateB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:Î
ç
securitylake#SubscriberNotificationConfigurationXaws:securitylake/SubscriberNotificationConfiguration:SubscriberNotificationConfigurationÿ
’Ã
httpsNotificationConfigurationÒBÓ:Î
Ë
securitylakeASubscriberNotificationConfigurationHttpsNotificationConfigurationîaws:securitylake/SubscriberNotificationConfigurationHttpsNotificationConfiguration:SubscriberNotificationConfigurationHttpsNotificationConfiguration6The configurations for HTTPS subscriber notification.
É
sqsNotificationConfigurationÎBË:Â
‚
securitylake?SubscriberNotificationConfigurationSqsNotificationConfigurationêaws:securitylake/SubscriberNotificationConfigurationSqsNotificationConfiguration:SubscriberNotificationConfigurationSqsNotificationConfigurationuThe configurations for SQS subscriber notification.
There are no parameters within `sqs_notification_configuration`.
:´
Ë
securitylakeASubscriberNotificationConfigurationHttpsNotificationConfigurationîaws:securitylake/SubscriberNotificationConfigurationHttpsNotificationConfiguration:SubscriberNotificationConfigurationHttpsNotificationConfigurationΩ
∫U
authorizationApiKeyNameB" 4The API key name for the notification subscription.
W
authorizationApiKeyValueB" 5The API key value for the notification subscription.
Ñ
endpoint" tThe subscription endpoint in Security Lake.
If you prefer notification with an HTTPS endpoint, populate this field.
o

httpMethodB" [The HTTP method used for the notification subscription.
Valid values are `POST` and `PUT`.
è
targetRoleArn" ˘The Amazon Resource Name (ARN) of the EventBridge API destinations IAM role that you created.
For more information about ARNs and how to use them in policies, see Managing data access and AWS Managed Policies in the Amazon Security Lake User Guide.
:È
‚
securitylake?SubscriberNotificationConfigurationSqsNotificationConfigurationêaws:securitylake/SubscriberNotificationConfigurationSqsNotificationConfiguration:SubscriberNotificationConfigurationSqsNotificationConfiguration
 :ø
T
securitylakeSubscriberSource2aws:securitylake/SubscriberSource:SubscriberSourceÊ
„º
awsLogSourceResourceôBñ:ì
ê
securitylake$SubscriberSourceAwsLogSourceResourceZaws:securitylake/SubscriberSourceAwsLogSourceResource:SubscriberSourceAwsLogSourceResourceáAmazon Security Lake supports log and event collection for natively supported AWS services. See `aws_log_source_resource` Block below.
°
customLogSourceResource¢Bü:ú
ô
securitylake'SubscriberSourceCustomLogSourceResource`aws:securitylake/SubscriberSourceCustomLogSourceResource:SubscriberSourceCustomLogSourceResourceaAmazon Security Lake supports custom source types. See `custom_log_source_resource` Block below.
:–
ê
securitylake$SubscriberSourceAwsLogSourceResourceZaws:securitylake/SubscriberSourceAwsLogSourceResource:SubscriberSourceAwsLogSourceResource∫
∑S

sourceName" AProvides data expiration details of Amazon Security Lake object.
`
sourceVersionB" IProvides data storage transition details of Amazon Security Lake object.
:»
ô
securitylake'SubscriberSourceCustomLogSourceResource`aws:securitylake/SubscriberSourceCustomLogSourceResource:SubscriberSourceCustomLogSourceResource©
¶†

attributes¿BΩ*∫:∑
¥
securitylake0SubscriberSourceCustomLogSourceResourceAttributeraws:securitylake/SubscriberSourceCustomLogSourceResourceAttribute:SubscriberSourceCustomLogSourceResourceAttributeOThe attributes of the third-party custom source. See `attributes` Block below.
¨
	providersΩB∫*∑:¥
±
securitylake/SubscriberSourceCustomLogSourceResourceProviderpaws:securitylake/SubscriberSourceCustomLogSourceResourceProvider:SubscriberSourceCustomLogSourceResourceProvider_The details of the log provider for the third-party custom source. See `provider` Block below.
d

sourceName" RThe name for a third-party custom source. This must be a Regionally unique value.
l
sourceVersionB" UThe version for a third-party custom source. This must be a Regionally unique value.
:Ù
¥
securitylake0SubscriberSourceCustomLogSourceResourceAttributeraws:securitylake/SubscriberSourceCustomLogSourceResourceAttribute:SubscriberSourceCustomLogSourceResourceAttribute∫
∑3

crawlerArn" !The ARN of the AWS Glue crawler.
O
databaseArn" <The ARN of the AWS Glue database where results are written.
/
tableArn" The ARN of the AWS Glue table.
:á
±
securitylake/SubscriberSourceCustomLogSourceResourceProviderpaws:securitylake/SubscriberSourceCustomLogSourceResourceProvider:SubscriberSourceCustomLogSourceResourceProvider–
ÕY
location" IThe location of the partition in the Amazon S3 bucket for Security Lake.
p
roleArn" aThe ARN of the IAM role to be used by the entity putting logs into your custom source partition.
:°
x
securitylakeSubscriberSubscriberIdentityJaws:securitylake/SubscriberSubscriberIdentity:SubscriberSubscriberIdentity§
°P

externalId" >The AWS Regions where Security Lake is automatically enabled.
M
	principal" <Provides encryption details of Amazon Security Lake object.
:°
Z
securitylakeSubscriberTimeouts6aws:securitylake/SubscriberTimeouts:SubscriberTimeouts¬
øÁ
createB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
Ë
deleteB" ◊A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
Á
updateB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:€
ó
servicecatalog%ProductProvisioningArtifactParameters^aws:servicecatalog/ProductProvisioningArtifactParameters:ProductProvisioningArtifactParametersæ
ªë
descriptionB" |Description of the provisioning artifact (i.e., version), including how it differs from the previous provisioning artifact.
î
disableTemplateValidationB
 qWhether AWS Service Catalog stops validating the specified provisioning artifact template even if it is invalid.
f
nameB" XName of the provisioning artifact (for example, `v1`, `v2beta`). No spaces are allowed.
í
templatePhysicalIdB" ıTemplate source as the physical ID of the resource that contains the template. Currently only supports CloudFormation stack ARN. Specify the physical ID as `arn:[partition]:cloudformation:[region]:[account ID]:stack/[stack name]/[resource ID]`.
Y
templateUrlB" DTemplate source as URL of the CloudFormation template in Amazon S3.
µ
typeB" ¶Type of provisioning artifact. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_ProvisioningArtifactProperties.html) for valid list of values.
:Ó
p
servicecatalogProvisionedProductOutputDaws:servicecatalog/ProvisionedProductOutput:ProvisionedProductOutputz
x4
descriptionB" The description of the output.

keyB" The output key.
!
valueB" The output value.
:Ò
ù
servicecatalog'ProvisionedProductProvisioningParameterbaws:servicecatalog/ProvisionedProductProvisioningParameter:ProvisionedProductProvisioningParameterŒ
À
key" Parameter key.
ä
usePreviousValueB
 pWhether to ignore `value` and keep the previous parameter value. Ignored when initially provisioning a product.
 
valueB" Parameter value.
:à
ª
servicecatalog1ProvisionedProductStackSetProvisioningPreferencesvaws:servicecatalog/ProvisionedProductStackSetProvisioningPreferences:ProvisionedProductStackSetProvisioningPreferences«
ƒì
accountsB*" ˛One or more AWS accounts that will have access to the provisioned product. The AWS accounts specified should be within the list of accounts in the STACKSET constraint. To get the list of accounts in the STACKSET constraint, use the `aws_servicecatalog_provisioning_parameters` data source. If no values are specified, the default value is all accounts from the STACKSET constraint.
´
failureToleranceCountB ãNumber of accounts, per region, for which this operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions. You must specify either `failure_tolerance_count` or `failure_tolerance_percentage`, but not both. The default value is 0 if no value is specified.
é
failureTolerancePercentageB ÈPercentage of accounts, per region, for which this stack operation can fail before AWS Service Catalog stops the operation in that region. If the operation is stopped in a region, AWS Service Catalog doesn't attempt the operation in any subsequent regions. When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number. You must specify either `failure_tolerance_count` or `failure_tolerance_percentage`, but not both.
™
maxConcurrencyCountB åMaximum number of accounts in which to perform this operation at one time. This is dependent on the value of `failure_tolerance_count`. `max_concurrency_count` is at most one more than the `failure_tolerance_count`. Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling. You must specify either `max_concurrency_count` or `max_concurrency_percentage`, but not both.
≤
maxConcurrencyPercentageB èMaximum percentage of accounts in which to perform this operation at one time. When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number. This is true except in cases where rounding down would result is zero. In this case, AWS Service Catalog sets the number as 1 instead. Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling. You must specify either `max_concurrency_count` or `max_concurrency_percentage`, but not both.
ä
regionsB*" ˆOne or more AWS Regions where the provisioned product will be available. The specified regions should be within the list of regions from the STACKSET constraint. To get the list of regions in the STACKSET constraint, use the `aws_servicecatalog_provisioning_parameters` data source. If no values are specified, the default value is all regions from the STACKSET constraint.
:”
m
servicecatalogServiceActionDefinitionBaws:servicecatalog/ServiceActionDefinition:ServiceActionDefinition·
ﬁŸ

assumeRoleB" ƒARN of the role that performs the self-service actions on your behalf. For example, `arn:aws:iam::12345678910:role/ActionRole`. To reuse the provisioned product launch role, set to `LAUNCH_ROLE`.
§
name" óName of the SSM document. For example, `AWS-RestartEC2Instance`. If you are using a shared SSM document, you must provide the ARN instead of the name.
±

parametersB" úList of parameters in JSON format. For example: `[{\"Name\":\"InstanceId\",\"Type\":\"TARGET\"}]` or `[{\"Name\":\"InstanceId\",\"Type\":\"TEXT_VALUE\"}]`.
l
typeB" ^Service action definition type. Valid value is `SSM_AUTOMATION`. Default is `SSM_AUTOMATION`.
7
version" (SSM document version. For example, `1`.
:´
g
servicecataloggetLaunchPathsSummary>aws:servicecatalog/getLaunchPathsSummary:getLaunchPathsSummaryø
ºç
constraintSummaries£*†:ù
ö
servicecatalog&getLaunchPathsSummaryConstraintSummary`aws:servicecatalog/getLaunchPathsSummaryConstraintSummary:getLaunchPathsSummaryConstraintSummaryPBlock for constraints on the portfolio-product relationship. See details below.
B
name" 6Name of the portfolio to which the path was assigned.
.
pathId"  Identifier of the product path.
6
tags2" (Tags associated with this product path.
:¿
ö
servicecatalog&getLaunchPathsSummaryConstraintSummary`aws:servicecatalog/getLaunchPathsSummaryConstraintSummary:getLaunchPathsSummaryConstraintSummary†
ù2
description" Description of the constraint.
g
type" [Type of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `STACKSET`, and `TEMPLATE`.
:ﬁ

servicecataloggetPortfolioConstraintsDetailNaws:servicecatalog/getPortfolioConstraintsDetail:getPortfolioConstraintsDetail⁄
◊2
constraintId" Identifier of the constraint.
2
description" Description of the constraint.

owner" P
portfolioId" =Portfolio identifier.

The following arguments are optional:
%
	productId" Product identifier.
g
type" [Type of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `STACKSET`, and `TEMPLATE`.
:¨
æ
servicecatalog2getProvisioningArtifactsProvisioningArtifactDetailxaws:servicecatalog/getProvisioningArtifactsProvisioningArtifactDetail:getProvisioningArtifactsProvisioningArtifactDetailË
Â?
active
 1Indicates whether the product version is active.
<
createdTime" )The UTC time stamp of the creation time.
A
description" .The description of the provisioning artifact.
Å
guidance" qInformation set by the administrator to provide guidance to end users about which provisioning artifacts to use.
7
id" -The identifier of the provisioning artifact.
3
name" 'The name of the provisioning artifact.
/
type" #The type of provisioning artifact.
:Ú
\
servicediscoveryServiceDnsConfig6aws:servicediscovery/ServiceDnsConfig:ServiceDnsConfigë
é¸

dnsRecords}*{:y
w
servicediscoveryServiceDnsConfigDnsRecordHaws:servicediscovery/ServiceDnsConfigDnsRecord:ServiceDnsConfigDnsRecordoAn array that contains one DnsRecord object for each resource record set. See `dns_records` Block for details.
I
namespaceId" 6The ID of the namespace to use for DNS configuration.
¡
routingPolicyB" ©The routing policy that you want to apply to all records that Route 53 creates when you register an instance and specify the service. Valid Values: MULTIVALUE, WEIGHTED
:õ
w
servicediscoveryServiceDnsConfigDnsRecordHaws:servicediscovery/ServiceDnsConfigDnsRecord:ServiceDnsConfigDnsRecordü
ú{
ttl pThe amount of time, in seconds, that you want DNS resolvers to cache the settings for this resource record set.
ú
type" èThe type of the resource, which indicates the value that Amazon Route 53 returns in response to DNS queries. Valid Values: A, AAAA, SRV, CNAME
:’
t
servicediscoveryServiceHealthCheckConfigFaws:servicediscovery/ServiceHealthCheckConfig:ServiceHealthCheckConfig‹
ŸX
failureThresholdB >The number of consecutive health checks. Maximum value of 10.
—
resourcePathB" ∫The path that you want Route 53 to request when performing health checks. Route 53 automatically adds the DNS name for the service. If you don't specify a value, the default value is /.
®
typeB" ôThe type of health check that you want to create, which indicates how Route 53 determines whether an endpoint is healthy. Valid Values: HTTP, HTTPS, TCP
:»
Ü
servicediscoveryServiceHealthCheckCustomConfigRaws:servicediscovery/ServiceHealthCheckCustomConfig:ServiceHealthCheckCustomConfigº
π∂
failureThresholdB õThe number of 30-second intervals that you want service discovery to wait before it changes the health status of a service instance.  Maximum value of 10.
:˝
e
servicediscoverygetServiceDnsConfig<aws:servicediscovery/getServiceDnsConfig:getServiceDnsConfigì
êâ

dnsRecordsâ*Ü:É
Ä
servicediscoverygetServiceDnsConfigDnsRecordNaws:servicediscovery/getServiceDnsConfigDnsRecord:getServiceDnsConfigDnsRecordoAn array that contains one DnsRecord object for each resource record set. See `dns_records` Block for details.
D
namespaceId" 1ID of the namespace that the service belongs to.
ª
routingPolicy" •Routing policy that you want to apply to all records that Route 53 creates when you register an instance and specify the service. Valid Values: MULTIVALUE, WEIGHTED
:´
Ä
servicediscoverygetServiceDnsConfigDnsRecordNaws:servicediscovery/getServiceDnsConfigDnsRecord:getServiceDnsConfigDnsRecord•
¢w
ttl lAmount of time, in seconds, that you want DNS resolvers to cache the settings for this resource record set.
¶
type" ôThe type of health check that you want to create, which indicates how Route 53 determines whether an endpoint is healthy. Valid Values: HTTP, HTTPS, TCP
:≥
}
servicediscoverygetServiceHealthCheckConfigLaws:servicediscovery/getServiceHealthCheckConfig:getServiceHealthCheckConfig±
Æ¥
failureThreshold õThe number of 30-second intervals that you want service discovery to wait before it changes the health status of a service instance.  Maximum value of 10.
À
resourcePath" ∂Path that you want Route 53 to request when performing health checks. Route 53 automatically adds the DNS name for the service. If you don't specify a value, the default value is /.
¶
type" ôThe type of health check that you want to create, which indicates how Route 53 determines whether an endpoint is healthy. Valid Values: HTTP, HTTPS, TCP
:œ
è
servicediscovery!getServiceHealthCheckCustomConfigXaws:servicediscovery/getServiceHealthCheckCustomConfig:getServiceHealthCheckCustomConfig∫
∑¥
failureThreshold õThe number of 30-second intervals that you want service discovery to wait before it changes the health status of a service instance.  Maximum value of 10.
:•
k
servicequotasServiceQuotaUsageMetricAaws:servicequotas/ServiceQuotaUsageMetric:ServiceQuotaUsageMetricµ
≤“
metricDimensions§B°*û:õ
ò
servicequotas&ServiceQuotaUsageMetricMetricDimension_aws:servicequotas/ServiceQuotaUsageMetricMetricDimension:ServiceQuotaUsageMetricMetricDimensionThe metric dimensions.
,

metricNameB" The name of the metric.
6
metricNamespaceB" The namespace of the metric.
u
metricStatisticRecommendationB" NThe metric statistic that AWS recommend you use when determining quota usage.
:ﬂ
ò
servicequotas&ServiceQuotaUsageMetricMetricDimension_aws:servicequotas/ServiceQuotaUsageMetricMetricDimension:ServiceQuotaUsageMetricMetricDimensionB
@
classB" 
resourceB" 
serviceB" 
typeB" :Æ
t
servicequotasgetServiceQuotaUsageMetricGaws:servicequotas/getServiceQuotaUsageMetric:getServiceQuotaUsageMetricµ
≤ÿ
metricDimensions™*ß:§
°
servicequotas)getServiceQuotaUsageMetricMetricDimensioneaws:servicequotas/getServiceQuotaUsageMetricMetricDimension:getServiceQuotaUsageMetricMetricDimensionThe metric dimensions.
*

metricName" The name of the metric.
4
metricNamespace" The namespace of the metric.
s
metricStatisticRecommendation" NThe metric statistic that AWS recommend you use when determining quota usage.
:‡
°
servicequotas)getServiceQuotaUsageMetricMetricDimensioneaws:servicequotas/getServiceQuotaUsageMetricMetricDimension:getServiceQuotaUsageMetricMetricDimension:
8
class" 
resource" 
service" 

type" :Â
b
servicequotasgetTemplatesTemplate;aws:servicequotas/getTemplatesTemplate:getTemplatesTemplate˛
˚:
globalQuota
 'Indicates whether the quota is global.
#
	quotaCode" Quota identifier.

	quotaName" Quota name.
=
region" /AWS Region to which the quota increases apply.
2
serviceCode" (Required) Service identifier.
!
serviceName" Service name.
!
unit" Unit of measurement.
@
value 3(Required) The new, increased value for the quota.
:˛
o
sesConfigurationSetDeliveryOptionsGaws:ses/ConfigurationSetDeliveryOptions:ConfigurationSetDeliveryOptionsä
áÑ
	tlsPolicyB" Whether messages that use the configuration set are required to use Transport Layer Security (TLS). If the value is `Require`, messages are only delivered if a TLS connection can be established. If the value is `Optional`, messages can be delivered in plain text if a TLS connection can't be established. Valid values: `Require` or `Optional`. Defaults to `Optional`.
:¸
o
sesConfigurationSetTrackingOptionsGaws:ses/ConfigurationSetTrackingOptions:ConfigurationSetTrackingOptionsà
ÖÇ
customRedirectDomainB" dCustom subdomain that is used to redirect email recipients to the Amazon SES event tracking domain.
:ﬂ
Å
ses%EventDestinationCloudwatchDestinationSaws:ses/EventDestinationCloudwatchDestination:EventDestinationCloudwatchDestinationÿ
’4
defaultValue"  The default value for the event
0
dimensionName" The name for the dimension
k
valueSource" XThe source for the value. May be any of `"messageTag"`, `"emailHeader"` or `"linkTag"`.
:à
x
ses"EventDestinationKinesisDestinationMaws:ses/EventDestinationKinesisDestination:EventDestinationKinesisDestinationã
àU
roleArn" FThe ARN of the role that has permissions to access the Kinesis Stream
/
	streamArn" The ARN of the Kinesis Stream
:ù
l
sesEventDestinationSnsDestinationEaws:ses/EventDestinationSnsDestination:EventDestinationSnsDestination-
+)
topicArn" The ARN of the SNS topic
:è
`
sesReceiptRuleAddHeaderAction=aws:ses/ReceiptRuleAddHeaderAction:ReceiptRuleAddHeaderAction™
ß0

headerName" The name of the header to add
2
headerValue" The value of the header to add
?
position /The position of the action in the receipt rule
:ú
W
sesReceiptRuleBounceAction7aws:ses/ReceiptRuleBounceAction:ReceiptRuleBounceAction¿
Ω#
message" The message to send
?
position /The position of the action in the receipt rule
.
sender"  The email address of the sender
2
smtpReplyCode" The RFC 5321 SMTP reply code
;

statusCodeB" 'The RFC 3463 SMTP enhanced status code
4
topicArnB" "The ARN of an SNS topic to notify
:À
W
sesReceiptRuleLambdaAction7aws:ses/ReceiptRuleLambdaAction:ReceiptRuleLambdaActionÔ
Ï<
functionArn" )The ARN of the Lambda function to invoke
5
invocationTypeB" `Event` or `RequestResponse`
?
position /The position of the action in the receipt rule
4
topicArnB" "The ARN of an SNS topic to notify
:Œ
K
sesReceiptRuleS3Action/aws:ses/ReceiptRuleS3Action:ReceiptRuleS3Action˛
˚,

bucketName" The name of the S3 bucket
Ï

iamRoleArnB" ◊The ARN of the IAM role to be used by Amazon Simple Email Service while writing to the Amazon S3 bucket, optionally encrypting your mail via the provided customer managed key, and publishing to the Amazon SNS topic
*
	kmsKeyArnB" The ARN of the KMS key
9
objectKeyPrefixB"  The key prefix of the S3 bucket
?
position /The position of the action in the receipt rule
4
topicArnB" "The ARN of an SNS topic to notify
:ø
N
sesReceiptRuleSnsAction1aws:ses/ReceiptRuleSnsAction:ReceiptRuleSnsActionÏ
Èr
encodingB" `The encoding to use for the email within the Amazon SNS notification. Default value is `UTF-8`.
?
position /The position of the action in the receipt rule
2
topicArn" "The ARN of an SNS topic to notify
:õ
Q
sesReceiptRuleStopAction3aws:ses/ReceiptRuleStopAction:ReceiptRuleStopAction≈
¬?
position /The position of the action in the receipt rule
I
scope" <The scope to apply. The only acceptable value is `RuleSet`.
4
topicArnB" "The ARN of an SNS topic to notify
:ö
]
sesReceiptRuleWorkmailAction;aws:ses/ReceiptRuleWorkmailAction:ReceiptRuleWorkmailAction∏
µ<
organizationArn" %The ARN of the WorkMail organization
?
position /The position of the action in the receipt rule
4
topicArnB" "The ARN of an SNS topic to notify
