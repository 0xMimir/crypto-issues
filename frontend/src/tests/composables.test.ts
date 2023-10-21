import { formatDate } from "@/composables/time";
import { test, expect } from "vitest";

test("composables", () => {
    const date = new Date("Sat Oct 21 2023 18:54:29 GMT+0200 (Central European Summer Time)");
    expect(formatDate(date), "21/8/2023")
})