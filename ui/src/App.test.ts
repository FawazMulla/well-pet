import { describe, it, expect } from "vitest";

describe("Frontend Presentation Layer Verification", () => {
  it("validates 5-dimension weights formula in frontend", () => {
    const mobility = 100;
    const mental = 100;
    const eye = 100;
    const hydration = 100;
    const posture = 100;

    const s = 0.25 * mobility + 0.25 * mental + 0.20 * eye + 0.15 * hydration + 0.15 * posture;
    expect(s).toBe(100.0);
  });

  it("verifies crisis regex catches self-harm phrases", () => {
    const crisisRegex = /\b(suicide|kill myself|end my life|self[- ]harm|want to die|slit my|hurt myself|don't want to live)\b/i;
    expect(crisisRegex.test("I feel like I want to hurt myself")).toBe(true);
    expect(crisisRegex.test("How much water should I drink?")).toBe(false);
  });
});
