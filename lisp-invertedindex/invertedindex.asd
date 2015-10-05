;;;; invertedIndex.asd

(asdf:defsystem #:invertedindex
  :description "Describe invertedIndex here"
  :author "Humberto Pinheiro"
  :license "Specify license here"
  :serial t
  :components ((:file "package")
               (:file "invertedindex"))
  :depends-on (#:cl-ppcre))
