import { expect, test } from "vitest"
import { Effect } from "effect"
import { gameValue, program } from "./part2"
import { InputProvider } from "../inputProvider"

// Part 1

test("gameValue", () => {
  const q1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
  const q2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
  const q3 =
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
  const q4 =
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
  const q5 = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"

  const a1 = 48
  const a2 = 12
  const a3 = 1560
  const a4 = 630
  const a5 = 36

  expect(gameValue(q1)).toBe(a1)
  expect(gameValue(q2)).toBe(a2)
  expect(gameValue(q3)).toBe(a3)
  expect(gameValue(q4)).toBe(a4)
  expect(gameValue(q5)).toBe(a5)
})

test("program", async () => {
  const runnable = Effect.provideService(
    program,
    InputProvider,
    InputProvider.of({
      get: () =>
        Effect.succeed([
          "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
          "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
          "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
          "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
          "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]),
    }),
  )

  const expected = 2286
  expect(Effect.runSync(runnable)).toBe(expected)
})
