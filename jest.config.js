const { createJsWithTsPreset } = require("ts-jest");

const tsJestTransformCfg = createJsWithTsPreset().transform;

/** @type {import("jest").Config} **/
module.exports = {
  testEnvironment: "node",
  transform: {
    ...tsJestTransformCfg,
  },
};