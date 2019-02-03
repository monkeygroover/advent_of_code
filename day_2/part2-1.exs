#! /usr/bin/env elixir

defmodule Day2 do
    def letter_frequencies(id) do
        Enum.reduce(
            String.graphemes(id),
            %{},
            fn (char, acc) ->
                Map.update(acc, char, 1, fn(count) -> count + 1 end)
            end)
    end

    def pairs_and_triples_reducer(freq_map, {pairs, triples}) do
        {pairs + contains_freq(freq_map, 2), triples + contains_freq(freq_map, 3)}
    end

    defp contains_freq(freq_map, freq) do
        case Map.values(freq_map) |> Enum.count(&(&1 == freq)) do
            0 -> 0
            _ -> 1
        end
    end

end

IO.stream(:stdio, :line)
|> Stream.map(&String.trim/1)
|> Stream.map(&Day2.letter_frequencies/1)
|> Enum.reduce({0,0}, &Day2.pairs_and_triples_reducer/2)
|> (fn {p,t} -> p * t end).()
|> IO.inspect
