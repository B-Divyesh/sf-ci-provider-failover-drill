import test from "node:test";
import assert from "node:assert/strict";
import { ready } from "../src/check.js";

test("accepts a release version", () => assert.equal(ready("1.4.0"), true));
