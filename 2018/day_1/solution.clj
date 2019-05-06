#!/usr/bin/env lein exec
(require '[clojure.string :as str])

(def input (map read-string (str/split-lines (slurp *in*))))
(defn freq-seq [input] (reductions + (cycle input)))
(defn reduce-until-duplicate [freq-stream] (reduce #(if (contains? %1 %2) (reduced %2) (conj %1 %2)) #{} freq-stream))

(println (reduce + input))
(println (reduce-until-duplicate (freq-seq input)))
