#! /usr/bin/env elixir

letter_frequencies = fn(id) ->
    Enum.reduce(
        String.graphemes(id),
        %{},
        fn (char, acc) ->
            Map.update(acc, char, 1, fn(count) -> count + 1 end)
        end)
end

contains_freq = fn(freq_map, freq) ->
    case Map.values(freq_map) |> Enum.count(&(&1 == freq)) do
        0 -> 0
        _ -> 1
    end
end

pairs_and_triples_reducer = fn(freq_map, {pairs, triples}) ->
    {pairs + contains_freq.(freq_map, 2), triples + contains_freq.(freq_map, 3)}
end

product = fn {p,t} -> p * t end

IO.stream(:stdio, :line)
|> Stream.map(&String.trim/1)
|> Stream.map(letter_frequencies)
|> Enum.reduce({0,0}, pairs_and_triples_reducer)
|> product.()
|> IO.inspect
