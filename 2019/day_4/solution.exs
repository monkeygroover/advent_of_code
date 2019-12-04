#!/usr/bin/env elixir
split_digits = fn x -> x |> Integer.to_charlist() |> Enum.chunk_every(1) end

is_monotonic? = fn digits ->
  digits |> Enum.chunk_every(2, 1, :discard)
  |> Enum.all?(fn [a, b] -> a <= b end)
end

run_lengths = fn digits ->
  Enum.chunk_by(digits, &(&1))
  |> Enum.map(fn digits_group -> length(digits_group) end)
end

run_lengths = 138307..654504
|> Enum.map(split_digits)
|> Enum.filter(fn x -> is_monotonic?.(x) end)
|> Enum.map(run_lengths)

run_lengths |> Enum.filter(fn x -> Enum.any?(x, &(&1 >= 2)) end) |> Enum.count() |> IO.inspect()
run_lengths |> Enum.filter(fn x -> Enum.any?(x, &(&1 == 2)) end) |> Enum.count() |> IO.inspect()
