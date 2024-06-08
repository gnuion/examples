/*instrumentation.ts*/
import * as opentelemetry from "@opentelemetry/sdk-node";
import { getNodeAutoInstrumentations } from "@opentelemetry/auto-instrumentations-node";
import {
  NestInstrumentation,
  Instrumentation,
} from "@opentelemetry/instrumentation-nestjs-core";
import { HttpInstrumentation } from "@opentelemetry/instrumentation-http";
import { ExpressInstrumentation } from "@opentelemetry/instrumentation-express";
import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-grpc";
import { OTLPMetricExporter } from "@opentelemetry/exporter-metrics-otlp-grpc";
import { PeriodicExportingMetricReader } from "@opentelemetry/sdk-metrics";
import { Resource } from "@opentelemetry/resources";
import { SEMRESATTRS_SERVICE_NAME } from "@opentelemetry/semantic-conventions";

// Define the resource with the service name
const sdk = new opentelemetry.NodeSDK({
  resource: new Resource({
    [SEMRESATTRS_SERVICE_NAME]: "nestjs-app", // replace with your service name
  }),
  traceExporter: new OTLPTraceExporter({
    // optional - default url is http://localhost:4318/v1/traces
    // url: '<your-otlp-endpoint>/v1/traces',
    // optional - collection of custom headers to be sent with each request, empty by default
    headers: {},
  }),
  metricReader: new PeriodicExportingMetricReader({
    exporter: new OTLPMetricExporter({
      // url: '<your-otlp-endpoint>/v1/metrics', // url is optional and can be omitted - default is http://localhost:4318/v1/metrics
      headers: {}, // an optional object containing custom headers to be sent with each request
    }),
  }),
  instrumentations: [
    // getNodeAutoInstrumentations(),
    new HttpInstrumentation(),
    new NestInstrumentation(),
    new ExpressInstrumentation(),
  ],
});
sdk.start();
