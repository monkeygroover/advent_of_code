defmodule Day7 do
  def main([]) do
    
    bags_map = IO.stream(:stdio, :line)
    |> Enum.map(&String.trim/1)
    |> Enum.reduce(%{}, fn line, acc -> 
      [bag_name, contains] = String.split(line, " bags contain ")
      inner_bags = parse_inner(contains)
      Map.put(acc, bag_name, inner_bags)
    end)
    
    bags_containing_gold = bags_map
    |> Enum.filter(fn {bag_name, _} -> bag_contains_bag(bags_map, bag_name, "shiny gold") end)
    |> Enum.count

    bags_inside_gold = count_bags(bags_map, "shiny gold")

    IO.puts("#{bags_containing_gold - 1} #{bags_inside_gold - 1}")

  end
 
  defp parse_inner("no other bags."), do: []
  defp parse_inner(inner_string) do
    inner_bags = String.split(inner_string, ",")
    |> Enum.map(fn part ->
      [count, bag] = Regex.run(~r/(\d+) (.*) bags?/, part, capture: :all_but_first)
      {count |> String.to_integer, bag}
    end)
    inner_bags
  end

  defp bag_contains_bag(_bags_map, bag_name, bag_name), do: true
  defp bag_contains_bag(bags_map, bag_name, bag_to_find) do
    case bags_map[bag_name] do
      [] -> false
      contained_bags -> Enum.any?(contained_bags, fn {_, bag_name} -> bag_contains_bag(bags_map, bag_name, bag_to_find) end)
    end
  end

  defp count_bags(bags_map, bag_name) do
    inner_bags = bags_map[bag_name]
    1 + Enum.reduce(inner_bags, 0, fn {count, name}, acc -> acc + count * count_bags(bags_map, name) end)
  end

end
