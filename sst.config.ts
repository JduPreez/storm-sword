/// <reference path="./.sst/platform/config.d.ts" />

export default $config({
  app(input) {
    return {
      name: "storm-sword",
      removal: input?.stage === "production" ? "retain" : "remove",
      protect: ["production"].includes(input?.stage),
      home: "aws",
    };
  },
  async run() {
    // Remember: We only have to define fields we want to index on.
    // Field ref is a 'namespaced' type & location for indexing:
    //  type + addressCountry + addressStateProvince.
    // Field id is a unique CUID.
    const eventsTable = new sst.aws.Dynamo("Events", {
      fields: {
        ns: "string",
        id: "string",  
        startDate: "number",
        endDate: "number",
        distanceMin: "number",
        distanceMax: "number",
      },
      primaryIndex: { hashKey: "ns", rangeKey: "id" },
      globalIndexes: {
        StartDateIndex: { hashKey: "startDate" },
        EndDateIndex: { hashKey: "endDate" },
        DistanceMinIndex: { hashKey: "distanceMin" },
        DistanceMaxIndex: { hashKey: "distanceMax" },
      }
    });

    // TODO: https://sst.dev/docs/examples/#aws-lamda-rust-multiple-binaries

    // Private Events Service Lambda - no HTTP endpoint
    const eventsService = new sst.aws.Function("EventsService", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/events/target/lambda/events",
      link: [eventsTable],
      dev: false,
      environment: {
        EVENTS_TABLE_NAME: eventsTable.name,
        RUST_LOG: "info",
      }
    });

    // Public API Gateway
    const api = new sst.aws.ApiGatewayV2("PublicApi");

    api.route("GET /events", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/public-api/target/lambda/public-api",
      dev: false,
      permissions: [
        {
          actions: ["lambda:InvokeFunction"],
          resources: [eventsService.arn],
        }
      ],
      environment: {
        EVENTS_LAMBDA_ARN: eventsService.arn,
        RUST_LOG: "info",
      }
    });

    return {
      api: api.url,
      eventsService: eventsService.name,
    };
  },
});
