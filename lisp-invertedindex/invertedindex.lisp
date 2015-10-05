;;;; invertedindex.lisp

(in-package #:invertedindex)

(defclass invertedindex ()
  ((table :accessor table :initform (make-hash-table :test 'equal))
   (files-parsed :accessor files-parsed :initform nil)))

(defun search-word (word)
  (gethash word (table *MAP*) nil))

(defclass occurrence ()
  ((filename :accessor filename :initarg :filename)
   (line :accessor line :initarg :line)
   (column :accessor column :initarg :column)))

(defparameter *MAP* (make-instance 'invertedindex))

(defun add-file (filename)
  (when (member filename (files-parsed *MAP*) :test-not #'string=)
    (with-open-file (file filename)
      (loop with line-number = 1
         for line = (read-line file nil 'eof) until (eq line 'eof) do
           (cl-ppcre:do-matches (column end "\\w+" line)
             (let* ((word (subseq line column end))
                    (result (make-instance 'occurrence
                                           :filename filename
                                           :line line-number
                                           :column column))
                    (hash (table *MAP*))
                    (occurrences (search-word word)))
               (setf (gethash word hash) (cons result occurrences))
               (setf (table *MAP*) hash)))
           (incf line-number))
      (setf (files-parsed *MAP*) (cons filename (files-parsed *MAP*))))))

(defun read-file-name ()
  (format t "> Give the name of the file~%")
  (string-trim "\r\n" (read-line)))

(defun read-word ()
  (format t "> Search for ...~%")
  (string-trim "\r\n" (read-line)))

(defun print-results (occurrences)
  (dolist (occurrence occurrences)
    (with-slots (filename line column) occurrence
      (format t "~s at line ~d and column ~d~%" filename line column))))

(defun repl ()
  (let ((summary "a> Parse file~%s> Search word~%q> Quit~%"))
    (format t summary)
    (loop for answer = (read-line) until (string= answer "q") do
         (cond ((string= answer "a") (add-file (read-file-name)))
               ((string= answer "s") (print-results (search-word (read-word)))))
         (format t summary))))
