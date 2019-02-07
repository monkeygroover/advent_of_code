#!/usr/bin/env lein exec
(require '[clojure.string :as str])

(def cycle-values (cycle (map read-string (str/split-lines (slurp *in*)))))
(print (reduce #(if (contains? %1 %2) (reduced %2) (conj %1 %2)) #{} (reductions + cycle-values)))
