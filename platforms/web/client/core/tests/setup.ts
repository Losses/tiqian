// Test isolation setup: worker-channel.js installs its bridge at module
// top level and reads GlobalServices immediately, so any test process that
// value-imports an engine module must initialize the container first. This
// module is preloaded by `node --import ./tests/setup.js` (see the `test`
// script) and runs before every test file, hence before its engine imports
// (ESM evaluates imports before each module body).
import { initializeGlobalServices } from "../src/services/global-services.js";
initializeGlobalServices();
