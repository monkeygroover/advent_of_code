#!/usr/bin/env lein exec
(require '[clojure.string :as str])

(def freq-seq (reductions + (cycle (map read-string (str/split-lines (slurp *in*))))))

(defn reduce-until-duplicate [freq-stream] (reduce #(if (contains? %1 %2) (reduced %2) (conj %1 %2)) #{} freq-stream))

(print (reduce-until-duplicate freq-seq))
