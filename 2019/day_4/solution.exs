#!/usr/bin/env elixir
split_digits = fn x -> x |> Integer.to_charlist() |> Enum.chunk_every(1) end

is_monotonic? = fn x -> x == Enum.sort(x) end

run_lengths = fn digits ->
  Enum.chunk_by(digits, &(&1))
  |> Enum.map(fn digits_group -> length(digits_group) end)
end

all_run_lengths = 138307..654504
|> Enum.map(split_digits)
|> Enum.filter(is_monotonic?)
|> Enum.map(run_lengths)

all_run_lengths |> Enum.filter(fn x -> Enum.any?(x, &(&1 >= 2)) end) |> Enum.count() |> IO.inspect()
all_run_lengths |> Enum.filter(fn x -> Enum.any?(x, &(&1 == 2)) end) |> Enum.count() |> IO.inspect()
