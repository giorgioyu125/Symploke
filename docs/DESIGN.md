(defun nth-element (list n)
  (cond ( (null list) () )
        ( (= n 1) (car list) )
        ( t  (nth-element (cdr list) (1- n))))) ;; Default case

(defun second-element (list)
  (car (cdr list)))

(* 2 (+ 1 2) (second-element '(4 5))

1. Si prende il primo elemento della lista puntata
2. Si vede nella lista delle funzioni quanti argomenti ha
3. Si appendono a quell'elemento n elementi successivi ad esso
4. Si ripete dal primo finche' tutto non e' ordinato e come elementi dell'albero rimangono solo atomi

           *
2          +           car
         1   2         cdr           =>     30
                     '(4 5)

;; S: X, if X is an atom
;;    (S1 . S2)
;;
;; Atom: a collection of letters of the alpabeth excluded the SEPARATOR (.)
;; Primitives Functions: These are transformations between various S-exp
;; Primitive Set: The set of all primitives functions (car cdr cons eval null atom quote defun...)
;;
;; Space = .
