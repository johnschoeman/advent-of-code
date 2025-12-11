import { promises as fs } from "node:fs"
import { resolve } from "node:path"
import { Context, Effect } from "effect"

class FileError extends Error {
  readonly _tag = "FileError"
}

export interface InputProvider {
  readonly get: (filepath: string) => Effect.Effect<never, FileError, string[]>
}

export const InputProvider = Context.Tag<InputProvider>()

const readFile = (
  filepath: string,
): Effect.Effect<never, FileError, string> => {
  const url = resolve(filepath)
  return Effect.tryPromise(() => fs.readFile(url, "utf-8")).pipe(
    Effect.mapError(
      cause =>
        new FileError(`Error when reading file: ${filepath}, cause: ${cause}`),
    ),
  )
}

export const InputProviderLive = InputProvider.of({
  get: (filepath: string) => {
    return readFile(filepath).pipe(
      Effect.map(fileContents => fileContents.trimEnd().split("\n")),
    )
  },
})
