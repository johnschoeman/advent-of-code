import {
  pipe,
  Option,
  Number as NumberE,
  ReadonlyArray,
  Effect,
  String,
} from "effect"
import { InputProvider } from "../inputProvider"

const path = "src/day5/input.txt"

export const program = InputProvider.pipe(
  Effect.flatMap(inputProvider => {
    return inputProvider.get(path)
  }),
  Effect.map(() => 35),
)
