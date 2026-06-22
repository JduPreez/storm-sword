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
    const dev = false;
    const eventsTable = new sst.aws.Dynamo("Events", {
      fields: {
        Ns: "string", // event_type~~country_code
        Id: "string", // CUID
        StartDate: "number",
        EndDate: "number",
        DistanceMin: "number",
        DistanceMax: "number",
        Date: "number",
        AddressAdministrativeAreaIdx: "string", // event_type~~country_code~~administrative_area
      },
      primaryIndex: { hashKey: "Ns", rangeKey: "Id" },
      globalIndexes: {
        StartDateIndex: { hashKey: "Ns", rangeKey: "StartDate" },
        DateIndex: { hashKey: "Ns", rangeKey: "Date" },
        EndDateIndex: { hashKey: "Ns", rangeKey: "EndDate" },
        DistanceMinIndex: { hashKey: "Ns", rangeKey: "DistanceMin" },
        DistanceMaxIndex: { hashKey: "Ns", rangeKey: "DistanceMax" },
        AddressAdministrativeAreaIndex: { hashKey: "AddressAdministrativeAreaIdx", rangeKey: "Date" },
      }
    });

    // TODO (maybe): https://sst.dev/docs/examples/#aws-lamda-rust-multiple-binaries

    // Private Events Service Lambda - no HTTP endpoint
    const eventsService = new sst.aws.Function("EventsService", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/events/target/lambda/events",
      link: [eventsTable],
      dev,
      environment: {
        EVENTS_TABLE_NAME: eventsTable.name,
        RUST_LOG: "info",
      }
    });

    // Public API Gateway
    const api = new sst.aws.ApiGatewayV2("PublicApi");

    api.route("GET /health", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/public-api/target/lambda/public-api",
      dev,
      environment: { RUST_LOG: "info" },
    });

    api.route("GET /events", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/public-api/target/lambda/public-api",
      dev,
      permissions: [{ actions: ["lambda:InvokeFunction"], resources: [eventsService.arn] }],
      environment: {
        EVENTS_LAMBDA_ARN: eventsService.arn,
        RUST_LOG: "info",
      },
    });

    api.route("POST /events", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/public-api/target/lambda/public-api",
      dev,
      permissions: [{ actions: ["lambda:InvokeFunction"], resources: [eventsService.arn] }],
      environment: {
        EVENTS_LAMBDA_ARN: eventsService.arn,
        RUST_LOG: "info",
      },
    });

    return {
      api: api.url,
      eventsService: eventsService.name,
    };
  },
});
