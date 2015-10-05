;;;; package.lisp

(defpackage #:invertedindex
  (:use #:cl)
  (:export
   :add-file
   :occurrence
   :search-word
   :repl))
