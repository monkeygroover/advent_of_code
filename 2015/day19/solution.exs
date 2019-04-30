#! /usr/bin/env elixir

replacements = %{
  :Al => [[:Th, :F], [:Th, :Rn, :F, :Ar]],
  :B => [[:B, :Ca], [:Ti, :B], [:Ti, :Rn, :F, :Ar]],
  :Ca => [[:Ca, :Ca], [:P, :B], [:P, :Rn, :F, :Ar], [:Si, :Rn, :F, :Y, :F, :Ar], [:Si, :Rn, :Mg, :Ar], [:Si, :Th]],
  :F => [[:Ca, :F], [:P, :Mg], [:Si, :Al]],
  :H => [[:C, :Rn, :Al, :Ar], [:C, :Rn, :F, :Y, :F, :Y, :F, :Ar], [:C, :Rn, :F, :Y, :Mg, :Ar], [:C, :Rn, :Mg, :Y, :F, :Ar], [:H, :Ca],
         [:N, :Rn, :F, :Y, :F, :Ar], [:N, :Rn, :Mg, :Ar], [:N, :Th], [:O, :B], [:O, :Rn, :F, :Ar]],
  :Mg => [[:B, :F], [:Ti, :Mg]],
  :N => [[:C, :Rn, :F, :Ar], [:H, :Si]],
  :O => [[:C, :Rn, :F, :Y, :F, :Ar], [:C, :Rn, :Mg, :Ar], [:H, :P], [:N, :Rn, :F, :Ar], [:O, :Ti]],
  :P => [[:Ca, :P], [:P, :Ti], [:Si, :Rn, :F, :Ar]],
  :Si => [[:Ca, :Si]],
  :Th => [[:Th, :Ca]],
  :Ti => [[:B, :P], [:Ti, :Ti]],
  :e => [[:H, :F], [:N, :Al], [:O, :Mg]],
}

input = [:C, :Rn, :Ca, :Ca, :Ca, :Si, :Rn, :B, :P, :Ti, :Mg, :Ar, :Si, :Rn, :Si, :Rn, :Mg, :Ar, :Si, :Rn, :Ca, :F, :Ar, :Ti, :Ti, :B, :Si, :Th, :F, :Y, :Ca, :F, :Ar, :Ca, :Ca, :Si, :Th, :Ca, :P, :B, :Si, :Th, :Si, :Th, :Ca, :Ca, :P, :Ti, :Rn, :P, :B, :Si, :Th, :Rn, :F, :Ar, :Ar, :Ca, :Ca, :Si, :Th, :Ca, :Si, :Th, :Si, :Rn, :Mg, :Ar, :Ca, :P, :Ti, :B, :P, :Rn, :F, :Ar, :Si, :Th, :Ca, :Si, :Rn, :F, :Ar, :B, :Ca, :Si, :Rn, :Ca, :P, :Rn, :F, :Ar, :P, :Mg, :Y, :Ca, :F, :Ar, :Ca, :P, :Ti, :Ti, :Ti, :B, :P, :B, :Si, :Th, :Ca, :P, :Ti, :B, :P, :B, :Si, :Rn, :F, :Ar, :B, :P, :B, :Si, :Rn, :Ca, :F, :Ar, :B, :P, :Rn, :Si, :Rn, :F, :Ar, :Rn, :Si, :Rn, :B, :F, :Ar, :Ca, :F, :Ar, :Ca, :Ca, :Ca, :Si, :Th, :Si, :Th, :Ca, :Ca, :P, :B, :P, :Ti, :Ti, :Rn, :F, :Ar, :Ca, :P, :Ti, :B, :Si, :Al, :Ar, :P, :B, :Ca, :Ca, :Ca, :Ca, :Ca, :Si, :Rn, :Mg, :Ar, :Ca, :Si, :Th, :F, :Ar, :Th, :Ca, :Si, :Th, :Ca, :Si, :Rn, :Ca, :F, :Y, :Ca, :Si, :Rn, :F, :Y, :F, :Ar, :F, :Ar, :Ca, :Si, :Rn, :F, :Y, :F, :Ar, :Ca, :Si, :Rn, :B, :P, :Mg, :Ar, :Si, :Th, :P, :Rn, :F, :Ar, :Ca, :Si, :Rn, :F, :Ar, :Ti, :Rn, :Si, :Rn, :F, :Y, :F, :Ar, :Ca, :Si, :Rn, :B, :F, :Ar, :Ca, :Si, :Rn, :Ti, :Mg, :Ar, :Si, :Th, :Ca, :Si, :Th, :Ca, :F, :Ar, :P, :Rn, :F, :Ar, :Si, :Rn, :F, :Ar, :Ti, :Ti, :Ti, :Ti, :B, :Ca, :Ca, :Si, :Rn, :Ca, :Ca, :F, :Y, :F, :Ar, :Si, :Th, :Ca, :P, :Ti, :B, :P, :Ti, :B, :Ca, :Si, :Th, :Si, :Rn, :Mg, :Ar, :Ca, :F]

result = Enum.reduce(0..(length(input)-1), MapSet.new(),
  fn(x, acc) ->
    value = Enum.at(input, x)
    if Map.has_key?(replacements, value) do
      replacements_list = replacements[value];
      Enum.reduce(replacements_list, acc,
      fn(repl, acc2) ->
        MapSet.put(acc2,
        List.flatten(List.replace_at(input, x, repl))) end)
    else
      acc
    end
  end
);

IO.inspect(MapSet.size(result))
