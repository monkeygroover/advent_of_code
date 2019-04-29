#!/usr/bin/env lein exec

(print (time (reduce + (map read-string (line-seq (java.io.BufferedReader. *in*))))))
