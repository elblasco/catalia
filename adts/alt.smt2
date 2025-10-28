; alt([1]).
; alt(x::y::l) :- alt(y::l), x+y=0.
; even([]).
; evel(x::y::l) :- even(l).
; hd(l)<0 :- even(l), alt(l).

(set-logic HORN)

(declare-datatypes ((Lst 1)) (
    (par (T) (
      nil (cons (head T) (tail (Lst T)) (condition Bool)))
    )
))

(declare-fun alt ((Lst Int)) Bool)
(declare-fun even ((Lst Int)) Bool)

(assert (forall ((dummy Int)) (=> true (alt (cons 1 nil true)))))

(assert (forall ((x Int) (y Int) (l (Lst Int)))
  (=>
    (and (alt (cons y l true)) (= (+ x y) 0))
    (alt (cons x (cons y l true) true))
  )
))

(assert (forall ((dummy Int)) (=> true (even nil))))

(assert (forall ((x Int) (y Int) (l (Lst Int)))
  (=>
    (even l)
    (even (cons x (cons y l true) true))
  )
))

(assert (forall ((l (Lst Int)) (l2 (Lst Int)) (h Int))
  (=>
    (and (even l) (alt l) (= l (cons h l2 false)))
    (< h 0)
  )
))

(check-sat)

