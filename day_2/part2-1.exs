#! /usr/bin/env elixir

defmodule Day2 do
    def letter_frequencies(id) do
        String.graphemes(id)
        |> Enum.reduce(
            %{},
            fn (char, acc) ->
                Map.update(acc, char, 1, fn(count) -> count + 1 end)
            end)
    end

    def count_pairs_and_triples(freq_map) do
        Enum.reduce(
            freq_map,
            {0,0},
            fn (freq_map, {pairs, triples}) ->
                {pairs + contains_freq(freq_map, 2), triples + contains_freq(freq_map, 3)}
            end)
    end

    defp contains_freq(freq_map, freq) do
        case Map.values(freq_map) |> Enum.count(&(&1 == freq)) do
            0 -> 0
            _ -> 1
        end
    end

    def checksum({p, t}) do p * t end
end

IO.stream(:stdio, :line)
|> Stream.map(&String.trim/1)
|> Stream.map(&Day2.letter_frequencies/1)
|> Day2.count_pairs_and_triples
|> Day2.checksum
|> IO.inspect
