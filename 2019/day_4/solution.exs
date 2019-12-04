#! /usr/bin/env elixir

split_digits_into_chunks = fn x ->
  x |> Integer.to_charlist()
  |> Enum.chunk_every(1)
  |> Enum.chunk_every(2, 1, :discard)
end

has_repeat? = fn list_of_chunks ->
  Enum.any?(list_of_chunks, fn [a, b] -> a == b end)
end

is_monotonic? = fn list_of_chunks ->
  Enum.all?(list_of_chunks, fn [a, b] -> a <= b end)
end

138307..654504
|> Enum.map(split_digits_into_chunks)
|> Enum.filter(fn x -> has_repeat?.(x) && is_monotonic?.(x) end)
|> Enum.count()
|> IO.inspect()
