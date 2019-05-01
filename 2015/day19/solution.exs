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

IO.inspect(length(input))

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



input2 = [:A, :Lp, :A, :A, :A, :A, :Lp, :A, :A, :A, :A, :Rp, :A, :Lp, :A, :Lp, :A, :Rp, :A, :Lp, :A, :A, :Rp, :A, :A, :A, :A, :A, :A, :_, :A, :A, :Rp, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :Lp, :A, :A, :A, :A, :Lp, :A, :Rp, :Rp, :A, :A, :A, :A, :A, :A, :A, :A, :Lp, :A, :Rp, :A, :A, :A, :A, :A, :Lp, :A, :Rp, :A, :A, :A, :A, :Lp, :A, :Rp, :A, :A, :A, :Lp, :A, :A, :Lp, :A, :Rp, :A, :A, :_, :A, :A, :Rp, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :Lp, :A, :Rp, :A, :A, :A, :A, :Lp, :A, :A, :Rp, :A, :A, :Lp, :A, :Lp, :A, :Rp, :Lp, :A, :Lp, :A, :A, :Rp, :A, :A, :Rp, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :Lp, :A, :Rp, :A, :A, :A, :A, :A, :A, :Rp, :A, :A, :A, :A, :A, :A, :A, :A, :Lp, :A, :Rp, :A, :A, :A, :A, :Rp, :A, :A, :A, :A, :A, :A, :Lp, :A, :A, :_, :A, :A, :Lp, :A, :_, :A, :Rp, :A, :Rp, :A, :A, :Lp, :A, :_, :A, :Rp, :A, :A, :Lp, :A, :A, :A, :Rp, :A, :A, :A, :Lp, :A, :Rp, :A, :A, :Lp, :A, :Rp, :A, :Lp, :A, :Lp, :A, :_, :A, :Rp, :A, :A, :Lp, :A, :A, :Rp, :A, :A, :Lp, :A, :A, :Rp, :A, :A, :A, :A, :A, :A, :A, :Rp, :A, :Lp, :A, :Rp, :A, :Lp, :A, :Rp, :A, :A, :A, :A, :A, :A, :A, :A, :Lp, :A, :A, :A, :_, :A, :Rp, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :A, :Lp, :A, :Rp, :A, :A]

IO.inspect(Enum.reduce(input2, MapSet.new(), fn(x, acc) -> MapSet.put(acc, x) end))


# part 2
# no need for code;
# observe that there are only two distinct forms of the production rule
# ones that end :Ar and the rest.
# splitting these into two groups we see, the :Ar ones have the shapes
# xRnxAr, xRnxYxAr or xRnxYxYxAr (where x is one of the "ordinary" atoms)
# ordinary being (Al|B|C|Ca|F|Mg|P|Si|Th|Ti) (i.e. they appear on the left hand side)
# effectively Rn Y and Ar can be considered punctuation, the Rn and Ar forming a parenthesis pair
# and Y as a separator, so let's substitute ( , and ) for them:
# C(CaCaCaSi(BPTiMg)Si(Si(Mg)Si(CaF)TiTiBSiThF,CaF)CaCaSiThCaPBSiThSiThCaCaPTi(PBSiTh(F))CaCaSiThCaSiThSi(Mg)CaPTiBP(F)SiThCaSi(F)BCaSi(CaP(F)PMg,CaF)CaPTiTiTiBPBSiThCaPTiBPBSi(F)BPBSi(CaF)BP(Si(F)(Si(BF)CaF)CaCaCaSiThSiThCaCaPBPTiTi(F)CaPTiBSiAl)PBCaCaCaCaCaSi(Mg)CaSiThF)ThCaSiThCaSi(CaF,CaSi(F,F)F)CaSi(F,F)CaSi(BPMg)SiThP(F)CaSi(F)Ti(Si(F,F)CaSi(BF)CaSi(TiMg)SiThCaSiThCaF)P(F)Si(F)TiTiTiTiBCaCaSi(CaCaF,F)SiThCaPTiBPTiBCaSiThSi(Mg)CaF
#
# analysis of the other production rules (the ones without :Ar) show they are ALL of the form
# x -> xx
# i.e. they take one "ordinary" atom and substitute two "ordinary" atoms
#
# so to work backwards from the end string to the original e, we jsut reverse this.
# each pair of "ordinary" atoms can (as they were originally produced from the same rules) be reversed back to a single "ordinary" atom
# also whenever we have x(x) or x(x,x) or x(x,x,x) this will in reverse become a single ordinary atom
# there are 295 ordinary atoms, 34 'bracket pairs' and 7 'commas' in the original input
# each xx -> x is one step and reduces it by 1 i.e. they count for one step
# x(x) -> x reduce by 3 x(x,x) -> x by 5 and x(x,x,x) -> x so by 7 in general we reduce by 3 for each bracket pair + 2 * number of commas
# so they reduce the number of steps by 2, 4, and 6 more than an "ordinary" reduction, you can just count 1 per bracket and 2 per comma
# to work this out
#
# so we have 295 elements, but we leave 1 at the end, so only 294 to reduce
# 294 - 34 - 34 - 2*7 = 212
