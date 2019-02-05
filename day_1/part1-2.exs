#! /usr/bin/env elixir

# reduce the stream of frequencies into a set of frequencies seen before
# halt on a duplicate and return it
reduce_until_duplicate = fn x, {acc_freq, acc_map} ->
    IO.inspect x
    acc_freq = acc_freq + x
    if MapSet.member?(acc_map, acc_freq) do
        {:halt, acc_freq}
    else
        {:cont, {acc_freq, MapSet.put(acc_map, acc_freq)}}
    end
end

IO.stream(:stdio, :line)
|> Enum.map(&String.trim/1)
|> Enum.map(&String.to_integer/1)
|> Stream.cycle
|> Enum.reduce_while(
    {0, MapSet.new()},
    reduce_until_duplicate
    )
|> IO.inspect
