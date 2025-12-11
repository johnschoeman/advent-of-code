import { Effect, pipe } from "effect"
import { program as part1Program } from "./part1"
import { program as part2Program } from "./part2"
import { InputProvider, InputProviderLive } from "../inputProvider"

const program = pipe(
  Effect.log("Day 2"),
  Effect.flatMap(() => {
    return part1Program.pipe(
      Effect.provideService(InputProvider, InputProviderLive),
      Effect.flatMap(result => Effect.log(`Part 1: ${result}`)),
    )
  }),
  Effect.flatMap(() => {
    return part2Program.pipe(
      Effect.provideService(InputProvider, InputProviderLive),
      Effect.flatMap(result => Effect.log(`Part 2: ${result}`)),
    )
  }),
)

void Effect.runPromise(program)
