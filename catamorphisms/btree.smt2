;; cons(a, nil) |-> forall x,y,z:int. (1, (ite (= y 0) x (ite (>= x z) x z)))
;; a |-> (w, x)     l |-> (y,z)

;; Every tree is reduced to 4 integers
;; (w, x, y, z)
;; w node value
;; x is the constuctor cons
;; y are current values sorted?
(
 ( "treeOfInt"
   ( "nodetreeOfInt"
	 ( (v lw lx ly rw rx ry)
	   v
	   1
	   (ite (and  (> v lw) (<= v rw)) 1 0)
	   )
	 )
   ( "leaftreeOfInt"
	 ( ()
	   0
	   0
	   1
	   )
	 )
   )

 ( "foobar"
   ( "foo"
	 ( (a b)
	   1
	   )
	 )
   ( "bar"
	 ( ()
	   0
	   )
	 )
   )
 )
