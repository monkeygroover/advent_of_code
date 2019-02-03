#! /usr/bin/env elixir

IO.stream(:stdio, :line)
|> Enum.map(&String.trim/1)
|> Enum.map(&String.to_integer/1)
|> Stream.cycle
|> Enum.reduce_while(
    {0, MapSet.new()},
    fn x, {acc_freq, acc_map} ->
        IO.inspect x #{acc_freq, x, acc_map}
        acc_freq = acc_freq + x
        if MapSet.member?(acc_map, acc_freq) do
            {:halt, acc_freq}
        else
            {:cont, {acc_freq, MapSet.put(acc_map, acc_freq)}}
        end
    end)
|> IO.inspect
