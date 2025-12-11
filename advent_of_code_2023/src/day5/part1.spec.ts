import { expect, test } from "vitest"
import { Effect } from "effect"
import { parseSeeds, groupMaps, program } from "./part1"
import { InputProvider } from "../inputProvider"

// Part 1

test("parseSeeds", () => {
  const input = "seeds: 1 2 3"

  const result = parseSeeds(input)

  const expected = [1, 2, 3]

  expect(result).toEqual(expected)
})

test("groupMaps", () => {
  const input = [
    "seeds: 1",
    "",
    "seed-to-soil map:",
    "1 2 3",
    "",
    "soil-to-fertilizer map:",
    "4 5 6",
    "7 8 9",
    "",
    "fertilizer-to-water map:",
    "10 11 12",
    "13 14 15",
    "16 17 18",
    "",
    "water-to-light map:",
    "19 20 21",
    "",
    "light-to-temperature map:",
    "22 23 24",
    "25 26 27",
    "",
    "temperature-to-humidity map:",
    "28 29 30",
    "",
    "humidity-to-location map:",
    "31 32 33",
  ]

  const result = groupMaps(input)

  const expected = [
    ["1 2 3"],
    ["4 5 6", "7 8 9"],
    ["10 11 12", "13 14 15", "16 17 18"],
    ["19 20 21"],
    ["22 23 24", "25 26 27"],
    ["28 29 30"],
    ["31 32 33"],
  ]

  expect(result).toEqual(expected)
})

test("program", async () => {
  const runnable = Effect.provideService(
    program,
    InputProvider,
    InputProvider.of({
      get: () =>
        Effect.succeed([
          "seeds: 79 14 55 13",
          "",
          "seed-to-soil map:",
          "50 98 2",
          "52 50 48",
          "",
          "soil-to-fertilizer map:",
          "0 15 37",
          "37 52 2",
          "39 0 15",
          "",
          "fertilizer-to-water map:",
          "49 53 8",
          "0 11 42",
          "42 0 7",
          "57 7 4",
          "",
          "water-to-light map:",
          "88 18 7",
          "18 25 70",
          "",
          "light-to-temperature map:",
          "45 77 23",
          "81 45 19",
          "68 64 13",
          "",
          "temperature-to-humidity map:",
          "0 69 1",
          "1 0 69",
          "",
          "humidity-to-location map:",
          "60 56 37",
          "56 93 4",
        ]),
    }),
  )

  const expected = 35
  expect(Effect.runSync(runnable)).toBe(expected)
})
