defmodule Solution do

  def main(_args) do

    g = IO.stream(:stdio, :line)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.split(&1, ")"))
    |> Enum.reduce(Graph.new(type: :directed),
      fn ([a, b], g) -> Graph.add_vertex(g, a) |> Graph.add_vertex(b) |> Graph.add_edge(a, b)
      end)

    root = Graph.arborescence_root(g)

    part1 = Elixir.Graph.Reducers.Dfs.reduce(g, 0,
      fn (v, acc) -> {:next,
        acc +
        case Graph.get_shortest_path(g, root, v) do
          nil -> 0
          points -> length(points) - 1
        end
      } end )

    IO.puts("part1: #{part1}")

    patha = Graph.get_shortest_path(g, root, "SAN")
    pathb = Graph.get_shortest_path(g, root, "YOU")
    intersection = patha -- (patha -- pathb)
    lub = Enum.at(intersection, -1)

    transfer = length(Graph.get_shortest_path(g, lub, "SAN")) +
    length(Graph.get_shortest_path(g, lub, "YOU")) - 4

    IO.puts("part2: #{transfer}")

  end

end
