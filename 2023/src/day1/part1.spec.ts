import { expect, test } from "vitest"
import { Effect } from "effect"
import { trebuchet, program } from "./part1"
import { InputProvider } from "../inputProvider"

// Part 1

test("trebuchet", () => {
  const test1 = "1abc2"
  const test2 = "pqr3stu8vwx"
  const test3 = "a1b2c3d4e5f"
  const test4 = "treb7uchet"

  const ans1 = 12
  const ans2 = 38
  const ans3 = 15
  const ans4 = 77

  expect(trebuchet(test1)).toBe(ans1)
  expect(trebuchet(test2)).toBe(ans2)
  expect(trebuchet(test3)).toBe(ans3)
  expect(trebuchet(test4)).toBe(ans4)
})

test("program", async () => {
  const runnable = Effect.provideService(
    program,
    InputProvider,
    InputProvider.of({
      get: () =>
        Effect.succeed(["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]),
    }),
  )

  const expected = 142
  expect(Effect.runSync(runnable)).toBe(expected)
})
