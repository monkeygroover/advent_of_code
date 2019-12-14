defmodule Day14 do

  def split_rule(rule) do
    [amount, thing] = String.split(rule, " ")
    {amount, thing}
  end

  def main(_args) do
    IO.stream(:stdio, :line)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.split(&1, "=>"))
    |> Enum.reduce(%{}, fn (raw_vals, acc) ->
      [x, y] =
        raw_vals
        |> Enum.map(&String.trim/1)
        |> IO.inspect

      to = y |> split_rule

      from = x
      |> String.split(",")
      |> Enum.map(&String.trim/1)
      |> Enum.map(&split_rule/1)

      Map.put(acc, to, from)
    end
    )
    |> IO.inspect()
  end



end
