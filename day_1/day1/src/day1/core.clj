(ns day1.core
  (:gen-class))

(defn freq-seq [in] (reductions + (cycle (map read-string (clojure.string/split-lines in)))))

(defn reduce-until-duplicate [freq-stream] (reduce #(if (contains? %1 %2) (reduced %2) (conj %1 %2)) #{} freq-stream))

(defn -main
  ""
  [& args]
  (println (reduce-until-duplicate (freq-seq (slurp *in*)))))
